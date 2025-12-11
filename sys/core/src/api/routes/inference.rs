//! Inference API Routes
//!
//! T127-T128: Inference API endpoints
//! US2: API endpoints for model inference

use axum::{
    extract::State,
    http::StatusCode,
    response::{sse::Event, IntoResponse, Json, Response, Sse},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::api::server::AppState;
use crate::neural::inference::{InferenceRequest, InferenceResponse};
use crate::services::NeuralService;

/// Inference request
#[derive(Deserialize)]
pub struct InferenceApiRequest {
    pub model_id: String,
    pub prompt: String,
    pub context_id: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub max_tokens: Option<usize>,
}

/// Inference response
#[derive(Serialize)]
pub struct InferenceApiResponse {
    pub content: String,
    pub tokens_predicted: usize,
    pub tokens_evaluated: usize,
    pub generation_time_ms: f64,
    pub context_id: Option<String>,
}

impl From<InferenceResponse> for InferenceApiResponse {
    fn from(response: InferenceResponse) -> Self {
        Self {
            content: response.content,
            tokens_predicted: response.tokens_predicted,
            tokens_evaluated: response.tokens_evaluated,
            generation_time_ms: response.generation_time_ms,
            context_id: response.context_id.map(|id| id.to_string()),
        }
    }
}

/// Create routes for inference
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/inference", post(infer))
        .route("/inference/stream", post(infer_stream))
}

/// POST /api/v1/inference - Run inference
async fn infer(
    State(state): State<AppState>,
    Json(request): Json<InferenceApiRequest>,
) -> impl IntoResponse {
    let db_path = &state.config.database.path;
    let conn = match crate::db::init_database(db_path) {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize database: {}", e),
            )
                .into_response()
        }
    };
    let service = NeuralService::new(conn);
    let engine = service.inference_engine();

    let context_id = match request.context_id.map(|c| Uuid::parse_str(&c)).transpose() {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                "Invalid UUID format for context_id".to_string(),
            )
                .into_response()
        }
    };

    let inference_request = InferenceRequest {
        model_id: request.model_id,
        prompt: request.prompt,
        context_id,
        temperature: request.temperature,
        top_p: request.top_p,
        top_k: request.top_k,
        max_tokens: request.max_tokens,
        stream: false,
    };

    let response = match engine.infer(inference_request).await {
        Ok(resp) => resp,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Inference failed: {}", e),
            )
                .into_response()
        }
    };

    (StatusCode::OK, Json(InferenceApiResponse::from(response))).into_response()
}

/// POST /api/v1/inference/stream - Run inference with streaming
async fn infer_stream(
    State(state): State<AppState>,
    Json(request): Json<InferenceApiRequest>,
) -> Response {
    let db_path = &state.config.database.path;
    let conn = match crate::db::init_database(db_path) {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to initialize database: {}", e),
            )
                .into_response()
        }
    };
    let service = NeuralService::new(conn);
    let engine = service.inference_engine();

    let context_id = match request.context_id.map(|c| Uuid::parse_str(&c)).transpose() {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                "Invalid UUID format for context_id".to_string(),
            )
                .into_response()
        }
    };

    let inference_request = InferenceRequest {
        model_id: request.model_id,
        prompt: request.prompt,
        context_id,
        temperature: request.temperature,
        top_p: request.top_p,
        top_k: request.top_k,
        max_tokens: request.max_tokens,
        stream: true,
    };

    let stream = match engine.infer_stream(inference_request).await {
        Ok(stream) => stream,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Inference stream failed: {}", e),
            )
                .into_response()
        }
    };

    // Convert to SSE events
    let sse_stream = stream.map(|chunk_result| match chunk_result {
        Ok(chunk) => {
            let data = serde_json::json!({
                "content": chunk.content,
                "done": chunk.done,
            });
            Ok::<Event, Infallible>(Event::default().data(data.to_string()))
        }
        Err(e) => Ok(Event::default().data(
            serde_json::json!({
                "error": e.to_string(),
                "done": true,
            })
            .to_string(),
        )),
    });

    Sse::new(sse_stream).into_response()
}
