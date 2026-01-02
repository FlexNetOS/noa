//! HTTP/gRPC server for the daemon.

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

use crate::config::DaemonConfig;
use crate::state::{AgentStatus, StateManager};

/// Shared application state.
struct AppState {
    config: DaemonConfig,
    state_manager: StateManager,
    shutdown_tx: broadcast::Sender<()>,
}

/// Run the daemon server.
pub async fn run(config: DaemonConfig, state_manager: StateManager) -> Result<()> {
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel::<()>(1);
    
    let app_state = Arc::new(AppState {
        config: config.clone(),
        state_manager,
        shutdown_tx,
    });
    
    // Build router
    let app = Router::new()
        // Health and status
        .route("/health", get(health))
        .route("/status", get(status))
        
        // Peer management
        .route("/peers", get(list_peers))
        .route("/peers", post(add_peer))
        
        // Agent management
        .route("/agents", get(list_agents))
        .route("/agents", post(register_agent))
        .route("/agents/:id/status", post(update_agent_status))
        
        // State sync
        .route("/state", get(get_state))
        .route("/state/sync", post(sync_state))
        
        // Shutdown
        .route("/shutdown", post(shutdown))
        
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(app_state.clone());
    
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    info!("Starting noa-hived server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // Run server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.recv().await;
            info!("Shutting down server...");
        })
        .await?;
    
    // Save state before exit
    app_state.state_manager.save().await?;
    info!("Server stopped");
    
    Ok(())
}

// === Health & Status ===

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[derive(Serialize)]
struct StatusResponse {
    running: bool,
    port: u16,
    peers: usize,
    agents: usize,
    state_version: u64,
}

async fn status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let daemon_state = state.state_manager.get_state().await;
    
    Json(StatusResponse {
        running: true,
        port: state.config.port,
        peers: daemon_state.peers.len(),
        agents: daemon_state.agents.len(),
        state_version: daemon_state.version,
    })
}

// === Peer Management ===

async fn list_peers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let daemon_state = state.state_manager.get_state().await;
    Json(daemon_state.peers.values().cloned().collect::<Vec<_>>())
}

#[derive(Deserialize)]
struct AddPeerRequest {
    id: String,
    address: String,
}

async fn add_peer(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddPeerRequest>,
) -> impl IntoResponse {
    match state.state_manager.add_peer(req.id, req.address).await {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({"success": true}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        ),
    }
}

// === Agent Management ===

async fn list_agents(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let daemon_state = state.state_manager.get_state().await;
    Json(daemon_state.agents.values().cloned().collect::<Vec<_>>())
}

#[derive(Deserialize)]
struct RegisterAgentRequest {
    id: String,
    name: String,
}

async fn register_agent(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterAgentRequest>,
) -> impl IntoResponse {
    match state.state_manager.register_agent(req.id, req.name).await {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({"success": true}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        ),
    }
}

#[derive(Deserialize)]
struct UpdateAgentStatusRequest {
    status: String,
}

async fn update_agent_status(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(req): Json<UpdateAgentStatusRequest>,
) -> impl IntoResponse {
    let status = match req.status.to_lowercase().as_str() {
        "idle" => AgentStatus::Idle,
        "running" => AgentStatus::Running,
        "paused" => AgentStatus::Paused,
        "error" => AgentStatus::Error,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Invalid status"})),
            )
        }
    };
    
    match state.state_manager.update_agent_status(&id, status).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        ),
    }
}

// === State Sync ===

async fn get_state(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let daemon_state = state.state_manager.get_state().await;
    Json(daemon_state)
}

async fn sync_state(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Trigger state sync (placeholder for P2P sync)
    match state.state_manager.save().await {
        Ok(_) => Json(serde_json::json!({"synced": true})),
        Err(e) => Json(serde_json::json!({"synced": false, "error": e.to_string()})),
    }
}

// === Shutdown ===

async fn shutdown(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    warn!("Shutdown requested");
    let _ = state.shutdown_tx.send(());
    Json(serde_json::json!({"shutting_down": true}))
}
