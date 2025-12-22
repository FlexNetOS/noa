//! Model Management API Routes
//!
//! T125, T126, T129, T130, T527-T529: Model management API endpoints
//! US2: API endpoints for model management

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::server::AppState;
use crate::db::repositories::{Model as DbModel, ModelRepository};
use crate::error::{NoaError, Result};

/// Model list response
#[derive(Serialize)]
pub struct ModelListResponse {
    pub models: Vec<ModelResponse>,
}

/// Model response
#[derive(Serialize)]
pub struct ModelResponse {
    pub id: String,
    pub name: String,
    pub model_type: String,
    pub provider: String,
    pub path: Option<String>,
    pub uri: Option<String>,
    pub size_bytes: Option<i64>,
    pub parameters: Option<String>,
    pub context_length: Option<i32>,
    pub license: Option<String>,
    pub status: String,
}

impl From<DbModel> for ModelResponse {
    fn from(model: DbModel) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name,
            model_type: format!("{:?}", model.model_type),
            provider: model.provider,
            path: model.path,
            uri: model.uri,
            size_bytes: model.size_bytes,
            parameters: model.parameters,
            context_length: model.context_length,
            license: model.license,
            status: format!("{:?}", model.status),
        }
    }
}

/// Download model request
#[derive(Deserialize)]
pub struct DownloadModelRequest {
    pub name: String,
    pub url: String,
    pub output_path: Option<String>,
}

/// Download model response
#[derive(Serialize)]
pub struct DownloadModelResponse {
    pub download_id: String,
    pub status: String,
}

/// Benchmark request
#[derive(Deserialize)]
pub struct BenchmarkRequest {
    pub iterations: Option<usize>,
    pub test_prompts: Option<Vec<String>>,
}

/// Benchmark response
#[derive(Serialize)]
pub struct BenchmarkResponse {
    pub model_id: String,
    pub average_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub tokens_per_second: f64,
    pub successful_iterations: usize,
    pub total_iterations: usize,
}

/// Ingest model request
#[derive(Deserialize)]
pub struct IngestModelRequest {
    pub name: String,
    pub path: String,
    pub model_type: String,
    pub provider: String,
}

/// Create routes for model management
pub fn routes() -> Router {
    Router::new()
        .route("/models", get(list_models))
        .route("/models/download", post(download_model))
        .route("/models/benchmark", post(benchmark_model))
        .route("/models/ingest", post(ingest_model))
        .route("/models/:id/load", post(load_model))
        .route("/models/:id/unload", post(unload_model))
        .route("/models/:id/status", get(get_model_status))
}

/// GET /api/v1/models - List all models
async fn list_models(Extension(state): Extension<AppState>) -> Result<Json<ModelListResponse>> {
    let mut conn = state.sqlite_conn()?;
    let repo = ModelRepository::new(conn.connection_mut());

    let models = repo.find_all()?;

    Ok(Json(ModelListResponse {
        models: models.into_iter().map(ModelResponse::from).collect(),
    }))
}

/// POST /api/v1/models/download - Download a model
async fn download_model(
    Extension(_state): Extension<AppState>,
    Json(request): Json<DownloadModelRequest>,
) -> Result<Json<DownloadModelResponse>> {
    use crate::services::ModelDownloadService;
    use std::path::PathBuf;

    let download_service = ModelDownloadService::new();
    let output_path = request
        .output_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("models").join(format!("{}.gguf", request.name)));

    let download_id = download_service
        .download_model(request.name, request.url, output_path)
        .await?;

    Ok(Json(DownloadModelResponse {
        download_id: download_id.to_string(),
        status: "started".to_string(),
    }))
}

/// POST /api/v1/models/benchmark - Benchmark a model
async fn benchmark_model(
    Extension(_state): Extension<AppState>,
    Json(_request): Json<BenchmarkRequest>,
) -> Result<Json<BenchmarkResponse>> {
    // TODO: Implement actual benchmarking
    // This requires model ID in request
    Err(NoaError::Api(crate::error::ApiError::BadRequest(
        "Benchmark endpoint requires model_id".to_string(),
    )))
}

/// POST /api/v1/models/ingest - Ingest a local model
async fn ingest_model(
    Extension(state): Extension<AppState>,
    Json(request): Json<IngestModelRequest>,
) -> Result<Json<ModelResponse>> {
    use crate::db::repositories::model_repository::{ModelStatus as DbStatus, ModelType as DbType};

    let model_type = match request.model_type.as_str() {
        "llm" => DbType::LLM,
        "embedding" => DbType::Embedding,
        "vision" => DbType::Vision,
        "audio" => DbType::Audio,
        _ => {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "model_type",
                "Invalid model type",
                "INVALID_MODEL_TYPE",
            )))
        }
    };

    let model = DbModel {
        id: Uuid::new_v4(),
        name: request.name,
        model_type,
        provider: request.provider,
        path: Some(request.path),
        uri: None,
        size_bytes: None,
        parameters: None,
        context_length: None,
        license: None,
        config: serde_json::json!({}),
        status: DbStatus::Available,
        metrics: None,
    };

    let mut conn = state.sqlite_conn()?;
    let repo = ModelRepository::new(conn.connection_mut());
    repo.create(&model)?;

    Ok(Json(ModelResponse::from(model)))
}

/// POST /api/v1/models/:id/load - Load a model
async fn load_model(Extension(_state): Extension<AppState>, Path(_id): Path<String>) -> Result<StatusCode> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// POST /api/v1/models/:id/unload - Unload a model
async fn unload_model(Extension(_state): Extension<AppState>, Path(_id): Path<String>) -> Result<StatusCode> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// GET /api/v1/models/:id/status - Get model status
async fn get_model_status(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ModelResponse>> {
    let model_id = Uuid::parse_str(&id).map_err(|_| {
        NoaError::Validation(crate::error::ValidationError::new(
            "id",
            "Invalid UUID format",
            "INVALID_UUID",
        ))
    })?;

    let mut conn = state.sqlite_conn()?;
    let repo = ModelRepository::new(conn.connection_mut());
    let model = repo
        .find_by_id(&model_id)?
        .ok_or_else(|| NoaError::NotFound {
            resource: "Model".to_string(),
            id,
        })?;

    Ok(Json(ModelResponse::from(model)))
}

