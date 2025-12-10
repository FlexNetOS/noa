//! Neural Runtime - Multi-SLM orchestration

use crate::llama::{CompletionRequest, LlamaClient, LlamaServer, LlamaServerConfig};
use crate::model::{Model, ModelConfig, ModelStatus};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct NeuralRuntime {
    models: Arc<RwLock<HashMap<String, Model>>>,
    llama_servers: Arc<RwLock<HashMap<String, LlamaServer>>>,
    llama_clients: Arc<RwLock<HashMap<String, LlamaClient>>>,
}

impl NeuralRuntime {
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            llama_servers: Arc::new(RwLock::new(HashMap::new())),
            llama_clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_model(&self, model: Model) -> Result<()> {
        let mut models = self.models.write().await;
        models.insert(model.id.clone(), model);
        Ok(())
    }

    pub async fn load_model(&self, model_id: &str) -> Result<()> {
        let mut models = self.models.write().await;
        let model = models.get_mut(model_id).context("Model not found")?;

        model.status = ModelStatus::Loading;

        if model.config.provider == "llama.cpp" {
            let model_path = model.config.file_path.clone().context("Model file path not set")?;
            let context_length = model.config.context_length;
            let n_gpu_layers = model.config.n_gpu_layers;
            let num_models = models.len();

            drop(models);

            let config = LlamaServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080 + num_models as u16,
                model_path: model_path.into(),
                context_size: context_length,
                n_gpu_layers,
                threads: None,
            };

            let mut server = LlamaServer::new(config.clone());
            server.start().context("Failed to start llama server")?;

            let client = LlamaClient::new(server.base_url());

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            if !client.health_check().await? {
                let mut models = self.models.write().await;
                if let Some(model) = models.get_mut(model_id) {
                    model.status = ModelStatus::Failed("Health check failed".to_string());
                }
                return Err(anyhow::anyhow!("Model health check failed"));
            }

            let mut servers = self.llama_servers.write().await;
            servers.insert(model_id.to_string(), server);

            let mut clients = self.llama_clients.write().await;
            clients.insert(model_id.to_string(), client);

            let mut models = self.models.write().await;
            if let Some(model) = models.get_mut(model_id) {
                model.status = ModelStatus::Loaded;
            }

            tracing::info!("Model {} loaded successfully", model_id);
        } else {
            model.status = ModelStatus::Failed("Unsupported provider".to_string());
            return Err(anyhow::anyhow!(
                "Unsupported provider: {}",
                model.config.provider
            ));
        }

        Ok(())
    }

    pub async fn unload_model(&self, model_id: &str) -> Result<()> {
        let mut servers = self.llama_servers.write().await;
        if let Some(mut server) = servers.remove(model_id) {
            server.stop()?;
        }

        let mut clients = self.llama_clients.write().await;
        clients.remove(model_id);

        let mut models = self.models.write().await;
        if let Some(model) = models.get_mut(model_id) {
            model.status = ModelStatus::Available;
        }

        tracing::info!("Model {} unloaded", model_id);

        Ok(())
    }

    pub async fn generate(&self, model_id: &str, prompt: String) -> Result<String> {
        let clients = self.llama_clients.read().await;
        let client = clients.get(model_id).context("Model not loaded")?;

        let request = CompletionRequest {
            prompt,
            temperature: Some(0.7),
            top_p: Some(0.9),
            top_k: Some(40),
            max_tokens: Some(512),
            stop: None,
        };

        let response = client.complete(request).await?;

        Ok(response.content)
    }

    pub async fn list_models(&self) -> Vec<Model> {
        let models = self.models.read().await;
        models.values().cloned().collect()
    }

    pub async fn get_model_status(&self, model_id: &str) -> Option<ModelStatus> {
        let models = self.models.read().await;
        models.get(model_id).map(|m| m.status.clone())
    }
}

impl Default for NeuralRuntime {
    fn default() -> Self {
        Self::new()
    }
}
