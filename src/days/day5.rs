use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};

async fn sub_slice_names(
    Query((offset, limit)): Query<(i32, i32)>,
    Json(names): Json<Vec<String>>,
) -> impl IntoResponse {
    "".to_string()
}

pub fn router() -> Router {
    Router::new().route("/", post(sub_slice_names))
}
