//! Transition Query API
//!
//! Implements T611: Transition query API
//! Provides query interface for retrieving and filtering plane transition records

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use std::collections::HashMap;

use super::transition_logger::{TransitionRecord, TransitionType, TransitionStatus};

/// Query filter for transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionFilter {
    pub transition_type: Option<TransitionType>,
    pub source_plane: Option<String>,
    pub target_plane: Option<String>,
    pub status: Option<TransitionStatus>,
    pub initiated_by: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Default for TransitionFilter {
    fn default() -> Self {
        Self {
            transition_type: None,
            source_plane: None,
            target_plane: None,
            status: None,
            initiated_by: None,
            date_from: None,
            date_to: None,
            limit: Some(100),
            offset: Some(0),
        }
    }
}

/// Query result with pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub transitions: Vec<TransitionRecord>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// Transition statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionStats {
    pub total_transitions: i64,
    pub by_type: HashMap<String, i64>,
    pub by_status: HashMap<String, i64>,
    pub by_source_plane: HashMap<String, i64>,
    pub by_target_plane: HashMap<String, i64>,
    pub average_duration_seconds: Option<f64>,
    pub success_rate: f64,
    pub failure_rate: f64,
}

/// Transition Query API
///
/// Provides query interface for retrieving and analyzing plane transition records
pub struct TransitionQuery {
    pool: SqlitePool,
}

