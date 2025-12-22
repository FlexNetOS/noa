use axum::{routing::{get, post}, Json, Router, extract::Path};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub status: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/agents", get(list_agents))
        .route("/agents/:id", get(get_agent))
        .route("/agents/:id/start", post(start_agent))
        .route("/agents/:id/stop", post(stop_agent))
        .route("/agents/:id/logs", get(agent_logs))
}

async fn list_agents() -> Json<Vec<Agent>> {
    Json(vec![Agent { id: "commander-chief".into(), status: "ready".into() }])
}

async fn get_agent(Path(id): Path<String>) -> Json<Agent> {
    Json(Agent { id: id.clone(), status: "ready".into() })
}

async fn start_agent(Path(id): Path<String>) -> Json<Agent> {
    Json(Agent { id: id.clone(), status: "starting".into() })
}

async fn stop_agent(Path(id): Path<String>) -> Json<Agent> {
    Json(Agent { id: id.clone(), status: "stopping".into() })
}

async fn agent_logs(Path(id): Path<String>) -> Json<Vec<String>> {
    Json(vec![format!("logs for {}", id)])
}
