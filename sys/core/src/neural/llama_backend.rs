//! llama-cpp-rs Bindings Integration
//!
//! T106: Integrate llama-cpp-rs bindings
//! §3.2: Local-First & Offline-Capable
//! US2: Neural runtime backend

use crate::error::{NoaError, Result};
use crate::neural::model_loader::{ModelLoader, ModelLoaderConfig};
use noa_neural::llama::{LlamaClient, LlamaServer, LlamaServerConfig};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// llama.cpp backend for model inference
pub struct LlamaBackend {
    servers: Arc<RwLock<std::collections::HashMap<String, LlamaServer>>>,
    clients: Arc<RwLock<std::collections::HashMap<String, Arc<LlamaClient>>>>,
    model_loader: ModelLoader,
}

impl LlamaBackend {
    /// Create a new llama backend
    pub fn new() -> Self {
        Self {
            servers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            clients: Arc::new(RwLock::new(std::collections::HashMap::new())),
            model_loader: ModelLoader::new(),
        }
    }

    /// Load a model using the backend
    pub async fn load_model(
        &self,
        model_id: &str,
        model_path: PathBuf,
        context_size: usize,
        n_gpu_layers: i32,
    ) -> Result<()> {
        // Use model loader to validate and prepare model
        let config = ModelLoaderConfig {
            model_path: model_path.clone(),
            context_size,
            n_gpu_layers,
            threads: None,
            auto_detect_gpu_layers: true,
        };

        let _loaded_model = self.model_loader.load_gguf(&config).await?;

        // Create llama server configuration
        let server_config = LlamaServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080, // TODO: Dynamic port assignment
            model_path,
            context_size,
            n_gpu_layers,
            threads: None,
        };

        let mut server = LlamaServer::new(server_config.clone());
        server.start().map_err(|e| NoaError::Internal {
            message: format!("Failed to start llama server: {}", e),
            source: None,
        })?;

        let client = Arc::new(LlamaClient::new(server.base_url()));

        // Wait for server to be ready
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Health check
        if !client.health_check().await.map_err(|e| NoaError::Internal {
            message: format!("Health check failed: {}", e),
            source: None,
        })? {
            return Err(NoaError::Internal {
                message: "Model health check failed".to_string(),
                source: None,
            });
        }

        // Store server and client
        let mut servers = self.servers.write().await;
        servers.insert(model_id.to_string(), server);

        let mut clients = self.clients.write().await;
        clients.insert(model_id.to_string(), client);

        Ok(())
    }

    /// Unload a model
    pub async fn unload_model(&self, model_id: &str) -> Result<()> {
        let mut servers = self.servers.write().await;
        if let Some(mut server) = servers.remove(model_id) {
            server.stop().map_err(|e| NoaError::Internal {
                message: format!("Failed to stop llama server: {}", e),
                source: None,
            })?;
        }

        let mut clients = self.clients.write().await;
        clients.remove(model_id);

        Ok(())
    }

    /// Check if a model is loaded
    pub async fn is_loaded(&self, model_id: &str) -> bool {
        let clients = self.clients.read().await;
        clients.contains_key(model_id)
    }

    /// Get client for a loaded model
    pub async fn get_client(&self, model_id: &str) -> Result<Arc<LlamaClient>> {
        let clients = self.clients.read().await;
        clients.get(model_id).cloned().ok_or_else(|| NoaError::NotFound {
            resource: "Model".to_string(),
            id: model_id.to_string(),
        })
    }
}

impl Default for LlamaBackend {
    fn default() -> Self {
        Self::new()
    }
}
