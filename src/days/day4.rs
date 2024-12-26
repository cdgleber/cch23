use std::collections::HashMap;

use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contestent {
    name: String,
    strength: i32,
    speed: f64,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

async fn calc_strength(Json(reindeers): Json<Vec<Reindeer>>) -> impl IntoResponse {
    let sum_strength: i32 = reindeers.iter().map(|r| r.strength).sum();
    let ret = format!("{sum_strength}");
    (StatusCode::OK, ret).into_response()
}

async fn contest_winners(Json(reindeers): Json<Vec<Contestent>>) -> impl IntoResponse {
    let fastest = reindeers
        .iter()
        .reduce(|a, b| if a.speed > b.speed { a } else { b })
        .unwrap();

    let tallest = reindeers.iter().max_by_key(|r| r.height).unwrap();
    let magician = reindeers.iter().max_by_key(|r| r.snow_magic_power).unwrap();
    let candiest = reindeers
        .iter()
        .max_by_key(|r| r.candies_eaten_yesterday)
        .unwrap();

    let fastest_str = format!(
        "Speeding past the finish line with a strength of {} is {}",
        fastest.strength, fastest.name
    );
    let tallest_str = format!(
        "{} is standing tall with his {} cm wide antlers",
        tallest.name, tallest.antler_width
    );
    let magician_str = format!(
        "{} could blast you away with a snow magic power of {}",
        magician.name, magician.snow_magic_power
    );
    let consumer_str = format!(
        "{} ate lots of candies, but also some {}",
        candiest.name, candiest.favorite_food
    );

    let mut winners: HashMap<&str, String> = HashMap::new();
    winners.insert("fastest", fastest_str);
    winners.insert("tallest", tallest_str);
    winners.insert("magician", magician_str);
    winners.insert("consumer", consumer_str);

    (StatusCode::OK, serde_json::to_string(&winners).unwrap()).into_response()
}

pub fn router() -> Router {
    Router::new()
        .route("/strength", post(calc_strength))
        .route("/contest", post(contest_winners))
}
