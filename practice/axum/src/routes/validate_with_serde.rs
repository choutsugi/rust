use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUser {
    username: String,
    password: Option<String>,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
}
