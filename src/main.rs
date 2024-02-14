use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
