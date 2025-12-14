use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub title: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalRequest {
    pub title: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/goals", get(list_goals).post(create_goal))
}

async fn list_goals() -> Json<Vec<Goal>> {
    Json(vec![Goal {
        id: "goal-1".into(),
        title: "Sample goal".into(),
        status: "open".into(),
    }])
}

async fn create_goal(Json(body): Json<GoalRequest>) -> Json<Goal> {
    Json(Goal {
        id: "goal-created".into(),
        title: body.title,
        status: "queued".into(),
    })
}
