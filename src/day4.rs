use axum::{extract, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

pub fn day2() -> Router {
    Router::new()
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest))
}

async fn contest(extract::Json(payload): extract::Json<Vec<CompetitionReindeer>>) -> Json<Records> {
    println!("{:?}", payload);
    let fastest = payload
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .map(|x| format!("name: {} , strength: {}", x.name, x.strength))
        .unwrap();
    let tallest = payload
        .iter()
        .max_by(|a, b| a.height.partial_cmp(&b.height).unwrap())
        .map(|x| format!("name: {} , antler width: {}", x.name, x.antler_width))
        .unwrap();
    let magician = payload
        .iter()
        .max_by(|a, b| a.snow_magic_power.partial_cmp(&b.snow_magic_power).unwrap())
        .map(|x| {
            format!(
                "name: {} , snow magic power: {}",
                x.name, x.snow_magic_power
            )
        })
        .unwrap();
    let consumer = payload
        .iter()
        .max_by(|a, b| a.candies.partial_cmp(&b.candies).unwrap())
        .map(|x| format!("name: {} , candies consumed: {}", x.name, x.favorite_food))
        .unwrap();

    let result = Records {
        fastest,
        tallest,
        magician,
        consumer,
    };

    Json(result)
}

#[derive(Debug, Serialize)]
struct Records {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[derive(Debug, Deserialize)]
struct CompetitionReindeer {
    name: String,
    strength: usize,
    speed: f64,
    height: usize,
    antler_width: usize,
    snow_magic_power: usize,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: usize,
}

async fn strength(extract::Json(payload): extract::Json<Vec<Reindeer>>) -> impl IntoResponse {
    let total = payload.iter().map(|x| x.strength).sum::<usize>();
    (StatusCode::OK, total.to_string())
}

#[derive(Debug, Deserialize)]
struct Reindeer {
    strength: usize,
}
