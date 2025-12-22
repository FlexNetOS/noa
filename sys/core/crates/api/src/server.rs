//! API Server implementation

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::routes;
use crate::state::AppState;

/// NOA API Server
pub struct Server {
    addr: SocketAddr,
    state: AppState,
}

impl Server {
    /// Create a new server instance
    pub fn new(addr: SocketAddr, state: AppState) -> Self {
        Self { addr, state }
    }

    /// Build the router with all routes
    pub fn router(state: AppState) -> Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        Router::new()
            .route("/health", get(routes::health))
            .route("/api/v1/status", get(routes::status))
            .route("/api/v1/auth/whoami", get(routes::auth::whoami))
            .route("/api/v1/auth/signup", post(routes::auth::signup))
            .route("/api/v1/auth/login", post(routes::auth::login))
            .route("/api/v1/auth/logout", post(routes::auth::logout))
            .route("/api/v1/auth/github/start", get(routes::auth::github_start))
            .route("/api/v1/auth/github/callback", get(routes::auth::github_callback))
            .route("/api/v1/auth/google/start", get(routes::auth::google_start))
            .route("/api/v1/auth/google/callback", get(routes::auth::google_callback))
            .route("/api/v1/tasks", get(routes::list_tasks))
            .route("/api/v1/tasks", post(routes::create_task))
            .route("/api/v1/chat", post(routes::chat::chat))
            .route("/api/v1/chat/stream", post(routes::chat::chat_stream))
            .with_state(state)
            .layer(cors)
            .layer(TraceLayer::new_for_http())
    }

    /// Run the server
    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = tokio::net::TcpListener::bind(self.addr).await?;
        tracing::info!("NOA API server listening on {}", self.addr);
        axum::serve(listener, Self::router(self.state)).await
    }
}

