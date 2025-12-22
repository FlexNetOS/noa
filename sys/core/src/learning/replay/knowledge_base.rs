//! External Knowledge Base Connector
//!
//! T663: Implement external knowledge base connector
//! US2: Connect to external knowledge bases

use crate::error::{NoaError, Result};
use crate::learning::replay::Experience;
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Knowledge base connector
pub struct KnowledgeBaseConnector {
    connection_string: String,
}

impl KnowledgeBaseConnector {
    /// Create a new connector
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }

    fn db_path(&self) -> PathBuf {
        let s = self.connection_string.trim();

        // Explicit sqlite scheme
        if let Some(rest) = s.strip_prefix("sqlite:///") {
            return PathBuf::from(rest);
        }
        if let Some(rest) = s.strip_prefix("sqlite://") {
            return PathBuf::from(rest);
        }
        if let Some(rest) = s.strip_prefix("sqlite:") {
            return PathBuf::from(rest);
        }

        // Tests / dev: stable, writable location
        if s.starts_with("test://") || s.starts_with("memory://") {
            let hash = blake3::hash(s.as_bytes());
            let file = format!("noa-kb-{}.sqlite", hash.to_hex());
            return std::env::temp_dir().join("noa").join("kb").join(file);
        }

        // Fallback: treat as a filesystem path.
        PathBuf::from(s)
    }

    fn open_and_migrate(path: &Path) -> Result<Connection> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;

        // Minimal schema for experiences.
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS noa_kb_experiences (
                id              TEXT PRIMARY KEY,
                timestamp       TEXT NOT NULL,
                reward          REAL NOT NULL,
                state_json      TEXT NOT NULL,
                action_json     TEXT NOT NULL,
                next_state_json TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_noa_kb_experiences_timestamp
                ON noa_kb_experiences(timestamp);
            "#,
        )?;

        Ok(conn)
    }

    fn join_err(op: &'static str, err: tokio::task::JoinError) -> NoaError {
        NoaError::Internal {
            message: format!("KnowledgeBaseConnector task join failed during {}: {}", op, err),
            source: None,
        }
    }

    /// Store experience in knowledge base
    pub async fn store_experience(&self, experience: &Experience) -> Result<()> {
        let db_path = self.db_path();
        let exp = experience.clone();

        tokio::task::spawn_blocking(move || -> Result<()> {
            let conn = Self::open_and_migrate(&db_path)?;

            let ts = exp.timestamp.to_rfc3339();
            let state_json = serde_json::to_string(&exp.state)?;
            let action_json = serde_json::to_string(&exp.action)?;
            let next_state_json = match &exp.next_state {
                Some(v) => Some(serde_json::to_string(v)?),
                None => None,
            };

            conn.execute(
                r#"
                INSERT INTO noa_kb_experiences (
                    id, timestamp, reward, state_json, action_json, next_state_json
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(id) DO UPDATE SET
                    timestamp = excluded.timestamp,
                    reward = excluded.reward,
                    state_json = excluded.state_json,
                    action_json = excluded.action_json,
                    next_state_json = excluded.next_state_json
                "#,
                params![
                    exp.id.to_string(),
                    ts,
                    exp.reward,
                    state_json,
                    action_json,
                    next_state_json,
                ],
            )?;

            Ok(())
        })
        .await
        .map_err(|e| Self::join_err("store_experience", e))??;

        Ok(())
    }

    /// Retrieve similar experiences
    pub async fn retrieve_similar(&self, query: &serde_json::Value, limit: usize) -> Result<Vec<Experience>> {
        if limit == 0 {
            return Ok(vec![]);
        }

        let db_path = self.db_path();
        let query = query.clone();

        tokio::task::spawn_blocking(move || -> Result<Vec<Experience>> {
            let conn = Self::open_and_migrate(&db_path)?;

            // Very small-but-real similarity strategy:
            // - If query contains {"contains": "..."}, do a LIKE search over state/action JSON.
            // - Otherwise return most recent experiences.
            let contains = query
                .get("contains")
                .and_then(|v| v.as_str())
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| format!("%{}%", s));

            let mut out = Vec::new();

            if let Some(pattern) = contains {
                let mut stmt = conn.prepare(
                    r#"
                    SELECT id, timestamp, reward, state_json, action_json, next_state_json
                    FROM noa_kb_experiences
                    WHERE state_json LIKE ?1 OR action_json LIKE ?1
                    ORDER BY timestamp DESC
                    LIMIT ?2
                    "#,
                )?;

                let mut rows = stmt.query(params![pattern, limit as i64])?;
                while let Some(row) = rows.next()? {
                    let id_str: String = row.get(0)?;
                    let ts: String = row.get(1)?;
                    let reward: f64 = row.get(2)?;
                    let state_json: String = row.get(3)?;
                    let action_json: String = row.get(4)?;
                    let next_state_json: Option<String> = row.get(5)?;

                    let id = Uuid::parse_str(&id_str)?;
                    let timestamp = chrono::DateTime::parse_from_rfc3339(&ts)?.with_timezone(&chrono::Utc);
                    let state = serde_json::from_str(&state_json)?;
                    let action = serde_json::from_str(&action_json)?;
                    let next_state = match next_state_json {
                        Some(s) => Some(serde_json::from_str(&s)?),
                        None => None,
                    };

                    out.push(Experience {
                        id,
                        state,
                        action,
                        reward,
                        next_state,
                        timestamp,
                    });
                }
            } else {
                let mut stmt = conn.prepare(
                    r#"
                    SELECT id, timestamp, reward, state_json, action_json, next_state_json
                    FROM noa_kb_experiences
                    ORDER BY timestamp DESC
                    LIMIT ?1
                    "#,
                )?;

                let mut rows = stmt.query(params![limit as i64])?;
                while let Some(row) = rows.next()? {
                    let id_str: String = row.get(0)?;
                    let ts: String = row.get(1)?;
                    let reward: f64 = row.get(2)?;
                    let state_json: String = row.get(3)?;
                    let action_json: String = row.get(4)?;
                    let next_state_json: Option<String> = row.get(5)?;

                    let id = Uuid::parse_str(&id_str)?;
                    let timestamp = chrono::DateTime::parse_from_rfc3339(&ts)?.with_timezone(&chrono::Utc);
                    let state = serde_json::from_str(&state_json)?;
                    let action = serde_json::from_str(&action_json)?;
                    let next_state = match next_state_json {
                        Some(s) => Some(serde_json::from_str(&s)?),
                        None => None,
                    };

                    out.push(Experience {
                        id,
                        state,
                        action,
                        reward,
                        next_state,
                        timestamp,
                    });
                }
            }

            Ok(out)
        })
        .await
        .map_err(|e| Self::join_err("retrieve_similar", e))?
    }

    /// Search experiences by metadata
    pub async fn search(&self, metadata: &serde_json::Value) -> Result<Vec<Experience>> {
        let db_path = self.db_path();
        let metadata = metadata.clone();

        tokio::task::spawn_blocking(move || -> Result<Vec<Experience>> {
            let conn = Self::open_and_migrate(&db_path)?;

            // For now, treat metadata as a JSON blob and do a coarse LIKE search against
            // state/action JSON. This is a functional baseline until a dedicated metadata
            // schema or vector index is introduced.
            let needle = serde_json::to_string(&metadata)?;
            let pattern = format!("%{}%", needle.trim_matches('"'));

            let mut stmt = conn.prepare(
                r#"
                SELECT id, timestamp, reward, state_json, action_json, next_state_json
                FROM noa_kb_experiences
                WHERE state_json LIKE ?1 OR action_json LIKE ?1
                ORDER BY timestamp DESC
                LIMIT 200
                "#,
            )?;

            let mut out = Vec::new();
            let mut rows = stmt.query(params![pattern])?;
            while let Some(row) = rows.next()? {
                let id_str: String = row.get(0)?;
                let ts: String = row.get(1)?;
                let reward: f64 = row.get(2)?;
                let state_json: String = row.get(3)?;
                let action_json: String = row.get(4)?;
                let next_state_json: Option<String> = row.get(5)?;

                let id = Uuid::parse_str(&id_str)?;
                let timestamp = chrono::DateTime::parse_from_rfc3339(&ts)?.with_timezone(&chrono::Utc);
                let state = serde_json::from_str(&state_json)?;
                let action = serde_json::from_str(&action_json)?;
                let next_state = match next_state_json {
                    Some(s) => Some(serde_json::from_str(&s)?),
                    None => None,
                };

                out.push(Experience {
                    id,
                    state,
                    action,
                    reward,
                    next_state,
                    timestamp,
                });
            }

            Ok(out)
        })
        .await
        .map_err(|e| Self::join_err("search", e))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_knowledge_base_connector() {
        let connector = KnowledgeBaseConnector::new(format!("test://db/{}", Uuid::new_v4()));
        let exp = Experience {
            id: Uuid::new_v4(),
            state: serde_json::json!({}),
            action: serde_json::json!({}),
            reward: 1.0,
            next_state: None,
            timestamp: chrono::Utc::now(),
        };

        // Should not error
        connector.store_experience(&exp).await.unwrap();

        let results = connector
            .retrieve_similar(&serde_json::json!({"contains": ""}), 10)
            .await
            .unwrap();
        assert!(!results.is_empty());
    }
}

