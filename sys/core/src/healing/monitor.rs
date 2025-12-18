//! Continuous Health Monitoring
//!
//! T612: Implement continuous health monitoring
//! FR-071: System MUST continuously monitor health metrics for all components
//! §3.4: Adaptive & Self-Improving

use crate::error::{NoaError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Health metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    CpuUsage,
    MemoryUsage,
    DiskUsage,
    NetworkLatency,
    ErrorRate,
    RequestRate,
    ResponseTime,
    QueueDepth,
    ConnectionPool,
    DatabaseHealth,
    ServiceHealth,
}

/// Health metric value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetric {
    pub component_id: String,
    pub component_type: String,
    pub metric_type: MetricType,
    pub value: f64,
    pub unit: String,
    pub threshold_warning: Option<f64>,
    pub threshold_critical: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Component health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Critical,
}

/// Component health snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealthSnapshot {
    pub component_id: String,
    pub component_type: String,
    pub health_status: ComponentHealth,
    pub metrics: Vec<HealthMetric>,
    pub timestamp: DateTime<Utc>,
    pub score: f64, // 0.0 (critical) to 1.0 (healthy)
}

/// Health monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitorConfig {
    pub collection_interval_secs: u64,
    pub retention_period_secs: u64,
    pub metric_thresholds: HashMap<String, MetricThresholds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricThresholds {
    pub warning: Option<f64>,
    pub critical: Option<f64>,
}

/// Continuous health monitor
pub struct HealthMonitor {
    config: HealthMonitorConfig,
    metrics_history: Arc<RwLock<Vec<HealthMetric>>>,
    component_snapshots: Arc<RwLock<HashMap<String, ComponentHealthSnapshot>>>,
    running: Arc<RwLock<bool>>,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(config: HealthMonitorConfig) -> Self {
        Self {
            config,
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            component_snapshots: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start continuous monitoring
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.write().await;
        if *running {
            return Err(NoaError::Internal {
                message: "Health monitor already running".to_string(),
                source: None,
            });
        }
        *running = true;
        drop(running);

        info!("Starting continuous health monitoring");

        let metrics_history = Arc::clone(&self.metrics_history);
        let component_snapshots = Arc::clone(&self.component_snapshots);
        let config = self.config.clone();
        let running = Arc::clone(&self.running);

        tokio::spawn(async move {
            loop {
                // Check if still running
                {
                    let r = running.read().await;
                    if !*r {
                        break;
                    }
                }

                // Collect metrics
                match Self::collect_metrics_internal(&config).await {
                    Ok(metrics) => {
                        let timestamp = Utc::now();

                        // Store metrics
                        {
                            let mut history = metrics_history.write().await;
                            history.extend(metrics.clone());
                            history.retain(|m| {
                                timestamp.signed_duration_since(m.timestamp).num_seconds()
                                    < config.retention_period_secs as i64
                            });
                        }

                        // Update component snapshots
                        let snapshots = Self::compute_health_snapshots(&metrics, &config);
                        {
                            let mut snapshots_map = component_snapshots.write().await;
                            for snapshot in snapshots {
                                snapshots_map.insert(snapshot.component_id.clone(), snapshot);
                            }
                        }

                        debug!("Collected {} health metrics", metrics.len());
                    }
                    Err(e) => {
                        warn!("Failed to collect health metrics: {}", e);
                    }
                }

                // Sleep until next collection
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    config.collection_interval_secs,
                ))
                .await;
            }
        });

