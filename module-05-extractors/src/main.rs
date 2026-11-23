mod handlers;
mod lesson1;
mod lesson2;
mod lesson3;
mod lesson4;

use crate::{
    lesson1::lesson1_routes, lesson2::lesson2_routes, lesson3::lesson3_routes,
    lesson4::lesson4_routes,
};
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let port = std::env::var("AXUM_PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3005);

    let app = Router::new()
        .route("/", get(handlers::dashboard))
        .merge(lesson1_routes())
        .merge(lesson2_routes())
        .merge(lesson3_routes())
        .merge(lesson4_routes());

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|error| panic!("error: not possible to bind port [{port}]: {error}"));

    axum::serve(listener, app)
        .await
        .expect("server stopped unexpectedly")
}
