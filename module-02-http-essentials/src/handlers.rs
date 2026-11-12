use crate::models::RepoLookup;
use axum::http::StatusCode;

pub async fn dashboard() -> &'static str {
    "Rust and HTTP essentials (module 02)

GET  /async-basics      async fn, .await, Tokio sleep, elapsed time
GET  /serde-roundtrip   a struct deserialized from JSON and re-serialized
GET  /counter           Arc + Mutex shared across three tokio::spawn tasks
POST /only-post         HTTP methods; try GET on it and get a 405
GET  /maybe-fail        Result and the ? operator
GET  /status/created|bad-request|teapot|rate-limited|server-error|maintenance
GET  /traits-generics   traits, generics, impl Trait (what IntoResponse is made of)
GET  /closures          closures, move, FnMut — the shape of tokio::spawn
ANY  /anatomy           your own request taken apart (method, URL, headers, body)
GET  /rest-map          REST: resources, verbs, status codes
GET  /curl-cheatsheet   curl and httpie, flag by flag

Anatomy tour with curl: curl -si http://127.0.0.1:3002/async-basics
"
}

pub async fn async_basic() -> String {
    let now = std::time::Instant::now();
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    let elapsed = now.elapsed();
    format!("Request elapsed {elapsed:?} ms")
}

pub async fn serde_roundtrip() -> String {
    let incoming = r#"{"name":"tokio-rs/axum","stars":23000,"topics":["web","http","rust"]}"#;
    let mut repo: RepoLookup =
        serde_json::from_str(&incoming).expect("not possible to deserialize the string");
    repo.stars += 1;
    let outcoming = serde_json::to_string(&repo).expect("Not possible to serialize the value");
    format!("Incoming:    {incoming}\nOutcoming:   {outcoming}")
}

pub async fn counter() -> String {
    let counter = std::sync::Arc::new(std::sync::Mutex::new(0_u64));
    let workers: u64 = 3;
    let counts: u64 = 1000;
    let mut handles = Vec::new();
    for worker in 0..workers {
        let count = std::sync::Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            for _ in 0..counts {
                let mut guard = count.lock().expect("mutex was poisoned");
                *guard += 1;
            }
            worker
        }));
    }

    let mut finished = Vec::new();
    for handle in handles {
        finished.push(
            handle
                .await
                .expect("counter task did not panic")
                .to_string(),
        );
    }

    let total = *counter.lock().expect("mutex was still fine");

    format!(
        "{workers} workers x {counts} increments in one request = {total} (exactly 3000).\nWorkers finished in order: {finished:?}.\nArc = shared owner, Mutex = safety, spawn = concurrency.\n"
    )
}

pub async fn created() -> StatusCode {
    StatusCode::CREATED
}

pub async fn bad_request() -> StatusCode {
    StatusCode::BAD_REQUEST
}

pub async fn teapot() -> StatusCode {
    StatusCode::IM_A_TEAPOT
}

pub async fn rate_limited() -> StatusCode {
    StatusCode::TOO_MANY_REQUESTS
}

pub async fn server_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub async fn maintenance() -> StatusCode {
    StatusCode::SERVICE_UNAVAILABLE
}
