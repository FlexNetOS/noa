//! System API Routes
//!
//! T095-T096: System info and health endpoints
//! US1: Initialize NOA Seed Environment

use axum::{
    extract::Extension,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::api::server::AppState;
use crate::db;
use crate::init::paths::NoaPaths;

/// System information response
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    pub noa_root: String,
    pub initialized: bool,
    pub database_path: String,
    pub directories: Vec<String>,
}

/// System health response
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemHealth {
    pub status: String,
    pub database: HealthStatus,
    pub directories: HealthStatus,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub message: Option<String>,
}

/// Create system routes
pub fn routes() -> Router {
    Router::new()
        .route("/system/info", get(get_system_info))
        .route("/system/health", get(get_system_health))
}

/// GET /api/v1/system/info
async fn get_system_info(Extension(_state): Extension<AppState>) -> Result<Json<SystemInfo>, StatusCode> {
    info!("GET /api/v1/system/info");

    // Get NOA_ROOT from environment or use default
    let noa_root = std::env::var("NOA_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("."));

    let db_path = NoaPaths::data(&noa_root).join("noa.db");

    let directories = NoaPaths::all_directories(&noa_root)
        .iter()
        .map(|p| p.display().to_string())
        .collect();

    let info = SystemInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        noa_root: noa_root.display().to_string(),
        initialized: db_path.exists(),
        database_path: db_path.display().to_string(),
        directories,
    };

    Ok(Json(info))
}

/// GET /api/v1/system/health
async fn get_system_health(Extension(state): Extension<AppState>) -> Result<Json<SystemHealth>, StatusCode> {
    info!("GET /api/v1/system/health");

    // Get NOA_ROOT from environment or use default
    let noa_root = std::env::var("NOA_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("."));

    let db_path = NoaPaths::data(&noa_root).join("noa.db");

    // Check database health
    let database = if db_path.exists() {
        match db::init_database(&db_path) {
            Ok(conn) => {
                match db::check_integrity(&conn) {
                    Ok(true) => HealthStatus {
                        status: "healthy".to_string(),
                        message: None,
                    },
                    Ok(false) => HealthStatus {
                        status: "degraded".to_string(),
                        message: Some("Database integrity check failed".to_string()),
                    },
                    Err(e) => HealthStatus {
                        status: "unhealthy".to_string(),
                        message: Some(format!("Database error: {}", e)),
                    },
                }
            }
            Err(e) => HealthStatus {
                status: "unhealthy".to_string(),
                message: Some(format!("Database connection failed: {}", e)),
            },
        }
    } else {
        HealthStatus {
            status: "uninitialized".to_string(),
            message: Some("Database not initialized".to_string()),
        }
    };

    // Check directories
    let missing = crate::init::DirectoryStructure::verify(&noa_root)
        .unwrap_or_else(|_| vec!["verification failed".to_string()]);

    let directories = if missing.is_empty() {
        HealthStatus {
            status: "healthy".to_string(),
            message: None,
        }
    } else {
        HealthStatus {
            status: "degraded".to_string(),
            message: Some(format!("Missing directories: {:?}", missing)),
        }
    };

    // Overall status
    let status = if database.status == "healthy" && directories.status == "healthy" {
        "healthy"
    } else if database.status == "unhealthy" || directories.status == "unhealthy" {
        "unhealthy"
    } else {
        "degraded"
    };

    let health = SystemHealth {
        status: status.to_string(),
        database,
        directories,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(health))
}

