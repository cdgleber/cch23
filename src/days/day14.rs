use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HtmlContent {
    content: String,
}

async fn unsafe_html(Json(insert_html): Json<HtmlContent>) -> impl IntoResponse {
    format!(
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>"#,
        insert_html.content
    )
}

async fn safe_html(Json(insert_html): Json<HtmlContent>) -> impl IntoResponse {
    format!(
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>"#,
        html_escape::encode_double_quoted_attribute(&insert_html.content)
    )
}

pub fn router() -> Router {
    Router::new()
        .route("/unsafe", post(unsafe_html))
        .route("/safe", post(safe_html))
}
