use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metric entry for knowledge capsule operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeMetric {
    pub name: String,
    pub value: f64,
    pub recorded_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// KMETRICS_CAP: Tracks ingestion, errors, and latency for capsules.
#[derive(Default, Debug)]
pub struct KnowledgeMetrics {
    metrics: Vec<KnowledgeMetric>,
}

impl KnowledgeMetrics {
    pub fn new() -> Self {
        Self { metrics: Vec::new() }
    }

    pub fn record(&mut self, name: impl Into<String>, value: f64, metadata: serde_json::Value) {
        self.metrics.push(KnowledgeMetric {
            name: name.into(),
            value,
            recorded_at: Utc::now(),
            metadata,
        });
    }

    pub fn ingested(&mut self, count: u64) {
        self.record("ingested", count as f64, serde_json::json!({}));
    }

    pub fn error(&mut self, count: u64) {
        self.record("error", count as f64, serde_json::json!({}));
    }

    pub fn latency_ms(&mut self, value: f64) {
        self.record("latency_ms", value, serde_json::json!({}));
    }

    pub fn summary(&self) -> serde_json::Value {
        let mut totals = std::collections::HashMap::new();
        for metric in &self.metrics {
            *totals.entry(metric.name.clone()).or_insert(0.0) += metric.value;
        }
        serde_json::json!(totals)
    }

    pub fn all(&self) -> &[KnowledgeMetric] {
        &self.metrics
    }
}
