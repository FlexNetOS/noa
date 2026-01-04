//! Model definitions and configsuration

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    LLM,
    Embedding,
    Vision,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modelconfigs {
    pub name: String,
    pub model_type: ModelType,
    pub provider: String,
    pub file_path: Option<String>,
    pub context_length: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub repeat_penalty: f32,
    pub n_gpu_layers: i32,
}

impl Default for Modelconfigs {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            model_type: ModelType::LLM,
            provider: "llama.cpp".to_string(),
            file_path: None,
            context_length: 2048,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            n_gpu_layers: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    pub id: String,
    pub configs: Modelconfigs,
    pub status: ModelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelStatus {
    Available,
    Loading,
    Loaded,
    Failed(String),
}

impl Model {
    pub fn new(id: String, configs: Modelconfigs) -> Self {
        Self {
            id,
            configs,
            status: ModelStatus::Available,
        }
    }
}
