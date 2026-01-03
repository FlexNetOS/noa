//! ML configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ML configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLConfig {
    pub enabled: bool,
    pub experiment_tracking: ExperimentTrackingConfig,
    pub model_registry: ModelRegistryConfig,
    pub pipeline: PipelineConfig,
    pub feature_store: FeatureStoreConfig,
    pub monitoring: MLMonitoringConfig,
}

/// Experiment tracking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentTrackingConfig {
    pub enabled: bool,
    pub backend: String,
    pub auto_log_metrics: bool,
    pub auto_log_params: bool,
    pub retention_days: u32,
}

/// Model registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistryConfig {
    pub enabled: bool,
    pub storage_path: String,
    pub versioning_enabled: bool,
    pub max_versions: u32,
}

/// Pipeline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
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

/// Feature store configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureStoreConfig {
    pub enabled: bool,
    pub backend: String,
    pub cache_enabled: bool,
    pub ttl_seconds: u64,
}

/// ML monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLMonitoringConfig {
    pub enabled: bool,
    pub drift_detection: bool,
    pub performance_tracking: bool,
    pub alert_thresholds: HashMap<String, f64>,
}

impl Default for MLConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            experiment_tracking: ExperimentTrackingConfig::default(),
            model_registry: ModelRegistryConfig::default(),
            pipeline: PipelineConfig::default(),
            feature_store: FeatureStoreConfig::default(),
            monitoring: MLMonitoringConfig::default(),
        }
    }
}

impl Default for ExperimentTrackingConfig {
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

impl Default for ModelRegistryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage_path: "./models".to_string(),
            versioning_enabled: true,
            max_versions: 10,
        }
    }
}

impl Default for PipelineConfig {
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

impl Default for FeatureStoreConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: "local".to_string(),
            cache_enabled: true,
            ttl_seconds: 3600,
        }
    }
}

impl Default for MLMonitoringConfig {
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
