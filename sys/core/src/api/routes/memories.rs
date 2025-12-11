//! Memory API Routes
//!
//! T149-T152: Implement memory API endpoints
//! 3.7: Total Memory Sovereignty
//! US3: Remember everything with instant recall

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use crate::api::server::AppState;
use crate::db::repositories::MemoryRepository;
use crate::db::vector_search::VectorSearch;
use crate::memory::MemoryType;
use crate::services::{MemoryService, SearchService};
use tokio::runtime::Handle;
use tokio::task;

/// Convenience result type for API operations in this module.
type ApiResult<T> = std::result::Result<T, (StatusCode, String)>;

/// Create memory routes
pub fn create_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/", post(create_memory).get(list_memories))
        .route("/:id", get(get_memory))
        .route("/search", post(search_memories))
}

/// Create memory request
#[derive(Debug, Deserialize)]
pub struct CreateMemoryRequest {
    pub r#type: String,
    pub content: String,
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub source_agent: Option<String>,
    pub parent_id: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Create memory response
#[derive(Debug, Serialize)]
pub struct CreateMemoryResponse {
    pub id: String,
    pub created_at: String,
}

/// List memories query parameters
#[derive(Debug, Deserialize)]
pub struct ListMemoriesQuery {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

/// List memories response
#[derive(Debug, Serialize)]
pub struct ListMemoriesResponse {
    pub memories: Vec<MemoryResponse>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
}

/// Memory response
#[derive(Debug, Serialize)]
pub struct MemoryResponse {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub r#type: String,
    pub content: String,
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub source_agent: Option<String>,
    pub parent_id: Option<String>,
    pub tags: Vec<String>,
    pub checksum: String,
}

/// Search memories request
#[derive(Debug, Deserialize)]
pub struct SearchMemoriesRequest {
    pub query: String,
    pub search_type: Option<String>,
    pub limit: Option<u32>,
    pub threshold: Option<f32>,
}

// Ensure request is Send for axum handlers.
#[allow(dead_code)]
fn _assert_send_search_request()
where
    SearchMemoriesRequest: Send,
{
}

/// Search memories response
#[derive(Debug, Serialize)]
pub struct SearchMemoriesResponse {
    pub results: Vec<SearchResultResponse>,
    pub count: usize,
}

/// Search result response
#[derive(Debug, Serialize)]
pub struct SearchResultResponse {
    pub memory: MemoryResponse,
    pub score: f32,
    pub distance: f32,
}

/// Create a new memory
async fn create_memory(State(state): State<AppState>, Json(request): Json<CreateMemoryRequest>) -> impl IntoResponse {
    let config = state.config.clone();
    let handle = Handle::current();

    let task = task::spawn_blocking(move || -> ApiResult<CreateMemoryResponse> {
        let db_path = &config.database.path;
        let conn = crate::db::init_database(db_path).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize memory service: {}", e),
            )
        })?;
        let memory_service = MemoryService::new(conn);

        let memory_type = match request.r#type.as_str() {
            "interaction" => MemoryType::Interaction,
            "decision" => MemoryType::Decision,
            "learning" => MemoryType::Learning,
            "artifact" => MemoryType::Artifact,
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Invalid memory type: {}", request.r#type),
                ))
            }
        };

        let agent_id = request
            .source_agent
            .as_deref()
            .map(Uuid::parse_str)
            .transpose()
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid agent ID: {}", e)))?;

        let parent_id = request
            .parent_id
            .as_deref()
            .map(Uuid::parse_str)
            .transpose()
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid parent ID: {}", e)))?;

        let tags: HashSet<String> = request.tags.unwrap_or_default().into_iter().collect();

        let id = handle
            .block_on(memory_service.create(
                memory_type,
                request.content,
                request.metadata,
                agent_id,
                parent_id,
                tags,
            ))
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create memory: {}", e),
                )
            })?;

        let memory = memory_service
            .get(&id)
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to retrieve memory: {}", e),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Memory not found after creation".to_string(),
                )
            })?;

        Ok(CreateMemoryResponse {
            id: id.to_string(),
            created_at: memory.created_at.to_rfc3339(),
        })
    });

    match task.await {
        Ok(Ok(response)) => (StatusCode::OK, Json(response)).into_response(),
        Ok(Err((status, message))) => (status, message).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to join memory creation task: {}", e),
        )
            .into_response(),
    }
}

