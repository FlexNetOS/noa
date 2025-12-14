//! Transition Logger - Before/After State Logging
//!
//! Implements T609: §3.5 Transition logger (before/after state)
//! Logs complete state snapshots before and after plane transitions for audit trail

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use uuid::Uuid;

/// Type alias for transition record database row data
pub type TransitionRowData = (
    String, String, String, String, String, String, String, String,
    Option<String>, Option<String>, Option<i64>, String, String,
    Option<String>, Option<String>, Option<String>, Option<String>,
    Option<String>, String, Option<String>, Option<String>, String, Option<String>
);

/// Transition type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionType {
    Promotion,
    Rollback,
    Migration,
    Failover,
}

impl TransitionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransitionType::Promotion => "promotion",
            TransitionType::Rollback => "rollback",
            TransitionType::Migration => "migration",
            TransitionType::Failover => "failover",
        }
    }
}

/// Transition status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionStatus {
    Pending,
    Preparing,
    InProgress,
    Validating,
    Completed,
    Failed,
    RolledBack,
}

impl TransitionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransitionStatus::Pending => "pending",
            TransitionStatus::Preparing => "preparing",
            TransitionStatus::InProgress => "in_progress",
            TransitionStatus::Validating => "validating",
            TransitionStatus::Completed => "completed",
            TransitionStatus::Failed => "failed",
            TransitionStatus::RolledBack => "rolled_back",
        }
    }
}

/// Validation status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Passed,
    Failed,
    Skipped,
}

impl ValidationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValidationStatus::Passed => "passed",
            ValidationStatus::Failed => "failed",
            ValidationStatus::Skipped => "skipped",
        }
    }
}

/// Pre-check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub status: String, // "passed", "failed", "warning"
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
}

/// Plane state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaneState {
    pub plane_id: String,
    pub plane_name: String,
    pub version: String,
    pub status: String,
    pub health_status: String,
    pub components: HashMap<String, ComponentState>,
    pub resources: Option<ResourceState>,
    pub config_hash: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Component state within a plane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentState {
    pub name: String,
    pub version: String,
    pub status: String,
    pub health: String,
    pub metadata: Option<serde_json::Value>,
}

/// Resource state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceState {
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<f64>,
    pub storage_usage: Option<f64>,
    pub network_io: Option<serde_json::Value>,
}

/// Transition record with before/after state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRecord {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub transition_type: TransitionType,
    pub source_plane: String,
    pub target_plane: String,
    pub source_version: String,
    pub target_version: String,
    pub status: TransitionStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub before_state: PlaneState,
    pub after_state: Option<PlaneState>,
    pub pre_checks: Vec<CheckResult>,
    pub post_checks: Vec<CheckResult>,
    pub validation_status: Option<ValidationStatus>,
    pub artifacts_transferred: Vec<String>,
    pub outcome: Option<String>,
    pub error_message: Option<String>,
    pub rollback_reason: Option<String>,
    pub initiated_by: String,
    pub approved_by: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Transition Logger
///
/// Logs plane transitions with complete before/after state snapshots
/// for audit trail and compliance with §3.5 (Transparent & Auditable)
pub struct TransitionLogger {
    conn: Arc<Mutex<Connection>>,
}

