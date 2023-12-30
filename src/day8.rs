use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use serde::Deserialize;

pub fn day8() -> Router {
    Router::new().route("/8/weight/:id", get(pokemon))
}

async fn pokemon(Path(id): Path<String>) -> impl IntoResponse {
    println!("{}", id);
    let response = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", id)).await;
    let body = response.unwrap().text().await.unwrap();
    let pokemon_data: PokemonData = serde_json::from_str(body.as_str()).unwrap();
    (StatusCode::OK, (pokemon_data.weight / 10_f64).to_string())
}

#[derive(Debug, Deserialize)]
struct PokemonData {
    weight: f64
}
