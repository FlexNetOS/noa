use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metric sample collected during self-analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSample {
    pub name: String,
    pub value: f64,
    pub recorded_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Collector for performance metrics.
#[derive(Default, Debug)]
pub struct PerformanceMetrics {
    samples: Vec<MetricSample>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn record(&mut self, name: impl Into<String>, value: f64, metadata: serde_json::Value) {
        self.samples.push(MetricSample {
            name: name.into(),
            value,
            recorded_at: Utc::now(),
            metadata,
        });
    }

    pub fn record_latency_ms(&mut self, value: f64) {
        self.record("latency_ms", value, serde_json::json!({}));
    }

    pub fn record_success_rate(&mut self, value: f64) {
        self.record("success_rate", value, serde_json::json!({}));
    }

    pub fn record_resource_utilization(&mut self, value: f64) {
        self.record("resource_utilization", value, serde_json::json!({}));
    }

    pub fn average(&self, metric: &str) -> Option<f64> {
        let values: Vec<f64> =
            self.samples.iter().filter(|s| s.name == metric).map(|s| s.value).collect();
        if values.is_empty() {
            None
        } else {
            Some(values.iter().sum::<f64>() / values.len() as f64)
        }
    }

    pub fn latest(&self, metric: &str) -> Option<MetricSample> {
        self.samples.iter().filter(|s| s.name == metric).cloned().last()
    }

    pub fn all(&self) -> &[MetricSample] {
        &self.samples
    }
}
