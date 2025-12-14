//! Decision Rationale Recorder
//!
//! Implements T610: Decision rationale recorder
//! Records the reasoning behind promotion/rollback decisions for audit and transparency

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use uuid::Uuid;

/// Decision type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionType {
    Approve,
    Reject,
    Defer,
    RequireManualReview,
}

impl DecisionType {
    fn as_str(&self) -> &'static str {
        match self {
            DecisionType::Approve => "approve",
            DecisionType::Reject => "reject",
            DecisionType::Defer => "defer",
            DecisionType::RequireManualReview => "require_manual_review",
        }
    }
}

/// Risk tier enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskTier {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskTier {
    fn as_str(&self) -> &'static str {
        match self {
            RiskTier::Low => "low",
            RiskTier::Medium => "medium",
            RiskTier::High => "high",
            RiskTier::Critical => "critical",
        }
    }
}

/// Policy gate result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    pub gate_name: String,
    pub passed: bool,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub required: bool,
}

/// Analytics evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsResult {
    pub model: String,
    pub score: f64,
    pub confidence: f64,
    pub reasoning: String,
    pub recommendations: Vec<String>,
}

/// Decision rationale record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub id: String,
    pub transition_id: String,
    pub created_at: DateTime<Utc>,
    pub decision_type: DecisionType,
    pub risk_tier: RiskTier,
    pub rationale: String,
    pub policy_gates: Vec<GateResult>,
    pub analytics_results: Vec<AnalyticsResult>,
    pub test_results: Option<serde_json::Value>,
    pub risk_assessment: Option<serde_json::Value>,
    pub decision_factors: Vec<String>,
    pub approved_by: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Decision Rationale Recorder
///
/// Records detailed decision rationale for all promotion/rollback decisions
/// Ensures transparency and auditability of coordinator decisions
pub struct DecisionRecorder {
    conn: Arc<Mutex<Connection>>,
}

impl DecisionRecorder {
    /// Create a new DecisionRecorder with database connection
    pub async fn new(conn: Arc<Mutex<Connection>>) -> anyhow::Result<Self> {
        // Ensure the decision_record table exists
        let conn_clone = Arc::clone(&conn);
        let _ = task::spawn_blocking(move || {
            let conn = conn_clone.blocking_lock();
            conn.execute(
                r#"
                CREATE TABLE IF NOT EXISTS decision_record (
                    id TEXT PRIMARY KEY,
                    transition_id TEXT NOT NULL,
                    created_at TEXT NOT NULL DEFAULT (datetime('now')),
                    decision_type TEXT NOT NULL CHECK (decision_type IN (
                        'approve', 'reject', 'defer', 'require_manual_review'
                    )),
                    risk_tier TEXT NOT NULL CHECK (risk_tier IN (
                        'low', 'medium', 'high', 'critical'
                    )),
                    rationale TEXT NOT NULL,
                    policy_gates TEXT NOT NULL,
                    analytics_results TEXT,
                    test_results TEXT,
                    risk_assessment TEXT,
                    decision_factors TEXT NOT NULL,
                    approved_by TEXT,
                    metadata TEXT,
                    checksum TEXT NOT NULL
                )
                "#,
                [],
            )?;

            // Create index for fast lookup by transition_id
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_decision_transition ON decision_record(transition_id)",
                [],
            )?;

            Ok::<(), rusqlite::Error>(())
        }).await?;

