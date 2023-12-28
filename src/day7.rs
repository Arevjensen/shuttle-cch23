use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use base64::Engine;

pub fn day7() -> Router {
    Router::new().route("/7/decode", get(decode_cookie_header))
}

async fn decode_cookie_header(headers: HeaderMap) -> impl IntoResponse {
    let cookie_base64 = headers.get("Cookie").unwrap().to_str().unwrap();
    let (_, encoded) = cookie_base64.split_once('=').unwrap();
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .expect("decode");
    (StatusCode::OK, decoded)
}
