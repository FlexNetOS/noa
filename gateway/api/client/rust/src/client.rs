//! API client implementation.

use reqwest::Client as HttpClient;
use futures::StreamExt;
use tracing::debug;

use crate::types::*;
use crate::{Error, Result};

/// NOA API client.
pub struct Client {
    http: HttpClient,
    base_url: String,
}

impl Client {
    /// Create a new client with the given base URL.
    pub fn new(base_url: &str) -> Self {
        Self {
            http: HttpClient::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Create a client with the default endpoint.
    pub fn default() -> Self {
        Self::new(crate::DEFAULT_ENDPOINT)
    }

    /// Health check.
    pub async fn health(&self) -> Result<HealthResponse> {
        let url = format!("{}/health", self.base_url);
        debug!(url = %url, "Health check");
        
        let response = self.http.get(&url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: body });
        }
        
        Ok(response.json().await?)
    }

    /// Get system status.
    pub async fn status(&self) -> Result<StatusResponse> {
        let url = format!("{}/api/v1/status", self.base_url);
        debug!(url = %url, "Status check");
        
        let response = self.http.get(&url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: body });
        }
        
        Ok(response.json().await?)
    }

    /// Send a chat message.
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/api/v1/chat", self.base_url);
        debug!(url = %url, provider = ?request.provider, "Chat request");
        
        let response = self.http.post(&url).json(&request).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body: ErrorResponse = response.json().await.unwrap_or_else(|_| ErrorResponse {
                error: "Unknown error".to_string(),
                code: None,
            });
            return Err(Error::Api { status: status.as_u16(), message: body.error });
        }
        
        Ok(response.json().await?)
    }

    /// Send a chat message with streaming response.
    pub async fn chat_stream(&self, request: ChatRequest) -> Result<impl futures::Stream<Item = Result<ChatStreamEvent>>> {
        let url = format!("{}/api/v1/chat/stream", self.base_url);
        debug!(url = %url, provider = ?request.provider, "Chat stream request");
        
        let response = self.http.post(&url).json(&request).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: body });
        }
        
        let stream = response.bytes_stream().map(|result| {
            match result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    // Parse SSE format
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if let Ok(event) = serde_json::from_str::<ChatStreamEvent>(data) {
                                return Ok(event);
                            }
                        }
                    }
                    Ok(ChatStreamEvent {
                        event: "token".to_string(),
                        data: text.to_string(),
                    })
                }
                Err(e) => Err(Error::Http(e)),
            }
        });
        
        Ok(stream)
    }

    /// List providers.
    pub async fn list_providers(&self) -> Result<ProvidersResponse> {
        let url = format!("{}/api/v1/providers", self.base_url);
        debug!(url = %url, "List providers");
        
        let response = self.http.get(&url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: body });
        }
        
        Ok(response.json().await?)
    }

    /// List models for a provider.
    pub async fn list_models(&self, provider_id: &str) -> Result<ModelsResponse> {
        let url = format!("{}/api/v1/providers/{}/models", self.base_url, provider_id);
        debug!(url = %url, "List models");
        
        let response = self.http.get(&url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: body });
        }
        
        Ok(response.json().await?)
    }

    /// List tasks.
    pub async fn list_tasks(&self, status: Option<&str>, limit: Option<u32>) -> Result<TasksResponse> {
        let mut url = format!("{}/api/v1/tasks", self.base_url);
        
        let mut params = vec![];
        if let Some(s) = status {
            params.push(format!("status={}", s));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }
        
        debug!(url = %url, "List tasks");
        
        let response = self.http.get(&url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: body });
        }
        
        Ok(response.json().await?)
    }

    /// Create a task.
    pub async fn create_task(&self, request: CreateTaskRequest) -> Result<TaskResponse> {
        let url = format!("{}/api/v1/tasks", self.base_url);
        debug!(url = %url, "Create task");
        
        let response = self.http.post(&url).json(&request).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: body });
        }
        
        Ok(response.json().await?)
    }

    /// Get task details.
    pub async fn get_task(&self, task_id: &str) -> Result<TaskResponse> {
        let url = format!("{}/api/v1/tasks/{}", self.base_url, task_id);
        debug!(url = %url, "Get task");
        
        let response = self.http.get(&url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: body });
        }
        
        Ok(response.json().await?)
    }
}
