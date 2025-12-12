//! Knowledge Graph API Routes
//!
//! T538-T541: Implement knowledge graph endpoints
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::server::AppState;
use crate::db::init_database;
use crate::db::repositories::{KnowledgeEdgeRepository, KnowledgeNodeRepository};
use crate::error::Result;
use crate::init::paths::NoaPaths;
use std::path::PathBuf;

/// Create knowledge graph API routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/knowledge/nodes", get(list_knowledge_nodes))
        .route("/knowledge/nodes/:id", get(get_knowledge_node))
        .route("/knowledge/edges", get(list_knowledge_edges))
        .route("/knowledge/query", post(query_knowledge))
}

/// Helper to get database path from AppState
fn get_db_path(_state: &AppState) -> PathBuf {
    let noa_root = std::env::var("NOA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    NoaPaths::data(&noa_root).join("noa.db")
}

/// List knowledge nodes
async fn list_knowledge_nodes(
    State(state): State<AppState>,
    Query(params): Query<KnowledgeQueryParams>,
) -> Result<Json<KnowledgeNodesResponse>> {
    let db_path = get_db_path(&state);
    let conn = init_database(&db_path)?;
    let repo = KnowledgeNodeRepository::new(conn);

    let nodes = if let Some(source_id) = params.source_id {
        let source_uuid = Uuid::parse_str(&source_id).map_err(|_| {
            crate::error::NoaError::Validation(crate::error::ValidationError::new(
                "source_id",
                "Invalid source ID format",
                "INVALID_UUID",
            ))
        })?;
        repo.find_by_source_digest(&source_uuid)?
    } else if let Some(node_type) = params.node_type {
        let node_type_enum = crate::db::repositories::KnowledgeNodeType::from_str(&node_type)
            .map_err(|_| {
                crate::error::NoaError::Validation(crate::error::ValidationError::new(
                    "node_type",
                    format!("Invalid node type: {}", node_type),
                    "INVALID_TYPE",
                ))
            })?;
        repo.find_by_type(node_type_enum)?
    } else {
        // TODO: Implement list_all method
        vec![]
    };

    Ok(Json(KnowledgeNodesResponse {
        nodes: nodes
            .into_iter()
            .map(|n| KnowledgeNodeResponse {
                id: n.id,
                name: n.name,
                node_type: n.node_type.as_str().to_string(),
                qualified_name: n.qualified_name,
                description: n.description,
            })
            .collect(),
    }))
}

/// Get knowledge node by ID
async fn get_knowledge_node(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<KnowledgeNodeResponse>> {
    let db_path = get_db_path(&state);
    let conn = init_database(&db_path)?;
    let repo = KnowledgeNodeRepository::new(conn);

    let node_id = Uuid::parse_str(&id).map_err(|_| {
        crate::error::NoaError::Validation(crate::error::ValidationError::new(
            "id",
            "Invalid node ID format",
            "INVALID_UUID",
        ))
    })?;

    let node = repo.find_by_id(&node_id)?.ok_or_else(|| crate::error::NoaError::NotFound {
        resource: "knowledge_node".to_string(),
        id: id.clone(),
    })?;

    Ok(Json(KnowledgeNodeResponse {
        id: node.id,
        name: node.name,
        node_type: node.node_type.as_str().to_string(),
        qualified_name: node.qualified_name,
        description: node.description,
    }))
}

/// List knowledge edges
async fn list_knowledge_edges(
    State(state): State<AppState>,
    Query(params): Query<KnowledgeEdgeParams>,
) -> Result<Json<KnowledgeEdgesResponse>> {
    let db_path = get_db_path(&state);
    let conn = init_database(&db_path)?;
    let repo = KnowledgeEdgeRepository::new(conn);

    let edges = if let Some(source_id) = params.source_node {
        let source_uuid = Uuid::parse_str(&source_id).map_err(|_| {
            crate::error::NoaError::Validation(crate::error::ValidationError::new(
                "source_node",
                "Invalid source node ID format",
                "INVALID_UUID",
            ))
        })?;
        repo.find_by_source_node(&source_uuid)?
    } else if let Some(target_id) = params.target_node {
        let target_uuid = Uuid::parse_str(&target_id).map_err(|_| {
            crate::error::NoaError::Validation(crate::error::ValidationError::new(
                "target_node",
                "Invalid target node ID format",
                "INVALID_UUID",
            ))
        })?;
        repo.find_by_target_node(&target_uuid)?
    } else {
        // TODO: Implement list_all method
        vec![]
    };

    Ok(Json(KnowledgeEdgesResponse {
        edges: edges
            .into_iter()
            .map(|e| KnowledgeEdgeResponse {
                id: e.id,
                source_node: e.source_node,
                target_node: e.target_node,
                relationship: e.relationship.as_str().to_string(),
                weight: e.weight,
            })
            .collect(),
    }))
}

/// Query knowledge graph
async fn query_knowledge(
    State(_state): State<AppState>,
    Json(_payload): Json<KnowledgeQueryRequest>,
) -> Result<Json<KnowledgeQueryResponse>> {
    // TODO: Implement knowledge graph query with semantic search
    Ok(Json(KnowledgeQueryResponse {
        nodes: vec![],
        edges: vec![],
    }))
}

// Request/Response types

#[derive(Debug, Deserialize)]
struct KnowledgeQueryParams {
    source_id: Option<String>,
    node_type: Option<String>,
    limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct KnowledgeEdgeParams {
    source_node: Option<String>,
    target_node: Option<String>,
    relationship: Option<String>,
    limit: Option<u64>,
}

#[derive(Debug, Serialize)]
struct KnowledgeNodesResponse {
    nodes: Vec<KnowledgeNodeResponse>,
}

#[derive(Debug, Serialize)]
struct KnowledgeNodeResponse {
    id: Uuid,
    name: String,
    node_type: String,
    qualified_name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
struct KnowledgeEdgesResponse {
    edges: Vec<KnowledgeEdgeResponse>,
}

#[derive(Debug, Serialize)]
struct KnowledgeEdgeResponse {
    id: Uuid,
    source_node: Uuid,
    target_node: Uuid,
    relationship: String,
    weight: f64,
}

#[derive(Debug, Deserialize)]
struct KnowledgeQueryRequest {
    query: String,
    limit: Option<u64>,
}

#[derive(Debug, Serialize)]
struct KnowledgeQueryResponse {
    nodes: Vec<KnowledgeNodeResponse>,
    edges: Vec<KnowledgeEdgeResponse>,
}
