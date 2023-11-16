use anyhow::Result;
use axum::{
    body::Body,
    extract::{MatchedPath, Path, Query},
    headers::HeaderName,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router, Server, TypedHeader,
};
use headers::{ContentType, Expires};
use petname::Petnames;
use serde::{de, Deserialize, Deserializer};
use tokio::signal;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{DefaultMakeSpan, MakeSpan, TraceLayer},
};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{
    collections::HashSet,
    fmt,
    future::ready,
    iter::repeat_with,
    net::SocketAddr,
    num::{NonZeroU8, NonZeroUsize},
    str::FromStr,
    time::{Duration, Instant, SystemTime},
};

use crate::{
    statics::{DEFAULT_NUMBER_OF_NAMES, DEFAULT_SEPARATOR, DEFAULT_WORDS_PER_NAME},
    templates::statics::StaticFile,
};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

mod metric;
#[macro_use]
mod render;
mod statics;

#[cfg(test)]
mod tests;

#[derive(Deserialize, Default)]
pub struct GenerateQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    words_per_name: Option<NonZeroU8>,
    // #[serde(default, deserialize_with = "empty_string_as_none")]
    separator: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    number_of_names: Option<NonZeroU8>,
}

// handle empty strings in query as `None`
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

fn capitalize_first_letters(input: &str) -> String {
    input
        .chars()
        .fold(
            (String::with_capacity(input.len()), true),
            |(mut builder, first), next| {
                builder.push(if first {
                    next.to_ascii_uppercase()
                } else {
                    next
                });
                (builder, false)
            },
        )
        .0
}

/// Generate random usernames. Since `petnames::iter_non_repeating` does currently not work as
/// expected, the names are collected into a `HashSet` to get unique names. The function calls
/// itself recursively until the set contains exactly `number_of_names` names.
pub(crate) fn generate_names(
    mut set: HashSet<String>,
    words_per_name: NonZeroU8,
    separator: &str,
    number_of_names: NonZeroUsize,
) -> HashSet<String> {
    // exit condition for recursion: `number_of_names` unique names were generated
    if set.len() >= number_of_names.get() {
        set
    } else {
        // first generate names with a non-empty separator so we can split and capitalize the first letters
        static TEMP_SEPARATOR: &str = "-";
        let petnames = Petnames::medium();
        let mut rng = rand::thread_rng();
        set.extend(
            repeat_with(|| {
                petnames
                    .generate(&mut rng, words_per_name.get(), TEMP_SEPARATOR)
                    .split(TEMP_SEPARATOR)
                    .map(capitalize_first_letters)
                    .collect::<Vec<_>>()
                    // use the user-supplied separator to join the capitalized parts of the username
                    .join(separator)
            })
            // only generate the missing amount of names to get exactly `number_of_names` entries
            .take(number_of_names.get() - set.len()),
        );
        // recurse
        generate_names(set, words_per_name, separator, number_of_names)
    }

    // TODO: wait for https://github.com/allenap/rust-petname/issues/61 to be fixed
    // TODO: the solution above might return non-unique names (https://github.com/vbrandl/petnames-generator/issues/1)
    // let names: Vec<_> = petnames
    //     .iter_non_repeating(&mut rng, words_per_name, TEMP_SEPARATOR)
    //     .map(|name| {
    //         name.split(DEFAULT_SEPARATOR)
    //             .map(capitalize_first_letters)
    //             .collect::<Vec<_>>()
    //              // use the user-supplied separator to join the capitalized parts of the username
    //             .join(separator)
    //     })
    //     .take(number_of_names)
    //     .collect();
}

async fn root(query: Option<Query<GenerateQuery>>) -> Response {
    if let Some(Query(
        ref query @ GenerateQuery {
            words_per_name,
            ref separator,
            number_of_names,
        },
    )) = query
    {
        let words_per_name = words_per_name.unwrap_or(DEFAULT_WORDS_PER_NAME);
        let separator = separator.as_deref().unwrap_or(DEFAULT_SEPARATOR);
        let number_of_names = number_of_names.map_or(DEFAULT_NUMBER_OF_NAMES, NonZeroUsize::from);
        let names = generate_names(HashSet::new(), words_per_name, separator, number_of_names);

        render!(
            templates::index_html,
            &(names.into_iter().collect::<Vec<_>>()),
            query,
            statics::VERSION_INFO
        )
        .into_response()
    } else {
        handler_400(
            "You performed an invalid requests, most likely, by passing a negative number or zero.",
        )
        .await
        .into_response()
    }
}

async fn static_files(Path(filename): Path<String>) -> impl IntoResponse {
    /// A duration to add to current time for a far expires header.
    static FAR: Duration = Duration::from_secs(180 * 24 * 60 * 60);
    match StaticFile::get(&filename) {
        Some(data) => {
            let far_expires = SystemTime::now() + FAR;
            (
                TypedHeader(ContentType::from(data.mime.clone())),
                TypedHeader(Expires::from(far_expires)),
                data.content,
            )
                .into_response()
        }
        None => handler_404().await.into_response(),
    }
}

#[allow(clippy::unused_async)]
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        render!(templates::not_found_html, statics::VERSION_INFO),
    )
}

#[allow(clippy::unused_async)]
async fn handler_400(message: &str) -> impl IntoResponse + '_ {
    (
        StatusCode::BAD_REQUEST,
        render!(templates::bad_request_html, message, statics::VERSION_INFO),
    )
}

async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}

fn metrics_app() -> Router {
    let prometheus_handle = metric::PROMETHEUS_HANDLE.clone();
    Router::new()
        .route("/metrics", get(move || ready(prometheus_handle.render())))
        .layer(TraceLayer::new_for_http())
}

fn app() -> Router {
    static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");
    Router::new()
        .route("/", get(root))
        .route("/static/:filename", get(static_files))
        .fallback(handler_404)
        .layer(
            TraceLayer::new_for_http()
                // add request-id to trace span
                .make_span_with(|request: &Request<Body>| {
                    let default_span = DefaultMakeSpan::default().make_span(request);
                    let requestid = if let Some(req_id) = request
                        .extensions()
                        .get::<RequestId>()
                        .map(RequestId::header_value)
                    {
                        req_id.to_str().unwrap_or("")
                    } else {
                        error!("cannot extract request-id");
                        ""
                    }
                    .to_string();
                    tracing::info_span!(parent: &default_span, "petnames", %requestid)
                }),
        )
        // PropagateRequestIdLayer must be befor SetRequestIdLayer
        .layer(PropagateRequestIdLayer::new(X_REQUEST_ID.clone()))
        .layer(SetRequestIdLayer::new(
            X_REQUEST_ID.clone(),
            MakeRequestUuid,
        ))
        .route_layer(middleware::from_fn(track_metrics))
}

async fn start_webserver() -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8080));
    Ok(Server::bind(&addr)
        .serve(app().into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?)
}

async fn start_metrics_server() -> Result<()> {
    let metrics_addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3000));
    Ok(Server::bind(&metrics_addr)
        .serve(metrics_app().into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "petnames_generator=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (webserver, metrics_server) = tokio::join!(start_webserver(), start_metrics_server());

    // start_webserver().await?;

    // consume the results
    webserver?;
    metrics_server?;

    Ok(())
}
