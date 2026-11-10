//! Module 01: Introduction and setup.
//! Topics: what Axum is, Tokio + Hyper + Tower, `axum::serve`, `#[tokio::main]`,
//! plain text and JSON responses, configuration from the environment, curl,
//! and auto-reload with `bacon`.
//! Run: cargo run -p module-01-intro   (listens on http://127.0.0.1:3001)
//!
//! ## Lesson Reading Guide
//!
//! Like every `src/main.rs` in this course, this file is a linear lesson:
//! read it from top to bottom, and each `LESSON` banner is one step. The doc
//! comment on every function says what it does and why it matters. Keep the
//! server running and try each endpoint with `curl` while you read — the
//! README lists the exact commands.
//!
//! Where Axum sits (one sentence per crate, because the whole stack matters):
//! - **Tokio** owns the async runtime: event loop, threads, timers, sockets.
//! - **Hyper** is the HTTP/1 and HTTP/2 engine: it parses bytes into requests
//!   and writes responses back.
//! - **Tower** contributes "layers": composable wrappers around services
//!   (timeouts, limits, logging). Axum is built on Tower services.
//! - **Axum** sits on top and gives you the pieces you will actually write:
//!   the `Router`, handlers as plain async functions, extractors for reading
//!   requests, and `IntoResponse` for writing them.
use axum::{Router, response::Json, routing::get};
use serde::Serialize;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize)]
struct HelloJson {
    message: String,
    course: String,
    module: u8,
}

#[derive(Serialize)]
struct AboutMe {
    name: String,
    age: u8,
}

#[tokio::main]
async fn main() {
    let port = requested_port();
    let address = format!("127.0.0.1:{port}");

    let app = Router::new()
        .route("/", get(root))
        .route("/text", get(text))
        .route("/json_hello", get(json_hello))
        .route("/port", get(port_from_env))
        .route("/now", get(now))
        .route("/bye", get(bye))
        .route("/about", get(about_me));

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|error| panic!("could not bind {address}: {error}"));

    // Printed hints double as the "Try It" section of the README.
    println!("Module 01 (Introduction and setup) listening on http://{address}");
    println!("Try it with curl:");
    println!("  curl -s http://{address}/");
    println!("  curl -s http://{address}/json");
    println!("  curl -si http://{address}/text");

    axum::serve(listener, app)
        .await
        .expect("server stopped unexpectedly");
}

async fn root() -> &'static str {
    "Hello to the axum full course"
}

async fn text() -> &'static str {
    "This is an endpoint that return an string"
}

async fn json_hello() -> Json<HelloJson> {
    Json(HelloJson {
        message: "Hello from the struct".to_string(),
        course: "Axum Full Course".to_string(),
        module: 1,
    })
}

async fn port_from_env() -> String {
    match env::var("AXUM_PORT") {
        Ok(value) => format!("AXUM_PORT={value}"),
        Err(e) => format!("AXUM_PORT is not set in the environment variables: {e}"),
    }
}

async fn now() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("not possible to convert to epoch")
        .as_secs();

    format!("Timestamp: {now}")
}

async fn bye() -> String {
    "Bye bye from the course".to_string()
}

async fn about_me() -> Json<AboutMe> {
    Json(AboutMe {
        name: "Alejandro".to_string(),
        age: 37,
    })
}

fn requested_port() -> u16 {
    env::var("AXUM_PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3001)
}
