mod handlers;
mod models;

use axum::{Router, routing::get};
use handlers::{
    created_with_body, dashboard, greetings, health, html_page, json_created, json_user,
    maybe_response, no_content, plain_bytes, redirect_permanent, redirect_see_other,
    redirect_temporary, response_types, teapot_with_body,
};

#[tokio::main]
async fn main() {
    let port = std::env::var("AXUM_PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3003);

    let app = Router::new()
        .route("/", get(dashboard))
        .route("/greetings", get(greetings))
        .route("/page", get(html_page))
        .route("/user", get(json_user))
        .route("/created", get(created_with_body))
        .route("/teapot", get(teapot_with_body))
        .route("/no-content", get(no_content))
        .route("/old-page", get(redirect_see_other))
        .route("/moved", get(redirect_permanent))
        .route("/health", get(health))
        .route("/maybe", get(maybe_response))
        .route("/types", get(response_types))
        .route("/temporary", get(redirect_temporary))
        .route("/json-created", get(json_created))
        .route("/plain-bytes", get(plain_bytes));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|error| panic!("error biding the port: {error}"));

    println!("server listening on http://localhost:{port}");

    axum::serve(listener, app)
        .await
        .expect("server stopped unexpectedly");
}
