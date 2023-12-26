use axum::{
    extract::{self, Query},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Deserialize;

pub fn day5() -> Router {
    Router::new().route("/5", post(slicing_the_loop))
}

async fn slicing_the_loop(
    stuff: Query<Stuff>,
    extract::Json(payload): extract::Json<Vec<String>>,
) -> impl IntoResponse {
    let result = payload.as_slice()[stuff.offset..stuff.offset + stuff.limit].to_vec();

    Json(result)
}
#[derive(Deserialize)]
struct Stuff {
    offset: usize,
    limit: usize,
}
