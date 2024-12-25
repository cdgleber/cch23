use axum::Router;
use shuttlings_cch23::days::minus1;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest("-1", minus1::router());

    Ok(router.into())
}
