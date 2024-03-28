use axum::{Router, routing::get, routing::post};
use axum::{http::StatusCode, http::HeaderMap, http::HeaderName, http::HeaderValue};

async fn handler1() -> &'static str {
    "I am handler1 function"
}

async fn handler2() -> StatusCode {
    StatusCode::OK
}

async fn handler3() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("x-request-id"), HeaderValue::from_static("123"));
    (headers, "custom headers")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/h1", get(handler1))
        .route("/h2", get(handler2))
        .route("/h3", post(handler3));

    let sock = tokio::net::TcpListener::bind("0.0.0.0:9527").await.unwrap();

    println!("[*] running serve on 9527");
    axum::serve(sock, app).await.unwrap();
}