/// Get memory by ID
async fn get_memory(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let config = state.config.clone();

    let task = task::spawn_blocking(move || -> ApiResult<MemoryResponse> {
        let memory_id = Uuid::parse_str(&id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid memory ID: {}", e)))?;
        let db_path = &config.database.path;
        let conn = crate::db::init_database(db_path).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize memory service: {}", e),
            )
        })?;
        let memory_service = MemoryService::new(conn);

        let memory = memory_service
            .get(&memory_id)
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to retrieve memory: {}", e),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    format!("Memory not found: {}", memory_id),
                )
            })?;

        Ok(MemoryResponse {
            id: memory.id.to_string(),
            created_at: memory.created_at.to_rfc3339(),
            updated_at: memory.updated_at.to_rfc3339(),
            r#type: memory.memory_type.as_str().to_string(),
            content: memory.content,
            metadata: memory.metadata,
            source_agent: memory.source_agent.map(|id| id.to_string()),
            parent_id: memory.parent_id.map(|id| id.to_string()),
            tags: memory.tags.into_iter().collect(),
            checksum: memory.checksum,
        })
    });

    match task.await {
        Ok(Ok(response)) => (StatusCode::OK, Json(response)).into_response(),
        Ok(Err((status, message))) => (status, message).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to join memory fetch task: {}", e),
        )
            .into_response(),
    }
}

/// List memories with pagination
async fn list_memories(State(state): State<AppState>, Query(params): Query<ListMemoriesQuery>) -> impl IntoResponse {
    let config = state.config.clone();
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(20);

    let task = task::spawn_blocking(move || -> ApiResult<ListMemoriesResponse> {
        let db_path = &config.database.path;
        let conn = crate::db::init_database(db_path).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize memory service: {}", e),
            )
        })?;
        let memory_service = MemoryService::new(conn);

        let memories = memory_service.list(offset, limit).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to list memories: {}", e),
            )
        })?;

        let total = memory_service.memory_repo().count().map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to count memories: {}", e),
            )
        })?;

        let memory_responses: Vec<MemoryResponse> = memories
            .into_iter()
            .map(|m| MemoryResponse {
                id: m.id.to_string(),
                created_at: m.created_at.to_rfc3339(),
                updated_at: m.updated_at.to_rfc3339(),
                r#type: m.memory_type.as_str().to_string(),
                content: m.content,
                metadata: m.metadata,
                source_agent: m.source_agent.map(|id| id.to_string()),
                parent_id: m.parent_id.map(|id| id.to_string()),
                tags: m.tags.into_iter().collect(),
                checksum: m.checksum,
            })
            .collect();

        Ok(ListMemoriesResponse {
            memories: memory_responses,
            total,
            offset,
            limit,
        })
    });

    match task.await {
        Ok(Ok(response)) => (StatusCode::OK, Json(response)).into_response(),
        Ok(Err((status, message))) => (status, message).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to join list memories task: {}", e),
        )
            .into_response(),
    }
}

/// Search memories
async fn search_memories(
    State(state): State<AppState>,
    Json(request): Json<SearchMemoriesRequest>,
) -> impl IntoResponse {
    let config = state.config.clone();
    let handle = Handle::current();

    let task = task::spawn_blocking(move || -> ApiResult<SearchMemoriesResponse> {
        let search_type = request
            .search_type
            .as_deref()
            .unwrap_or("hybrid")
            .to_string();
        let limit = request.limit.unwrap_or(10);
        let threshold = request.threshold.unwrap_or(0.7);

        let db_path = &config.database.path;
        let conn1 = crate::db::init_database(db_path).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize memory repository: {}", e),
            )
        })?;
        let conn2 = crate::db::init_database(db_path).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize vector search: {}", e),
            )
        })?;
        let memory_repo = MemoryRepository::new(conn1);
        let vector_search = VectorSearch::new(conn2).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize vector search: {}", e),
            )
        })?;
        let search_service = SearchService::new(memory_repo, vector_search);

        let results = handle
            .block_on(search_service.search(
                &request.query,
                &search_type,
                limit,
                threshold,
            ))
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to search memories: {}", e),
                )
            })?;

        let result_responses: Vec<SearchResultResponse> = results
            .into_iter()
            .map(|r| SearchResultResponse {
                memory: MemoryResponse {
                    id: r.memory.id.to_string(),
                    created_at: r.memory.created_at.to_rfc3339(),
                    updated_at: r.memory.updated_at.to_rfc3339(),
                    r#type: r.memory.memory_type.as_str().to_string(),
                    content: r.memory.content,
                    metadata: r.memory.metadata,
                    source_agent: r.memory.source_agent.map(|id| id.to_string()),
                    parent_id: r.memory.parent_id.map(|id| id.to_string()),
                    tags: r.memory.tags.into_iter().collect(),
                    checksum: r.memory.checksum,
                },
                score: r.score,
                distance: r.distance,
            })
            .collect();

        Ok(SearchMemoriesResponse {
            count: result_responses.len(),
            results: result_responses,
        })
    });

    match task.await {
        Ok(Ok(response)) => (StatusCode::OK, Json(response)).into_response(),
        Ok(Err((status, message))) => (status, message).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to join memory search task: {}", e),
        )
            .into_response(),
    }
}
