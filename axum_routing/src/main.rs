use axum::{Router, routing::get, routing::post};

async fn hello() -> &'static str {
    "hello"
}

async fn world() -> &'static str {
    "world"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/world", post(world));

    let sock = tokio::net::TcpListener::bind("0.0.0.0:9527").await.unwrap();

    println!("[*] runing serve on :9527");
    axum::serve(sock, app).await.unwrap();
}
