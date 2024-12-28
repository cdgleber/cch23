use std::{ collections::HashMap, sync::Arc };
use axum::{
    extract::{ Path, State },
    http::StatusCode,
    response::{ IntoResponse, Response },
    routing::{ get, post },
    Json,
    Router,
};
use jiff::{ tz::TimeZone, Timestamp };
use thiserror::Error;
use tokio::sync::RwLock;
use ulid::Ulid;
use uuid::Uuid;

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
    let timestamp = match packet_map.get(&packet) {
        Some(ts) => ts,
        None => {
            return Err(AppError::PacketNotFound);
        }
    };
    let duration = timestamp.duration_until(Timestamp::now());
    Ok(format!("{}", duration.as_secs()))
}

async fn ulid_to_uuid(Json(ulids): Json<Vec<String>>) -> Result<impl IntoResponse, AppError> {
    let mut temp_vec: Vec<Uuid> = Vec::new();
    for ulid_string in &ulids {
        let ulid: Ulid = match Ulid::from_string(ulid_string) {
            Ok(u) => u,
            Err(_) => {
                return Err(AppError::ParseULIDError);
            }
        };
        let uuid: Uuid = ulid.into();
        temp_vec.push(uuid);
    }
    temp_vec.reverse();
    Ok(serde_json::to_string(&temp_vec).unwrap())
}

async fn ulid_to_dates(
    Path(weekday): Path<i8>,
    Json(ulids): Json<Vec<String>>
) -> Result<impl IntoResponse, AppError> {
    let mut temp_map: HashMap<&str, u32> = HashMap::from([
        ("christmas eve", 0u32),
        ("weekday", 0u32),
        ("in the future", 0u32),
        ("LSB is 1", 0u32),
    ]);
    for ulid_string in &ulids {
        let ulid: Ulid = match Ulid::from_string(ulid_string) {
            Ok(u) => u,
            Err(_) => {
                return Err(AppError::ParseULIDError);
            }
        };
        let time: Timestamp = match Timestamp::from_millisecond(ulid.timestamp_ms() as i64) {
            Ok(t) => t,
            Err(_) => {
                return Err(AppError::ParseTimestampError);
            }
        };

        let zoned = time.to_zoned(TimeZone::UTC);

        if zoned.month() == 12 && zoned.day() == 24 {
            temp_map.entry("christmas eve").and_modify(|count| {
                *count += 1;
            });
        }

        if zoned.weekday().to_monday_zero_offset() == weekday {
            temp_map.entry("weekday").and_modify(|count| {
                *count += 1;
            });
        }

        if time.duration_until(Timestamp::now()).as_secs() < 0 {
            temp_map.entry("in the future").and_modify(|count| {
                *count += 1;
            });
        }

        if (ulid.0 & 1) == 1 {
            temp_map.entry("LSB is 1").and_modify(|count| {
                *count += 1;
            });
        }
    }

    Ok(serde_json::to_string(&temp_map).unwrap())
}

pub fn router() -> Router {
    let packet_map = Arc::new(RwLock::new(HashMap::<String, Timestamp>::new()));
    Router::new()
        .route("/save/:packet", post(save_packet))
        .route("/load/:packet", get(load_packet))
        .route("/ulids", post(ulid_to_uuid))
        .route("/ulids/:weekday", post(ulid_to_dates))
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
