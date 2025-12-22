//! API Routes Module
//!
//! Defines all API routes for NOA.

pub mod health;
pub mod system;
pub mod memories;
pub mod providers;
pub mod p2p;
pub mod models;
pub mod inference;
pub mod planes;
pub mod promotions;
pub mod healing;
pub mod goals;
pub mod activity;
pub mod agents;
pub mod tasks;
pub mod orchestration;
pub mod capsule;
pub mod crm;

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
        .merge(agents::routes())
        .merge(tasks::routes())
        .merge(orchestration::routes())
        .merge(capsule::routes())
        .merge(crm::routes())
        .merge(memories::create_routes())
}
