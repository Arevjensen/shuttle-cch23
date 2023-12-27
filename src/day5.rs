use axum::{
    extract::{self, Query},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn day5() -> Router {
    Router::new().route("/5", post(slicing_the_loop))
}

async fn slicing_the_loop(
    stuff: Query<Stuff>,
    extract::Json(payload): extract::Json<Vec<String>>,
) -> impl IntoResponse {
    println!("this is a new request");
    println!("query paramts: {:?}, vec: {:?}", stuff, payload);
    let offset = stuff.offset.unwrap_or(0).clamp(0, payload.len());
    let limit = stuff.limit.unwrap_or(payload.len()).clamp(0, payload.len());
    let limit = (limit + offset).clamp(0, payload.len());

    println!("offset: {}, limit: {}", offset, limit);

    let result = payload.as_slice()[offset..limit].to_vec();
    if let Some(splitting) = stuff.split {
        let some_result = result
            .chunks(splitting)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<String>>>();
        return Json(Things::VecOfVec(some_result));
    }

    Json(Things::SingleVec(result))
}

#[derive(Deserialize, Debug)]
struct Stuff {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Things {
    SingleVec(Vec<String>),
    VecOfVec(Vec<Vec<String>>),
}
