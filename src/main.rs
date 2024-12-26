use axum::{ routing::get, Router };
use shuttlings_cch23::days::minus1;

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world)).nest("/-1", minus1::router());

    Ok(router.into())
}
