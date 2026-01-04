//! llama.cpp integration

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::path::PathBuf;

fn resolve_noa_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("NOA_ROOT") {
        let p = PathBuf::from(root);
        if p.exists() {
            return Some(p);
        }
    }

    // Best-effort: walk up from current working dir looking for a repo marker.
    let mut dir = std::env::current_dir().ok()?;
    for _ in 0..10 {
        if dir.join("pnpm-workspace.yaml").exists() || dir.join(".git").exists() || dir.join("AGENT.md").exists() {
            return Some(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

fn resolve_llama_server_path() -> PathBuf {
    if let Ok(p) = std::env::var("NOA_LLAMA_SERVER_PATH") {
        return PathBuf::from(p);
    }

    let bin_name = if cfg!(windows) { "llama-server.exe" } else { "llama-server" };
    if let Some(root) = resolve_noa_root() {
        return root.join("opt/llama.cpp/build/bin").join(bin_name);
    }

    PathBuf::from(bin_name)
}

fn resolve_model_path(model_path: &PathBuf) -> PathBuf {
    if let Ok(p) = std::env::var("NOA_LLAMA_MODEL_PATH") {
        return PathBuf::from(p);
    }

    if model_path.is_absolute() {
        return model_path.clone();
    }

    if let Some(root) = resolve_noa_root() {
        return root.join(model_path);
    }

    model_path.clone()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaServerconfigs {
    pub host: String,
    pub port: u16,
    pub model_path: PathBuf,
    pub context_size: usize,
    pub n_gpu_layers: i32,
    pub threads: Option<usize>,
}

impl Default for LlamaServerconfigs {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            // Resolved at runtime via NOA_ROOT/NOA_LLAMA_MODEL_PATH.
            model_path: PathBuf::from("opt/models/default.gguf"),
            context_size: 2048,
            n_gpu_layers: 0,
            threads: None,
        }
    }
}

pub struct LlamaServer {
    configs: LlamaServerconfigs,
    process: Option<Child>,
}

impl LlamaServer {
    pub fn new(configs: LlamaServerconfigs) -> Self {
        Self {
            configs,
            process: None,
        }
    }
    
    pub fn start(&mut self) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        let llama_server_path = resolve_llama_server_path();
        let model_path = resolve_model_path(&self.configs.model_path);
        
        if !llama_server_path.exists() {
            anyhow::bail!(
                "llama-server not found at {:?}. Set NOA_LLAMA_SERVER_PATH or NOA_ROOT.",
                llama_server_path
            );
        }

        if !model_path.exists() {
            anyhow::bail!(
                "Model file not found at {:?}. Set NOA_LLAMA_MODEL_PATH or place a .gguf at opt/models/default.gguf under NOA_ROOT.",
                model_path
            );
        }
        
        let mut cmd = Command::new(&llama_server_path);
        cmd.arg("--host").arg(&self.configs.host)
            .arg("--port").arg(self.configs.port.to_string())
            .arg("--model").arg(&model_path)
            .arg("--ctx-size").arg(self.configs.context_size.to_string())
            .arg("--n-gpu-layers").arg(self.configs.n_gpu_layers.to_string());
        
        if let Some(threads) = self.configs.threads {
            cmd.arg("--threads").arg(threads.to_string());
        }
        
        // Avoid deadlocks: piping stdout/stderr without draining can block long-running servers.
        // Use inherit so logs are visible and buffers can't fill.
        cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        
        let child = cmd.spawn()
            .context("Failed to start llama-server")?;
        
        self.process = Some(child);
        
        tracing::info!("llama-server started on {}:{}", self.configs.host, self.configs.port);
        
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
        format!("http://{}:{}", self.configs.host, self.configs.port)
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
