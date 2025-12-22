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
use crate::db::{ConnectionPool, PooledConnection};
use crate::error::{ApiError, NoaError, Result};
use crate::config::access::ConfigAccess;

use super::middleware::{
    RateLimitConfig, RateLimiter,
    RequestLimitsConfig, RequestLimits,
    AuthConfig, AuthState,
};

/// Database handle used by the API server.
///
/// Today, most routes are still SQLite-first (rusqlite-based repositories).
/// We keep SQLite as a first-class option while allowing the server to boot
/// with PostgreSQL state for server deployments.
#[derive(Clone)]
pub enum AppDatabase {
    Sqlite(Arc<ConnectionPool>),

    #[cfg(feature = "full")]
    Postgres(sqlx::PgPool),
}

impl AppDatabase {
    pub fn kind(&self) -> &'static str {
        match self {
            AppDatabase::Sqlite(_) => "sqlite",
            #[cfg(feature = "full")]
            AppDatabase::Postgres(_) => "postgresql",
        }
    }
}

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

    /// Maximum request body size in bytes
    pub max_body_size: usize,

    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,

    /// Enable rate limiting
    pub enable_rate_limiting: bool,

    /// Rate limit: max requests per minute (default endpoints)
    pub rate_limit_requests_per_minute: u32,

    /// Rate limit: max requests per minute for expensive endpoints
    pub rate_limit_expensive_per_minute: u32,

    /// Enable authentication
    pub enable_auth: bool,

    /// JWT secret for token signing/verification
    pub jwt_secret: Option<String>,
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
            max_body_size: 10 * 1024 * 1024, // 10MB
            max_concurrent_requests: 100,
            enable_rate_limiting: true,
            rate_limit_requests_per_minute: 100,
            rate_limit_expensive_per_minute: 20,
            enable_auth: false, // Disabled by default for development
            jwt_secret: None,
        }
    }
}

impl ApiConfig {
    pub fn from_noa_config(noa_config: &NoaConfig) -> Self {
        let mut cfg = ApiConfig::default();

        if let Some(noa_server) = noa_config.raw.get("noa_server") {
            // Support either a dedicated api section or legacy top-level server keys.
            // Canonical (authoritative) path: noa_server.api.{host,port,timeout_secs,max_connections,shutdown_timeout_secs}
            if let Some(api) = noa_server.get("api") {
                if let Some(host) = api.get("host").and_then(|v| v.as_str()) {
                    cfg.host = host.to_string();
                }
                if let Some(port) = api.get("port").and_then(|v| v.as_u64()) {
                    cfg.port = port.min(u16::MAX as u64) as u16;
                }
                if let Some(timeout) = api.get("timeout_secs").and_then(|v| v.as_u64()) {
                    cfg.timeout_secs = timeout;
                }
                if let Some(shutdown) = api.get("shutdown_timeout_secs").and_then(|v| v.as_u64()) {
                    cfg.shutdown_timeout_secs = shutdown;
                }
                if let Some(enable_cors) = api.get("enable_cors").and_then(|v| v.as_bool()) {
                    cfg.enable_cors = enable_cors;
                }
                if let Some(enable_tracing) = api.get("enable_tracing").and_then(|v| v.as_bool()) {
                    cfg.enable_tracing = enable_tracing;
                }
                if let Some(origins) = api.get("cors_origins").and_then(|v| v.as_array()) {
                    cfg.cors_origins = origins
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                }
                if let Some(max_body) = api.get("max_body_size").and_then(|v| v.as_u64()) {
                    cfg.max_body_size = max_body as usize;
                }
                if let Some(max_concurrent) = api.get("max_concurrent_requests").and_then(|v| v.as_u64()) {
                    cfg.max_concurrent_requests = max_concurrent as usize;
                }
                if let Some(enable_rate) = api.get("enable_rate_limiting").and_then(|v| v.as_bool()) {
                    cfg.enable_rate_limiting = enable_rate;
                }
                if let Some(rate_limit) = api.get("rate_limit_requests_per_minute").and_then(|v| v.as_u64()) {
                    cfg.rate_limit_requests_per_minute = rate_limit as u32;
                }
                if let Some(expensive_limit) = api.get("rate_limit_expensive_per_minute").and_then(|v| v.as_u64()) {
                    cfg.rate_limit_expensive_per_minute = expensive_limit as u32;
                }
                if let Some(enable_auth) = api.get("enable_auth").and_then(|v| v.as_bool()) {
                    cfg.enable_auth = enable_auth;
                }
                if let Some(jwt_secret) = api.get("jwt_secret").and_then(|v| v.as_str()) {
                    cfg.jwt_secret = Some(jwt_secret.to_string());
                }
            }

            // Back-compat: noa_server.server.{host,port,timeout_secs,max_connections,shutdown_timeout_secs}
            if let Some(server) = noa_server.get("server") {
                if let Some(host) = server.get("host").and_then(|v| v.as_str()) {
                    cfg.host = host.to_string();
                }
                if let Some(port) = server.get("port").and_then(|v| v.as_u64()) {
                    cfg.port = port.min(u16::MAX as u64) as u16;
                }
                if let Some(timeout) = server.get("timeout_secs").and_then(|v| v.as_u64()) {
                    cfg.timeout_secs = timeout;
                }
                if let Some(shutdown) = server
                    .get("shutdown_timeout_secs")
                    .and_then(|v| v.as_u64())
                {
                    cfg.shutdown_timeout_secs = shutdown;
                }
            }
        }

        cfg
    }
}

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: AppDatabase,

    /// Application configuration
    pub config: Arc<NoaConfig>,

    /// Central dynamic config accessor
    pub config_access: Arc<ConfigAccess>,

    /// Server start time for uptime tracking
    pub start_time: std::time::Instant,

    /// Rate limiter for API endpoints
    pub rate_limiter: Arc<RateLimiter>,

    /// Request limits (body size, concurrency)
    pub request_limits: Arc<RequestLimits>,

    /// Authentication state
    pub auth: Arc<AuthState>,
}

