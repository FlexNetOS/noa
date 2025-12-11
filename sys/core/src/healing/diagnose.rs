//! Root Cause Analysis Engine
//!
//! T614: Implement root cause analysis engine
//! FR-073: System MUST perform root cause analysis for detected anomalies
//! §3.4: Adaptive & Self-Improving

use crate::error::{NoaError, Result};
use crate::healing::anomaly::Anomaly;
use crate::healing::monitor::{ComponentHealthSnapshot, HealthMetric};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Root cause category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RootCauseCategory {
    ResourceExhaustion,
    ConfigurationError,
    DependencyFailure,
    NetworkIssue,
    DatabaseIssue,
    ServiceFailure,
    CodeBug,
    ExternalDependency,
    Unknown,
}

/// Root cause analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCause {
    pub category: RootCauseCategory,
    pub description: String,
    pub confidence: f64, // 0.0 to 1.0
    pub evidence: Vec<String>,
    pub suggested_fixes: Vec<String>,
    pub related_components: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Root cause analyzer
pub struct RootCauseAnalyzer {
    knowledge_base: HashMap<String, Vec<RootCausePattern>>,
}

/// Pattern for root cause matching
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RootCausePattern {
    category: RootCauseCategory,
    indicators: Vec<Indicator>,
    confidence: f64,
    description_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Indicator {
    metric_type: String,
    condition: Condition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Condition {
    AboveThreshold(f64),
    BelowThreshold(f64),
    Spike,
    Drop,
    Zero,
    NonZero,
}

impl RootCauseAnalyzer {
    /// Create a new root cause analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            knowledge_base: HashMap::new(),
        };
        analyzer.initialize_patterns();
        analyzer
    }

    /// Analyze anomaly and determine root cause
    pub async fn analyze(
        &self,
        anomaly: &Anomaly,
        snapshots: &[ComponentHealthSnapshot],
    ) -> Result<String> {
        info!(
            component_id = %anomaly.component_id,
            anomaly_type = %anomaly.anomaly_type,
            "Analyzing root cause"
        );

        // Find matching patterns
        let mut candidates: Vec<(&RootCausePattern, f64)> = Vec::new();

        for (_, patterns) in &self.knowledge_base {
            for pattern in patterns {
                let match_score = self.match_pattern(pattern, anomaly, snapshots);
                if match_score > 0.3 {
                    candidates.push((pattern, match_score));
                }
            }
        }

        // Sort by confidence
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        if let Some((pattern, score)) = candidates.first() {
            let root_cause = RootCause {
                category: pattern.category.clone(),
                description: pattern.description_template.clone(),
                confidence: score * pattern.confidence,
                evidence: self.collect_evidence(anomaly, snapshots),
                suggested_fixes: self.suggest_fixes(&pattern.category),
                related_components: self.find_related_components(anomaly, snapshots),
                metadata: HashMap::new(),
            };

            debug!(
                root_cause = %root_cause.description,
                confidence = root_cause.confidence,
                "Root cause identified"
            );

            Ok(serde_json::to_string(&root_cause)
                .map_err(|e| NoaError::Serialization(e.to_string()))?)
        } else {
            // Default: unknown root cause
            let root_cause = RootCause {
                category: RootCauseCategory::Unknown,
                description: format!("Unknown root cause for {}", anomaly.anomaly_type),
                confidence: 0.5,
                evidence: vec![anomaly.description.clone()],
                suggested_fixes: vec!["Investigate manually".to_string()],
                related_components: vec![anomaly.component_id.clone()],
                metadata: HashMap::new(),
            };

            Ok(serde_json::to_string(&root_cause)
                .map_err(|e| NoaError::Serialization(e.to_string()))?)
        }
    }

    /// Match pattern against anomaly and snapshots
    fn match_pattern(
        &self,
        pattern: &RootCausePattern,
        anomaly: &Anomaly,
        snapshots: &[ComponentHealthSnapshot],
    ) -> f64 {
        let mut score = 0.0;
        let mut total_indicators = 0;

        for indicator in &pattern.indicators {
            total_indicators += 1;
            if self.check_indicator(indicator, anomaly, snapshots) {
                score += 1.0;
            }
        }

        if total_indicators > 0 {
            score / total_indicators as f64
        } else {
            0.0
        }
    }

    /// Check if indicator matches
    fn check_indicator(
        &self,
        indicator: &Indicator,
        anomaly: &Anomaly,
        snapshots: &[ComponentHealthSnapshot],
    ) -> bool {
        // Find matching metric
        for snapshot in snapshots {
            for metric in &snapshot.metrics {
                if format!("{:?}", metric.metric_type) == indicator.metric_type {
                    match &indicator.condition {
                        Condition::AboveThreshold(threshold) => {
                            return metric.value >= *threshold;
                        }
                        Condition::BelowThreshold(threshold) => {
                            return metric.value <= *threshold;
                        }
                        Condition::Spike => {
                            return anomaly.anomaly_type == "spike";
                        }
                        Condition::Drop => {
                            return anomaly.anomaly_type == "drop";
                        }
                        Condition::Zero => {
                            return metric.value == 0.0;
                        }
                        Condition::NonZero => {
                            return metric.value != 0.0;
                        }
                    }
                }
            }
        }
        false
    }

    /// Collect evidence for root cause
    fn collect_evidence(
        &self,
        anomaly: &Anomaly,
        snapshots: &[ComponentHealthSnapshot],
    ) -> Vec<String> {
        let mut evidence = Vec::new();
        evidence.push(format!("Anomaly: {}", anomaly.description));
        evidence.push(format!("Component: {}", anomaly.component_id));
        evidence.push(format!(
            "Metric: {} = {}",
            anomaly.metric_type, anomaly.current_value
        ));
        evidence
    }

    /// Suggest fixes based on category
    fn suggest_fixes(&self, category: &RootCauseCategory) -> Vec<String> {
        match category {
            RootCauseCategory::ResourceExhaustion => {
                vec![
                    "Restart component".to_string(),
                    "Scale up resources".to_string(),
                    "Clear caches".to_string(),
                ]
            }
            RootCauseCategory::ConfigurationError => {
                vec![
                    "Reload configuration".to_string(),
                    "Validate configuration".to_string(),
                    "Rollback to previous config".to_string(),
                ]
            }
            RootCauseCategory::DependencyFailure => {
                vec![
                    "Check dependency health".to_string(),
                    "Retry with backoff".to_string(),
                    "Use fallback service".to_string(),
                ]
            }
            RootCauseCategory::NetworkIssue => {
                vec![
                    "Check network connectivity".to_string(),
                    "Retry connection".to_string(),
                    "Use alternative route".to_string(),
                ]
            }
            RootCauseCategory::DatabaseIssue => {
                vec![
                    "Check database connection".to_string(),
                    "Restart database".to_string(),
                    "Clear connection pool".to_string(),
                ]
            }
            RootCauseCategory::ServiceFailure => {
                vec![
                    "Restart service".to_string(),
                    "Check service logs".to_string(),
                    "Switch to backup instance".to_string(),
                ]
            }
            _ => vec!["Investigate manually".to_string()],
        }
    }

    /// Find related components
    fn find_related_components(
        &self,
        anomaly: &Anomaly,
        snapshots: &[ComponentHealthSnapshot],
    ) -> Vec<String> {
        let mut related = vec![anomaly.component_id.clone()];

        // Find components with similar issues
        for snapshot in snapshots {
            if snapshot.component_id != anomaly.component_id {
                if matches!(
                    snapshot.health_status,
                    crate::healing::monitor::ComponentHealth::Degraded
                        | crate::healing::monitor::ComponentHealth::Unhealthy
                        | crate::healing::monitor::ComponentHealth::Critical
                ) {
                    related.push(snapshot.component_id.clone());
                }
            }
        }

        related
    }

    /// Initialize root cause patterns
    fn initialize_patterns(&mut self) {
        // Resource exhaustion patterns
        self.knowledge_base.insert(
            "resource_exhaustion".to_string(),
            vec![RootCausePattern {
                category: RootCauseCategory::ResourceExhaustion,
                indicators: vec![
                    Indicator {
                        metric_type: "CpuUsage".to_string(),
                        condition: Condition::AboveThreshold(95.0),
                    },
                    Indicator {
                        metric_type: "MemoryUsage".to_string(),
                        condition: Condition::AboveThreshold(95.0),
                    },
                ],
                confidence: 0.9,
                description_template: "Resource exhaustion: CPU or memory usage critically high"
                    .to_string(),
            }],
        );

        // Database issue patterns
        self.knowledge_base.insert(
            "database_issue".to_string(),
            vec![RootCausePattern {
                category: RootCauseCategory::DatabaseIssue,
                indicators: vec![
                    Indicator {
                        metric_type: "DatabaseHealth".to_string(),
                        condition: Condition::BelowThreshold(0.5),
                    },
                    Indicator {
                        metric_type: "ConnectionPool".to_string(),
                        condition: Condition::BelowThreshold(0.1),
                    },
                ],
                confidence: 0.85,
                description_template: "Database connectivity or health issue detected".to_string(),
            }],
        );

        // Service failure patterns
        self.knowledge_base.insert(
            "service_failure".to_string(),
            vec![RootCausePattern {
                category: RootCauseCategory::ServiceFailure,
                indicators: vec![
                    Indicator {
                        metric_type: "ServiceHealth".to_string(),
                        condition: Condition::Zero,
                    },
                    Indicator {
                        metric_type: "ErrorRate".to_string(),
                        condition: Condition::Spike,
                    },
                ],
                confidence: 0.8,
                description_template: "Service failure or high error rate detected".to_string(),
            }],
        );
    }
}

impl Default for RootCauseAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_cause_analyzer_creation() {
        let analyzer = RootCauseAnalyzer::new();
        assert!(!analyzer.knowledge_base.is_empty());
    }

    #[test]
    fn test_suggest_fixes() {
        let analyzer = RootCauseAnalyzer::new();
        let fixes = analyzer.suggest_fixes(&RootCauseCategory::ResourceExhaustion);
        assert!(!fixes.is_empty());
    }
}
