//! API route handlers

pub mod chat;
pub mod auth;

use axum::Json;
use serde::{Serialize, Deserialize};

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

/// Health check endpoint
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// Status response
#[derive(Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
    pub components: ComponentStatus,
}

/// Component status
#[derive(Serialize)]
pub struct ComponentStatus {
    pub api: bool,
    pub database: bool,
    pub embedder: bool,
    pub agents: bool,
    pub p2p: bool,
}

/// Status endpoint
pub async fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "operational",
        components: ComponentStatus {
            api: true,
            database: true,
            embedder: false,
            agents: false,
            p2p: false,
        },
    })
}

/// Create task request
#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub description: String,
    pub priority: Option<String>,
}

/// Create task response
#[derive(Serialize)]
pub struct CreateTaskResponse {
    pub task_id: String,
    pub status: String,
}

/// Create task endpoint
pub async fn create_task(Json(_payload): Json<CreateTaskRequest>) -> Json<CreateTaskResponse> {
    let task_id = uuid::Uuid::new_v4().to_string();

    Json(CreateTaskResponse {
        task_id,
        status: "queued".to_string(),
    })
}

/// Task information
#[derive(Serialize)]
pub struct TaskInfo {
    pub id: String,
    pub description: String,
    pub status: String,
    pub priority: String,
}

/// List tasks response
#[derive(Serialize)]
pub struct ListTasksResponse {
    pub tasks: Vec<TaskInfo>,
}

/// List tasks endpoint
pub async fn list_tasks() -> Json<ListTasksResponse> {
    Json(ListTasksResponse {
        tasks: vec![],
    })
}

// --- Stub Handlers for UI ---

#[derive(Serialize)]
pub struct CapsulesResponse { pub capsules: Vec<String> }
pub async fn get_capsules() -> Json<CapsulesResponse> { Json(CapsulesResponse { capsules: vec![] }) }

#[derive(Serialize)]
pub struct JobsResponse { pub jobs: Vec<String> }
pub async fn get_jobs() -> Json<JobsResponse> { Json(JobsResponse { jobs: vec![] }) }

#[derive(Serialize)]
pub struct ArtifactsResponse { pub artifacts: Vec<String> }
pub async fn get_artifacts() -> Json<ArtifactsResponse> { Json(ArtifactsResponse { artifacts: vec![] }) }

#[derive(Serialize)]
pub struct ModelsResponse { pub models: Vec<String> }
pub async fn get_models() -> Json<ModelsResponse> { Json(ModelsResponse { models: vec![] }) }

#[derive(Serialize)]
pub struct ActivityResponse { pub entries: Vec<String> }
pub async fn get_activity() -> Json<ActivityResponse> { Json(ActivityResponse { entries: vec![] }) }


