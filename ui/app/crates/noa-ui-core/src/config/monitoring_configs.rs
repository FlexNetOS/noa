//! Monitoring configsuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Monitoring configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitoringconfigs {
    pub enabled: bool,
    pub metrics: Metricsconfigs,
    pub logging: Loggingconfigs,
    pub tracing: Tracingconfigs,
    pub alerting: Alertingconfigs,
}

/// Metrics collection configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metricsconfigs {
    pub enabled: bool,
    pub endpoint: Option<String>,
    pub interval_seconds: u64,
    pub include_system_metrics: bool,
    pub custom_metrics: Vec<String>,
}

/// Logging configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loggingconfigs {
    pub level: String,
    pub format: LogFormat,
    pub output: LogOutput,
    pub include_caller: bool,
    pub include_timestamp: bool,
}

/// Log format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Text,
    Json,
    Compact,
}

/// Log output configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogOutput {
    Stdout,
    Stderr,
    File { path: String },
    Both { path: String },
}

/// Distributed tracing configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracingconfigs {
    pub enabled: bool,
    pub backend: TracingBackend,
    pub sample_rate: f64,
    pub propagation_format: String,
}

/// Tracing backend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TracingBackend {
    None,
    Jaeger { endpoint: String },
    Zipkin { endpoint: String },
    Otlp { endpoint: String },
}

/// Alerting configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alertingconfigs {
    pub enabled: bool,
    pub channels: Vec<AlertChannel>,
    pub rules: Vec<AlertRule>,
}

/// Alert channel configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannel {
    pub name: String,
    pub channel_type: AlertChannelType,
    pub configs: HashMap<String, String>,
}

/// Alert channel types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertChannelType {
    Email,
    Slack,
    Webhook,
    PagerDuty,
}

/// Alert rule configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    pub condition: String,
    pub threshold: f64,
    pub duration_seconds: u64,
    pub severity: AlertSeverity,
    pub channels: Vec<String>,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl Default for Monitoringconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics: Metricsconfigs::default(),
            logging: Loggingconfigs::default(),
            tracing: Tracingconfigs::default(),
            alerting: Alertingconfigs::default(),
        }
    }
}

impl Default for Metricsconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: None,
            interval_seconds: 60,
            include_system_metrics: true,
            custom_metrics: Vec::new(),
        }
    }
}

impl Default for Loggingconfigs {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: LogFormat::Text,
            output: LogOutput::Stdout,
            include_caller: true,
            include_timestamp: true,
        }
    }
}

impl Default for Tracingconfigs {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: TracingBackend::None,
            sample_rate: 0.1,
            propagation_format: "w3c".to_string(),
        }
    }
}

impl Default for Alertingconfigs {
    fn default() -> Self {
        Self {
            enabled: false,
            channels: Vec::new(),
            rules: Vec::new(),
        }
    }
}
