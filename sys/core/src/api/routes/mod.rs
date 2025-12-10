//! API Routes Module
//!
//! Defines all API routes for NOA.

pub mod activity;
pub mod digest;
pub mod goals;
pub mod healing;
pub mod health;
pub mod inference;
pub mod knowledge;
pub mod memories;
pub mod models;
pub mod p2p;
pub mod planes;
pub mod promotions;
pub mod providers;
pub mod system;

use crate::api::server::AppState;
use axum::Router;

/// Create the v1 API router with all routes
pub fn api_v1() -> Router<AppState> {
    Router::<AppState>::new().nest("/api/v1", v1_routes())
}

/// V1 API routes
fn v1_routes() -> Router<AppState> {
    Router::<AppState>::new()
        .merge(health::routes())
        .merge(system::routes())
        .merge(providers::routes())
        .merge(p2p::routes())
        .merge(models::routes())
        .merge(inference::routes())
        .merge(planes::routes())
        .merge(promotions::routes())
        .merge(healing::routes())
        .merge(goals::routes())
        .merge(activity::routes())
        .merge(memories::create_routes())
        .merge(digest::routes())
        .merge(knowledge::routes())
    // Future routes:
    // .merge(agents::routes())
    // .merge(tasks::routes())
}
