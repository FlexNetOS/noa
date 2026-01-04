//! ML configsuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ML configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLconfigs {
    pub enabled: bool,
    pub experiment_tracking: ExperimentTrackingconfigs,
    pub model_registry: ModelRegistryconfigs,
    pub pipeline: Pipelineconfigs,
    pub feature_store: FeatureStoreconfigs,
    pub monitoring: MLMonitoringconfigs,
}

/// Experiment tracking configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentTrackingconfigs {
    pub enabled: bool,
    pub backend: String,
    pub auto_log_metrics: bool,
    pub auto_log_params: bool,
    pub retention_days: u32,
}

/// Model registry configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistryconfigs {
    pub enabled: bool,
    pub storage_path: String,
    pub versioning_enabled: bool,
    pub max_versions: u32,
}

/// Pipeline configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipelineconfigs {
    pub enabled: bool,
    pub max_concurrent_runs: u32,
    pub default_timeout_minutes: u32,
    pub retry_policy: RetryPolicy,
}

/// Retry policy for pipeline steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_multiplier: f64,
    pub initial_delay_ms: u64,
}

/// Feature store configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureStoreconfigs {
    pub enabled: bool,
    pub backend: String,
    pub cache_enabled: bool,
    pub ttl_seconds: u64,
}

/// ML monitoring configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLMonitoringconfigs {
    pub enabled: bool,
    pub drift_detection: bool,
    pub performance_tracking: bool,
    pub alert_thresholds: HashMap<String, f64>,
}

impl Default for MLconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            experiment_tracking: ExperimentTrackingconfigs::default(),
            model_registry: ModelRegistryconfigs::default(),
            pipeline: Pipelineconfigs::default(),
            feature_store: FeatureStoreconfigs::default(),
            monitoring: MLMonitoringconfigs::default(),
        }
    }
}

impl Default for ExperimentTrackingconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: "local".to_string(),
            auto_log_metrics: true,
            auto_log_params: true,
            retention_days: 90,
        }
    }
}

impl Default for ModelRegistryconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            storage_path: "./models".to_string(),
            versioning_enabled: true,
            max_versions: 10,
        }
    }
}

impl Default for Pipelineconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            max_concurrent_runs: 4,
            default_timeout_minutes: 60,
            retry_policy: RetryPolicy::default(),
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            backoff_multiplier: 2.0,
            initial_delay_ms: 1000,
        }
    }
}

impl Default for FeatureStoreconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: "local".to_string(),
            cache_enabled: true,
            ttl_seconds: 3600,
        }
    }
}

impl Default for MLMonitoringconfigs {
    fn default() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert("accuracy_drop".to_string(), 0.05);
        thresholds.insert("latency_increase_ms".to_string(), 100.0);
        thresholds.insert("error_rate".to_string(), 0.01);
        
        Self {
            enabled: true,
            drift_detection: true,
            performance_tracking: true,
            alert_thresholds: thresholds,
        }
    }
}
