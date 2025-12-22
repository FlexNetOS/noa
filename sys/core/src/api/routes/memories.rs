//! Memory API Routes
//!
//! T149-T152: Implement memory API endpoints
//! §3.7: Total Memory Sovereignty
//! US3: Remember everything with instant recall

use axum::{
    extract::{Extension, Json, Path, Query},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use crate::api::server::AppState;
use crate::db::repositories::{Memory, MemoryRepository};
use crate::db::vector_search::{VectorSearch, VectorSearchResult};
use crate::error::{ApiError, NoaError, Result};
use crate::memory::{EmbeddingGenerator, MemoryType};
use chrono::Utc;
use sha2::{Digest, Sha256};

/// Create memory routes
pub fn create_routes() -> Router {
    Router::new()
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
    Extension(state): Extension<AppState>,
    Json(request): Json<CreateMemoryRequest>,
) -> Result<Json<CreateMemoryResponse>> {
    let mut conn = state.sqlite_conn()?;
    let memory_repo = MemoryRepository::new(conn.connection());

    let memory_type = match request.r#type.as_str() {
        "interaction" => MemoryType::Interaction,
        "decision" => MemoryType::Decision,
        "learning" => MemoryType::Learning,
        "artifact" => MemoryType::Artifact,
        _ => {
            return Err(NoaError::Api(ApiError::BadRequest(format!(
                "Invalid memory type: {}",
                request.r#type
            ))));
        }
    };

    let agent_id = request
        .source_agent
        .as_deref()
        .map(Uuid::parse_str)
        .transpose()?;

    let parent_id = request
        .parent_id
        .as_deref()
        .map(Uuid::parse_str)
        .transpose()?;

    let tags: HashSet<String> = request.tags.unwrap_or_default().into_iter().collect();

    let created_at = Utc::now();
    let updated_at = created_at;
    let id = Uuid::new_v4();
    let checksum = {
        let mut hasher = Sha256::new();
        hasher.update(request.content.as_bytes());
        format!("{:x}", hasher.finalize())
    };

    let memory = Memory {
        id,
        created_at,
        updated_at,
        memory_type,
        content: request.content,
        metadata: request.metadata,
        source_agent: agent_id,
        parent_id,
        tags,
        embedding_id: None,
        checksum,
    };

    memory_repo.create(&memory)?;

    Ok(Json(CreateMemoryResponse {
        id: id.to_string(),
        created_at: memory.created_at.to_rfc3339(),
    }))
}

/// Get memory by ID
async fn get_memory(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Result<Json<MemoryResponse>> {
    let mut conn = state.sqlite_conn()?;
    let memory_repo = MemoryRepository::new(conn.connection());
    let memory_id = Uuid::parse_str(&id)?;

    let memory = memory_repo
        .find_by_id(&memory_id)?
        .ok_or_else(|| NoaError::NotFound {
            resource: "Memory".to_string(),
            id: id.clone(),
        })?;

    Ok(Json(MemoryResponse {
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
}

/// List memories with pagination
async fn list_memories(
    Extension(state): Extension<AppState>,
    Query(params): Query<ListMemoriesQuery>,
) -> Result<Json<ListMemoriesResponse>> {
    let mut conn = state.sqlite_conn()?;
    let memory_repo = MemoryRepository::new(conn.connection());
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(20);

    let memories = memory_repo.list(offset, limit)?;
    let total = memory_repo.count()?;

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

    Ok(Json(ListMemoriesResponse {
        memories: memory_responses,
        total,
        offset,
        limit,
    }))
}

/// Search memories
async fn search_memories(
    Extension(state): Extension<AppState>,
    Json(request): Json<SearchMemoriesRequest>,
) -> Result<Json<SearchMemoriesResponse>> {
    let search_type = request.search_type.as_deref().unwrap_or("hybrid");
    let limit = request.limit.unwrap_or(10);
    let threshold = request.threshold.unwrap_or(0.7);

    // IMPORTANT: avoid holding a rusqlite connection (non-Sync) across await.
    // Any async work must happen before opening the DB connection.
    let query_vector = if search_type == "semantic" || search_type == "hybrid" {
        let generator = EmbeddingGenerator::new("all-MiniLM-L6-v2").await?;
        Some(generator.generate(&request.query).await?)
    } else {
        None
    };

    let mut conn = state.sqlite_conn()?;
    let memory_repo = MemoryRepository::new(conn.connection());
    let vector_search = VectorSearch::new(conn.connection())?;

    let mut combined: std::collections::HashMap<Uuid, SearchResultResponse> = std::collections::HashMap::new();

    // Semantic
    if let Some(ref qv) = query_vector {
        let semantic: Vec<VectorSearchResult> = vector_search.search_memory(qv, limit, threshold)?;
        for hit in semantic {
            if let Some(memory) = memory_repo.find_by_id(&hit.id)? {
                combined.insert(
                    memory.id,
                    SearchResultResponse {
                        memory: MemoryResponse {
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
                        },
                        score: hit.score,
                        distance: hit.distance,
                    },
                );
            }
        }
    }

    // Keyword
    if search_type == "keyword" || search_type == "hybrid" {
        let all_memories = memory_repo.list(0, (limit as u64) * 2)?;
        let query_lower = request.query.to_lowercase();

        for memory in all_memories
            .into_iter()
            .filter(|m| {
                m.content.to_lowercase().contains(&query_lower)
                    || m.tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .take(limit as usize)
        {
            combined
                .entry(memory.id)
                .and_modify(|existing| {
                    // Boost score if found by both.
                    existing.score = (existing.score + 1.0).min(2.0);
                })
                .or_insert(SearchResultResponse {
                    memory: MemoryResponse {
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
                    },
                    score: 1.0,
                    distance: 0.0,
                });
        }
    }

    if search_type != "semantic" && search_type != "keyword" && search_type != "hybrid" {
        return Err(NoaError::Api(ApiError::BadRequest(format!(
            "Invalid search type: {}",
            search_type
        ))));
    }

    let mut result_responses: Vec<SearchResultResponse> = combined.into_values().collect();
    result_responses.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    result_responses.truncate(limit as usize);

    Ok(Json(SearchMemoriesResponse {
        count: result_responses.len(),
        results: result_responses,
    }))
}

