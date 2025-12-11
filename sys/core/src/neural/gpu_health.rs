//! GPU Health Monitoring
//!
//! T485: Implement GPU health monitoring
//! US2: Monitor GPU health and performance

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GPU health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GpuHealthStatus {
    Healthy,
    Warning,
    Critical,
    Offline,
}

/// GPU health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuHealthMetrics {
    pub device_id: u32,
    pub status: GpuHealthStatus,
    pub temperature_celsius: f64,
    pub power_usage_watts: f64,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub utilization_percent: f64,
    pub error_count: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// GPU health monitor
pub struct GpuHealthMonitor {
    metrics: Arc<RwLock<HashMap<u32, GpuHealthMetrics>>>,
}

impl GpuHealthMonitor {
    /// Create a new GPU health monitor
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Update health metrics for a device
    pub async fn update_metrics(&self, device_id: u32, metrics: GpuHealthMetrics) -> Result<()> {
        let mut health_metrics = self.metrics.write().await;
        health_metrics.insert(device_id, metrics);
        Ok(())
    }

    /// Get health metrics for a device
    pub async fn get_metrics(&self, device_id: u32) -> Option<GpuHealthMetrics> {
        let metrics = self.metrics.read().await;
        metrics.get(&device_id).cloned()
    }

    /// Check device health status
    pub async fn check_health(&self, device_id: u32) -> Result<GpuHealthStatus> {
        let metrics = self.metrics.read().await;

        if let Some(metric) = metrics.get(&device_id) {
            // Determine health based on metrics
            let status = if metric.temperature_celsius > 85.0 || metric.error_count > 100 {
                GpuHealthStatus::Critical
            } else if metric.temperature_celsius > 75.0
                || metric.utilization_percent > 95.0
                || metric.error_count > 10
            {
                GpuHealthStatus::Warning
            } else {
                GpuHealthStatus::Healthy
            };

            Ok(status)
        } else {
            Ok(GpuHealthStatus::Offline)
        }
    }

    /// Get all device health statuses
    pub async fn get_all_health_statuses(&self) -> Result<HashMap<u32, GpuHealthStatus>> {
        let metrics = self.metrics.read().await;
        let mut statuses = HashMap::new();

        for device_id in metrics.keys() {
            let status = self.check_health(*device_id).await?;
            statuses.insert(*device_id, status);
        }

        Ok(statuses)
    }

    /// Get devices with critical health
    pub async fn get_critical_devices(&self) -> Result<Vec<u32>> {
        let statuses = self.get_all_health_statuses().await?;
        Ok(statuses
            .into_iter()
            .filter(|(_, status)| *status == GpuHealthStatus::Critical)
            .map(|(id, _)| id)
            .collect())
    }
}

impl Default for GpuHealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_monitoring() {
        let monitor = GpuHealthMonitor::new();

        let metrics = GpuHealthMetrics {
            device_id: 0,
            status: GpuHealthStatus::Healthy,
            temperature_celsius: 50.0,
            power_usage_watts: 100.0,
            memory_used_bytes: 4 * 1024 * 1024 * 1024,
            memory_total_bytes: 8 * 1024 * 1024 * 1024,
            utilization_percent: 50.0,
            error_count: 0,
            last_updated: chrono::Utc::now(),
        };

        monitor.update_metrics(0, metrics).await.unwrap();
        let status = monitor.check_health(0).await.unwrap();
        assert_eq!(status, GpuHealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_critical_health() {
        let monitor = GpuHealthMonitor::new();

        let metrics = GpuHealthMetrics {
            device_id: 0,
            status: GpuHealthStatus::Critical,
            temperature_celsius: 90.0,
            power_usage_watts: 200.0,
            memory_used_bytes: 7 * 1024 * 1024 * 1024,
            memory_total_bytes: 8 * 1024 * 1024 * 1024,
            utilization_percent: 99.0,
            error_count: 150,
            last_updated: chrono::Utc::now(),
        };

        monitor.update_metrics(0, metrics).await.unwrap();
        let status = monitor.check_health(0).await.unwrap();
        assert_eq!(status, GpuHealthStatus::Critical);
    }
}
