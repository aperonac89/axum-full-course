use crate::models::{Course, Describe, RepoLookup, RestRow, Student, Subject, describe_all, shout};
use crate::utils::coin_flip;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

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
    let counter = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let workers: u64 = 8;
    let counts: u64 = 5000;
    let mut handles = Vec::new();
    for worker in 0..workers {
        let count = std::sync::Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            for _ in 0..counts {
                //let mut guard = count.lock().expect("mutex was poisoned");
                //*guard += 1;
                count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
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

    // let total = *counter.lock().expect("mutex was still fine");
    let total = counter.load(std::sync::atomic::Ordering::SeqCst);

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

pub async fn gone() -> StatusCode {
    StatusCode::GONE
}

pub async fn post_only() -> String {
    "Thank you for your posting.".to_string()
}

pub async fn maybe_fail() -> Result<String, StatusCode> {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.subsec_micros())
        .unwrap_or(0);

    let veredict = coin_flip(nanos)?;
    Ok(format!(
        "Coin came up Ok: 200 with this body ({veredict}). Run again to get a 503.\n"
    ))
}

pub async fn traits_generics() -> String {
    let students = [Student { name: "Asha" }, Student { name: "Ravi" }];
    let courses = [Course {
        title: "Axum",
        modules: 44,
    }];

    let mixed: Vec<Box<dyn Describe>> = vec![
        Box::new(Student { name: "Meera" }),
        Box::new(Course {
            title: "Rust",
            modules: 30,
        }),
        Box::new(Subject {
            title: "Mathematics",
            hours_per_week: 8,
        }),
    ];

    format!(
        "generic over Student: {:?}\ngeneric over Course:  {:?}\nimpl Display arg:      {}\ntrait objects:         {:?}\n",
        describe_all(&students),
        describe_all(&courses),
        shout("impl trait"),
        mixed.iter().map(|item| item.describe()).collect::<Vec<_>>(),
    )
}

pub async fn closures() -> String {
    // Fn
    let prefix = String::from("AXUM");
    let label = |name: &str| format!("{prefix}-{name}");

    // FnMut
    let mut calls = 0;
    let mut counter = || {
        calls += 1;
        calls
    };
    let first = counter();
    let second = counter();

    // FnOnce
    let owned = String::from("owned by the task");
    let handle = tokio::spawn(async move { owned.len() });
    let owned_len = handle.await.expect("task did panic");

    format!(
        "Fn:     {}\nFnMut:  counter gave {first} then {second}\nmove:   spawned task measured {owned_len} bytes it owned\n",
        label("closures"),
    )
}

pub async fn anatomy(request: Request) -> Response {
    let (parts, body) = request.into_parts();

    // Body bytes counted with a hard cap so a huge upload cannot exhaust memory
    // (Module 09 teaches DefaultBodyLimit properly).
    let bytes = match axum::body::to_bytes(body, 64 * 1024).await {
        Ok(bytes) => bytes,
        Err(_) => {
            return (StatusCode::PAYLOAD_TOO_LARGE, "body larger than 64 KiB\n").into_response();
        }
    };

    let mut report = String::new();
    report.push_str(&format!("method:   {}\n", parts.method));
    report.push_str(&format!("path:     {}\n", parts.uri.path()));
    report.push_str(&format!(
        "query:    {}\n",
        parts.uri.query().unwrap_or("(none)")
    ));
    report.push_str(&format!("version:  {:?}\n", parts.version));
    report.push_str("headers:\n");
    for (name, value) in &parts.headers {
        report.push_str(&format!(
            "  {name}: {}\n",
            value.to_str().unwrap_or("<binary>")
        ));
    }
    report.push_str(&format!("body:     {} bytes\n", bytes.len()));
    if !bytes.is_empty() {
        report.push_str(&format!(
            "          {:?}\n",
            String::from_utf8_lossy(&bytes)
        ));
    }
    report.into_response()
}

pub async fn rest_map() -> axum::Json<Vec<RestRow>> {
    axum::Json(vec![
        RestRow {
            method: "GET",
            path: "/books",
            meaning: "list books",
            success: 200,
            idempotent: true,
            safe: true,
        },
        RestRow {
            method: "GET",
            path: "/books/7",
            meaning: "read one book",
            success: 200,
            idempotent: true,
            safe: true,
        },
        RestRow {
            method: "POST",
            path: "/books",
            meaning: "create a book",
            success: 201,
            idempotent: false,
            safe: false,
        },
        RestRow {
            method: "PUT",
            path: "/books/7",
            meaning: "replace book 7",
            success: 200,
            idempotent: true,
            safe: false,
        },
        RestRow {
            method: "PATCH",
            path: "/books/7",
            meaning: "change some fields",
            success: 200,
            idempotent: false,
            safe: false,
        },
        RestRow {
            method: "DELETE",
            path: "/books/7",
            meaning: "remove book 7",
            success: 204,
            idempotent: true,
            safe: false,
        },
    ])
}

pub async fn curl_cheatsheet() -> &'static str {
    "curl flag        meaning                               httpie twin
-s               silent (no progress bar)              (default)
-i               include response headers              --print=hb (default)
-v               verbose: request AND response         -v
-X POST          choose the method                     http POST url
-H 'k: v'        add a header                          k:v
-d 'a=1'         send a body (implies POST)            a=1 (form) / a:=1 (json)
--json '{...}'   JSON body + content-type headers      http url key=value
-o file          save the body to a file               --download
-L               follow redirects                      --follow
"
}
