//! Neural Service
//!
//! T118: Implement NeuralService with model management
//! US2: Neural service for model management

use crate::db::repositories::{Model, ModelRepository, ModelStatus};
use crate::db::Connection;
use crate::error::{NoaError, Result};
use crate::neural::inference::InferenceEngine;
use crate::neural::llama_backend::LlamaBackend;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

/// Neural service for model management
pub struct NeuralService {
    repository: ModelRepository,
    backend: Arc<LlamaBackend>,
    inference_engine: Arc<InferenceEngine>,
}

impl NeuralService {
    /// Create a new neural service
    pub fn new(conn: Connection) -> Self {
        let backend = Arc::new(LlamaBackend::new());
        let inference_engine = Arc::new(InferenceEngine::new(backend.clone()));

        Self {
            repository: ModelRepository::new(conn),
            backend,
            inference_engine,
        }
    }

    /// Register a new model
    pub fn register_model(&self, model: Model) -> Result<Uuid> {
        self.repository.create(&model)
    }

    /// Load a model
    pub async fn load_model(&self, model_id: &Uuid) -> Result<()> {
        let mut model =
            self.repository.find_by_id(model_id)?.ok_or_else(|| NoaError::NotFound {
                resource: "Model".to_string(),
                id: model_id.to_string(),
            })?;

        // Update status to loading
        self.repository.update_status(model_id, ModelStatus::Loading)?;

        // Get model path
        let model_path = model
            .path
            .as_ref()
            .ok_or_else(|| {
                NoaError::Validation(crate::error::ValidationError::new(
                    "path",
                    "Model path not set",
                    "MISSING_MODEL_PATH",
                ))
            })?
            .clone();

        let path = PathBuf::from(model_path);
        let context_size = model.context_length.unwrap_or(2048) as usize;
        let n_gpu_layers =
            model.config.get("n_gpu_layers").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

        // Load model in backend
        match self
            .backend
            .load_model(&model_id.to_string(), path, context_size, n_gpu_layers)
            .await
        {
            Ok(_) => {
                self.repository.update_status(model_id, ModelStatus::Loaded)?;
                Ok(())
            }
            Err(e) => {
                self.repository.update_status(model_id, ModelStatus::Error)?;
                Err(e)
            }
        }
    }

    /// Unload a model
    pub async fn unload_model(&self, model_id: &Uuid) -> Result<()> {
        self.repository.update_status(model_id, ModelStatus::Loading)?;

        match self.backend.unload_model(&model_id.to_string()).await {
            Ok(_) => {
                self.repository.update_status(model_id, ModelStatus::Available)?;
                Ok(())
            }
            Err(e) => {
                self.repository.update_status(model_id, ModelStatus::Error)?;
                Err(e)
            }
        }
    }

    /// List all models
    pub fn list_models(&self) -> Result<Vec<Model>> {
        self.repository.find_all()
    }

    /// Get model by ID
    pub fn get_model(&self, model_id: &Uuid) -> Result<Option<Model>> {
        self.repository.find_by_id(model_id)
    }

    /// Get loaded models
    pub fn get_loaded_models(&self) -> Result<Vec<Model>> {
        self.repository.find_by_status(ModelStatus::Loaded)
    }

    /// Get inference engine
    pub fn inference_engine(&self) -> Arc<InferenceEngine> {
        self.inference_engine.clone()
    }
}
