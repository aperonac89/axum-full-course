use axum::http::StatusCode;

pub fn requested_port() -> u16 {
    std::env::var("AXUM_PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(3002)
}

pub fn coin_flip(nanos: u32) -> Result<&'static str, StatusCode> {
    if nanos & 1 == 0 {
        Ok("heads")
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
