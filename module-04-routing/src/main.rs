mod handlers;

use crate::handlers::{create_item, dashboard, list_items, read_item};
use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(dashboard))
        .route("/items", get(list_items))
        .route("/items/{item}", post(create_item).get(read_item));

    let port = std::env::var("AXUM_PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3004);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|error| panic!("not possible to bind the port {port}: {error}"));

    println!("Axum server listening on http://localhost:{port}");
    axum::serve(listener, app)
        .await
        .expect("server stopped unexpectedly");
}
