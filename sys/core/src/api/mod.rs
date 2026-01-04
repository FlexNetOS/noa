//! NOA API Module
//!
//! Provides HTTP API server with axum framework.
//! §3.2: API server implementation
//! FR-021-030: Core API endpoints

pub mod server;
pub mod routes;
pub mod middleware;

pub use server::{ApiServer, Apiconfigs};

