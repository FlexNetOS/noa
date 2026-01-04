use anyhow::Context;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::db::Database;
use noa_neural::llama::{LlamaServer, LlamaServerconfigs};

#[derive(Clone)]
pub struct AppState {
    pub llama: Arc<LlamaProxy>,
    pub db: Arc<Database>,
}

impl AppState {
    pub fn from_env(db: Arc<Database>) -> Self {
        let host = std::env::var("NOA_LLAMA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("NOA_LLAMA_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(8080);

        let model_path = std::env::var("NOA_LLAMA_MODEL_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("opt/models/default.gguf"));

        let context_size = std::env::var("NOA_LLAMA_CTX_SIZE")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(2048);

        let n_gpu_layers = std::env::var("NOA_LLAMA_N_GPU_LAYERS")
            .ok()
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0);

        let threads = std::env::var("NOA_LLAMA_THREADS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok());

        let auto_start = match std::env::var("NOA_LLAMA_AUTO_START") {
            Ok(v) => matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"),
            Err(_) => true,
        };

        let configs = LlamaServerconfigs {
            host: host.clone(),
            port,
            model_path,
            context_size,
            n_gpu_layers,
            threads,
        };

        let base_url = format!("http://{}:{}", host, port);

        Self {
            llama: Arc::new(LlamaProxy {
                base_url,
                client: reqwest::Client::new(),
                auto_start,
                server: Mutex::new(LlamaServer::new(configs)),
            }),
            db,
        }
    }
}

pub struct LlamaProxy {
    base_url: String,
    client: reqwest::Client,
    auto_start: bool,
    server: Mutex<LlamaServer>,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyTemplateRequest {
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyTemplateResponse {
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl LlamaProxy {
    pub async fn ensure_ready(&self) -> Result<(), (StatusCode, ApiErrorBody)> {
        // Fast path: if /health says OK, we're good.
        if self.health().await.unwrap_or(false) {
            return Ok(());
        }

        if !self.auto_start {
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                ApiErrorBody {
                    error: "llama-server is not healthy and NOA_LLAMA_AUTO_START=false".to_string(),
                },
            ));
        }

        {
            let mut server = self.server.lock().await;
            if !server.is_running() {
                server.start().map_err(|e| {
                    (
                        StatusCode::SERVICE_UNAVAILABLE,
                        ApiErrorBody {
                            error: format!("Failed to start llama-server: {e}"),
                        },
                    )
                })?;
            }
        }

        // Wait briefly for /health to become OK.
        for _ in 0..40 {
            if self.health().await.unwrap_or(false) {
                return Ok(());
            }
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        }

        Err((
            StatusCode::SERVICE_UNAVAILABLE,
            ApiErrorBody {
                error: "llama-server did not become healthy in time".to_string(),
            },
        ))
    }

    pub async fn health(&self) -> anyhow::Result<bool> {
        let url = format!("{}/health", self.base_url);
        let resp = self.client.get(url).send().await?;
        Ok(resp.status().is_success())
    }

    pub async fn apply_template(&self, messages: Vec<ChatMessage>) -> anyhow::Result<String> {
        let url = format!("{}/apply-template", self.base_url);
        let resp = self
            .client
            .post(url)
            .json(&ApplyTemplateRequest { messages })
            .send()
            .await
            .context("Failed to call llama-server /apply-template")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("/apply-template failed with {status}: {body}");
        }

        let parsed: ApplyTemplateResponse = resp
            .json()
            .await
            .context("Failed to parse /apply-template response")?;
        Ok(parsed.prompt)
    }

    pub async fn completion(&self, prompt: String) -> anyhow::Result<serde_json::Value> {
        let url = format!("{}/completion", self.base_url);
        let resp = self
            .client
            .post(url)
            .json(&serde_json::json!({
                "prompt": prompt,
                "stream": false,
                "n_predict": 512
            }))
            .send()
            .await
            .context("Failed to call llama-server /completion")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("/completion failed with {status}: {body}");
        }

        Ok(resp.json().await.context("Failed to parse /completion response")?)
    }

    pub async fn completion_stream(&self, prompt: String) -> anyhow::Result<reqwest::Response> {
        let url = format!("{}/completion", self.base_url);
        let resp = self
            .client
            .post(url)
            .json(&serde_json::json!({
                "prompt": prompt,
                "stream": true,
                "n_predict": 512
            }))
            .send()
            .await
            .context("Failed to call llama-server /completion (stream)")?;

        Ok(resp)
    }
}
