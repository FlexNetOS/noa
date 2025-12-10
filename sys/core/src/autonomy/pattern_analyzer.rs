//! Pattern Analysis for Improvement Opportunities
//!
//! Implements pattern analysis to detect improvement opportunities.
//! FR-066-070: Autonomous Goal Generation
//!
//! T635: Implement pattern analysis for improvement opportunities

use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// Performance degradation
    PerformanceDegradation,
    /// Error rate increase
    ErrorRateIncrease,
    /// Resource inefficiency
    ResourceInefficiency,
    /// Repeated failures
    RepeatedFailures,
    /// Usage pattern change
    UsagePatternChange,
}

/// Detected pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPattern {
    pub id: Uuid,
    pub pattern_type: PatternType,
    pub component: String,
    pub severity: f64, // 0.0 to 1.0
    pub description: String,
    pub evidence: serde_json::Value,
    pub detected_at: DateTime<Utc>,
    pub suggested_improvement: Option<String>,
}

/// Pattern analyzer
pub struct PatternAnalyzer {
    patterns: Arc<RwLock<Vec<DetectedPattern>>>,
    analysis_history: Arc<RwLock<Vec<DateTime<Utc>>>>,
}

impl PatternAnalyzer {
    /// Create a new pattern analyzer
    pub fn new() -> Self {
        Self {
            patterns: Arc::new(RwLock::new(Vec::new())),
            analysis_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Analyze data for patterns
    pub async fn analyze(
        &self,
        component: String,
        metrics: serde_json::Value,
    ) -> Result<Vec<DetectedPattern>> {
        let mut detected = Vec::new();

        // Check for performance degradation
        if let Some(latency) = metrics.get("latency_ms") {
            if let Some(latency_val) = latency.as_f64() {
                if latency_val > 1000.0 {
                    detected.push(DetectedPattern {
                        id: Uuid::new_v4(),
                        pattern_type: PatternType::PerformanceDegradation,
                        component: component.clone(),
                        severity: (latency_val / 2000.0).min(1.0),
                        description: format!("High latency detected: {}ms", latency_val),
                        evidence: serde_json::json!({"latency_ms": latency_val}),
                        detected_at: Utc::now(),
                        suggested_improvement: Some(
                            "Consider optimizing queries or adding caching".to_string(),
                        ),
                    });
                }
            }
        }

        // Check for error rate increase
        if let Some(error_rate) = metrics.get("error_rate") {
            if let Some(rate) = error_rate.as_f64() {
                if rate > 0.05 {
                    detected.push(DetectedPattern {
                        id: Uuid::new_v4(),
                        pattern_type: PatternType::ErrorRateIncrease,
                        component: component.clone(),
                        severity: (rate * 10.0).min(1.0),
                        description: format!("High error rate: {:.2}%", rate * 100.0),
                        evidence: serde_json::json!({"error_rate": rate}),
                        detected_at: Utc::now(),
                        suggested_improvement: Some(
                            "Review error logs and improve error handling".to_string(),
                        ),
                    });
                }
            }
        }

        // Check for resource inefficiency
        if let Some(cpu_usage) = metrics.get("cpu_usage") {
            if let Some(cpu) = cpu_usage.as_f64() {
                if cpu > 0.9 {
                    detected.push(DetectedPattern {
                        id: Uuid::new_v4(),
                        pattern_type: PatternType::ResourceInefficiency,
                        component: component.clone(),
                        severity: cpu,
                        description: format!("High CPU usage: {:.1}%", cpu * 100.0),
                        evidence: serde_json::json!({"cpu_usage": cpu}),
                        detected_at: Utc::now(),
                        suggested_improvement: Some(
                            "Consider optimizing algorithms or scaling resources".to_string(),
                        ),
                    });
                }
            }
        }

        // Store detected patterns
        if !detected.is_empty() {
            let mut patterns = self.patterns.write().await;
            patterns.extend(detected.clone());
        }

        // Record analysis
        let mut history = self.analysis_history.write().await;
        history.push(Utc::now());

        Ok(detected)
    }

    /// Get all detected patterns
    pub async fn all_patterns(&self) -> Vec<DetectedPattern> {
        self.patterns.read().await.clone()
    }

    /// Get patterns by type
    pub async fn patterns_by_type(&self, pattern_type: &PatternType) -> Vec<DetectedPattern> {
        self.patterns
            .read()
            .await
            .iter()
            .filter(|p| match (&p.pattern_type, pattern_type) {
                (PatternType::PerformanceDegradation, PatternType::PerformanceDegradation) => true,
                (PatternType::ErrorRateIncrease, PatternType::ErrorRateIncrease) => true,
                (PatternType::ResourceInefficiency, PatternType::ResourceInefficiency) => true,
                (PatternType::RepeatedFailures, PatternType::RepeatedFailures) => true,
                (PatternType::UsagePatternChange, PatternType::UsagePatternChange) => true,
                _ => false,
            })
            .cloned()
            .collect()
    }

    /// Get patterns by component
    pub async fn patterns_by_component(&self, component: &str) -> Vec<DetectedPattern> {
        self.patterns
            .read()
            .await
            .iter()
            .filter(|p| p.component == component)
            .cloned()
            .collect()
    }

    /// Get high severity patterns
    pub async fn high_severity_patterns(&self, threshold: f64) -> Vec<DetectedPattern> {
        self.patterns
            .read()
            .await
            .iter()
            .filter(|p| p.severity >= threshold)
            .cloned()
            .collect()
    }

    /// Get analysis history
    pub async fn analysis_history(&self) -> Vec<DateTime<Utc>> {
        self.analysis_history.read().await.clone()
    }

    /// Clear all patterns
    pub async fn clear(&self) {
        let mut patterns = self.patterns.write().await;
        patterns.clear();
    }
}

impl Default for PatternAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_degradation_detection() {
        let analyzer = PatternAnalyzer::new();

        let metrics = serde_json::json!({
            "latency_ms": 1500.0
        });

        let patterns = analyzer.analyze("Database".to_string(), metrics).await.unwrap();
        assert_eq!(patterns.len(), 1);
        assert!(matches!(
            patterns[0].pattern_type,
            PatternType::PerformanceDegradation
        ));
    }

    #[tokio::test]
    async fn test_error_rate_detection() {
        let analyzer = PatternAnalyzer::new();

        let metrics = serde_json::json!({
            "error_rate": 0.1
        });

        let patterns = analyzer.analyze("API".to_string(), metrics).await.unwrap();
        assert_eq!(patterns.len(), 1);
        assert!(matches!(
            patterns[0].pattern_type,
            PatternType::ErrorRateIncrease
        ));
    }

    #[tokio::test]
    async fn test_multiple_patterns() {
        let analyzer = PatternAnalyzer::new();

        let metrics = serde_json::json!({
            "latency_ms": 1500.0,
            "error_rate": 0.1,
            "cpu_usage": 0.95
        });

        let patterns = analyzer.analyze("Service".to_string(), metrics).await.unwrap();
        assert_eq!(patterns.len(), 3);
    }

    #[tokio::test]
    async fn test_high_severity_filter() {
        let analyzer = PatternAnalyzer::new();

        let metrics = serde_json::json!({
            "latency_ms": 1500.0,
            "error_rate": 0.05
        });

        analyzer.analyze("Service".to_string(), metrics).await.unwrap();

        let high_severity = analyzer.high_severity_patterns(0.5).await;
        assert!(!high_severity.is_empty());
    }
}
