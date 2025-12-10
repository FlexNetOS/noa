//! Model Management API Routes
//!
//! T125, T126, T129, T130, T527-T529: Model management API endpoints
//! US2: API endpoints for model management

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::server::AppState;
use noa_neural::{Model as NeuralModel, ModelStatus};
use crate::db::repositories::model_repository::Model as RepoModel;
use crate::services::NeuralService;
use axum::response::IntoResponse;

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

impl From<NeuralModel> for ModelResponse {
    fn from(model: NeuralModel) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.config.name.clone(),
            model_type: format!("{:?}", model.config.model_type),
            provider: model.config.provider.clone(),
            path: model.config.file_path.clone(),
            uri: None,
            size_bytes: None,
            parameters: None,
            context_length: Some(model.config.context_length as i32),
            license: None,
            status: format!("{:?}", model.status),
        }
    }
}

impl From<RepoModel> for ModelResponse {
    fn from(model: RepoModel) -> Self {
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
pub fn routes() -> Router<AppState> {
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
async fn list_models(State(state): State<AppState>) -> impl IntoResponse {
    // Use database path from config
    let db_path = &state.config.database.path;
    let conn = match crate::db::init_database(db_path) {
        Ok(conn) => conn,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to initialize database: {}", e)).into_response(),
    };
    let service = NeuralService::new(conn);
    let models = match service.list_models() {
        Ok(models) => models,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to list models: {}", e)).into_response(),
    };

    // Convert model_repository::Model to ModelResponse
    let model_responses: Vec<ModelResponse> = models.into_iter().map(|m| {
        ModelResponse {
            id: m.id.to_string(),
            name: m.name,
            model_type: m.model_type.as_str().to_string(),
            provider: m.provider,
            path: m.path,
            uri: m.uri,
            size_bytes: m.size_bytes,
            parameters: m.parameters,
            context_length: m.context_length,
            license: m.license,
            status: m.status.as_str().to_string(),
        }
    }).collect();

    (StatusCode::OK, Json(ModelListResponse {
        models: model_responses,
    })).into_response()
}

/// POST /api/v1/models/download - Download a model
async fn download_model(
    State(state): State<AppState>,
    Json(request): Json<DownloadModelRequest>,
) -> std::result::Result<Json<DownloadModelResponse>, (StatusCode, String)> {
    use crate::services::ModelDownloadService;
    use std::path::PathBuf;

    let download_service = ModelDownloadService::new();
    let output_path = request.output_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("models").join(format!("{}.gguf", request.name)));

    let download_id = download_service.download_model(
        request.name,
        request.url,
        output_path,
    ).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to download model: {}", e)))?;

    Ok(Json(DownloadModelResponse {
        download_id: download_id.to_string(),
        status: "started".to_string(),
    }))
}

/// POST /api/v1/models/benchmark - Benchmark a model
async fn benchmark_model(
    State(state): State<AppState>,
    Json(request): Json<BenchmarkRequest>,
) -> std::result::Result<Json<BenchmarkResponse>, (StatusCode, String)> {
    // TODO: Implement actual benchmarking
    // This requires model ID in request
    Err((
        StatusCode::BAD_REQUEST,
        "Benchmark endpoint requires model_id".to_string(),
    ))
}

/// POST /api/v1/models/ingest - Ingest a local model
async fn ingest_model(
    State(state): State<AppState>,
    Json(request): Json<IngestModelRequest>,
) -> std::result::Result<Json<ModelResponse>, (StatusCode, String)> {
    let db_path = &state.config.database.path;
    let conn = crate::db::init_database(db_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to initialize database: {}", e)))?;
    let service = NeuralService::new(conn);

    use noa_neural::ModelType;
    let model_type = match request.model_type.as_str() {
        "llm" => ModelType::LLM,
        "embedding" => ModelType::Embedding,
        "vision" => ModelType::Vision,
        "audio" => ModelType::Audio,
        _ => return Err((
            StatusCode::BAD_REQUEST,
            "Invalid model type. Must be: llm, embedding, vision, or audio".to_string(),
        )),
    };

    use crate::db::repositories::model_repository::{ModelType as RepoModelType, ModelStatus as RepoModelStatus};
    let repo_model_type = match model_type {
        noa_neural::ModelType::LLM => RepoModelType::LLM,
        noa_neural::ModelType::Embedding => RepoModelType::Embedding,
        noa_neural::ModelType::Vision => RepoModelType::Vision,
        noa_neural::ModelType::Audio => RepoModelType::Audio,
    };

    let model = RepoModel {
        id: Uuid::new_v4(),
        name: request.name.clone(),
        model_type: repo_model_type,
        provider: request.provider.clone(),
        path: Some(request.path.clone()),
        uri: None,
        size_bytes: None,
        parameters: None,
        context_length: Some(2048),
        license: None,
        config: serde_json::json!({}),
        status: RepoModelStatus::Available,
        metrics: None,
    };

    service.register_model(model.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to register model: {}", e)))?;

    Ok(Json(ModelResponse::from(model)))
}

/// POST /api/v1/models/:id/load - Load a model
async fn load_model(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> std::result::Result<StatusCode, (StatusCode, String)> {
    let db_path = &state.config.database.path;
    let conn = crate::db::init_database(db_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to initialize database: {}", e)))?;
    let service = NeuralService::new(conn);
    let model_id = Uuid::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, format!("Invalid UUID format: {}", id)))?;

    service.load_model(&model_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to load model: {}", e)))?;

    Ok(StatusCode::OK)
}

/// POST /api/v1/models/:id/unload - Unload a model
async fn unload_model(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> std::result::Result<StatusCode, (StatusCode, String)> {
    let db_path = &state.config.database.path;
    let conn = crate::db::init_database(db_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to initialize database: {}", e)))?;
    let service = NeuralService::new(conn);
    let model_id = Uuid::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, format!("Invalid UUID format: {}", id)))?;

    service.unload_model(&model_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to unload model: {}", e)))?;

    Ok(StatusCode::OK)
}

/// GET /api/v1/models/:id/status - Get model status
async fn get_model_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> std::result::Result<Json<ModelResponse>, (StatusCode, String)> {
    let db_path = &state.config.database.path;
    let conn = crate::db::init_database(db_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to initialize database: {}", e)))?;
    let service = NeuralService::new(conn);
    let model_id = Uuid::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, format!("Invalid UUID format: {}", id)))?;

    let model = service.get_model(&model_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get model: {}", e)))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Model not found: {}", id)))?;

    Ok(Json(ModelResponse::from(model)))
}

