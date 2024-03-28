use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // 创建路由app实例，添加/hello路由处理函数
    let app = Router::new().route("/hello", get(|| async { "hello axum" }));

    // 监听9527端口
    let sock = tokio::net::TcpListener::bind("0.0.0.0:9527").await.unwrap();

    // 开启http服务
    axum::serve(sock, app).await.unwrap();
}
