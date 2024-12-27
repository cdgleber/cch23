use num::Float;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{ IntoResponse, Response },
    routing::get,
    Router,
};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
enum AppError {
    #[error("incorrect pokedex id")] IncorrectPokedexID(rustemon::error::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::IncorrectPokedexID(e) =>
                (StatusCode::BAD_REQUEST, format!("Incorrect request to PokeAPI: {e}")),
        };

        (status, error_message).into_response()
    }
}

async fn get_weight(Path(pokedex): Path<i64>) -> Result<impl IntoResponse, AppError> {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon = match rustemon::pokemon::pokemon::get_by_id(pokedex, &rustemon_client).await {
        Ok(poke) => poke,
        Err(e) => {
            return Err(AppError::IncorrectPokedexID(e));
        }
    };

    Ok(format!("{}", (pokemon.weight as f64) / 10.0f64))
}

async fn leave_dent(Path(pokedex): Path<i64>) -> Result<impl IntoResponse, AppError> {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemon = match rustemon::pokemon::pokemon::get_by_id(pokedex, &rustemon_client).await {
        Ok(poke) => poke,
        Err(e) => {
            return Err(AppError::IncorrectPokedexID(e));
        }
    };

    let vel: f64 = Float::sqrt(20_f64 / 9.825) * 9.825;
    let force = ((pokemon.weight as f64) * vel) / 10.0;

    // println!("{} {} {}", force, vel, pokemon.weight);

    Ok(format!("{}", force))
}

pub fn router() -> Router {
    Router::new()
        .route("/weight/:pokedex", get(get_weight))
        .route("/drop/:pokedex", get(leave_dent))
}
