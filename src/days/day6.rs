use std::collections::HashMap;

use axum::{response::IntoResponse, routing::post, Router};

async fn shelf_elf(body: String) -> impl IntoResponse {
    let elf_index: Vec<(usize, &str)> = body.match_indices("elf").collect();
    let shelf_index: Vec<(usize, &str)> = body.match_indices("shelf").collect();

    let elf_count = elf_index.len();
    let mut elf_shelf_count = 0usize;
    for (i, _) in &shelf_index {
        let start = i.saturating_sub("elf on a ".len());
        let end = *i;
        if &body[start..end] == "elf on a " {
            elf_shelf_count += 1;
        }
    }
    let shelf_without_elf = shelf_index.len() - elf_shelf_count;

    let map = HashMap::from([
        ("elf", elf_count),
        ("elf on a shelf", elf_shelf_count),
        ("shelf with no elf on it", shelf_without_elf),
    ]);

    println!("{}", body);
    println!("{:?}", map);

    serde_json::to_string(&map).unwrap()
}

pub fn router() -> Router {
    Router::new().route("/", post(shelf_elf))
}