impl TransitionLogger {
    /// Create a new TransitionLogger with database connection
    pub async fn new(conn: Connection) -> anyhow::Result<Self> {
        let conn = Arc::new(Mutex::new(conn));
        
        // Ensure the plane_transition table exists
        {
            let conn_clone = Arc::clone(&conn);
            task::spawn_blocking(move || {
                let conn = conn_clone.blocking_lock();
                conn.execute(
                    r#"
                    CREATE TABLE IF NOT EXISTS plane_transition (
                        id TEXT PRIMARY KEY,
                        created_at TEXT NOT NULL DEFAULT (datetime('now')),
                        type TEXT NOT NULL CHECK (type IN ('promotion', 'rollback', 'migration', 'failover')),
                        source_plane TEXT NOT NULL,
                        target_plane TEXT NOT NULL,
                        source_version TEXT NOT NULL,
                        target_version TEXT NOT NULL,
                        status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN (
                            'pending', 'preparing', 'in_progress', 'validating',
                            'completed', 'failed', 'rolled_back'
                        )),
                        started_at TEXT,
                        completed_at TEXT,
                        duration_seconds INTEGER,
                        pre_checks TEXT,
                        post_checks TEXT,
                        validation_status TEXT CHECK (validation_status IN ('passed', 'failed', 'skipped')),
                        artifacts_transferred TEXT,
                        outcome TEXT,
                        error_message TEXT,
                        rollback_reason TEXT,
                        initiated_by TEXT NOT NULL,
                        approved_by TEXT,
                        metadata TEXT,
                        checksum TEXT NOT NULL,
                        before_state TEXT NOT NULL,
                        after_state TEXT
                    )
                    "#,
                    [],
                )
            }).await.map_err(anyhow::Error::from)??;
        }

