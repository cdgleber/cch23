use std::collections::{HashMap, HashSet};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("Parse error")]
    ParseError(serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ParseError(e) => (StatusCode::BAD_REQUEST, format!("Parse error: {e}")),
        };

        (status, error_message).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct NiceInput {
    input: String,
}

impl NiceInput {
    fn is_nice(&self) -> String {
        let doubled = self.input.chars().any(|c| {
            let doubled_char = format!("{}{}", c, c);
            self.input.find(&doubled_char).is_some()
        });
        if !doubled {
            return "naughty".to_string();
        }

        let substrings = ["ab", "cd", "pq", "xy"];
        for ss in substrings {
            if self.input.find(ss).is_some() {
                return "naughty".to_string();
            }
        }

        let vowels: HashSet<char> = "aeiouy".chars().collect();
        let num_vowels = self.input.chars().filter(|c| vowels.contains(c)).count();
        if num_vowels < 3 {
            return "naughty".to_string();
        }

        return "nice".to_string();
    }
}

async fn nice(body: String) -> Result<impl IntoResponse, AppError> {
    let input: NiceInput = match serde_json::from_str(&body) {
        Ok(input) => input,
        Err(e) => return Err(AppError::ParseError(e)),
    };

    let temp_map = HashMap::from([("result".to_string(), input.is_nice())]);
    Ok((StatusCode::OK, serde_json::to_string(&temp_map).unwrap()))
}

async fn game(body: String) -> Result<impl IntoResponse, AppError> {
    let input: NiceInput = match serde_json::from_str(&body) {
        Ok(input) => input,
        Err(e) => return Err(AppError::ParseError(e)),
    };

    Ok((StatusCode::OK, input.input))
}

pub fn router() -> Router {
    Router::new()
        .route("/nice", post(nice))
        .route("/game", post(game))
}
