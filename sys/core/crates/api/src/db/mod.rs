//! Database module for NOA API

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::Path;
use std::str::FromStr;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(db_path: &Path) -> Result<Self, sqlx::Error> {
        let db_url = format!("sqlite:{}", db_path.display());

        let options = SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(options).await?;

        let db = Self { pool };
        db.initialize_schema().await?;

        Ok(db)
    }

    async fn initialize_schema(&self) -> Result<(), sqlx::Error> {
        let schema = include_str!("../../../../../../init/migrations/001_initial.sql");

        sqlx::raw_sql(schema).execute(&self.pool).await?;

        tracing::info!("Database schema initialized successfully");
        Ok(())
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}
