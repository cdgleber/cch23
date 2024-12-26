use axum::{extract::Path, response::IntoResponse, routing::get, Router};

async fn sled_id(Path(path): Path<String>) -> impl IntoResponse {
    path.split_terminator('/')
        .map(|i| i.parse::<i32>().unwrap_or_default())
        .reduce(|x, y| x ^ y)
        .map(|o| o.pow(3))
        .unwrap()
        .to_string()
}

pub fn router() -> Router {
    Router::new().route("/*path", get(sled_id))
}