impl AppState {
    pub fn new(db: AppDatabase, config: NoaConfig) -> Self {
        Self::with_api_config(db, config, ApiConfig::default())
    }

    pub fn with_api_config(db: AppDatabase, config: NoaConfig, api_config: ApiConfig) -> Self {
        let config_access = Arc::new(ConfigAccess::from_config(&config));

        // Create rate limiter from config
        let rate_limiter = Arc::new(RateLimiter::new(RateLimitConfig {
            max_requests: api_config.rate_limit_requests_per_minute,
            window: Duration::from_secs(60),
            per_ip: true,
            per_token: true,
        }));

        // Create request limits from config
        let request_limits = Arc::new(RequestLimits::new(RequestLimitsConfig {
            max_body_size: api_config.max_body_size,
            max_concurrent: api_config.max_concurrent_requests,
            timeout_secs: api_config.timeout_secs,
        }));

        // Create auth state from config
        let auth = Arc::new(AuthState::new(AuthConfig {
            enabled: api_config.enable_auth,
            allow_health_unauthenticated: true,
            jwt_secret: api_config.jwt_secret.clone(),
            api_keys: Vec::new(), // API keys loaded from config/database separately
        }));

        Self {
            db,
            config: Arc::new(config),
            config_access,
            start_time: std::time::Instant::now(),
            rate_limiter,
            request_limits,
            auth,
        }
    }

    pub fn new_sqlite(db: ConnectionPool, config: NoaConfig) -> Self {
        Self::new(AppDatabase::Sqlite(Arc::new(db)), config)
    }

    #[cfg(feature = "full")]
    pub fn new_postgres(pool: sqlx::PgPool, config: NoaConfig) -> Self {
        Self::new(AppDatabase::Postgres(pool), config)
    }

