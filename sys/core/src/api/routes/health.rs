//! Health Check Endpoint
//!
//! Implements GET /api/v1/health for health monitoring.
//! FR-155: Observability

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sysinfo::System;

use crate::api::server::AppState;
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
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/live", get(liveness_check))
        .route("/health/ready", get(readiness_check))
}

/// Full health check endpoint
/// GET /api/v1/health
async fn health_check(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
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
async fn readiness_check(State(state): State<AppState>) -> StatusCode {
    // Check if we can serve requests
    let db_health = check_database(&state).await;

    if db_health.status == HealthStatus::Unhealthy {
        return StatusCode::SERVICE_UNAVAILABLE;
    }

    StatusCode::OK
}

/// Check database health
async fn check_database(state: &AppState) -> ComponentStatus {
    match state.db.get() {
        Ok(conn) => {
            // Try to run integrity check, but ignore FTS table errors as they're non-critical
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
                Err(e) => {
                    // Check if error is related to FTS tables (non-critical)
                    let error_msg = e.to_string();
                    if error_msg.contains("fts") || error_msg.contains("memory_fts") || error_msg.contains("vtable") {
                        // FTS table errors are non-critical - database is still functional
                        ComponentStatus::degraded(format!("FTS index issue (non-critical): {}", e))
                    } else {
                        ComponentStatus::degraded(format!("Integrity check error: {}", e))
                    }
                }
            }
        }
        Err(e) => ComponentStatus::unhealthy(format!("Database connection failed: {}", e)),
    }
}

/// Check memory usage
fn check_memory() -> ComponentStatus {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();
    let usage_percent = if total_memory > 0 {
        (used_memory as f64 / total_memory as f64) * 100.0
    } else {
        0.0
    };

    // Determine health based on memory usage
    let (status, message) = if usage_percent > 95.0 {
        (
            HealthStatus::Unhealthy,
            Some(format!("Critical memory usage: {:.1}%", usage_percent)),
        )
    } else if usage_percent > 85.0 {
        (
            HealthStatus::Degraded,
            Some(format!("High memory usage: {:.1}%", usage_percent)),
        )
    } else {
        (HealthStatus::Healthy, None)
    };

    ComponentStatus {
        status,
        message,
        details: Some(serde_json::json!({
            "total_bytes": total_memory,
            "used_bytes": used_memory,
            "available_bytes": available_memory,
            "usage_percent": format!("{:.1}", usage_percent),
            "total_mb": total_memory / 1024 / 1024,
            "used_mb": used_memory / 1024 / 1024,
            "available_mb": available_memory / 1024 / 1024
        })),
    }
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
