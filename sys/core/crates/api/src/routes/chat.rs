use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::state::{ApiErrorBody, AppState, ChatMessage};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub provider: Option<String>,
    pub message: String,
    pub history: Option<Vec<ChatMessage>>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub content: String,
    pub provider: String,
    pub model: Option<String>,
}

fn provider_or_default(provider: Option<String>) -> String {
    provider.unwrap_or_else(|| "llama.cpp".to_string())
}

fn to_llama_messages(mut history: Vec<ChatMessage>, message: String) -> Vec<ChatMessage> {
    history.push(ChatMessage {
        role: "user".to_string(),
        content: message,
    });
    history
}

pub async fn chat(State(state): State<AppState>, Json(payload): Json<ChatRequest>) -> impl IntoResponse {
    let provider = provider_or_default(payload.provider);

    if provider != "llama.cpp" {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorBody {
                error: format!("Unsupported provider: {provider}"),
            }),
        )
            .into_response();
    }

    if let Err((status, body)) = state.llama.ensure_ready().await {
        return (status, Json(body)).into_response();
    }

    let history = payload.history.unwrap_or_default();
    let messages = to_llama_messages(history, payload.message);

    let prompt = match state.llama.apply_template(messages).await {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("llama-server /apply-template error: {e}"),
                }),
            )
                .into_response();
        }
    };

    let completion = match state.llama.completion(prompt).await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("llama-server /completion error: {e}"),
                }),
            )
                .into_response();
        }
    };

    let content = completion
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let model = completion
        .get("model")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Json(ChatResponse {
        content,
        provider,
        model,
    })
    .into_response()
}

pub async fn chat_stream(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let provider = provider_or_default(payload.provider);

    if provider != "llama.cpp" {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorBody {
                error: format!("Unsupported provider: {provider}"),
            }),
        )
            .into_response();
    }

    if let Err((status, body)) = state.llama.ensure_ready().await {
        return (status, Json(body)).into_response();
    }

    let history = payload.history.unwrap_or_default();
    let messages = to_llama_messages(history, payload.message);

    let prompt = match state.llama.apply_template(messages).await {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("llama-server /apply-template error: {e}"),
                }),
            )
                .into_response();
        }
    };

    let upstream = match state.llama.completion_stream(prompt).await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("llama-server /completion(stream) error: {e}"),
                }),
            )
                .into_response();
        }
    };

    if !upstream.status().is_success() {
        let status = upstream.status();
        let body = upstream.text().await.unwrap_or_default();
        return (
            StatusCode::BAD_GATEWAY,
            Json(ApiErrorBody {
                error: format!("llama-server returned {status}: {body}"),
            }),
        )
            .into_response();
    }

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/event-stream"),
    );
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    let stream = upstream
        .bytes_stream()
        .map(|chunk| chunk.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
        ;

    let mut response = Response::new(Body::from_stream(stream));
    *response.status_mut() = StatusCode::OK;
    *response.headers_mut() = headers;

    response.into_response()
}
