//! NOA Neural Runtime
//!
//! Multi-SLM neural runtime with llama.cpp integration

pub mod llama;
pub mod model;
pub mod runtime;

pub use runtime::NeuralRuntime;
pub use model::{Model, Modelconfigs, ModelStatus, ModelType};
