use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
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
    let sum_strength = reindeers
        .iter()
        .reduce(|acc, r| acc.speed.max(r.speed))
        .unwrap();

    let ret = format!("ad");
    (StatusCode::OK, ret).into_response()
}

// {
//     "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
//     "tallest": "Dasher is standing tall with his 36 cm wide antlers",
//     "magician": "Dasher could blast you away with a snow magic power of 9001",
//     "consumer": "Dancer ate lots of candies, but also some grass"
//   }

pub fn router() -> Router {
    Router::new().route("/strength", post(calc_strength))
}
