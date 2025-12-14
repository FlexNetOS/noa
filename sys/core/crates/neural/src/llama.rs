//! llama.cpp integration

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaServerConfig {
    pub host: String,
    pub port: u16,
    pub model_path: PathBuf,
    pub context_size: usize,
    pub n_gpu_layers: i32,
    pub threads: Option<usize>,
}

impl Default for LlamaServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            model_path: PathBuf::from("models/default.gguf"),
            context_size: 2048,
            n_gpu_layers: 0,
            threads: None,
        }
    }
}

pub struct LlamaServer {
    config: LlamaServerConfig,
    process: Option<Child>,
}

impl LlamaServer {
    pub fn new(config: LlamaServerConfig) -> Self {
        Self {
            config,
            process: None,
        }
    }
    
    pub fn start(&mut self) -> Result<()> {
        let llama_server_path = PathBuf::from("opt/llama.cpp/build/bin/llama-server.exe");
        
        if !llama_server_path.exists() {
            anyhow::bail!("llama-server not found at {:?}", llama_server_path);
        }
        
        let mut cmd = Command::new(&llama_server_path);
        cmd.arg("--host").arg(&self.config.host)
            .arg("--port").arg(self.config.port.to_string())
            .arg("--model").arg(&self.config.model_path)
            .arg("--ctx-size").arg(self.config.context_size.to_string())
            .arg("--n-gpu-layers").arg(self.config.n_gpu_layers.to_string());
        
        if let Some(threads) = self.config.threads {
            cmd.arg("--threads").arg(threads.to_string());
        }
        
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        let child = cmd.spawn()
            .context("Failed to start llama-server")?;
        
        self.process = Some(child);
        
        tracing::info!("llama-server started on {}:{}", self.config.host, self.config.port);
        
        Ok(())
    }
    
    pub fn stop(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            process.kill().context("Failed to kill llama-server")?;
            tracing::info!("llama-server stopped");
        }
        Ok(())
    }
    
    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }
    
    pub fn base_url(&self) -> String {
        format!("http://{}:{}", self.config.host, self.config.port)
    }
}

impl Drop for LlamaServer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub prompt: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub max_tokens: Option<usize>,
    pub stop: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub content: String,
    pub tokens_predicted: usize,
    pub tokens_evaluated: usize,
    pub generation_time_ms: f64,
}

pub struct LlamaClient {
    base_url: String,
    client: reqwest::Client,
}

impl LlamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let url = format!("{}/completion", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send completion request")?;
        
        let result: CompletionResponse = response
            .json()
            .await
            .context("Failed to parse completion response")?;
        
        Ok(result)
    }
    
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}
