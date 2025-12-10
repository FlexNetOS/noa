//! API Server implementation

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::routes;

/// NOA API Server
pub struct Server {
    addr: SocketAddr,
}

impl Server {
    /// Create a new server instance
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    /// Build the router with all routes
    pub fn router() -> Router {
        let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

        Router::new()
            .route("/health", get(routes::health))
            .route("/api/v1/status", get(routes::status))
            .route("/api/v1/tasks", get(routes::list_tasks))
            .route("/api/v1/tasks", post(routes::create_task))
            .layer(cors)
            .layer(TraceLayer::new_for_http())
    }

    /// Run the server
    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = tokio::net::TcpListener::bind(self.addr).await?;
        tracing::info!("NOA API server listening on {}", self.addr);
        axum::serve(listener, Self::router()).await
    }
}
