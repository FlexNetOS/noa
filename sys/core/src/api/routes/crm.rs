use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrmToggleRequest {
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrmResponse {
    pub status: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/crm/toggle", post(toggle))
        .route("/crm/rollback", post(rollback))
}

async fn toggle(Json(body): Json<CrmToggleRequest>) -> Json<CrmResponse> {
    Json(CrmResponse {
        status: format!("mode set to {}", body.mode),
    })
}

async fn rollback() -> Json<CrmResponse> {
    Json(CrmResponse {
        status: "rollback complete".into(),
    })
}
