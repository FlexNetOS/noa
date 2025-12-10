use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrchestrationStatus {
    pub status: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/orchestration/status", get(status))
}

async fn status() -> Json<OrchestrationStatus> {
    Json(OrchestrationStatus {
        status: "ok".into(),
    })
}
