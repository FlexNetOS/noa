use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CapsuleSpawnRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapsuleSpawnResponse {
    pub status: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/capsule/spawn", post(spawn))
}

async fn spawn(Json(body): Json<CapsuleSpawnRequest>) -> Json<CapsuleSpawnResponse> {
    Json(CapsuleSpawnResponse {
        status: format!("spawned {}", body.name),
    })
}
