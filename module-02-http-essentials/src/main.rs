mod handlers;
mod models;
mod utils;

use axum::routing::{get, post};

use crate::handlers::{
    anatomy, async_basic, bad_request, closures, counter, created, curl_cheatsheet, dashboard,
    maintenance, maybe_fail, post_only, rate_limited, rest_map, serde_roundtrip, server_error,
    teapot, traits_generics,
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
        .route("/status/maintenance", get(maintenance))
        .route("/post-only", post(post_only))
        .route("/maybe-fail", get(maybe_fail))
        .route("/trait-generics", get(traits_generics))
        .route("/closures", get(closures))
        .route("/anatomy", get(anatomy))
        .route("/rest-map", get(rest_map))
        .route("/curl_cheatsheet", get(curl_cheatsheet));

    let port = requested_port();
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|error| panic!("couldn't bind port {port}: {error}"));

    println!("Listening on http://localhost:{port}");
    axum::serve(listener, app)
        .await
        .expect("server stopped unexpectedly");
}
