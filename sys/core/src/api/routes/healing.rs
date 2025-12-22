use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealingStatus {
    pub incidents: u32,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealingEvent {
    pub id: String,
    pub description: String,
    pub status: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/healing/status", get(status))
        .route("/healing/events", get(events))
}

async fn status() -> Json<HealingStatus> {
    Json(HealingStatus {
        incidents: 0,
        active: false,
    })
}

async fn events() -> Json<Vec<HealingEvent>> {
    Json(vec![])
}
