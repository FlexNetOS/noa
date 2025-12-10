//! Memory API Routes
//!
//! T149-T152: Implement memory API endpoints
//! §3.7: Total Memory Sovereignty
//! US3: Remember everything with instant recall

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use crate::api::server::AppState;
use crate::db::init_database;
use crate::db::repositories::MemoryRepository;
use crate::db::vector_search::VectorSearch;
use crate::error::Result;
use crate::init::paths::NoaPaths;
use crate::memory::MemoryType;
use crate::services::{MemoryService, SearchService};
use axum::response::IntoResponse;
use std::path::PathBuf;

/// Create memory routes
pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_memory).get(list_memories))
        .route("/:id", get(get_memory))
        .route("/search", post(search_memories))
}

/// Helper to get database path from AppState
fn get_db_path(state: &AppState) -> PathBuf {
    let noa_root = std::env::var("NOA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    NoaPaths::data(&noa_root).join("noa.db")
}

/// Helper to create memory service from AppState
fn get_memory_service(state: &AppState) -> Result<MemoryService> {
    let db_path = get_db_path(state);
    let conn = init_database(&db_path)?;
    Ok(MemoryService::new(conn))
}

/// Helper to create search service from AppState
fn get_search_service(state: &AppState) -> Result<SearchService> {
    let db_path = get_db_path(state);
    let conn1 = init_database(&db_path)?;
    let conn2 = init_database(&db_path)?;
    let memory_repo = MemoryRepository::new(conn1);
    let vector_search = VectorSearch::new(conn2)?;
    Ok(SearchService::new(memory_repo, vector_search))
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
async fn create_memory(
    State(state): State<AppState>,
    Json(request): Json<CreateMemoryRequest>,
) -> impl IntoResponse {
    let memory_service = match get_memory_service(&state) {
        Ok(service) => service,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize memory service: {}", e),
            )
                .into_response()
        }
    };
    let memory_type = match request.r#type.as_str() {
        "interaction" => MemoryType::Interaction,
        "decision" => MemoryType::Decision,
        "learning" => MemoryType::Learning,
        "artifact" => MemoryType::Artifact,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid memory type: {}", request.r#type),
            )
                .into_response()
        }
    };

    let agent_id = match request
        .source_agent
        .map(|s| Uuid::parse_str(&s))
        .transpose()
    {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid agent ID: {}", e),
            )
                .into_response()
        }
    };

    let parent_id = match request
        .parent_id
        .map(|s| Uuid::parse_str(&s))
        .transpose()
    {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid parent ID: {}", e),
            )
                .into_response()
        }
    };

    let tags: HashSet<String> = request.tags.unwrap_or_default().into_iter().collect();

    let id = match memory_service
        .create(
            memory_type,
            request.content,
            request.metadata,
            agent_id,
            parent_id,
            tags,
        )
        .await
    {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create memory: {}", e),
            )
                .into_response()
        }
    };

    let memory = match memory_service.get(&id) {
        Ok(Some(memory)) => memory,
        Ok(None) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Memory not found after creation".to_string(),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to retrieve memory: {}", e),
            )
                .into_response()
        }
    };

    (StatusCode::OK, Json(CreateMemoryResponse {
        id: id.to_string(),
        created_at: memory.created_at.to_rfc3339(),
    }))
        .into_response()
}

/// Get memory by ID
async fn get_memory(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let memory_service = match get_memory_service(&state) {
        Ok(service) => service,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize memory service: {}", e),
            )
                .into_response()
        }
    };
    let memory_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid memory ID: {}", e),
            )
                .into_response()
        }
    };

    let memory = match memory_service.get(&memory_id) {
        Ok(Some(memory)) => memory,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                format!("Memory not found: {}", id),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to retrieve memory: {}", e),
            )
                .into_response()
        }
    };

    (StatusCode::OK, Json(MemoryResponse {
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
    }))
        .into_response()
}

/// List memories with pagination
async fn list_memories(
    State(state): State<AppState>,
    Query(params): Query<ListMemoriesQuery>,
) -> impl IntoResponse {
    let memory_service = match get_memory_service(&state) {
        Ok(service) => service,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize memory service: {}", e),
            )
                .into_response()
        }
    };
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(20);

    let memories = match memory_service.list(offset, limit) {
        Ok(memories) => memories,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to list memories: {}", e),
            )
                .into_response()
        }
    };

    let total = match memory_service.memory_repo().count() {
        Ok(total) => total,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to count memories: {}", e),
            )
                .into_response()
        }
    };

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

    (StatusCode::OK, Json(ListMemoriesResponse {
        memories: memory_responses,
        total,
        offset,
        limit,
    }))
        .into_response()
}

/// Search memories
async fn search_memories(
    State(state): State<AppState>,
    Json(request): Json<SearchMemoriesRequest>,
) -> impl IntoResponse {
    let search_service = match get_search_service(&state) {
        Ok(service) => service,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize search service: {}", e),
            )
                .into_response()
        }
    };
    let search_type = request.search_type.as_deref().unwrap_or("hybrid");
    let limit = request.limit.unwrap_or(10);
    let threshold = request.threshold.unwrap_or(0.7);

    let results = match search_type {
        "semantic" => match search_service
            .search_semantic(&request.query, limit, threshold)
            .await
        {
            Ok(results) => results,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Semantic search failed: {}", e),
                )
                    .into_response()
            }
        },
        "keyword" => match search_service.search_keyword(&request.query, limit) {
            Ok(results) => results,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Keyword search failed: {}", e),
                )
                    .into_response()
            }
        },
        "hybrid" => match search_service
            .search_hybrid(&request.query, limit, threshold)
            .await
        {
            Ok(results) => results,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Hybrid search failed: {}", e),
                )
                    .into_response()
            }
        },
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid search type: {}", search_type),
            )
                .into_response()
        }
    };

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

    Ok(Json(SearchMemoriesResponse {
        count: result_responses.len(),
        results: result_responses,
    }))
}
