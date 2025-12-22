//! Health Check Endpoint
//!
//! Implements GET /api/v1/health for health monitoring.
//! FR-155: Observability

use axum::{
    extract::Extension,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::api::server::{AppDatabase, AppState};
use crate::db;

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Overall health status
    pub status: HealthStatus,

    /// Server version
    pub version: String,

    /// Server uptime in seconds
    pub uptime_secs: u64,

    /// Component health details
    pub components: ApiComponentHealth,

    /// Timestamp of this health check
    pub timestamp: String,
}

/// Overall health status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Component health details
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiComponentHealth {
    /// Database health
    pub database: ComponentStatus,

    /// Memory usage
    pub memory: ComponentStatus,

    /// AI providers (optional)
    pub providers: Option<ComponentStatus>,
}

/// Individual component status
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentStatus {
    pub status: HealthStatus,
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
}

impl ComponentStatus {
    pub fn healthy() -> Self {
        Self {
            status: HealthStatus::Healthy,
            message: None,
            details: None,
        }
    }

    pub fn healthy_with_details(details: serde_json::Value) -> Self {
        Self {
            status: HealthStatus::Healthy,
            message: None,
            details: Some(details),
        }
    }

    pub fn unhealthy(message: impl Into<String>) -> Self {
        Self {
            status: HealthStatus::Unhealthy,
            message: Some(message.into()),
            details: None,
        }
    }

    pub fn degraded(message: impl Into<String>) -> Self {
        Self {
            status: HealthStatus::Degraded,
            message: Some(message.into()),
            details: None,
        }
    }
}

/// Create health check routes
pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/live", get(liveness_check))
        .route("/health/ready", get(readiness_check))
}

/// Full health check endpoint
/// GET /api/v1/health
async fn health_check(
    Extension(state): Extension<AppState>,
) -> (StatusCode, Json<HealthResponse>) {
    let db_health = check_database(&state).await;
    let memory_health = check_memory();

    let overall_status = determine_overall_status(&[&db_health, &memory_health]);

    let response = HealthResponse {
        status: overall_status.clone(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_secs: state.uptime_secs(),
        components: ApiComponentHealth {
            database: db_health,
            memory: memory_health,
            providers: None,
        },
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let status_code = match overall_status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK,
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    (status_code, Json(response))
}

/// Kubernetes liveness probe
/// GET /api/v1/health/live
async fn liveness_check() -> StatusCode {
    // Simple check - if we can respond, we're alive
    StatusCode::OK
}

/// Kubernetes readiness probe
/// GET /api/v1/health/ready
async fn readiness_check(
    Extension(state): Extension<AppState>,
) -> StatusCode {
    // Check if we can serve requests
    let db_health = check_database(&state).await;

    if db_health.status == HealthStatus::Unhealthy {
        return StatusCode::SERVICE_UNAVAILABLE;
    }

    StatusCode::OK
}

/// Check database health
async fn check_database(state: &AppState) -> ComponentStatus {
    match &state.db {
        AppDatabase::Sqlite(_) => match state.sqlite_conn() {
            Ok(conn) => {
                // Try to run integrity check
                match db::check_integrity(&conn) {
                    Ok(true) => {
                        // Get stats
                        match db::get_stats(&conn) {
                            Ok(stats) => ComponentStatus::healthy_with_details(serde_json::json!({
                                "total_size_bytes": stats.total_size_bytes,
                                "used_size_bytes": stats.used_size_bytes,
                                "total_pages": stats.total_pages,
                            })),
                            Err(_) => ComponentStatus::healthy(),
                        }
                    }
                    Ok(false) => ComponentStatus::unhealthy("Database integrity check failed"),
                    Err(e) => ComponentStatus::degraded(format!("Integrity check error: {}", e)),
                }
            }
            Err(e) => ComponentStatus::unhealthy(format!("Database connection failed: {}", e)),
        },

        #[cfg(feature = "full")]
        AppDatabase::Postgres(pool) => {
            match sqlx::query_scalar::<_, i64>("SELECT 1")
                .fetch_one(pool)
                .await
            {
                Ok(_) => ComponentStatus::healthy(),
                Err(e) => ComponentStatus::unhealthy(format!("PostgreSQL query failed: {}", e)),
            }
        }
    }
}

/// Check memory usage
fn check_memory() -> ComponentStatus {
    // Basic memory check using sys_info or similar
    // For now, just report healthy
    ComponentStatus::healthy_with_details(serde_json::json!({
        "note": "Memory monitoring not fully implemented"
    }))
}

/// Determine overall status from component statuses
fn determine_overall_status(components: &[&ComponentStatus]) -> HealthStatus {
    let mut has_degraded = false;

    for component in components {
        match component.status {
            HealthStatus::Unhealthy => return HealthStatus::Unhealthy,
            HealthStatus::Degraded => has_degraded = true,
            HealthStatus::Healthy => {}
        }
    }

    if has_degraded {
        HealthStatus::Degraded
    } else {
        HealthStatus::Healthy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_status_healthy() {
        let status = ComponentStatus::healthy();
        assert_eq!(status.status, HealthStatus::Healthy);
        assert!(status.message.is_none());
    }

    #[test]
    fn test_component_status_unhealthy() {
        let status = ComponentStatus::unhealthy("test error");
        assert_eq!(status.status, HealthStatus::Unhealthy);
        assert_eq!(status.message.as_deref(), Some("test error"));
    }

    #[test]
    fn test_determine_overall_status() {
        let healthy = ComponentStatus::healthy();
        let degraded = ComponentStatus::degraded("degraded");
        let unhealthy = ComponentStatus::unhealthy("unhealthy");

        assert_eq!(
            determine_overall_status(&[&healthy, &healthy]),
            HealthStatus::Healthy
        );
        assert_eq!(
            determine_overall_status(&[&healthy, &degraded]),
            HealthStatus::Degraded
        );
        assert_eq!(
            determine_overall_status(&[&healthy, &unhealthy]),
            HealthStatus::Unhealthy
        );
    }
}

