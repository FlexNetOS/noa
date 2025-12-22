use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;
use crate::providers::{
    registry::{enable_provider, providers},
    shared_memory::SharedProviderMemory,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderListResponse {
    pub providers: Vec<crate::providers::ProviderInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub provider: String,
    pub task: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub provider: String,
    pub result: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/providers", get(list_providers))
        .route("/providers/:id/enable", post(enable))
        .route("/providers/context", get(get_context))
        .route("/providers/execute", post(execute_task))
}

async fn list_providers() -> Json<ProviderListResponse> {
    let list = providers().unwrap_or_default();
    Json(ProviderListResponse { providers: list })
}

async fn enable(Path(id): Path<String>, Extension(_state): Extension<AppState>) -> Json<ProviderListResponse> {
    let _ = enable_provider(&id);
    let list = providers().unwrap_or_default();
    Json(ProviderListResponse { providers: list })
}

async fn get_context() -> Json<Vec<String>> {
    // Placeholder: return the list of shared contexts
    let memory = SharedProviderMemory::new();
    let contexts = memory
        .all()
        .into_iter()
        .map(|c| format!("{}:{}", c.provider, c.context_type))
        .collect();
    Json(contexts)
}

async fn execute_task(
    Json(body): Json<ExecuteRequest>,
) -> Json<ExecuteResponse> {
    let result = format!("executed '{}' via {}", body.task, body.provider);
    Json(ExecuteResponse {
        provider: body.provider,
        result,
    })
}
