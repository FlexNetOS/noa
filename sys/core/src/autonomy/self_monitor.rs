//! Performance Self-Monitoring
//!
//! T625: Implement performance self-monitoring
//! FR-055: System MUST self-monitor performance metrics and autonomously adjust execution strategies
//! §3.4: Adaptive & Self-Improving

use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, warn};

/// Performance metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
    pub component: Option<String>,
}

/// Performance baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub metric_name: String,
    pub average: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub min: f64,
    pub max: f64,
    pub sample_count: usize,
}

/// Performance degradation detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationAlert {
    pub metric_name: String,
    pub current_value: f64,
    pub baseline_value: f64,
    pub degradation_percent: f64,
    pub severity: DegradationSeverity,
    pub timestamp: DateTime<Utc>,
}

/// Degradation severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DegradationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Performance self-monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfMonitorConfig {
    pub monitoring_interval_secs: u64,
    pub baseline_window_secs: u64,
    pub degradation_threshold_percent: f64,
    pub enable_auto_adjustment: bool,
}

impl Default for SelfMonitorConfig {
    fn default() -> Self {
        Self {
            monitoring_interval_secs: 30,
            baseline_window_secs: 3600, // 1 hour
            degradation_threshold_percent: 20.0,
            enable_auto_adjustment: true,
        }
    }
}

/// Performance self-monitor
pub struct PerformanceSelfMonitor {
    config: SelfMonitorConfig,
    metrics_history: HashMap<String, Vec<PerformanceMetric>>,
    baselines: HashMap<String, PerformanceBaseline>,
}

impl PerformanceSelfMonitor {
    /// Create a new performance self-monitor
    pub fn new(config: SelfMonitorConfig) -> Self {
        Self {
            config,
            metrics_history: HashMap::new(),
            baselines: HashMap::new(),
        }
    }

    /// Record a performance metric
    pub async fn record_metric(&mut self, metric: PerformanceMetric) -> Result<()> {
        let history = self
            .metrics_history
            .entry(metric.name.clone())
            .or_insert_with(Vec::new);

        history.push(metric.clone());

        // Trim old metrics
        let cutoff = Utc::now()
            - chrono::Duration::seconds(self.config.baseline_window_secs as i64);
        history.retain(|m| m.timestamp >= cutoff);

        // Update baseline if needed
        self.update_baseline(&metric.name).await?;

        debug!(
            metric_name = %metric.name,
            value = metric.value,
            "Recorded performance metric"
        );

        Ok(())
    }

    /// Check for performance degradation
    pub async fn check_degradation(&self) -> Result<Vec<DegradationAlert>> {
        let mut alerts = Vec::new();

        for (metric_name, history) in &self.metrics_history {
            if let Some(baseline) = self.baselines.get(metric_name) {
                if let Some(latest) = history.last() {
                    let degradation = ((latest.value - baseline.average) / baseline.average) * 100.0;

                    if degradation.abs() > self.config.degradation_threshold_percent {
                        let severity = if degradation.abs() > 50.0 {
                            DegradationSeverity::Critical
                        } else if degradation.abs() > 30.0 {
                            DegradationSeverity::High
                        } else if degradation.abs() > 20.0 {
                            DegradationSeverity::Medium
                        } else {
                            DegradationSeverity::Low
                        };

                        alerts.push(DegradationAlert {
                            metric_name: metric_name.clone(),
                            current_value: latest.value,
                            baseline_value: baseline.average,
                            degradation_percent: degradation,
                            severity,
                            timestamp: Utc::now(),
                        });
                    }
                }
            }
        }

        if !alerts.is_empty() {
            warn!(
                alerts = alerts.len(),
                "Performance degradation detected"
            );
        }

        Ok(alerts)
    }

    /// Update baseline for a metric
    async fn update_baseline(&mut self, metric_name: &str) -> Result<()> {
        let history = match self.metrics_history.get(metric_name) {
            Some(h) => h,
            None => return Ok(()),
        };

        if history.len() < 10 {
            return Ok(()); // Need more samples
        }

        let values: Vec<f64> = history.iter().map(|m| m.value).collect();
        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let count = sorted.len();
        let average = values.iter().sum::<f64>() / count as f64;
        let p50 = sorted[count / 2];
        let p95 = sorted[(count * 95) / 100];
        let p99 = sorted[(count * 99) / 100];
        let min = sorted[0];
        let max = sorted[count - 1];

        self.baselines.insert(
            metric_name.to_string(),
            PerformanceBaseline {
                metric_name: metric_name.to_string(),
                average,
                p50,
                p95,
                p99,
                min,
                max,
                sample_count: count,
            },
        );

        debug!(
            metric_name = %metric_name,
            average = average,
            "Updated performance baseline"
        );

        Ok(())
    }

    /// Get baseline for a metric
    pub fn get_baseline(&self, metric_name: &str) -> Option<&PerformanceBaseline> {
        self.baselines.get(metric_name)
    }

    /// Get all baselines
    pub fn get_all_baselines(&self) -> &HashMap<String, PerformanceBaseline> {
        &self.baselines
    }
}

impl Default for PerformanceSelfMonitor {
    fn default() -> Self {
        Self::new(SelfMonitorConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_self_monitor() {
        let mut monitor = PerformanceSelfMonitor::default();

        for i in 0..20 {
            let metric = PerformanceMetric {
                name: "test_metric".to_string(),
                value: 10.0 + (i as f64),
                unit: "ms".to_string(),
                timestamp: Utc::now(),
                component: None,
            };
            monitor.record_metric(metric).await.unwrap();
        }

        let baseline = monitor.get_baseline("test_metric");
        assert!(baseline.is_some());
    }
}

