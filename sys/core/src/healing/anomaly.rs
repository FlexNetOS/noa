//! Anomaly Detection
//!
//! T613: Implement anomaly detection
//! FR-072: System MUST detect anomalies in health metrics
//! §3.4: Adaptive & Self-Improving

use crate::error::Result;
use crate::healing::monitor::{ComponentHealth, ComponentHealthSnapshot, HealthMetric};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;

/// Anomaly type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyType {
    /// Sudden spike in metric value
    Spike,
    /// Sudden drop in metric value
    Drop,
    /// Metric consistently above threshold
    ThresholdExceeded,
    /// Metric consistently below threshold
    ThresholdUndershot,
    /// Unusual pattern detected
    PatternAnomaly,
    /// Service unavailable
    ServiceUnavailable,
    /// Error rate spike
    ErrorRateSpike,
    /// Resource exhaustion
    ResourceExhaustion,
}

/// Detected anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub component_id: String,
    pub component_type: String,
    pub anomaly_type: String,
    pub health_status: ComponentHealth,
    pub metric_type: String,
    pub current_value: f64,
    pub expected_value: Option<f64>,
    pub severity: AnomalySeverity,
    pub detected_at: DateTime<Utc>,
    pub description: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Anomaly severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Anomaly detector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectorConfig {
    /// Minimum change percentage to trigger anomaly
    pub spike_threshold_percent: f64,
    /// Number of consecutive violations before alerting
    pub consecutive_violations: u32,
    /// Time window for pattern analysis (seconds)
    pub pattern_window_secs: u64,
    /// Enable statistical anomaly detection
    pub enable_statistical: bool,
}

impl Default for AnomalyDetectorConfig {
    fn default() -> Self {
        Self {
            spike_threshold_percent: 50.0,
            consecutive_violations: 3,
            pattern_window_secs: 300,
            enable_statistical: true,
        }
    }
}

/// Anomaly detector
pub struct AnomalyDetector {
    config: AnomalyDetectorConfig,
    metric_history: Mutex<HashMap<String, Vec<(DateTime<Utc>, f64)>>>,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    pub fn new(config: AnomalyDetectorConfig) -> Self {
        Self {
            config,
            metric_history: Mutex::new(HashMap::new()),
        }
    }

