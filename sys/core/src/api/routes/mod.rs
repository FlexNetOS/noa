//! API Routes Module
//!
//! Defines all API routes for NOA.

pub mod health;

use axum::Router;
use crate::api::server::AppState;

/// Create the v1 API router with all routes
pub fn api_v1() -> Router<AppState> {
    Router::new()
        .nest("/api/v1", v1_routes())
}

/// V1 API routes
fn v1_routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        // Future routes:
        // .merge(agents::routes())
        // .merge(tasks::routes())
        // .merge(memory::routes())
        // .merge(providers::routes())
}

