//! API route handlers

use axum::Json;
use serde::Serialize;

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

/// Health check endpoint
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// Status response
#[derive(Serialize)]
pub struct StatusResponse {
    pub status: &'static str,
    pub components: ComponentStatus,
}

/// Component status
#[derive(Serialize)]
pub struct ComponentStatus {
    pub database: bool,
    pub embedder: bool,
    pub agents: bool,
    pub p2p: bool,
}

/// Status endpoint
pub async fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "operational",
        components: ComponentStatus {
            database: true,
            embedder: false,
            agents: false,
            p2p: false,
        },
    })
}

