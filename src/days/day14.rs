use axum::{response::IntoResponse, routing::post, Json, Router};

async fn unsafe_html(Json(insert_html): Json<String>) -> impl IntoResponse {
    format!(
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>"#,
        insert_html
    )
}

async fn safe_html(Json(insert_html): Json<String>) -> impl IntoResponse {
    format!(
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>"#,
        html_escape::encode_safe(&insert_html)
    )
}

pub fn router() -> Router {
    Router::new()
        .route("/unsafe", post(unsafe_html))
        .route("/safe", post(safe_html))
}
