use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub id: String,
    pub message: String,
}

pub fn routes() -> Router {
    Router::new().route("/activity/stream", get(stream_activity))
}

async fn stream_activity() -> Json<Vec<ActivityEvent>> {
    Json(vec![ActivityEvent {
        id: "evt-1".into(),
        message: "activity stream placeholder".into(),
    }])
}
