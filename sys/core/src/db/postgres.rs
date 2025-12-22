//! PostgreSQL support helpers (feature-gated)
//!
//! This module provides minimal PostgreSQL connectivity, migration, and health checks
//! for the CLI path. The runtime API server is still SQLite-first.

use std::path::Path;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::error::{DatabaseError, NoaError, Result};

pub async fn connect_postgres(url: &str, max_connections: u32) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::ConnectionFailed(e.to_string())))
}

pub async fn check_postgres(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map_err(|e| {
            NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT 1".to_string(),
                error: e.to_string(),
            })
        })?;

    Ok(())
}

pub async fn migrate_postgres(pool: &PgPool, migrations_dir: &Path) -> Result<()> {
    let migrator = sqlx::migrate::Migrator::new(migrations_dir)
        .await
        .map_err(|e| {
            NoaError::Database(DatabaseError::MigrationFailed {
                version: "discovery".to_string(),
                error: e.to_string(),
            })
        })?;

    migrator.run(pool).await.map_err(|e| {
        NoaError::Database(DatabaseError::MigrationFailed {
            version: "run".to_string(),
            error: e.to_string(),
        })
    })?;

    Ok(())
}
