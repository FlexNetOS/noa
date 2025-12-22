//! Database module for NOA API

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub async fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path).with_context(|| {
            format!("Failed to open database at {}", db_path.display())
        })?;

        // Enable WAL mode for better concurrency
        conn.pragma_update(None, "journal_mode", "WAL")
            .context("Failed to enable WAL mode")?;
        conn.pragma_update(None, "synchronous", "NORMAL")
            .context("Failed to set synchronous=NORMAL")?;

        // Foreign key constraints are disabled by default in SQLite and are
        // connection-local. We must enable them on every new connection.
        //
        // Note: Attempting to set PRAGMA foreign_keys within a migration that
        // runs inside a transaction may be ignored by SQLite, so do it here.
        conn.pragma_update(None, "foreign_keys", "ON")
            .context("Failed to enable foreign key constraints")?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        db.apply_repo_migrations()?;

        Ok(db)
    }

    pub fn with_conn<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Connection) -> Result<R>,
    {
        let conn = self
            .conn
            .lock()
            .expect("database connection mutex poisoned");
        f(&conn)
    }

    pub async fn health_check(&self) -> Result<()> {
        // `Connection::execute` is for statements that do NOT return rows.
        // A SELECT will trigger rusqlite::Error::ExecuteReturnedResults.
        self.with_conn(|conn| {
            let _: i32 = conn.query_row("SELECT 1", [], |row| row.get(0))?;
            Ok(())
        })?;
        Ok(())
    }

    fn apply_repo_migrations(&self) -> Result<()> {
        let migrations_dir = find_migrations_dir()
            .context("Could not locate init/migrations directory for DB migrations")?;

        // The repo-level migrations directory currently contains both "minimal" and
        // "full" schema migrations. Some of the "full" migrations reference tables
        // that are not present in the minimal bootstrap schema.
        //
        // For noa-api we default to applying only the minimal schema plus auth-related
        // migrations, unless explicitly overridden.
        let apply_all = std::env::var("NOA_API_APPLY_ALL_MIGRATIONS")
            .ok()
            .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false);

        self.with_conn(|conn| {
            conn.execute_batch(
                r#"
                CREATE TABLE IF NOT EXISTS schema_migrations (
                    version TEXT PRIMARY KEY,
                    description TEXT,
                    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
                );
                "#,
            )
            .context("Failed to ensure schema_migrations table exists")?;

            let mut stmt = conn
                .prepare("SELECT version FROM schema_migrations ORDER BY version")
                .context("Failed to prepare migration version query")?;

            let applied: HashSet<String> = stmt
                .query_map([], |row| row.get(0))
                .context("Failed to query applied migrations")?
                .filter_map(|r| r.ok())
                .collect();

            let mut entries: Vec<_> = fs::read_dir(&migrations_dir)
                .with_context(|| format!("Failed to read {}", migrations_dir.display()))?
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "sql")
                        .unwrap_or(false)
                })
                .collect();

            entries.sort_by_key(|e| e.file_name());

            for entry in entries {
                let path = entry.path();
                let version = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();

                if !apply_all {
                    // Always apply the minimal bootstrap migration.
                    // Then apply any auth-related migrations.
                    let is_minimal = version == "001_initial";
                    let is_auth_related = version.contains("_auth");
                    if !is_minimal && !is_auth_related {
                        continue;
                    }
                }

                if version.is_empty() || applied.contains(&version) {
                    continue;
                }

                let description = version
                    .split('_')
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(" ");

                let sql = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read migration {}", path.display()))?;

                let tx = conn
                    .unchecked_transaction()
                    .context("Failed to start migration transaction")?;

                tx.execute_batch(&sql).with_context(|| {
                    format!("Failed to execute migration {version} ({})", path.display())
                })?;

                tx.execute(
                    "INSERT OR REPLACE INTO schema_migrations (version, description, applied_at) VALUES (?1, ?2, datetime('now'))",
                    params![version, description],
                )
                .context("Failed to record applied migration")?;

                tx.commit().context("Failed to commit migration")?;
            }

            Ok(())
        })?;

        tracing::info!("Database migrations applied successfully");
        Ok(())
    }
}

fn find_migrations_dir() -> Option<PathBuf> {
    // 1) Explicit repo root override
    if let Ok(root) = std::env::var("NOA_ROOT") {
        let cand = PathBuf::from(root).join("init").join("migrations");
        if cand.exists() {
            return Some(cand);
        }
    }

    // 2) Walk upwards from current working directory
    if let Ok(mut dir) = std::env::current_dir() {
        loop {
            let cand = dir.join("init").join("migrations");
            if cand.exists() {
                return Some(cand);
            }
            if !dir.pop() {
                break;
            }
        }
    }

    // 3) Dev fallback (works when running from within the repo)
    let cand = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../init/migrations");
    if cand.exists() {
        return Some(cand);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn health_check_does_not_return_results_error() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();

        let db_path = std::env::temp_dir().join(format!("noa-api-test-{nonce}.db"));
        let _ = std::fs::remove_file(&db_path);

        let db = Database::new(&db_path).await.expect("db init");
        db.health_check().await.expect("health check");

        let _ = std::fs::remove_file(&db_path);
    }
}
