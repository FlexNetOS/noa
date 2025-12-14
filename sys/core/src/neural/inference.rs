//! Inference Engine with Streaming
//!
//! T109: Implement inference engine with streaming
//! §3.2: Local-First & Offline-Capable
//! US2: Inference engine for neural runtime

use crate::error::{Result, NoaError};
use crate::neural::context::{InferenceContext, MessageRole};
use crate::neural::llama_backend::LlamaBackend;
use noa_neural::llama::{CompletionRequest, CompletionResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use futures::Stream;
use uuid::Uuid;

/// Inference request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub model_id: String,
    pub prompt: String,
    pub context_id: Option<Uuid>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub max_tokens: Option<usize>,
    pub stream: bool,
}

/// Inference response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub content: String,
    pub tokens_predicted: usize,
    pub tokens_evaluated: usize,
    pub generation_time_ms: f64,
    pub context_id: Option<Uuid>,
}

/// Streaming chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub content: String,
    pub done: bool,
}

/// Inference engine
pub struct InferenceEngine {
    backend: Arc<LlamaBackend>,
    contexts: Arc<tokio::sync::RwLock<crate::neural::context::ContextManager>>,
}

impl InferenceEngine {
    /// Create a new inference engine
    pub fn new(backend: Arc<LlamaBackend>) -> Self {
        Self {
            backend,
            contexts: Arc::new(tokio::sync::RwLock::new(
                crate::neural::context::ContextManager::new(),
            )),
        }
    }

    /// Run inference (non-streaming)
    pub async fn infer(&self, request: InferenceRequest) -> Result<InferenceResponse> {
        // Get or create context
        let mut context = if let Some(context_id) = request.context_id {
            let contexts = self.contexts.read().await;
            contexts.get_context(&context_id).await
                .ok_or_else(|| NoaError::NotFound {
                    resource: "InferenceContext".to_string(),
                    id: context_id.to_string(),
                })?
        } else {
            InferenceContext::new(request.model_id.clone(), 2048)
        };

        // Format prompt with context
        let formatted_prompt = context.format_prompt(&request.prompt);

        // Get llama client
        let client = self.backend.get_client(&request.model_id).await?;

        // Create completion request
        let completion_request = CompletionRequest {
            prompt: formatted_prompt.clone(),
            temperature: request.temperature,
            top_p: request.top_p,
            top_k: request.top_k,
            max_tokens: request.max_tokens,
            stop: None,
        };

        // Execute inference
        let completion_response = client.complete(completion_request).await
            .map_err(|e| NoaError::Internal {
                message: format!("Inference failed: {}", e),
                source: Some(Box::new(e)),
            })?;

        // Estimate tokens (rough approximation: 1 token ≈ 4 characters)
        let prompt_tokens = formatted_prompt.len() / 4;
        let response_tokens = completion_response.content.len() / 4;

        // Update context with user message and response
        context.add_message(MessageRole::User, request.prompt, prompt_tokens)?;
        context.add_message(MessageRole::Assistant, completion_response.content.clone(), response_tokens)?;

        // Save context if it was provided
        if let Some(context_id) = request.context_id {
            let contexts = self.contexts.write().await;
            contexts.update_context(context).await?;
        }

        Ok(InferenceResponse {
            content: completion_response.content,
            tokens_predicted: completion_response.tokens_predicted,
            tokens_evaluated: completion_response.tokens_evaluated,
            generation_time_ms: completion_response.generation_time_ms,
            context_id: request.context_id,
        })
    }

    /// Run inference with streaming
    pub async fn infer_stream(
        &self,
        request: InferenceRequest,
    ) -> Result<impl Stream<Item = Result<StreamChunk>>> {
        // For now, return a simple stream that yields the full response
        // TODO: Implement actual streaming from llama-server
        let response = self.infer(request).await?;

        let chunk = Ok(StreamChunk {
            content: response.content,
            done: true,
        });

        // Use futures::stream::once for single-item stream
        Ok(futures::stream::once(async move { chunk }))
    }

    /// Create a new inference context
    pub async fn create_context(&self, model_id: String, max_context_length: usize) -> Uuid {
        let contexts = self.contexts.write().await;
        contexts.create_context(model_id, max_context_length).await
    }

    /// Get inference context
    pub async fn get_context(&self, context_id: &Uuid) -> Option<InferenceContext> {
        let contexts = self.contexts.read().await;
        contexts.get_context(context_id).await
    }

    /// Delete inference context
    pub async fn delete_context(&self, context_id: &Uuid) {
        let contexts = self.contexts.write().await;
        contexts.delete_context(context_id).await;
    }
}