    /// Get a SQLite pooled connection if this instance is configured for SQLite.
    ///
    /// Returns a ServiceUnavailable error when running on PostgreSQL, since most
    /// current repositories/routes are still SQLite-first.
    pub fn sqlite_conn(&self) -> Result<PooledConnection> {
        match &self.db {
            AppDatabase::Sqlite(pool) => pool.get(),
            #[cfg(feature = "full")]
            AppDatabase::Postgres(_) => Err(NoaError::Api(ApiError::ServiceUnavailable(
                "This endpoint requires the SQLite backend; PostgreSQL support is not implemented yet.".to_string(),
            ))),
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

        // Timeout layer (via tower-http)
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

        // Request limits middleware (body size, concurrency)
        let limits = self.state.request_limits.clone();
        router = router.layer(axum::middleware::from_fn(move |req, next| {
            let limits = limits.clone();
            async move {
                super::middleware::request_limits_middleware(limits, req, next).await
            }
        }));

        // Rate limiting middleware (if enabled)
        if self.config.enable_rate_limiting {
            let limiter = self.state.rate_limiter.clone();
            router = router.layer(axum::middleware::from_fn(move |req, next| {
                let limiter = limiter.clone();
                async move {
                    super::middleware::rate_limit_middleware(limiter, req, next).await
                }
            }));
        }

        // Authentication middleware (if enabled)
        if self.config.enable_auth {
            let auth = self.state.auth.clone();
            router = router.layer(axum::middleware::from_fn(move |req, next| {
                let auth = auth.clone();
                async move {
                    super::middleware::auth_middleware(auth, req, next).await
                }
            }));
        }

        // Add custom middleware
        router = router
            .layer(axum::middleware::from_fn(super::middleware::logging::log_request))
            .layer(axum::middleware::from_fn(super::middleware::validation::validate_request));

        // Provide AppState via request extensions
        router.layer(axum::Extension(self.state.clone()))
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

        // Start config hot reload polling (multi-source configs)
        let reload_stop = self
            .state
            .config_access
            .start_polling_reload(Duration::from_secs(2));

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
            .with_graceful_shutdown(async move {
                shutdown_signal().await;
                let _ = reload_stop.send(true);
            })
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
    host: Option<String>,
    port: Option<u16>,
    timeout_secs: Option<u64>,
    enable_cors: Option<bool>,
    cors_origins: Option<Vec<String>>, 
    enable_tracing: Option<bool>,
    shutdown_timeout_secs: Option<u64>,
    db: Option<AppDatabase>,
    noa_config: Option<NoaConfig>,
}

impl ApiServerBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
            timeout_secs: None,
            enable_cors: None,
            cors_origins: None,
            enable_tracing: None,
            shutdown_timeout_secs: None,
            db: None,
            noa_config: None,
        }
    }

    pub fn with_config(self, config: ApiConfig) -> Self {
        self.with_host(config.host)
            .with_port(config.port)
            .with_timeout(config.timeout_secs)
            .with_enable_cors(config.enable_cors)
            .with_cors_origins(config.cors_origins)
            .with_enable_tracing(config.enable_tracing)
            .with_shutdown_timeout_secs(config.shutdown_timeout_secs)
    }

    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = Some(timeout_secs);
        self
    }

    pub fn with_enable_cors(mut self, enable_cors: bool) -> Self {
        self.enable_cors = Some(enable_cors);
        self
    }

    pub fn with_cors_origins(mut self, cors_origins: Vec<String>) -> Self {
        self.cors_origins = Some(cors_origins);
        self
    }

    pub fn with_enable_tracing(mut self, enable_tracing: bool) -> Self {
        self.enable_tracing = Some(enable_tracing);
        self
    }

    pub fn with_shutdown_timeout_secs(mut self, shutdown_timeout_secs: u64) -> Self {
        self.shutdown_timeout_secs = Some(shutdown_timeout_secs);
        self
    }

    pub fn with_database(mut self, db: ConnectionPool) -> Self {
        self.db = Some(AppDatabase::Sqlite(Arc::new(db)));
        self
    }

    #[cfg(feature = "full")]
    pub fn with_postgres_pool(mut self, pool: sqlx::PgPool) -> Self {
        self.db = Some(AppDatabase::Postgres(pool));
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

        // noa-server.json is authoritative: derive base ApiConfig from it.
        let mut api_cfg = ApiConfig::from_noa_config(&noa_config);

        // Apply explicit overrides only.
        if let Some(host) = self.host {
            api_cfg.host = host;
        }
        if let Some(port) = self.port {
            api_cfg.port = port;
        }
        if let Some(timeout_secs) = self.timeout_secs {
            api_cfg.timeout_secs = timeout_secs;
        }
        if let Some(enable_cors) = self.enable_cors {
            api_cfg.enable_cors = enable_cors;
        }
        if let Some(cors_origins) = self.cors_origins {
            api_cfg.cors_origins = cors_origins;
        }
        if let Some(enable_tracing) = self.enable_tracing {
            api_cfg.enable_tracing = enable_tracing;
        }
        if let Some(shutdown_timeout_secs) = self.shutdown_timeout_secs {
            api_cfg.shutdown_timeout_secs = shutdown_timeout_secs;
        }

        let state = AppState::with_api_config(db, noa_config, api_cfg.clone());
        Ok(ApiServer::new(api_cfg, state))
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

