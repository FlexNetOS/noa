//! HTTP server implementation with axum

use anyhow::Result;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{sse::{Event, Sse}, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use futures::stream::Stream;
use std::convert::Infallible;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::models::ModelManager;
use crate::types::*;

/// Inference server state
#[derive(Clone)]
pub struct AppState {
    pub model_manager: Arc<ModelManager>,
}

/// Main inference server
pub struct InferenceServer {
    port: u16,
    host: String,
}

impl InferenceServer {
    /// Create a new inference server
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    /// Run the server
    pub async fn run(self, model_manager: Arc<ModelManager>) -> Result<()> {
        let state = AppState { model_manager };

        // Build the router
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/v1/models", get(list_models))
            .route("/v1/chat/completions", post(chat_completions))
            .route("/v1/moe/classify", post(moe_classify))
            .route("/v1/moe/route", post(moe_route))
            .route("/v1/moe/stats", get(moe_stats))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
            .with_state(state);

        let addr = format!("{}:{}", self.host, self.port);
        info!("🚀 Inference server starting on {}", addr);
        info!("📊 Health check: http://{}/health", addr);
        info!("🤖 Chat completions: http://{}/v1/chat/completions", addr);
        info!("🧠 MOE Classification: http://{}/v1/moe/classify", addr);
        info!("🎯 MOE Routing: http://{}/v1/moe/route", addr);
        info!("📈 MOE Stats: http://{}/v1/moe/stats", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let model_loaded = state.model_manager.is_model_loaded().await;

    // Get system information
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_all();

    let system_info = SystemInfo {
        total_memory_gb: sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        available_memory_gb: sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        cpu_cores: sys.cpus().len(),
        hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
    };

    Json(HealthResponse {
        status: "ok".to_string(),
        model_loaded,
        models_available: vec![
            "qwen3-1.7b".to_string(),  // Default: Fast, 32K context, AI PC optimized
            "llama-3.2-1b".to_string(),
            "phi-3-mini".to_string(),
        ],
        system_info,
    })
}

/// List available models
async fn list_models(State(state): State<AppState>) -> Json<ModelsResponse> {
    let mut models = vec![];

    if let Some(model_info) = state.model_manager.get_model_info().await {
        models.push(model_info);
    }

    // Add available models
    let available_models = vec![
        ("qwen3-1.7b", "1.7B", 32768),  // Qwen3: Smaller, faster, longer context
        ("llama-3.2-1b", "1B", 4096),
        ("phi-3-mini", "3.8B", 4096),
    ];

    for (name, params, ctx_len) in available_models {
        if models.iter().any(|m| m.id == name) {
            continue;
        }
        models.push(ModelInfo {
            id: name.to_string(),
            object: "model".to_string(),
            created: chrono::Utc::now().timestamp(),
            owned_by: "local".to_string(),
            parameters: Some(params.to_string()),
            context_length: ctx_len,
        });
    }

    Json(ModelsResponse {
        object: "list".to_string(),
        data: models,
    })
}

/// Chat completions endpoint
async fn chat_completions(
    State(state): State<AppState>,
    Json(request): Json<ChatCompletionRequest>,
) -> Response {
    // Ensure model is loaded
    if !state.model_manager.is_model_loaded().await {
        // Auto-load default model
        if let Err(e) = state.model_manager.load_model(&request.model).await {
            return error_response(
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Failed to load model: {}", e),
            );
        }
    }

    // Handle streaming vs non-streaming
    if request.stream {
        match state.model_manager.generate_stream(request).await {
            Ok(rx) => {
                let stream = stream_chat_completion(rx);
                Sse::new(stream).into_response()
            }
            Err(e) => error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Streaming failed: {}", e),
            ),
        }
    } else {
        match state.model_manager.generate_completion(request).await {
            Ok(response) => Json(response).into_response(),
            Err(e) => error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Completion failed: {}", e),
            ),
        }
    }
}

/// Stream chat completion chunks
fn stream_chat_completion(
    mut rx: tokio::sync::mpsc::Receiver<Result<ChatCompletionChunk>>,
) -> impl Stream<Item = Result<Event, Infallible>> {
    async_stream::stream! {
        while let Some(result) = rx.recv().await {
            match result {
                Ok(chunk) => {
                    let json = serde_json::to_string(&chunk).unwrap_or_default();
                    yield Ok(Event::default().data(format!("data: {}\n\n", json)));
                }
                Err(e) => {
                    tracing::error!("Stream error: {}", e);
                    break;
                }
            }
        }
        // Send done signal
        yield Ok(Event::default().data("data: [DONE]\n\n"));
    }
}

/// Create an error response
/// MOE: Classify query
async fn moe_classify(
    State(_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, Response> {
    let query = payload["query"]
        .as_str()
        .ok_or_else(|| error_response(StatusCode::BAD_REQUEST, "Missing 'query' field".to_string()))?;
    
    let classification = ModelManager::classify_query(query);
    
    Ok(Json(serde_json::json!({
        "query": query,
        "specialization": classification.specialization.as_str(),
        "confidence": classification.confidence,
        "keywords": classification.keywords,
        "reasoning": classification.reasoning,
    })))
}

/// MOE: Route query to expert
async fn moe_route(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, Response> {
    let query = payload["query"]
        .as_str()
        .ok_or_else(|| error_response(StatusCode::BAD_REQUEST, "Missing 'query' field".to_string()))?;
    
    let (specialization, model_id, confidence) = state.model_manager
        .route_query(query)
        .await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "query": query,
        "routed_to": {
            "specialization": specialization.as_str(),
            "model_id": model_id,
            "confidence": confidence,
        },
    })))
}

/// MOE: Get statistics
async fn moe_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, Response> {
    let stats = state.model_manager.get_moe_stats().await;
    
    Ok(Json(serde_json::json!({
        "moe": {
            "enabled": stats.enabled,
            "total_experts": stats.total_experts,
            "parallel_consultation": {
                "enabled": stats.parallel_enabled,
                "max_parallel": stats.max_parallel,
            },
            "aggregation_strategy": format!("{:?}", stats.aggregation),
            "specializations": stats.specializations,
        },
    })))
}

fn error_response(status: StatusCode, message: String) -> Response {
    let error = ErrorResponse {
        error: ErrorDetail {
            message,
            r#type: "server_error".to_string(),
            code: Some(status.as_str().to_string()),
        },
    };
    (status, Json(error)).into_response()
}
