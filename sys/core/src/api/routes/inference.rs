//! Inference API Routes
//!
//! T127-T128: Inference API endpoints
//! US2: API endpoints for model inference

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{sse::Event, Sse},
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio_stream::Stream;
use uuid::Uuid;

use crate::api::server::AppState;
use crate::neural::inference::{InferenceRequest, InferenceResponse};
use crate::error::{ApiError, NoaError, Result};
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
pub fn routes() -> Router {
    Router::new()
        .route("/inference", post(infer))
        .route("/inference/stream", post(infer_stream))
}

/// POST /api/v1/inference - Run inference
async fn infer(
    Extension(state): Extension<AppState>,
    Json(request): Json<InferenceApiRequest>,
) -> Result<Json<InferenceApiResponse>> {
    let engine = {
        let conn = state.sqlite_conn()?;
        let service = NeuralService::new(conn.connection());
        service.inference_engine()
    };

    let context_id = request.context_id
        .map(|c| Uuid::parse_str(&c))
        .transpose()
        .map_err(|_| NoaError::Validation(crate::error::ValidationError::new(
            "context_id",
            "Invalid UUID format",
            "INVALID_UUID",
        )))?;

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

    let response = engine.infer(inference_request).await?;

    Ok(Json(InferenceApiResponse::from(response)))
}

/// POST /api/v1/inference/stream - Run inference with streaming
async fn infer_stream(
    Extension(state): Extension<AppState>,
    Json(request): Json<InferenceApiRequest>,
) -> std::result::Result<Sse<impl Stream<Item = std::result::Result<Event, Infallible>>>, (StatusCode, String)> {
    let engine = {
        let conn = state.sqlite_conn().map_err(|e| {
            let status = match &e {
                NoaError::Api(ApiError::ServiceUnavailable(_)) => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, format!("Failed to acquire database connection: {}", e))
        })?;
        let service = NeuralService::new(conn.connection());
        service.inference_engine()
    };

    let context_id = request.context_id
        .map(|c| Uuid::parse_str(&c))
        .transpose()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

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

    let stream = engine.infer_stream(inference_request).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Inference failed: {}", e)))?;

    // Convert to SSE events
    let sse_stream = tokio_stream::StreamExt::map(stream, |chunk_result| {
        match chunk_result {
            Ok(chunk) => {
                let data = serde_json::json!({
                    "content": chunk.content,
                    "done": chunk.done,
                });
                Ok(Event::default().data(data.to_string()))
            }
            Err(e) => {
                Ok(Event::default().data(serde_json::json!({
                    "error": e.to_string(),
                    "done": true,
                }).to_string()))
            }
        }
    });

    Ok(Sse::new(sse_stream))
}

