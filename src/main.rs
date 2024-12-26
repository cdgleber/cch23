use axum::{routing::get, Router};
use shuttlings_cch23::days::{day1, day4, day5, minus1};

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/-1", minus1::router())
        .nest("/1", day1::router())
        .nest("/1", day5::router())
        .nest("/4", day4::router());

    Ok(router.into())
}