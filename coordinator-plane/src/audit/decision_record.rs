//! Decision Rationale Recorder
//!
//! Implements T610: Decision rationale recorder
//! Records the reasoning behind promotion/rollback decisions for audit and transparency

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
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
    pool: SqlitePool,
}

impl DecisionRecorder {
    /// Create a new DecisionRecorder with database connection
    pub async fn new(pool: SqlitePool) -> Result<Self, sqlx::Error> {
        // Ensure the decision_record table exists
        sqlx::query(
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
        )
        .execute(&pool)
        .await?;

        // Create index for fast lookup by transition_id
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_decision_transition ON decision_record(transition_id)",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    /// Record a decision rationale
    ///
    /// Captures the complete reasoning behind a promotion/rollback decision,
    /// including policy gate results, analytics evaluations, and risk assessment
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
    ) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Serialize complex fields
        let policy_gates_json = serde_json::to_string(&policy_gates)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let analytics_results_json = serde_json::to_string(&analytics_results)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let test_results_json = test_results
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let risk_assessment_json = risk_assessment
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let decision_factors_json = serde_json::to_string(&decision_factors)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let metadata_json = metadata
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        // Calculate checksum of rationale and key decision data
        let checksum_data = format!("{}{}{}", rationale, decision_type.as_str(), risk_tier.as_str());
        let checksum = self.calculate_checksum(&checksum_data);

        sqlx::query(
            r#"
            INSERT INTO decision_record (
                id, transition_id, created_at, decision_type, risk_tier,
                rationale, policy_gates, analytics_results, test_results,
                risk_assessment, decision_factors, approved_by, metadata, checksum
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(transition_id)
        .bind(now.to_rfc3339())
        .bind(decision_type.as_str())
        .bind(risk_tier.as_str())
        .bind(rationale)
        .bind(&policy_gates_json)
        .bind(&analytics_results_json)
        .bind(&test_results_json)
        .bind(&risk_assessment_json)
        .bind(&decision_factors_json)
        .bind(approved_by)
        .bind(&metadata_json)
        .bind(&checksum)
        .execute(&self.pool)
        .await?;

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
    ) -> Result<Option<DecisionRecord>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, transition_id, created_at, decision_type, risk_tier,
                   rationale, policy_gates, analytics_results, test_results,
                   risk_assessment, decision_factors, approved_by, metadata
            FROM decision_record
            WHERE transition_id = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(transition_id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(|r| self.row_to_decision_record(&r))
            .transpose()
    }

    /// Get decision record by ID
    pub async fn get_decision(
        &self,
        decision_id: &str,
    ) -> Result<Option<DecisionRecord>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, transition_id, created_at, decision_type, risk_tier,
                   rationale, policy_gates, analytics_results, test_results,
                   risk_assessment, decision_factors, approved_by, metadata
            FROM decision_record
            WHERE id = ?
            "#,
        )
        .bind(decision_id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(|r| self.row_to_decision_record(&r))
            .transpose()
    }

    /// List all decisions for a given risk tier
    pub async fn list_decisions_by_risk_tier(
        &self,
        risk_tier: RiskTier,
        limit: Option<i64>,
    ) -> Result<Vec<DecisionRecord>, sqlx::Error> {
        let mut query = sqlx::query(
            r#"
            SELECT id, transition_id, created_at, decision_type, risk_tier,
                   rationale, policy_gates, analytics_results, test_results,
                   risk_assessment, decision_factors, approved_by, metadata
            FROM decision_record
            WHERE risk_tier = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(risk_tier.as_str());

        if let Some(limit) = limit {
            query = sqlx::query(
                r#"
                SELECT id, transition_id, created_at, decision_type, risk_tier,
                       rationale, policy_gates, analytics_results, test_results,
                       risk_assessment, decision_factors, approved_by, metadata
                FROM decision_record
                WHERE risk_tier = ?
                ORDER BY created_at DESC
                LIMIT ?
                "#,
            )
            .bind(risk_tier.as_str())
            .bind(limit);
        }

        let rows = query.fetch_all(&self.pool).await?;

        rows.into_iter()
            .map(|r| self.row_to_decision_record(&r))
            .collect()
    }

    /// Calculate SHA-256 checksum for integrity
    fn calculate_checksum(&self, data: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Convert database row to DecisionRecord
    fn row_to_decision_record(
        &self,
        row: &sqlx::sqlite::SqliteRow,
    ) -> Result<DecisionRecord, sqlx::Error> {
        let decision_type_str: String = row.try_get("decision_type")?;
        let decision_type = match decision_type_str.as_str() {
            "approve" => DecisionType::Approve,
            "reject" => DecisionType::Reject,
            "defer" => DecisionType::Defer,
            "require_manual_review" => DecisionType::RequireManualReview,
            _ => return Err(sqlx::Error::Decode("Invalid decision type".into())),
        };

        let risk_tier_str: String = row.try_get("risk_tier")?;
        let risk_tier = match risk_tier_str.as_str() {
            "low" => RiskTier::Low,
            "medium" => RiskTier::Medium,
            "high" => RiskTier::High,
            "critical" => RiskTier::Critical,
            _ => return Err(sqlx::Error::Decode("Invalid risk tier".into())),
        };

        let policy_gates_json: String = row.try_get("policy_gates")?;
        let policy_gates: Vec<GateResult> = serde_json::from_str(&policy_gates_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let analytics_results_json: Option<String> = row.try_get("analytics_results")?;
        let analytics_results: Vec<AnalyticsResult> = analytics_results_json
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?
            .unwrap_or_default();

        let test_results: Option<String> = row.try_get("test_results")?;
        let test_results = test_results
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?;

        let risk_assessment: Option<String> = row.try_get("risk_assessment")?;
        let risk_assessment = risk_assessment
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?;

        let decision_factors_json: String = row.try_get("decision_factors")?;
        let decision_factors: Vec<String> = serde_json::from_str(&decision_factors_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let metadata: Option<String> = row.try_get("metadata")?;
        let metadata = metadata
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?;

        let created_at_str: String = row.try_get("created_at")?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .with_timezone(&Utc);

        Ok(DecisionRecord {
            id: row.try_get("id")?,
            transition_id: row.try_get("transition_id")?,
            created_at,
            decision_type,
            risk_tier,
            rationale: row.try_get("rationale")?,
            policy_gates,
            analytics_results,
            test_results,
            risk_assessment,
            decision_factors,
            approved_by: row.try_get("approved_by")?,
            metadata,
        })
    }
}

