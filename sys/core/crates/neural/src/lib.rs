//! NOA Neural Runtime
//!
//! Multi-SLM neural runtime with llama.cpp integration

pub mod llama;
pub mod model;
pub mod runtime;

pub use model::{Model, ModelConfig, ModelStatus, ModelType};
pub use runtime::NeuralRuntime;
