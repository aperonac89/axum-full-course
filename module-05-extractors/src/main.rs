mod handlers;
mod lesson1;
mod lesson2;
mod lesson3;
mod lesson4;
mod lesson5;
mod lesson6;
mod lesson7;
mod lesson8;
mod lesson9;

use crate::{
    lesson1::lesson1_routes, lesson2::lesson2_routes, lesson3::lesson3_routes,
    lesson4::lesson4_routes, lesson5::lesson5_routes, lesson6::lesson6_routes,
    lesson7::lesson7_routes, lesson8::lesson8_routes, lesson9::lesson9_routes,
};
use axum::{Router, routing::get};
use std::net::SocketAddr;

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
        .merge(lesson4_routes())
        .merge(lesson5_routes())
        .merge(lesson6_routes())
        .merge(lesson7_routes())
        .merge(lesson8_routes())
        .nest("/custom", lesson9_routes());

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|error| panic!("error: not possible to bind port [{port}]: {error}"));

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("server stopped unexpectedly")
}
