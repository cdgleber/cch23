use std::{ collections::HashMap, sync::Arc };
use axum::{
    extract::{ Path, State },
    http::StatusCode,
    response::{ IntoResponse, Response },
    routing::post,
    Router,
};
use jiff::Timestamp;
use thiserror::Error;
use tokio::sync::RwLock;
use ulid::Ulid;

#[derive(Error, Debug)]
enum AppError {
    #[error("Incorrect ULID")]
    ParseULIDError,
    #[error("Incorrect Timestamp")]
    ParseTimestampError,
    #[error("Packet not found")]
    PacketNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ParseULIDError => (StatusCode::BAD_REQUEST, "Incorrect ULID".to_string()),
            AppError::ParseTimestampError =>
                (StatusCode::BAD_REQUEST, "Incorrect Timestamp".to_string()),
            AppError::PacketNotFound => (StatusCode::BAD_REQUEST, "Packet not found".to_string()),
        };

        (status, error_message).into_response()
    }
}

type PacketMap = Arc<RwLock<HashMap<String, Timestamp>>>;

async fn save_packet(
    Path(packet): Path<String>,
    State(packet_map_state): State<PacketMap>
) -> Result<impl IntoResponse, AppError> {
    let mut packet_map = packet_map_state.write().await;
    packet_map.insert(packet, Timestamp::now());
    Ok(format!("{:?}", packet_map.clone()))
}

async fn load_packet(
    Path(packet): Path<String>,
    State(packet_map_state): State<PacketMap>
) -> Result<impl IntoResponse, AppError> {
    let packet_map = packet_map_state.read().await;
    let timestamp = packet_map.get(&packet).unwrap();
    Ok(format!("{:?}", timestamp))
}

pub fn router() -> Router {
    let packet_map = Arc::new(RwLock::new(HashMap::<String, Timestamp>::new()));
    Router::new()
        .route("/save/:packet", post(save_packet))
        .route("/load/:packet", post(load_packet))
        .with_state(packet_map.clone())
}

// let ulid: Ulid = match Ulid::from_string(&ulid_to_save) {
//     Ok(u) => u,
//     Err(_) => {
//         return Err(AppError::ParseULIDError);
//     }
// };
// let time: Timestamp = match Timestamp::from_millisecond(ulid.timestamp_ms() as i64) {
//     Ok(t) => t,
//     Err(_) => {
//         return Err(AppError::ParseTimestampError);
//     }
// };
