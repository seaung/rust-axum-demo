use axum::{Router, response::Json};
use axum::routing::{get, post};
use axum::http::{StatusCode};
use serde_json::{json, Value};


async fn response1() -> &'static str {
    "response text"
}

async fn response2() -> Json<Value> {
    Json(json!({"hello": "world", "lang": "rust", "version": 1.77}))
}

async fn response3() -> StatusCode {
    StatusCode::NOT_FOUND
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/r1", get(response1))
        .route("/r2", get(response2))
        .route("/r3", post(response3));

    let sock = tokio::net::TcpListener::bind("0.0.0.0:9527").await.unwrap();

    println!("[*] running server on 9527");
    axum::serve(sock, app).await.unwrap();
}
