//! HTTP Server Implementation
//!
//! Implements HTTP server with axum for NOA API.
//! §3.2: API server with graceful shutdown

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::config::NoaConfig;
use crate::db::ConnectionPool;
use crate::error::Result;

/// API server configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Server host address
    pub host: String,

    /// Server port
    pub port: u16,

    /// Request timeout in seconds
    pub timeout_secs: u64,

    /// Enable CORS
    pub enable_cors: bool,

    /// Allowed origins for CORS
    pub cors_origins: Vec<String>,

    /// Enable request tracing
    pub enable_tracing: bool,

    /// Graceful shutdown timeout in seconds
    pub shutdown_timeout_secs: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            timeout_secs: 30,
            enable_cors: true,
            cors_origins: vec!["http://localhost:*".to_string()],
            enable_tracing: true,
            shutdown_timeout_secs: 30,
        }
    }
}

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: Arc<ConnectionPool>,

    /// Application configuration
    pub config: Arc<NoaConfig>,

    /// Server start time for uptime tracking
    pub start_time: std::time::Instant,
}

impl AppState {
    pub fn new(db: ConnectionPool, config: NoaConfig) -> Self {
        Self {
            db: Arc::new(db),
            config: Arc::new(config),
            start_time: std::time::Instant::now(),
        }
    }

    /// Get uptime in seconds
    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

/// NOA API Server
pub struct ApiServer {
    config: ApiConfig,
    state: AppState,
}

impl ApiServer {
    /// Create a new API server
    pub fn new(config: ApiConfig, state: AppState) -> Self {
        Self { config, state }
    }

    /// Build the router with all routes and middleware
    pub fn build_router(&self) -> Router {
        let mut router = Router::new();

        // Add API routes
        router = router
            .merge(super::routes::health::routes())
            .merge(super::routes::api_v1());

        // Timeout layer
        router = router.layer(TimeoutLayer::new(Duration::from_secs(
            self.config.timeout_secs,
        )));

        // Tracing layer
        if self.config.enable_tracing {
            router = router.layer(TraceLayer::new_for_http());
        }

        // CORS layer
        if self.config.enable_cors {
            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any);
            router = router.layer(cors);
        }

        // Add custom middleware
        router = router
            .layer(axum::middleware::from_fn(super::middleware::logging::log_request))
            .layer(axum::middleware::from_fn(super::middleware::validation::validate_request));

        // Add state
        router.with_state(self.state.clone())
    }

    /// Start the server
    pub async fn start(self) -> Result<()> {
        let addr: SocketAddr = format!("{}:{}", self.config.host, self.config.port)
            .parse()
            .map_err(|e| crate::error::NoaError::Internal {
                message: format!("Invalid server address: {}", e),
                source: None,
            })?;

        let router = self.build_router();

        tracing::info!(
            host = %self.config.host,
            port = %self.config.port,
            "Starting NOA API server"
        );

        let listener = TcpListener::bind(addr).await.map_err(|e| {
            crate::error::NoaError::Internal {
                message: format!("Failed to bind to {}: {}", addr, e),
                source: Some(Box::new(e)),
            }
        })?;

        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e: std::io::Error| crate::error::NoaError::Internal {
                message: format!("Server error: {}", e),
                source: Some(Box::new(e)),
            })?;

        tracing::info!("Server shutdown complete");
        Ok(())
    }

    /// Get the server address
    pub fn address(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }
}

/// Shutdown signal handler for graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C, starting graceful shutdown");
        }
        _ = terminate => {
            tracing::info!("Received terminate signal, starting graceful shutdown");
        }
    }
}

/// Builder pattern for ApiServer
pub struct ApiServerBuilder {
    config: ApiConfig,
    db: Option<ConnectionPool>,
    noa_config: Option<NoaConfig>,
}

impl ApiServerBuilder {
    pub fn new() -> Self {
        Self {
            config: ApiConfig::default(),
            db: None,
            noa_config: None,
        }
    }

    pub fn with_config(mut self, config: ApiConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.config.host = host.into();
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.config.timeout_secs = timeout_secs;
        self
    }

    pub fn with_database(mut self, db: ConnectionPool) -> Self {
        self.db = Some(db);
        self
    }

    pub fn with_noa_config(mut self, config: NoaConfig) -> Self {
        self.noa_config = Some(config);
        self
    }

    pub fn build(self) -> Result<ApiServer> {
        let db = self.db.ok_or_else(|| crate::error::NoaError::Internal {
            message: "Database connection pool required".to_string(),
            source: None,
        })?;

        let noa_config = self.noa_config.ok_or_else(|| crate::error::NoaError::Internal {
            message: "NOA configuration required".to_string(),
            source: None,
        })?;

        let state = AppState::new(db, noa_config);
        Ok(ApiServer::new(self.config, state))
    }
}

impl Default for ApiServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_config_default() {
        let config = ApiConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.timeout_secs, 30);
    }

    #[test]
    fn test_app_state_uptime() {
        // This would require mocking the database
        // For now, just test that the struct compiles
    }
}

