use std::collections::HashMap;

use axum::{
    http::{ HeaderMap, StatusCode },
    response::{ IntoResponse, Response },
    routing::get,
    Json,
    Router,
};
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use thiserror::Error;
use tower_cookies::{ CookieManagerLayer, Cookies };
use base64::{ engine::general_purpose::STANDARD, Engine as _ };

#[derive(Error, Debug)]
enum AppError {
    #[error("Missing Recipe.")]
    MissingRecipe,
    #[error("Decode Error.")] DecodeError(base64::DecodeError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MissingRecipe => (StatusCode::BAD_REQUEST, "Missing Recipe".to_string()),
            AppError::DecodeError(e) => (StatusCode::BAD_REQUEST, format!("Decode error {e}")),
        };

        (status, error_message).into_response()
    }
}

async fn decode_recipe(cookie: Cookies, headers: HeaderMap) -> Result<String, AppError> {
    if headers.get("Cookie").is_none() {
        println!("Missing Cookie");
        return Err(AppError::MissingRecipe);
    }

    let encoded_recipe = match cookie.get("recipe") {
        Some(c) => c.value().to_string(),
        None => {
            return Err(AppError::MissingRecipe);
        }
    };

    let recipe_bytes = match STANDARD.decode(encoded_recipe.as_bytes()) {
        Ok(s) => s,
        Err(e) => {
            return Err(AppError::DecodeError(e));
        }
    };

    // let recipe_json: Value = serde_json
    //     ::from_str(&String::from_utf8(recipe_bytes).unwrap())
    //     .unwrap();

    Ok(String::from_utf8(recipe_bytes).unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
struct BakeItems {
    recipe: HashMap<String, u32>,
    pantry: HashMap<String, u32>,
}

async fn bake_recipe(cookie: Cookies, headers: HeaderMap) -> Result<String, AppError> {
    if headers.get("Cookie").is_none() {
        println!("Missing Cookie");
        return Err(AppError::MissingRecipe);
    }

    let encoded_recipe = match cookie.get("recipe") {
        Some(c) => c.value().to_string(),
        None => {
            return Err(AppError::MissingRecipe);
        }
    };

    let recipe_bytes = match STANDARD.decode(encoded_recipe.as_bytes()) {
        Ok(s) => s,
        Err(e) => {
            return Err(AppError::DecodeError(e));
        }
    };

    let bake_items: BakeItems = serde_json
        ::from_str(&String::from_utf8(recipe_bytes).unwrap())
        .unwrap();

    // todo!("implement bake logic");

    Ok(format!("{:?}", bake_items))
}

pub fn router() -> Router {
    Router::new()
        .route("/decode", get(decode_recipe))
        .route("/bake", get(bake_recipe))
        .layer(CookieManagerLayer::new())
}
