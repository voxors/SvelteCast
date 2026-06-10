use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello from Rust!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
