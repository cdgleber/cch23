use std::collections::{HashMap, HashSet};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
            self.input.find(&doubled_char).is_some() && c.is_alphabetic()
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
        Err(e) => {
            return Err(AppError::ParseError(e));
        }
    };

    let temp_map = HashMap::from([("result".to_string(), input.is_nice())]);

    match input.is_nice().as_str() {
        "nice" => Ok((StatusCode::OK, serde_json::to_string(&temp_map).unwrap())),
        "naughty" => Ok((
            StatusCode::BAD_REQUEST,
            serde_json::to_string(&temp_map).unwrap(),
        )),
        _ => unreachable!(),
    }
}

async fn game(body: String) -> Result<impl IntoResponse, AppError> {
    let input: NiceInput = match serde_json::from_str(&body) {
        Ok(input) => input,
        Err(e) => {
            return Err(AppError::ParseError(e));
        }
    };

    if input.input.len() < 8 {
        let ret_string = format!(r#"{{"result":"naughty","reason":"8 chars"}}"#);
        return Ok((StatusCode::BAD_REQUEST, ret_string));
    }

    if !input.input.chars().any(|c| c.is_uppercase())
        || !input.input.chars().any(|c| c.is_lowercase())
        || !input.input.chars().any(|c| c.is_digit(10))
    {
        let ret_string = format!(r#"{{"result":"naughty","reason":"more types of chars"}}"#);
        return Ok((StatusCode::BAD_REQUEST, ret_string));
    }

    if input.input.chars().filter(|c| c.is_digit(10)).count() < 5 {
        let ret_string = format!(r#"{{"result":"naughty","reason":"55555"}}"#);
        return Ok((StatusCode::BAD_REQUEST, ret_string));
    }

    let sum_digits: u32 = input
        .input
        .split(|c: char| !c.is_numeric())
        .filter_map(|x| x.parse::<u32>().ok())
        .sum::<u32>();

    if sum_digits != 2023u32 {
        let ret_string = format!(r#"{{"result":"naughty","reason":"math is hard"}}"#);
        return Ok((StatusCode::BAD_REQUEST, ret_string));
    }

    let joy_vec: Vec<char> = input
        .input
        .chars()
        .filter(|c| (*c == 'j' || *c == 'o' || *c == 'y'))
        .collect();
    let joy_dir = joy_vec
        .windows(3)
        .all(|s| s[0] == 'j' && s[1] == 'o' && s[2] == 'y');
    println!("{} {:?}", joy_dir, joy_vec);
    if !joy_dir || joy_vec.len() < 3 {
        let ret_string = format!(r#"{{"result":"naughty","reason":"not joyful enough"}}"#);
        return Ok((StatusCode::NOT_ACCEPTABLE, ret_string));
    }

    let sandwich = input
        .input
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|s| s[0] == s[2] && s[1] != s[0] && s[1].is_alphabetic() && s[0].is_alphabetic());
    if !sandwich {
        let ret_string = format!(r#"{{"result":"naughty","reason":"illegal: no sandwich"}}"#);
        return Ok((StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, ret_string));
    }

    let range = input
        .input
        .chars()
        .any(|c| (c as u32) > (0x00002980 as u32) && (c as u32) < (0x00002BFF as u32));
    if !range {
        let ret_string = format!(r#"{{"result":"naughty","reason":"outranged"}}"#);
        return Ok((StatusCode::RANGE_NOT_SATISFIABLE, ret_string));
    }

    let emoji = input.input.chars().any(|c| {
        (c as u32) > (0x00001F600 as u32) && (c as u32) < (0x00001F64F as u32)
            || ((c as u32) > (0x00001F900 as u32) && (c as u32) < (0x00001F9FF as u32))
    });
    if !emoji {
        let ret_string = format!(r#"{{"result":"naughty","reason":"😳"}}"#);
        return Ok((StatusCode::UPGRADE_REQUIRED, ret_string));
    }

    let hash = Sha256::digest(input.input.as_bytes());
    let last_hex_char = format!("{:x}", hash)
        .chars()
        .nth_back(0)
        .is_some_and(|c| c == 'a');
    if !last_hex_char {
        let ret_string = format!(r#"{{"result":"naughty","reason":"not a coffee brewer"}}"#);
        return Ok((StatusCode::IM_A_TEAPOT, ret_string));
    }

    let ret_string = format!(r#"{{"result":"nice","reason":"that's a nice password"}}"#);
    return Ok((StatusCode::OK, ret_string));
}

pub fn router() -> Router {
    Router::new()
        .route("/nice", post(nice))
        .route("/game", post(game))
}