impl TransitionQuery {
    /// Create a new TransitionQuery with database connection
    pub async fn new(pool: SqlitePool) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }

    /// Query transitions with filters
    pub async fn query_transitions(
        &self,
        filter: TransitionFilter,
    ) -> Result<QueryResult, sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::new(
            r#"
            SELECT id, created_at, type, source_plane, target_plane,
                   source_version, target_version, status, started_at,
                   completed_at, duration_seconds, pre_checks, post_checks,
                   validation_status, artifacts_transferred, outcome,
                   error_message, rollback_reason, initiated_by, approved_by,
                   metadata, before_state, after_state
            FROM plane_transition
            WHERE 1=1
            "#,
        );

        // Apply filters
        if let Some(ref transition_type) = filter.transition_type {
            query_builder.push(" AND type = ");
            query_builder.push_bind(transition_type.as_str());
        }

        if let Some(ref source_plane) = filter.source_plane {
            query_builder.push(" AND source_plane = ");
            query_builder.push_bind(source_plane);
        }

        if let Some(ref target_plane) = filter.target_plane {
            query_builder.push(" AND target_plane = ");
            query_builder.push_bind(target_plane);
        }

        if let Some(ref status) = filter.status {
            query_builder.push(" AND status = ");
            query_builder.push_bind(status.as_str());
        }

        if let Some(ref initiated_by) = filter.initiated_by {
            query_builder.push(" AND initiated_by = ");
            query_builder.push_bind(initiated_by);
        }

        if let Some(ref date_from) = filter.date_from {
            query_builder.push(" AND created_at >= ");
            query_builder.push_bind(date_from.to_rfc3339());
        }

        if let Some(ref date_to) = filter.date_to {
            query_builder.push(" AND created_at <= ");
            query_builder.push_bind(date_to.to_rfc3339());
        }

        // Order by created_at descending
        query_builder.push(" ORDER BY created_at DESC");

        // Apply limit and offset
        let limit = filter.limit.unwrap_or(100);
        let offset = filter.offset.unwrap_or(0);
        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset);

        let query = query_builder.build();
        let rows = query.fetch_all(&self.pool).await?;

        // Get total count
        let total = self.count_transitions(&filter).await?;

        // Convert rows to TransitionRecord
        let transitions: Result<Vec<TransitionRecord>, sqlx::Error> = rows
            .into_iter()
            .map(|row| self.row_to_transition_record(&row))
            .collect();

        Ok(QueryResult {
            transitions: transitions?,
            total,
            limit,
            offset,
        })
    }

    /// Get transition by ID
    pub async fn get_transition(
        &self,
        transition_id: &str,
    ) -> Result<Option<TransitionRecord>, sqlx::Error> {
        let row = sqlx::query(
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
        )
        .bind(transition_id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(|r| self.row_to_transition_record(&r))
            .transpose()
    }

    /// Get statistics for transitions
    pub async fn get_statistics(
        &self,
        date_from: Option<DateTime<Utc>>,
        date_to: Option<DateTime<Utc>>,
    ) -> Result<TransitionStats, sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT COUNT(*) as total FROM plane_transition WHERE 1=1",
        );

        if let Some(ref date_from) = date_from {
            query_builder.push(" AND created_at >= ");
            query_builder.push_bind(date_from.to_rfc3339());
        }

        if let Some(ref date_to) = date_to {
            query_builder.push(" AND created_at <= ");
            query_builder.push_bind(date_to.to_rfc3339());
        }

        let total: i64 = query_builder
            .build()
            .try_map(|row: sqlx::sqlite::SqliteRow| row.try_get("total"))
            .fetch_one(&self.pool)
            .await?;

        // Count by type
        let mut by_type = HashMap::new();
        let type_rows = sqlx::query(
            r#"
            SELECT type, COUNT(*) as count
            FROM plane_transition
            GROUP BY type
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        for row in type_rows {
            let type_name: String = row.try_get("type")?;
            let count: i64 = row.try_get("count")?;
            by_type.insert(type_name, count);
        }

        // Count by status
        let mut by_status = HashMap::new();
        let status_rows = sqlx::query(
            r#"
            SELECT status, COUNT(*) as count
            FROM plane_transition
            GROUP BY status
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        for row in status_rows {
            let status_name: String = row.try_get("status")?;
            let count: i64 = row.try_get("count")?;
            by_status.insert(status_name, count);
        }

        // Count by source plane
        let mut by_source_plane = HashMap::new();
        let source_rows = sqlx::query(
            r#"
            SELECT source_plane, COUNT(*) as count
            FROM plane_transition
            GROUP BY source_plane
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        for row in source_rows {
            let plane: String = row.try_get("source_plane")?;
            let count: i64 = row.try_get("count")?;
            by_source_plane.insert(plane, count);
        }

        // Count by target plane
        let mut by_target_plane = HashMap::new();
        let target_rows = sqlx::query(
            r#"
            SELECT target_plane, COUNT(*) as count
            FROM plane_transition
            GROUP BY target_plane
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        for row in target_rows {
            let plane: String = row.try_get("target_plane")?;
            let count: i64 = row.try_get("count")?;
            by_target_plane.insert(plane, count);
        }

        // Calculate average duration
        let avg_duration: Option<f64> = sqlx::query(
            r#"
            SELECT AVG(duration_seconds) as avg_duration
            FROM plane_transition
            WHERE duration_seconds IS NOT NULL
            "#,
        )
        .try_map(|row: sqlx::sqlite::SqliteRow| row.try_get("avg_duration"))
        .fetch_optional(&self.pool)
        .await?;

        // Calculate success and failure rates
        let completed: i64 = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM plane_transition
            WHERE status = 'completed'
            "#,
        )
        .try_map(|row: sqlx::sqlite::SqliteRow| row.try_get("count"))
        .fetch_one(&self.pool)
        .await?;

        let failed: i64 = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM plane_transition
            WHERE status = 'failed'
            "#,
        )
        .try_map(|row: sqlx::sqlite::SqliteRow| row.try_get("count"))
        .fetch_one(&self.pool)
        .await?;

        let success_rate = if total > 0 {
            (completed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let failure_rate = if total > 0 {
            (failed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Ok(TransitionStats {
            total_transitions: total,
            by_type,
            by_status,
            by_source_plane,
            by_target_plane,
            average_duration_seconds: avg_duration,
            success_rate,
            failure_rate,
        })
    }

    /// Count transitions matching filter
    async fn count_transitions(&self, filter: &TransitionFilter) -> Result<i64, sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT COUNT(*) as total FROM plane_transition WHERE 1=1",
        );

        if let Some(ref transition_type) = filter.transition_type {
            query_builder.push(" AND type = ");
            query_builder.push_bind(transition_type.as_str());
        }

        if let Some(ref source_plane) = filter.source_plane {
            query_builder.push(" AND source_plane = ");
            query_builder.push_bind(source_plane);
        }

        if let Some(ref target_plane) = filter.target_plane {
            query_builder.push(" AND target_plane = ");
            query_builder.push_bind(target_plane);
        }

        if let Some(ref status) = filter.status {
            query_builder.push(" AND status = ");
            query_builder.push_bind(status.as_str());
        }

        if let Some(ref initiated_by) = filter.initiated_by {
            query_builder.push(" AND initiated_by = ");
            query_builder.push_bind(initiated_by);
        }

        if let Some(ref date_from) = filter.date_from {
            query_builder.push(" AND created_at >= ");
            query_builder.push_bind(date_from.to_rfc3339());
        }

        if let Some(ref date_to) = filter.date_to {
            query_builder.push(" AND created_at <= ");
            query_builder.push_bind(date_to.to_rfc3339());
        }

        let total: i64 = query_builder
            .build()
            .try_map(|row: sqlx::sqlite::SqliteRow| row.try_get("total"))
            .fetch_one(&self.pool)
            .await?;

        Ok(total)
    }

    /// Convert database row to TransitionRecord
    fn row_to_transition_record(
        &self,
        row: &sqlx::sqlite::SqliteRow,
    ) -> Result<TransitionRecord, sqlx::Error> {

        let before_state_json: String = row.try_get("before_state")?;
        let before_state: super::transition_logger::PlaneState = serde_json::from_str(&before_state_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let after_state: Option<super::transition_logger::PlaneState> = row
            .try_get::<Option<String>, _>("after_state")?
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?;

        let pre_checks_json: String = row.try_get("pre_checks")?;
        let pre_checks: Vec<super::transition_logger::CheckResult> = serde_json::from_str(&pre_checks_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let post_checks_json: String = row.try_get("post_checks")?;
        let post_checks: Vec<super::transition_logger::CheckResult> = serde_json::from_str(&post_checks_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let type_str: String = row.try_get("type")?;
        let transition_type = match type_str.as_str() {
            "promotion" => TransitionType::Promotion,
            "rollback" => TransitionType::Rollback,
            "migration" => TransitionType::Migration,
            "failover" => TransitionType::Failover,
            _ => return Err(sqlx::Error::Decode("Invalid transition type".into())),
        };

        let status_str: String = row.try_get("status")?;
        let status = match status_str.as_str() {
            "pending" => TransitionStatus::Pending,
            "preparing" => TransitionStatus::Preparing,
            "in_progress" => TransitionStatus::InProgress,
            "validating" => TransitionStatus::Validating,
            "completed" => TransitionStatus::Completed,
            "failed" => TransitionStatus::Failed,
            "rolled_back" => TransitionStatus::RolledBack,
            _ => return Err(sqlx::Error::Decode("Invalid transition status".into())),
        };

        let validation_status: Option<String> = row.try_get("validation_status")?;
        let validation_status = validation_status.map(|s| match s.as_str() {
            "passed" => super::transition_logger::ValidationStatus::Passed,
            "failed" => super::transition_logger::ValidationStatus::Failed,
            "skipped" => super::transition_logger::ValidationStatus::Skipped,
            _ => super::transition_logger::ValidationStatus::Skipped,
        });

        let created_at_str: String = row.try_get("created_at")?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .with_timezone(&Utc);

        let started_at: Option<String> = row.try_get("started_at")?;
        let started_at = started_at
            .map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .transpose()?;

        let completed_at: Option<String> = row.try_get("completed_at")?;
        let completed_at = completed_at
            .map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .transpose()?;

        let artifacts_transferred: Option<String> = row.try_get("artifacts_transferred")?;
        let artifacts_transferred: Vec<String> = artifacts_transferred
            .map(|json| {
                serde_json::from_str(&json)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?
            .unwrap_or_default();

        let metadata: Option<String> = row.try_get("metadata")?;
        let metadata = metadata
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?;

        Ok(TransitionRecord {
            id: row.try_get("id")?,
            created_at,
            transition_type,
            source_plane: row.try_get("source_plane")?,
            target_plane: row.try_get("target_plane")?,
            source_version: row.try_get("source_version")?,
            target_version: row.try_get("target_version")?,
            status,
            started_at,
            completed_at,
            duration_seconds: row.try_get("duration_seconds")?,
            before_state,
            after_state,
            pre_checks,
            post_checks,
            validation_status,
            artifacts_transferred,
            outcome: row.try_get("outcome")?,
            error_message: row.try_get("error_message")?,
            rollback_reason: row.try_get("rollback_reason")?,
            initiated_by: row.try_get("initiated_by")?,
            approved_by: row.try_get("approved_by")?,
            metadata,
        })
    }
}

