use axum::{
    Router,
    extract::{
        Extension, NestedPath, OriginalUri, Path, Query, RawForm, RawPathParams, RawQuery, Request,
    },
    http::{Method, Uri, Version},
    response::Json,
    routing::{get, post},
};
use axum_extra::{TypedHeader, headers::UserAgent};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct RepoRef {
    org: String,
    repo_name: String,
}

#[derive(Clone)]
struct BuildInfo {
    version: &'static str,
    module: u8,
}

#[derive(Clone)]
struct NeverAttached;

pub fn lesson8_routes() -> Router {
    Router::new()
        .route("/all-params", get(get_params_map))
        .route("/repo-struct", get(repo_struct))
        .route("/raw-params", get(raw_params))
        .route("/query-map", get(query_map))
        .route("/raw-query", get(raw_query))
        .route("/uri-parts", get(uri_parts))
        .nest("/inner", Router::new().route("/where", get(nested_where)))
        .route("/extension", get(extension_demo))
        .route("/typed-header", get(typed_header_demo))
        .route("/raw-form", post(raw_form))
        .route("/whole-request", post(whole_request))
        .layer(Extension(BuildInfo {
            version: "0.1.0",
            module: 5,
        }))
}

async fn get_params_map(
    Path(params): Path<HashMap<String, String>>,
) -> Json<HashMap<String, String>> {
    Json(params)
}
async fn repo_struct(Path(repo): Path<RepoRef>) -> String {
    format!("org  = {}\nrepo = {}", repo.org, repo.repo_name)
}
async fn raw_params(params: RawPathParams) -> String {
    let mut out = String::from("raw path params:\n");
    for (name, value) in &params {
        out.push_str(&format!("  - {name} = {value}\n"))
    }
    out
}
async fn query_map(Query(params): Query<HashMap<String, String>>) -> Json<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (name, value) in params.iter() {
        map.insert(name.to_string(), value.to_string());
    }
    Json(map)
}
async fn raw_query(RawQuery(query): RawQuery) -> String {
    match query {
        Some(text) => format!("raw query string: {text}\n"),
        None => "no raw query string\n".to_string(),
    }
}
async fn uri_parts(method: Method, uri: Uri, version: Version) -> String {
    format!(
        "method: {method}\nuri: {}\nversion: {:?}\n",
        uri.path(),
        version
    )
}
async fn nested_where(nested: NestedPath, OriginalUri(original): OriginalUri, uri: Uri) -> String {
    format!(
        "NestedPath: {}\nOriginalUri: {original}\nUri: {uri}\n",
        nested.as_str()
    )
}
async fn extension_demo(
    Extension(info): Extension<BuildInfo>,
    missing: Option<Extension<NeverAttached>>,
) -> String {
    format!(
        "Build info: version {}, module {}\nNeverAttached present: {}\n",
        info.version,
        info.module,
        missing.is_some()
    )
}
async fn typed_header_demo(TypedHeader(agent): TypedHeader<UserAgent>) -> String {
    format!("you are: {}\n", agent.as_str())
}
async fn raw_form(RawForm(bytes): RawForm) -> String {
    format!(
        "raw form body({}bytes: {}\n",
        bytes.len(),
        String::from_utf8_lossy(&bytes)
    )
}

async fn whole_request(request: Request) -> String {
    let (parts, body) = request.into_parts();
    let size = axum::body::to_bytes(body, 1024 * 1024)
        .await
        .map(|b| b.len())
        .unwrap_or(0);
    format!(
        "{} {} with {} header(s) and {} body byte(s)\n",
        parts.method,
        parts.uri,
        parts.headers.len(),
        size
    )
}