        Ok(Self { conn })
    }

    /// Start logging a transition with before state
    ///
    /// Captures the complete state of the source plane before transition begins
    #[allow(clippy::too_many_arguments)] // Required for comprehensive audit logging
    pub async fn log_transition_start(
        &self,
        transition_type: TransitionType,
        source_plane: &str,
        target_plane: &str,
        source_version: &str,
        target_version: &str,
        before_state: PlaneState,
        pre_checks: Vec<CheckResult>,
        initiated_by: &str,
        metadata: Option<serde_json::Value>,
    ) -> anyhow::Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Serialize state and checks
        let before_state_json = serde_json::to_string(&before_state)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let pre_checks_json = serde_json::to_string(&pre_checks)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let metadata_json = metadata
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        // Calculate checksum of before_state
        let checksum = self.calculate_checksum(&before_state_json);

        let conn = Arc::clone(&self.conn);
        let source_plane_clone = source_plane.to_string();
        let target_plane_clone = target_plane.to_string();
        let source_version_clone = source_version.to_string();
        let target_version_clone = target_version.to_string();
        let initiated_by_clone = initiated_by.to_string();
        let transition_type_str = transition_type.as_str().to_string();
        let id_clone = id.clone();
        
        task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            conn.execute(
                r#"
                INSERT INTO plane_transition (
                    id, created_at, type, source_plane, target_plane,
                    source_version, target_version, status, before_state,
                    pre_checks, initiated_by, metadata, checksum
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                params![
                    &id_clone,
                    now.to_rfc3339(),
                    transition_type_str,
                    source_plane_clone,
                    target_plane_clone,
                    source_version_clone,
                    target_version_clone,
                    TransitionStatus::Pending.as_str(),
                    &before_state_json,
                    &pre_checks_json,
                    initiated_by_clone,
                    &metadata_json,
                    &checksum
                ],
            )
        }).await.map_err(anyhow::Error::from)??;

        tracing::info!(
            transition_id = %id,
            transition_type = %transition_type.as_str(),
            source_plane = %source_plane,
            target_plane = %target_plane,
            "Transition logging started"
        );

        Ok(id)
    }

    pub async fn update_status(
        &self,
        transition_id: &str,
        status: TransitionStatus,
    ) -> anyhow::Result<()> {
        let now = Utc::now();

        let conn = Arc::clone(&self.conn);
        let transition_id_clone = transition_id.to_string();
        let status_str = status.as_str().to_string();
        
        task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            conn.execute(
                r#"
                UPDATE plane_transition
                SET status = ?, started_at = COALESCE(started_at, ?)
                WHERE id = ?
                "#,
                params![status_str, now.to_rfc3339(), transition_id_clone],
            )
        }).await.map_err(anyhow::Error::from)??;

        tracing::debug!(
            transition_id = %transition_id,
            status = %status.as_str(),
            "Transition status updated"
        );

        Ok(())
    }

    pub async fn log_transition_complete(
        &self,
        transition_id: &str,
        after_state: PlaneState,
        post_checks: Vec<CheckResult>,
        validation_status: Option<ValidationStatus>,
        outcome: Option<String>,
        error_message: Option<String>,
    ) -> anyhow::Result<()> {
        let now = Utc::now();

        // Get started_at to calculate duration
        let conn_clone = Arc::clone(&self.conn);
        let transition_id_clone = transition_id.to_string();
        let started_at_str: Option<String> = task::spawn_blocking(move || {
            let conn = conn_clone.blocking_lock();
            let mut stmt = conn.prepare("SELECT started_at FROM plane_transition WHERE id = ?")?;
            let mut rows = stmt.query_map([transition_id_clone], |row| row.get(0))?;
            rows.next().transpose()
        }).await.map_err(anyhow::Error::from)??;

        let duration_seconds = started_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|start| (now - start.with_timezone(&Utc)).num_seconds());

        // Serialize state and checks
        let after_state_json = serde_json::to_string(&after_state)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let post_checks_json = serde_json::to_string(&post_checks)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        let status = if error_message.is_some() {
            TransitionStatus::Failed
        } else {
            TransitionStatus::Completed
        };

        let conn = Arc::clone(&self.conn);
        let transition_id_clone = transition_id.to_string();
        let status_str = status.as_str().to_string();
        let validation_status_str = validation_status.map(|v| v.as_str().to_string());
        
        task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            conn.execute(
                r#"
                UPDATE plane_transition
                SET status = ?, completed_at = ?, duration_seconds = ?,
                    after_state = ?, post_checks = ?, validation_status = ?,
                    outcome = ?, error_message = ?
                WHERE id = ?
                "#,
                params![
                    status_str,
                    now.to_rfc3339(),
                    duration_seconds,
                    &after_state_json,
                    &post_checks_json,
                    validation_status_str,
                    &outcome,
                    &error_message,
                    transition_id_clone
                ],
            )
        }).await.map_err(anyhow::Error::from)??;

        tracing::info!(
            transition_id = %transition_id,
            status = %status.as_str(),
            duration_seconds = ?duration_seconds,
            "Transition logging completed"
        );

        Ok(())
    }

    pub async fn get_transition(
        &self,
        transition_id: &str,
    ) -> anyhow::Result<Option<TransitionRecord>> {
        let conn = Arc::clone(&self.conn);
        let transition_id = transition_id.to_string();
        let result = task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            let mut stmt = conn.prepare(
                r#"
                SELECT id, created_at, type, source_plane, target_plane,
                       source_version, target_version, status, started_at,
                       completed_at, duration_seconds, pre_checks, post_checks,
                       validation_status, artifacts_transferred, outcome,
                       error_message, rollback_reason, initiated_by, approved_by,
                       metadata, before_state, after_state
                FROM plane_transition
                WHERE id = ?
                "#,
            )?;
            
            let mut rows = stmt.query_map([transition_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,  // id
                    row.get::<_, String>(1)?,  // created_at
                    row.get::<_, String>(2)?,  // type
                    row.get::<_, String>(3)?,  // source_plane
                    row.get::<_, String>(4)?,  // target_plane
                    row.get::<_, String>(5)?,  // source_version
                    row.get::<_, String>(6)?,  // target_version
                    row.get::<_, String>(7)?,  // status
                    row.get::<_, Option<String>>(8)?,  // started_at
                    row.get::<_, Option<String>>(9)?,  // completed_at
                    row.get::<_, Option<i64>>(10)?,  // duration_seconds
                    row.get::<_, String>(11)?,  // pre_checks
                    row.get::<_, String>(12)?,  // post_checks
                    row.get::<_, Option<String>>(13)?,  // validation_status
                    row.get::<_, Option<String>>(14)?,  // artifacts_transferred
                    row.get::<_, Option<String>>(15)?,  // outcome
                    row.get::<_, Option<String>>(16)?,  // error_message
                    row.get::<_, Option<String>>(17)?,  // rollback_reason
                    row.get::<_, String>(18)?,  // initiated_by
                    row.get::<_, Option<String>>(19)?,  // approved_by
                    row.get::<_, Option<String>>(20)?,  // metadata
                    row.get::<_, String>(21)?,  // before_state
                    row.get::<_, Option<String>>(22)?,  // after_state
                ))
            })?;

            rows.next().transpose()
        }).await.map_err(anyhow::Error::from)?;

        match result {
            Ok(Some(row_data)) => {
                let record = self.row_data_to_transition_record(&row_data)?;
                Ok(Some(record))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Calculate SHA-256 checksum for state integrity
    fn calculate_checksum(&self, data: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Convert database row data to TransitionRecord
    fn row_data_to_transition_record(
        &self,
        row_data: &TransitionRowData,
    ) -> anyhow::Result<TransitionRecord> {
        let (
            id, created_at_str, type_str, source_plane, target_plane,
            source_version, target_version, status_str, started_at_str,
            completed_at_str, duration_seconds, pre_checks_json, post_checks_json,
            validation_status_str, artifacts_transferred_json, outcome,
            error_message, rollback_reason, initiated_by, approved_by,
            metadata_json, before_state_json, after_state_json
        ) = row_data;

        let before_state: PlaneState = serde_json::from_str(before_state_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

        let after_state: Option<PlaneState> = after_state_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        let pre_checks: Vec<CheckResult> = serde_json::from_str(pre_checks_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

        let post_checks: Vec<CheckResult> = serde_json::from_str(post_checks_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

        let transition_type = match type_str.as_str() {
            "promotion" => TransitionType::Promotion,
            "rollback" => TransitionType::Rollback,
            "migration" => TransitionType::Migration,
            "failover" => TransitionType::Failover,
            _ => return Err(anyhow::anyhow!("Invalid transition type: {}", type_str)),
        };

        let status = match status_str.as_str() {
            "pending" => TransitionStatus::Pending,
            "preparing" => TransitionStatus::Preparing,
            "in_progress" => TransitionStatus::InProgress,
            "validating" => TransitionStatus::Validating,
            "completed" => TransitionStatus::Completed,
            "failed" => TransitionStatus::Failed,
            "rolled_back" => TransitionStatus::RolledBack,
            _ => return Err(anyhow::anyhow!("Invalid transition status: {}", status_str)),
        };

        let validation_status = validation_status_str.as_ref().map(|s| match s.as_str() {
            "passed" => ValidationStatus::Passed,
            "failed" => ValidationStatus::Failed,
            "skipped" => ValidationStatus::Skipped,
            _ => ValidationStatus::Skipped,
        });

        let created_at = DateTime::parse_from_rfc3339(created_at_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);

        let started_at = started_at_str
            .as_ref()
            .map(|s| {
                DateTime::parse_from_rfc3339(s)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .transpose()?;

        let completed_at = completed_at_str
            .as_ref()
            .map(|s| {
                DateTime::parse_from_rfc3339(s)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .transpose()?;

        let artifacts_transferred: Vec<String> = artifacts_transferred_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?
            .unwrap_or_default();

        let metadata = metadata_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        Ok(TransitionRecord {
            id: id.clone(),
            created_at,
            transition_type,
            source_plane: source_plane.clone(),
            target_plane: target_plane.clone(),
            source_version: source_version.clone(),
            target_version: target_version.clone(),
            status,
            started_at,
            completed_at,
            duration_seconds: *duration_seconds,
            before_state,
            after_state,
            pre_checks,
            post_checks,
            validation_status,
            artifacts_transferred,
            outcome: outcome.clone(),
            error_message: error_message.clone(),
            rollback_reason: rollback_reason.clone(),
            initiated_by: initiated_by.clone(),
            approved_by: approved_by.clone(),
            metadata,
        })
    }
}

