//! Digest API Routes
//!
//! T189-T191, T509-T513, T536-T537: Implement digest API endpoints
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::server::AppState;
use crate::db::init_database;
use crate::db::repositories::{DigestRepository, DigestSource, DigestSourceType};
use crate::error::Result;
use crate::init::paths::NoaPaths;
use crate::services::DigestService;
use axum::response::IntoResponse;
use std::path::PathBuf;

/// Create digest API routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/digest", post(create_digest))
        .route("/digest/:job_id", get(get_digest_job))
        .route("/digest/:job_id/artifacts", get(get_digest_artifacts))
        .route("/digest/sources", get(list_sources))
        .route("/digest/sources", post(create_source))
        .route("/digest/sources/:id", get(get_source))
        .route("/digest/sources/:id/profile", get(get_source_profile))
        .route(
            "/digest/sources/:id/system-card",
            get(get_source_system_card),
        )
        .route("/digest/sources/:id/sbom", get(get_source_sbom))
        .route("/digest/sources/:id/security", get(get_source_security))
    // Knowledge routes moved to knowledge.rs module
}

/// Create a new digest job
async fn create_digest(
    State(_state): State<AppState>,
    Json(payload): Json<CreateDigestRequest>,
) -> Result<Json<DigestJobResponse>> {
    // TODO: Implement digest job creation
    let job_id = Uuid::new_v4();
    Ok(Json(DigestJobResponse {
        job_id,
        status: "pending".to_string(),
        source_uri: payload.uri,
    }))
}

/// Get digest job status
async fn get_digest_job(
    State(_state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<Json<DigestJobResponse>> {
    // TODO: Implement job status retrieval
    let job_uuid = Uuid::parse_str(&job_id).map_err(|_| {
        crate::error::NoaError::Validation(crate::error::ValidationError::new(
            "job_id",
            "Invalid job ID format",
            "INVALID_UUID",
        ))
    })?;

    Ok(Json(DigestJobResponse {
        job_id: job_uuid,
        status: "pending".to_string(),
        source_uri: "".to_string(),
    }))
}

/// Get digest artifacts
async fn get_digest_artifacts(
    State(_state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<Json<DigestArtifactsResponse>> {
    // TODO: Implement artifact retrieval
    Ok(Json(DigestArtifactsResponse {
        job_id: Uuid::parse_str(&job_id).unwrap_or_default(),
        artifacts: vec![],
    }))
}

/// Helper to get database path from AppState
fn get_db_path(state: &AppState) -> PathBuf {
    let noa_root = std::env::var("NOA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    NoaPaths::data(&noa_root).join("noa.db")
}

/// List digest sources
async fn list_sources(
    State(state): State<AppState>,
    Query(params): Query<ListSourcesParams>,
) -> Result<Json<SourceListResponse>> {
    let db_path = get_db_path(&state);
    let conn = init_database(&db_path)?;
    let repo = DigestRepository::new(conn);

    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(20);

    let sources = repo.list(offset, limit)?;

    Ok(Json(SourceListResponse {
        sources: sources
            .into_iter()
            .map(|s| SourceResponse {
                id: s.id,
                uri: s.uri,
                name: s.name,
                status: s.status.as_str().to_string(),
            })
            .collect(),
        total: 0, // TODO: Get total count
    }))
}

/// Create a new digest source
async fn create_source(
    State(state): State<AppState>,
    Json(payload): Json<CreateSourceRequest>,
) -> impl IntoResponse {
    let db_path = get_db_path(&state);
    let conn = match init_database(&db_path) {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize database: {}", e),
            )
                .into_response()
        }
    };
    let repo = DigestRepository::new(conn);

    let source_type = match payload.source_type.as_str() {
        "repository" => DigestSourceType::Repository,
        "file" => DigestSourceType::File,
        "api" => DigestSourceType::Api,
        "document" => DigestSourceType::Document,
        _ => {
            return (StatusCode::BAD_REQUEST, "Invalid source type".to_string()).into_response();
        }
    };

    let source = DigestSource {
        id: Uuid::new_v4(),
        source_type,
        uri: payload.uri.clone(),
        name: payload.name.clone(),
        status: crate::db::repositories::DigestStatus::Pending,
        last_digest: None,
        version: None,
        profile: None,
        sbom: None,
        security_report: None,
        stats: None,
    };

    match repo.create(&source) {
        Ok(_) => (),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create source: {}", e),
            )
                .into_response()
        }
    }

    (
        StatusCode::CREATED,
        Json(SourceResponse {
            id: source.id,
            uri: source.uri,
            name: source.name,
            status: source.status.as_str().to_string(),
        }),
    )
        .into_response()
}

/// Get digest source details
async fn get_source(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<SourceResponse>> {
    let db_path = get_db_path(&state);
    let conn = init_database(&db_path)?;
    let repo = DigestRepository::new(conn);

    let source_id = Uuid::parse_str(&id).map_err(|_| {
        crate::error::NoaError::Validation(crate::error::ValidationError::new(
            "id",
            "Invalid source ID format",
            "INVALID_UUID",
        ))
    })?;

    let source = repo.find_by_id(&source_id)?.ok_or_else(|| crate::error::NoaError::NotFound {
        resource: "digest_source".to_string(),
        id: id.clone(),
    })?;

    Ok(Json(SourceResponse {
        id: source.id,
        uri: source.uri,
        name: source.name,
        status: source.status.as_str().to_string(),
    }))
}

/// Get source profile
async fn get_source_profile(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    // TODO: Implement profile retrieval
    Ok(Json(serde_json::json!({})))
}

/// Get source system card
async fn get_source_system_card(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<String> {
    // TODO: Implement system card retrieval
    Ok("# System Card\n\nNot yet implemented".to_string())
}

/// Get source SBOM
async fn get_source_sbom(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    // TODO: Implement SBOM retrieval
    Ok(Json(serde_json::json!({})))
}

/// Get source security report
async fn get_source_security(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    // TODO: Implement security report retrieval
    Ok(Json(serde_json::json!({})))
}

// Request/Response types

#[derive(Debug, Deserialize)]
struct CreateDigestRequest {
    uri: String,
    #[allow(dead_code)]
    source_type: String,
}

#[derive(Debug, Serialize)]
struct DigestJobResponse {
    job_id: Uuid,
    status: String,
    source_uri: String,
}

#[derive(Debug, Serialize)]
struct DigestArtifactsResponse {
    job_id: Uuid,
    artifacts: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ListSourcesParams {
    offset: Option<u64>,
    limit: Option<u64>,
}

#[derive(Debug, Serialize)]
struct SourceListResponse {
    sources: Vec<SourceResponse>,
    total: u64,
}

#[derive(Debug, Serialize)]
struct SourceResponse {
    id: Uuid,
    uri: String,
    name: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct CreateSourceRequest {
    uri: String,
    name: String,
    #[allow(dead_code)]
    source_type: String,
}
