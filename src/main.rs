use anyhow::Result;
use axum::{
    extract::{MatchedPath, Query},
    handler::Handler,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use hyper::{header::HeaderName, Body};
use lazy_static::lazy_static;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use serde::{de, Deserialize, Deserializer};
use tokio::signal;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{DefaultMakeSpan, MakeSpan, TraceLayer},
};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{fmt, future::ready, net::SocketAddr, str::FromStr, time::Instant};

#[cfg(test)]
mod tests;

lazy_static! {
    static ref PROMETHEUS_HANDLE: PrometheusHandle =
        setup_metrics_recorder().expect("cannot setup metrics recorder");
}

#[derive(Deserialize)]
struct GenerateQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    words: Option<u8>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    separator: Option<String>,
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

async fn root(
    Query(GenerateQuery { words, separator }): Query<GenerateQuery>,
) -> impl IntoResponse {
    petname::petname(words.unwrap_or(2), separator.as_deref().unwrap_or("-"))
}

fn app() -> Router {
    let x_request_id = HeaderName::from_static("x-request-id");
    let prometheus_handle = PROMETHEUS_HANDLE.clone();
    Router::new()
        .route("/", get(root))
        .route("/metrics", get(move || ready(prometheus_handle.render())))
        .fallback(handler_404.into_service())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let default_span = DefaultMakeSpan::default().make_span(request);
                let requestid = match request
                    .extensions()
                    .get::<RequestId>()
                    .map(RequestId::header_value)
                {
                    Some(req_id) => req_id.to_str().unwrap_or(""),
                    None => {
                        error!("cannot extract request-id");
                        ""
                    }
                }
                .to_string();
                tracing::info_span!(parent: &default_span, "petnames", %requestid)
            }),
        )
        // PropagateRequestIdLayer must be befor SetRequestIdLayer
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(x_request_id, MakeRequestUuid))
        .route_layer(middleware::from_fn(track_metrics))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "petnames_fly=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8080));
    Server::bind(&addr)
        .serve(app().into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

fn setup_metrics_recorder() -> Result<PrometheusHandle> {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    Ok(PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )?
        .install_recorder()?)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "page not found")
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
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
