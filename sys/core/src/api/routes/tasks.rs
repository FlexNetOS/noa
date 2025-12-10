use axum::{routing::get, Json, Router, extract::Path};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub status: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", get(task_info).patch(update_task))
}

async fn list_tasks() -> Json<Vec<Task>> {
    Json(vec![Task { id: "task-1".into(), status: "pending".into() }])
}

async fn create_task() -> Json<Task> {
    Json(Task { id: "task-created".into(), status: "queued".into() })
}

async fn task_info(Path(id): Path<String>) -> Json<Task> {
    Json(Task { id, status: "pending".into() })
}

async fn update_task(Path(id): Path<String>) -> Json<Task> {
    Json(Task { id, status: "updated".into() })
}
