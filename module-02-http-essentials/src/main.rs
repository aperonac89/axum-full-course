mod handlers;
mod models;
mod utils;

use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::handlers::{
    async_basic, bad_request, counter, created, dashboard, maintenance, rate_limited,
    serde_roundtrip, server_error, teapot,
};
use crate::utils::requested_port;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(dashboard))
        .route("/async-basic", get(async_basic))
        .route("/serde-roundtrip", get(serde_roundtrip))
        .route("/counter", get(counter))
        .route("/status/created", get(created))
        .route("/status/bad-request", get(bad_request))
        .route("/status/teapot", get(teapot))
        .route("/status/rate-limited", get(rate_limited))
        .route("/status/server-error", get(server_error))
        .route("/status/maintenance", get(maintenance));

    let port = requested_port();
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|error| panic!("couldn't bind port {port}: {error}"));

    println!("Listening on http://localhost:{port}");
    axum::serve(listener, app)
        .await
        .expect("server stopped unexpectedly");
}
