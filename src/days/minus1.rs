use axum::{ http::StatusCode, response::IntoResponse, routing::get, Router };

async fn fake_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn router() -> Router {
    Router::new().route("/error", get(fake_error))
}
