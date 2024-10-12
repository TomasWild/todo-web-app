use axum::{
    body::{self, Body},
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use tower::ServiceExt;
use todo_web_app::api::handler::health_check::health_check;

fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}

#[tokio::test]
async fn test_health_check() {
    let app = app();

    let request = Request::builder()
        .method("GET")
        .uri("/health_check")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(body_bytes.into_iter().collect()).unwrap();

    assert_eq!(body, "OK");
}