        Ok(())
    }

    /// Stop monitoring
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.write().await;
        *running = false;
        info!("Stopped health monitoring");
        Ok(())
    }

    /// Collect current health metrics
    pub async fn collect_metrics(&self) -> Result<Vec<HealthMetric>> {
        Self::collect_metrics_internal(&self.config).await
    }

    /// Get component health snapshot
    pub async fn get_component_health(
        &self,
        component_id: &str,
    ) -> Result<Option<ComponentHealthSnapshot>> {
        let snapshots = self.component_snapshots.read().await;
        Ok(snapshots.get(component_id).cloned())
    }

    /// Get all component health snapshots
    pub async fn get_all_health_snapshots(&self) -> Vec<ComponentHealthSnapshot> {
        let snapshots = self.component_snapshots.read().await;
        snapshots.values().cloned().collect()
    }

    /// Internal metric collection
    async fn collect_metrics_internal(
        config: &HealthMonitorConfig,
    ) -> Result<Vec<HealthMetric>> {
        let mut metrics = Vec::new();
        let timestamp = Utc::now();

        // TODO: Implement actual metric collection from:
        // - System resources (CPU, memory, disk)
        // - Database connections
        // - Service health endpoints
        // - Agent status
        // - Network latency

        // Example: CPU usage (placeholder)
        metrics.push(HealthMetric {
            component_id: "system".to_string(),
            component_type: "system".to_string(),
            metric_type: MetricType::CpuUsage,
            value: 0.0, // TODO: Get actual CPU usage
            unit: "percent".to_string(),
            threshold_warning: config
                .metric_thresholds
                .get("cpu")
                .and_then(|t| t.warning),
            threshold_critical: config
                .metric_thresholds
                .get("cpu")
                .and_then(|t| t.critical),
            timestamp,
            metadata: HashMap::new(),
        });

        Ok(metrics)
    }

    /// Compute health snapshots from metrics
    fn compute_health_snapshots(
        metrics: &[HealthMetric],
        _config: &HealthMonitorConfig,
    ) -> Vec<ComponentHealthSnapshot> {
        let mut snapshots: HashMap<String, Vec<&HealthMetric>> = HashMap::new();

        // Group metrics by component
        for metric in metrics {
            snapshots
                .entry(metric.component_id.clone())
                .or_insert_with(Vec::new)
                .push(metric);
        }

        // Compute health status for each component
        snapshots
            .into_iter()
            .map(|(component_id, component_metrics)| {
                let component_type = component_metrics
                    .first()
                    .map(|m| m.component_type.clone())
                    .unwrap_or_else(|| "unknown".to_string());

                let (health_status, score) = Self::compute_health_status(&component_metrics);

                ComponentHealthSnapshot {
                    component_id,
                    component_type,
                    health_status,
                    metrics: component_metrics.iter().map(|m| (*m).clone()).collect(),
                    timestamp: Utc::now(),
                    score,
                }
            })
            .collect()
    }

    /// Compute health status from metrics
    fn compute_health_status(metrics: &[&HealthMetric]) -> (ComponentHealth, f64) {
        let mut critical_count = 0;
        let mut warning_count = 0;
        let mut total_score = 0.0;
        let mut count = 0;

        for metric in metrics {
            let is_critical = metric
                .threshold_critical
                .map(|t| metric.value >= t)
                .unwrap_or(false);
            let is_warning = metric
                .threshold_warning
                .map(|t| metric.value >= t)
                .unwrap_or(false);

            if is_critical {
                critical_count += 1;
                total_score += 0.0;
            } else if is_warning {
                warning_count += 1;
                total_score += 0.5;
            } else {
                total_score += 1.0;
            }
            count += 1;
        }

        let score = if count > 0 {
            total_score / count as f64
        } else {
            1.0
        };

        let health = if critical_count > 0 {
            ComponentHealth::Critical
        } else if warning_count > metrics.len() / 2 {
            ComponentHealth::Unhealthy
        } else if warning_count > 0 {
            ComponentHealth::Degraded
        } else {
            ComponentHealth::Healthy
        };

        (health, score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_monitor_creation() {
        let config = HealthMonitorConfig {
            collection_interval_secs: 5,
            retention_period_secs: 300,
            metric_thresholds: HashMap::new(),
        };
        let monitor = HealthMonitor::new(config);
        assert!(!*monitor.running.read().await);
    }

    #[test]
    fn test_compute_health_status() {
        let metrics = vec![];
        let (health, score) = HealthMonitor::compute_health_status(&metrics);
        assert_eq!(health, ComponentHealth::Healthy);
        assert_eq!(score, 1.0);
    }
}

