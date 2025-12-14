//! GGUF Model Loader Interface
//!
//! T105: Create GGUF model loader interface
//! §3.2: Local-First & Offline-Capable
//! US2: Model loading with GPU layer auto-detection

use crate::error::{Result, NoaError};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// Model loader configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLoaderConfig {
    /// Model file path
    pub model_path: PathBuf,
    /// Context size
    pub context_size: usize,
    /// Number of GPU layers (0 = CPU only)
    pub n_gpu_layers: i32,
    /// Number of threads (None = auto)
    pub threads: Option<usize>,
    /// Enable GPU layer auto-detection
    pub auto_detect_gpu_layers: bool,
}

impl Default for ModelLoaderConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("models/default.gguf"),
            context_size: 2048,
            n_gpu_layers: 0,
            threads: None,
            auto_detect_gpu_layers: true,
        }
    }
}

/// Model loader interface for GGUF files
pub struct ModelLoader;

impl ModelLoader {
    /// Create a new model loader
    pub fn new() -> Self {
        Self
    }

    /// Load a GGUF model with the given configuration
    pub async fn load_gguf(&self, config: &ModelLoaderConfig) -> Result<LoadedModel> {
        // Validate model path exists
        if !config.model_path.exists() {
            return Err(NoaError::NotFound {
                resource: "Model file".to_string(),
                id: config.model_path.display().to_string(),
            });
        }

        // Validate it's a GGUF file
        if !config.model_path.extension().map(|e| e == "gguf").unwrap_or(false) {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "model_path",
                "Model file must have .gguf extension",
                "INVALID_MODEL_FORMAT",
            )));
        }

        // Auto-detect GPU layers if enabled
        let n_gpu_layers = if config.auto_detect_gpu_layers {
            self.detect_optimal_gpu_layers().await?
        } else {
            config.n_gpu_layers
        };

        Ok(LoadedModel {
            path: config.model_path.clone(),
            context_size: config.context_size,
            n_gpu_layers,
            threads: config.threads,
        })
    }

    /// Detect optimal number of GPU layers
    async fn detect_optimal_gpu_layers(&self) -> Result<i32> {
        // TODO: Implement actual GPU detection
        // For now, return 0 (CPU only)
        // This will be enhanced in T478-T485 (Multi-GPU Support)
        Ok(0)
    }

    /// Validate GGUF file integrity
    pub async fn validate_gguf(&self, path: &Path) -> Result<bool> {
        if !path.exists() {
            return Err(NoaError::NotFound {
                resource: "Model file".to_string(),
                id: path.display().to_string(),
            });
        }

        // Check file extension
        if !path.extension().map(|e| e == "gguf").unwrap_or(false) {
            return Ok(false);
        }

        // TODO: Implement actual GGUF validation
        // Check magic bytes, version, etc.
        Ok(true)
    }

    /// Get model metadata from GGUF file
    pub async fn get_metadata(&self, path: &Path) -> Result<ModelMetadata> {
        if !path.exists() {
            return Err(NoaError::NotFound {
                resource: "Model file".to_string(),
                id: path.display().to_string(),
            });
        }

        // TODO: Implement actual GGUF metadata extraction
        // For now, return basic metadata
        Ok(ModelMetadata {
            name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            size_bytes: std::fs::metadata(path)
                .map(|m| m.len() as i64)
                .unwrap_or(0),
            format: "gguf".to_string(),
            quantization: None,
            architecture: None,
        })
    }
}

impl Default for ModelLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Loaded model information
#[derive(Debug, Clone)]
pub struct LoadedModel {
    pub path: PathBuf,
    pub context_size: usize,
    pub n_gpu_layers: i32,
    pub threads: Option<usize>,
}

/// Model metadata extracted from GGUF file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub name: String,
    pub size_bytes: i64,
    pub format: String,
    pub quantization: Option<String>,
    pub architecture: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_model_loader_validation() {
        let loader = ModelLoader::new();
        let dir = tempdir().unwrap();
        let model_path = dir.path().join("test.gguf");

        // Create a dummy file
        std::fs::write(&model_path, b"dummy").unwrap();

        let valid = loader.validate_gguf(&model_path).await.unwrap();
        assert!(valid);
    }

    #[tokio::test]
    async fn test_invalid_extension() {
        let loader = ModelLoader::new();
        let dir = tempdir().unwrap();
        let model_path = dir.path().join("test.txt");

        std::fs::write(&model_path, b"dummy").unwrap();

        let valid = loader.validate_gguf(&model_path).await.unwrap();
        assert!(!valid);
    }
}

