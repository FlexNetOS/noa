//! ML DevOps Platform - Local Inference Server
//!
//! Privacy-focused local ML inference using Candle and Rust.
//! Provides OpenAI-compatible API for seamless integration.

pub mod models;
pub mod moe;
pub mod server;
pub mod types;

/// Re-export main types
pub use server::InferenceServer;
pub use types::*;