    /// Detect anomalies in health metrics
    pub async fn detect(
        &self,
        snapshots: &[ComponentHealthSnapshot],
    ) -> Result<Option<Anomaly>> {
        for snapshot in snapshots {
            // Check if component is already unhealthy
            if matches!(
                snapshot.health_status,
                ComponentHealth::Unhealthy | ComponentHealth::Critical
            ) {
                // Analyze metrics for this component
                for metric in &snapshot.metrics {
                    if let Some(anomaly) = self.analyze_metric(metric, snapshot).await? {
                        return Ok(Some(anomaly));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Analyze a single metric for anomalies
    async fn analyze_metric(
        &self,
        metric: &HealthMetric,
        snapshot: &ComponentHealthSnapshot,
    ) -> Result<Option<Anomaly>> {
        let metric_key = format!(
            "{}:{}",
            metric.component_id,
            format!("{:?}", metric.metric_type)
        );

        // Update history
        let history = {
            let mut history_guard = self.metric_history.lock().await;
            let history_entry =
                history_guard.entry(metric_key.clone()).or_insert_with(Vec::new);
            history_entry.push((metric.timestamp, metric.value));

            // Keep only recent history
            let cutoff =
                Utc::now() - chrono::Duration::seconds(self.config.pattern_window_secs as i64);
            history_entry.retain(|(ts, _)| *ts >= cutoff);
            history_entry.clone()
        };

        // Check threshold violations
        if let Some(critical_threshold) = metric.threshold_critical {
            if metric.value >= critical_threshold {
                return Ok(Some(Anomaly {
                    component_id: metric.component_id.clone(),
                    component_type: metric.component_type.clone(),
                    anomaly_type: "threshold_exceeded".to_string(),
                    health_status: ComponentHealth::Critical,
                    metric_type: format!("{:?}", metric.metric_type),
                    current_value: metric.value,
                    expected_value: Some(critical_threshold),
                    severity: AnomalySeverity::Critical,
                    detected_at: Utc::now(),
                    description: format!(
                        "Metric {} exceeded critical threshold: {} >= {}",
                        format!("{:?}", metric.metric_type),
                        metric.value,
                        critical_threshold
                    ),
                    metadata: HashMap::new(),
                }));
            }
        }

        if let Some(warning_threshold) = metric.threshold_warning {
            if metric.value >= warning_threshold {
                // Check for consecutive violations
                let consecutive = Self::count_consecutive_in_history(&history, warning_threshold);
                if consecutive >= self.config.consecutive_violations {
                    return Ok(Some(Anomaly {
                        component_id: metric.component_id.clone(),
                        component_type: metric.component_type.clone(),
                        anomaly_type: "threshold_exceeded".to_string(),
                        health_status: ComponentHealth::Degraded,
                        metric_type: format!("{:?}", metric.metric_type),
                        current_value: metric.value,
                        expected_value: Some(warning_threshold),
                        severity: AnomalySeverity::High,
                        detected_at: Utc::now(),
                        description: format!(
                            "Metric {} exceeded warning threshold {} times: {} >= {}",
                            format!("{:?}", metric.metric_type),
                            consecutive,
                            metric.value,
                            warning_threshold
                        ),
                        metadata: HashMap::new(),
                    }));
                }
            }
        }

        // Statistical anomaly detection
        if self.config.enable_statistical && history.len() >= 10 {
            if let Some(anomaly) = self.detect_statistical_anomaly(metric, &history)? {
                return Ok(Some(anomaly));
            }
        }

        Ok(None)
    }

    fn count_consecutive_in_history(history: &[(DateTime<Utc>, f64)], threshold: f64) -> u32 {
        let mut count = 0;
        for (_, value) in history.iter().rev() {
            if *value >= threshold {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    /// Detect statistical anomalies (spikes/drops)
    fn detect_statistical_anomaly(
        &self,
        metric: &HealthMetric,
        history: &[(DateTime<Utc>, f64)],
    ) -> Result<Option<Anomaly>> {
        if history.len() < 10 {
            return Ok(None);
        }

        // Calculate mean and standard deviation
        let values: Vec<f64> = history.iter().map(|(_, v)| *v).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();

        // Check for spike (value > mean + 2*std_dev)
        let spike_threshold = mean + 2.0 * std_dev;
        if metric.value > spike_threshold {
            let change_percent = ((metric.value - mean) / mean) * 100.0;
            if change_percent >= self.config.spike_threshold_percent {
                return Ok(Some(Anomaly {
                    component_id: metric.component_id.clone(),
                    component_type: metric.component_type.clone(),
                    anomaly_type: "spike".to_string(),
                    health_status: ComponentHealth::Degraded,
                    metric_type: format!("{:?}", metric.metric_type),
                    current_value: metric.value,
                    expected_value: Some(mean),
                    severity: if change_percent > 100.0 {
                        AnomalySeverity::Critical
                    } else {
                        AnomalySeverity::High
                    },
                    detected_at: Utc::now(),
                    description: format!(
                        "Spike detected: {}% increase ({} -> {})",
                        change_percent, mean, metric.value
                    ),
                    metadata: HashMap::new(),
                }));
            }
        }

        // Check for drop (value < mean - 2*std_dev)
        let drop_threshold = mean - 2.0 * std_dev;
        if metric.value < drop_threshold && metric.value >= 0.0 {
            let change_percent = ((mean - metric.value) / mean) * 100.0;
            if change_percent >= self.config.spike_threshold_percent {
                return Ok(Some(Anomaly {
                    component_id: metric.component_id.clone(),
                    component_type: metric.component_type.clone(),
                    anomaly_type: "drop".to_string(),
                    health_status: ComponentHealth::Degraded,
                    metric_type: format!("{:?}", metric.metric_type),
                    current_value: metric.value,
                    expected_value: Some(mean),
                    severity: AnomalySeverity::Medium,
                    detected_at: Utc::now(),
                    description: format!(
                        "Drop detected: {}% decrease ({} -> {})",
                        change_percent, mean, metric.value
                    ),
                    metadata: HashMap::new(),
                }));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anomaly_detector_creation() {
        let config = AnomalyDetectorConfig::default();
        let detector = AnomalyDetector::new(config);
        let guard = detector.metric_history.lock().await;
        assert!(guard.is_empty());
    }

    #[test]
    fn test_anomaly_severity_ordering() {
        assert!(AnomalySeverity::Critical > AnomalySeverity::High);
        assert!(AnomalySeverity::High > AnomalySeverity::Medium);
        assert!(AnomalySeverity::Medium > AnomalySeverity::Low);
    }
}
