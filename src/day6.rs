use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::Serialize;

pub fn day6() -> Router {
    Router::new().route("/6", post(count_elves))
}

async fn count_elves(body: String) -> impl IntoResponse {
    let elf_count = body
        .as_bytes()
        .windows(3)
        .filter(|x| String::from_utf8(x.to_vec()).unwrap() == r#"elf"#)
        .inspect(|x| println!("{}", String::from_utf8(x.to_vec()).unwrap()))
        .count();

    println!("in str {}, elf count: {}", body, elf_count);

    Json(Counter {
        elf: elf_count,
        elf_shelf: 0,
        elf_not_shelf: 0,
    })
}

#[derive(Serialize)]
struct Counter {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    elf_not_shelf: usize,
}