        Ok(Self { conn })
    }

    /// Record a decision rationale
    ///
    /// Captures the complete reasoning behind a promotion/rollback decision,
    /// including policy gate results, analytics evaluations, and risk assessment
    #[allow(clippy::too_many_arguments)]
    pub async fn record_decision(
        &self,
        transition_id: &str,
        decision_type: DecisionType,
        risk_tier: RiskTier,
        rationale: &str,
        policy_gates: Vec<GateResult>,
        analytics_results: Vec<AnalyticsResult>,
        test_results: Option<serde_json::Value>,
        risk_assessment: Option<serde_json::Value>,
        decision_factors: Vec<String>,
        approved_by: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> anyhow::Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Serialize complex fields
        let policy_gates_json = serde_json::to_string(&policy_gates)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let analytics_results_json = serde_json::to_string(&analytics_results)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let test_results_json = test_results
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let risk_assessment_json = risk_assessment
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let decision_factors_json = serde_json::to_string(&decision_factors)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let metadata_json = metadata
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        // Calculate checksum of rationale and key decision data
        let checksum_data = format!("{}{}{}", rationale, decision_type.as_str(), risk_tier.as_str());
        let checksum = self.calculate_checksum(&checksum_data);

        let conn = Arc::clone(&self.conn);
        let transition_id_clone = transition_id.to_string();
        let rationale_clone = rationale.to_string();
        let approved_by_clone = approved_by.map(|s| s.to_string());
        let id_clone = id.clone();
        
        task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            conn.execute(
                r#"
                INSERT INTO decision_record (
                    id, transition_id, created_at, decision_type, risk_tier,
                    rationale, policy_gates, analytics_results, test_results,
                    risk_assessment, decision_factors, approved_by, metadata, checksum
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                params![
                    &id_clone,
                    &transition_id_clone,
                    now.to_rfc3339(),
                    decision_type.as_str(),
                    risk_tier.as_str(),
                    rationale_clone,
                    &policy_gates_json,
                    &analytics_results_json,
                    &test_results_json,
                    &risk_assessment_json,
                    &decision_factors_json,
                    &approved_by_clone,
                    &metadata_json,
                    &checksum
                ],
            )
        }).await.map_err(anyhow::Error::from)??;

        tracing::info!(
            decision_id = %id,
            transition_id = %transition_id,
            decision_type = %decision_type.as_str(),
            risk_tier = %risk_tier.as_str(),
            "Decision rationale recorded"
        );

        Ok(id)
    }

    /// Get decision record by transition ID
    pub async fn get_decision_by_transition(
        &self,
        transition_id: &str,
    ) -> anyhow::Result<Option<DecisionRecord>> {
        let conn = Arc::clone(&self.conn);
        let transition_id = transition_id.to_string();
        
        Ok(task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            let mut stmt = conn.prepare(
                r#"
                SELECT id, transition_id, created_at, decision_type, risk_tier,
                       rationale, policy_gates, analytics_results, test_results,
                       risk_assessment, decision_factors, approved_by, metadata
                FROM decision_record
                WHERE transition_id = ?
                ORDER BY created_at DESC
                LIMIT 1
                "#,
            )?;
            
            let mut rows = stmt.query_map([transition_id], |row| {
                Self::row_to_decision_record_static(row)
            })?;
            
            rows.next().transpose()
        }).await.map_err(anyhow::Error::from)??)
    }

    /// Get decision record by ID
    pub async fn get_decision(
        &self,
        decision_id: &str,
    ) -> anyhow::Result<Option<DecisionRecord>> {
        let conn = Arc::clone(&self.conn);
        let decision_id = decision_id.to_string();
        
        Ok(task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            let mut stmt = conn.prepare(
                r#"
                SELECT id, transition_id, created_at, decision_type, risk_tier,
                       rationale, policy_gates, analytics_results, test_results,
                       risk_assessment, decision_factors, approved_by, metadata
                FROM decision_record
                WHERE id = ?
                "#,
            )?;
            
            let mut rows = stmt.query_map([decision_id], |row| {
                Self::row_to_decision_record_static(row)
            })?;
            
            rows.next().transpose()
        }).await.map_err(anyhow::Error::from)??)
    }

    /// List all decisions for a given risk tier
    pub async fn list_decisions_by_risk_tier(
        &self,
        risk_tier: RiskTier,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<DecisionRecord>> {
        let conn = Arc::clone(&self.conn);
        let risk_tier_str = risk_tier.as_str().to_string();
        
        Ok(task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            let sql = if limit.is_some() {
                format!("SELECT id, transition_id, created_at, decision_type, risk_tier,
                       rationale, policy_gates, analytics_results, test_results,
                       risk_assessment, decision_factors, approved_by, metadata
                FROM decision_record
                WHERE risk_tier = ?
                ORDER BY created_at DESC
                LIMIT {}", limit.unwrap())
            } else {
                "SELECT id, transition_id, created_at, decision_type, risk_tier,
                       rationale, policy_gates, analytics_results, test_results,
                       risk_assessment, decision_factors, approved_by, metadata
                FROM decision_record
                WHERE risk_tier = ?
                ORDER BY created_at DESC".to_string()
            };
            
            let mut stmt = conn.prepare(&sql)?;
            let mut rows = stmt.query_map([risk_tier_str], |row| {
                Self::row_to_decision_record_static(row)
            })?;
            
            let mut results = Vec::new();
            while let Some(record) = rows.next() {
                results.push(record?);
            }
            Ok::<_, rusqlite::Error>(results)
        }).await.map_err(anyhow::Error::from)??)
    }

    /// Calculate SHA-256 checksum for integrity
    fn calculate_checksum(&self, data: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Convert database row to DecisionRecord (static version)
    fn row_to_decision_record_static(
        row: &rusqlite::Row,
    ) -> Result<DecisionRecord, rusqlite::Error> {
        let decision_type_str: String = row.get(3)?;
        let decision_type = match decision_type_str.as_str() {
            "approve" => DecisionType::Approve,
            "reject" => DecisionType::Reject,
            "defer" => DecisionType::Defer,
            "require_manual_review" => DecisionType::RequireManualReview,
            _ => return Err(rusqlite::Error::InvalidColumnType(3, "Invalid decision type".to_string(), rusqlite::types::Type::Text)),
        };

        let risk_tier_str: String = row.get(4)?;
        let risk_tier = match risk_tier_str.as_str() {
            "low" => RiskTier::Low,
            "medium" => RiskTier::Medium,
            "high" => RiskTier::High,
            "critical" => RiskTier::Critical,
            _ => return Err(rusqlite::Error::InvalidColumnType(4, "Invalid risk tier".to_string(), rusqlite::types::Type::Text)),
        };

        let policy_gates_json: String = row.get(6)?;
        let policy_gates: Vec<GateResult> = serde_json::from_str(&policy_gates_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(6, rusqlite::types::Type::Text, Box::new(e)))?;

        let analytics_results_json: Option<String> = row.get(7)?;
        let analytics_results: Vec<AnalyticsResult> = analytics_results_json
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(7, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?
            .unwrap_or_default();

        let test_results: Option<String> = row.get(8)?;
        let test_results = test_results
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        let risk_assessment: Option<String> = row.get(9)?;
        let risk_assessment = risk_assessment
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(9, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        let decision_factors_json: String = row.get(10)?;
        let decision_factors: Vec<String> = serde_json::from_str(&decision_factors_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(10, rusqlite::types::Type::Text, Box::new(e)))?;

        let metadata: Option<String> = row.get(12)?;
        let metadata = metadata
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(12, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        let created_at_str: String = row.get(2)?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);

        Ok(DecisionRecord {
            id: row.get(0)?,
            transition_id: row.get(1)?,
            created_at,
            decision_type,
            risk_tier,
            rationale: row.get(5)?,
            policy_gates,
            analytics_results,
            test_results,
            risk_assessment,
            decision_factors,
            approved_by: row.get(11)?,
            metadata,
        })
    }
}

