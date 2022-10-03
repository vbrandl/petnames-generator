use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use quickcheck_macros::quickcheck;
use tower::{Service, ServiceExt};

use super::*;

#[tokio::test]
async fn root() -> Result<()> {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn reject_negative() -> Result<()> {
    let mut app = app();

    let response = app
        .call(
            Request::builder()
                .uri("/?words_per_name=-2")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = app
        .call(
            Request::builder()
                .uri("/?number_of_names=-1")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn reject_zero() -> Result<()> {
    let mut app = app();

    let response = app
        .call(
            Request::builder()
                .uri("/?words_per_name=0")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = app
        .call(
            Request::builder()
                .uri("/?number_of_names=0")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn not_found() -> Result<()> {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/does-not-exist")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
async fn contains_request_id() -> Result<()> {
    let mut app = app();

    let response = app
        .call(Request::builder().uri("/").body(Body::empty())?)
        .await?;

    assert!(response.headers().contains_key("x-request-id"));

    let response = app
        .call(
            Request::builder()
                .uri("/does-not-exist")
                .body(Body::empty())?,
        )
        .await?;

    assert!(response.headers().contains_key("x-request-id"));

    Ok(())
}

#[tokio::test]
async fn metrics_not_available_on_webserver() -> Result<()> {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/metrics").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
async fn metrics_server() -> Result<()> {
    let mut app = metrics_app();

    let response = app
        .call(Request::builder().uri("/metrics").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[quickcheck]
fn generated_names_are_unique(number_of_names: NonZeroU8) {
    let number_of_names = NonZeroUsize::from(number_of_names);
    assert_eq!(
        generate_names(
            HashSet::new(),
            statics::DEFAULT_WORDS_PER_NAME,
            statics::DEFAULT_SEPARATOR,
            number_of_names
        )
        .len(),
        number_of_names.get()
    );
}
