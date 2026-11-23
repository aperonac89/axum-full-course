use axum::{
    Router,
    body::Bytes,
    extract::{ConnectInfo, MatchedPath},
    http::HeaderMap,
    response::Json,
    routing::{get, post},
};
use std::{collections::HashMap, net::SocketAddr};

pub fn lesson7_routes() -> Router {
    Router::new()
        .route("/headers", get(read_headers))
        .route("/echo-raw", post(echo_raw))
        .route("/meta/route", get(route_info))
        .route("/ip", get(client_ip))
}

async fn read_headers(headers: HeaderMap) -> Json<HashMap<String, String>> {
    let mut listing: HashMap<String, String> = HashMap::new();
    for (name, value) in headers.iter() {
        if let Ok(value) = value.to_str() {
            listing.insert(name.to_string(), value.to_string());
        }
    }

    Json(listing)
}

async fn echo_raw(body: Bytes) -> Bytes {
    body
}

async fn route_info(path: MatchedPath) -> String {
    format!("matched route pattern: {}\n", path.as_str())
}

async fn client_ip(ConnectInfo(info): ConnectInfo<SocketAddr>) -> String {
    format!(
        "client socket: {info}\n(behind a proxy this is the proxy - Module 32 covers headers)\n"
    )
}
