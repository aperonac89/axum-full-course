use axum::response::Json;
use serde_json::{Value, json};

pub async fn dashboard() -> Json<Value> {
    Json(json!({
        "module": "05 Extractors",
        "lessons": [
            "GET /products/7                 Path<u32>",
            "GET /users/7/posts/3            Path<(u32, u32)>",
            "GET /search?q=axum              Query<SearchParams>",
            "POST /users {json}              Json<UserInput>",
            "POST /orders/5?q=..&.. {json}   order: Path, Query, Json-last",
            "POST /feedback {form}           Form<Feedback>",
            "GET /theme  GET /theme/dark     Option<Path>",
            "POST /optional-json [json]      Option<Json>",
            "POST /strict-json {json}        Result<Json, JsonRejection>",
            "GET /headers                    HeaderMap echo",
            "POST /echo-raw {any body}       Bytes extractor",
            "GET /meta/route                 MatchedPath",
            "GET /ip                         ConnectInfo<SocketAddr>",
            "GET /all-params/{org}/{repo}    Path<HashMap<String,String>>",
            "GET /repo-struct/{org}/{repo}   Path<struct>",
            "GET /raw-params/{org}/{repo}    RawPathParams (no parsing)",
            "GET /query-map?a=1&b=2          Query<HashMap>",
            "GET /raw-query?anything         RawQuery",
            "GET /uri-parts                  Method + Uri + Version",
            "GET /inner/where                NestedPath + OriginalUri",
            "GET /extension                  Extension + Option<Extension>",
            "GET /typed-header               TypedHeader<UserAgent>",
            "POST /raw-form                  RawForm",
            "POST /whole-request             Request (everything)",
            "GET /custom/api-key             CUSTOM FromRequestParts (x-api-key)",
            "GET /custom/api-key-optional    CUSTOM Option<ApiKey>",
            "POST /custom/lines              CUSTOM FromRequest (reads body)",
            "GET /custom/client              CUSTOM composed extractor"
        ]
    }))
}
