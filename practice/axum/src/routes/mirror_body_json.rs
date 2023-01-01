use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorJson {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorJsonResponse {
    messgae: String,
    messgae_from_server: String,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        messgae: body.message,
        messgae_from_server: "Hello from Axum".to_owned(),
    })
}
