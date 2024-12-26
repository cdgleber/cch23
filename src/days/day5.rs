use axum::{
    extract::{ Json, Query },
    http::StatusCode,
    response::{ IntoResponse, Response },
    routing::post,
    Router,
};
use serde::{ Deserialize, Serialize };
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("Out of range")]
    OutOfRange,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::OutOfRange => (StatusCode::RANGE_NOT_SATISFIABLE, "Out of Range".to_string()),
        };

        (status, error_message).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Pagination {
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn sub_slice_names(
    pagination: Query<Pagination>,
    Json(names): Json<Vec<String>>
) -> Result<impl IntoResponse, AppError> {
    // println!("{:?} {:?}", pagination, names);
    let start = pagination.offset;
    let mut end = if pagination.limit.is_some() {
        pagination.offset + pagination.limit.unwrap()
    } else {
        names.len()
    };
    if end > names.len() {
        end = names.len();
    }

    let temp_vec: Vec<String> = names[start..end]
        .iter()
        .map(|s| s.clone())
        .collect();

    if pagination.split.is_some() {
        let temp_vec: Vec<Vec<String>> = names[start..end]
            .chunks(pagination.split.unwrap())
            .map(|s|
                s
                    .iter()
                    .map(|st| st.clone())
                    .collect::<Vec<String>>()
            )
            .collect();

        return Ok(serde_json::to_string(&temp_vec).unwrap());
    }

    Ok(serde_json::to_string(&temp_vec).unwrap())
}

pub fn router() -> Router {
    Router::new().route("/", post(sub_slice_names))
}
