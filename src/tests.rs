use super::*;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::{Service, ServiceExt};

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
async fn metrics() -> Result<()> {
    let mut app = app();

    let response = app
        .call(Request::builder().uri("/metrics").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .call(Request::builder().uri("/metrics").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec()).unwrap();
    assert!(dbg!(body).contains(
        "# TYPE http_requests_total counter\nhttp_requests_total{method=\"GET\",path=\"/metrics\",status=\"200\"}"
    ));

    Ok(())
}
