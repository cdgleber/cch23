use std::io::Cursor;

use axum::{extract::Multipart, response::IntoResponse, routing::post, Router};
use image::{GenericImageView, ImageReader};
use tower_http::services::ServeDir;

async fn magic_reds(mut multipart: Multipart) -> impl IntoResponse {
    let mut magic_count = 0u32;
    if let Some(field) = multipart.next_field().await.unwrap() {
        let _name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let reader = ImageReader::new(Cursor::new(data))
            .with_guessed_format()
            .expect("Cursor io never fails");
        let image = reader.decode().unwrap(); //ERROR HERE

        for (_, _, rgb) in image.pixels() {
            if rgb[0] > rgb[1] + rgb[2] {
                magic_count += 1;
            }
        }
    }

    format!("{}", magic_count)
}

pub fn router() -> Router {
    Router::new()
        .route("/red_pixels", post(magic_reds))
        .nest_service("/assets", ServeDir::new("assets"))
}
