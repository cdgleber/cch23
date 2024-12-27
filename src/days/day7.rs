use std::collections::HashMap;

use axum::{
    http::{ HeaderMap, StatusCode },
    response::{ IntoResponse, Response },
    routing::get,
    Json,
    Router,
};
use serde::{ Deserialize, Serialize };
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
    recipe: HashMap<String, i64>,
    pantry: HashMap<String, i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BakedCookies {
    cookies: i64,
    pantry: HashMap<String, i64>,
}

impl BakeItems {
    fn cookies_available(&self) -> i64 {
        let mut cookies_possible = Vec::new();
        for (ingredient, amount_needed) in &self.recipe {
            if let Some(pantry_amount) = &self.pantry.get(ingredient) {
                if amount_needed < pantry_amount {
                    cookies_possible.push(**pantry_amount / *amount_needed);
                } else if *amount_needed != 0i64 {
                    return 0i64;
                }
            } else {
                if *amount_needed != 0i64 {
                    return 0i64;
                }
            }
        }

        *cookies_possible.iter().min().unwrap()
    }

    fn remaining_pantry(&self) -> Self {
        let cookies_to_bake = self.cookies_available();
        let mut new_bake_items = BakeItems {
            recipe: HashMap::<String, i64>::new(),
            pantry: HashMap::<String, i64>::new(),
        };

        for (ingredient, pantry_amount) in &self.pantry {
            if let Some(amount_needed) = &self.recipe.get(ingredient) {
                let new_amount = pantry_amount.saturating_sub(cookies_to_bake * **amount_needed);
                new_bake_items.pantry.insert(ingredient.clone(), new_amount);
            } else {
                new_bake_items.pantry.insert(ingredient.clone(), *pantry_amount);
            }
        }

        println!("{:?}", new_bake_items);

        new_bake_items
    }
}

async fn bake_recipe(cookie: Cookies, headers: HeaderMap) -> Result<Json<BakedCookies>, AppError> {
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

    let baked = bake_items.remaining_pantry();

    let baked_cookies = BakedCookies {
        cookies: bake_items.cookies_available(),
        pantry: baked.pantry,
    };

    Ok(Json(baked_cookies))
}

pub fn router() -> Router {
    Router::new()
        .route("/decode", get(decode_recipe))
        .route("/bake", get(bake_recipe))
        .layer(CookieManagerLayer::new())
}
