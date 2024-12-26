use axum::{ extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router };

async fn cube_the_bits(Path((one, two)): Path<(i32, i32)>) -> impl IntoResponse {
    let return_int = (one ^ two).pow(3);
    format!("{return_int}")
}

async fn sled_id(Path(packets): Path<Vec<i32>>) -> impl IntoResponse {
    let folded = packets.iter().fold(0, |acc, x| acc ^ x);
    let return_int = folded.pow(3);
    format!("{return_int}")
}

pub fn router() -> Router {
    Router::new().route("/:one/:two", get(cube_the_bits)).route("/:one", get(sled_id))
}
