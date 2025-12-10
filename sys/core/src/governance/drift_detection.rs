use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

/// Drift severity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DriftStatus {
    Healthy,
    Warning,
    Critical,
}

/// Drift signal captured during testing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftSignal {
    pub metric: String,
    pub value: f64,
    pub threshold: f64,
    pub observed_at: DateTime<Utc>,
    pub details: String,
}

/// Drift detector with in-memory window.
pub struct DriftDetector {
    signals: Arc<RwLock<Vec<DriftSignal>>>,
    max_signals: usize,
    warning_ratio: f64,
    critical_ratio: f64,
}

impl DriftDetector {
    pub fn new(max_signals: usize) -> Self {
        Self {
            signals: Arc::new(RwLock::new(Vec::new())),
            max_signals,
            warning_ratio: 0.2,
            critical_ratio: 0.4,
        }
    }

    pub async fn record_signal(&self, signal: DriftSignal) {
        let mut signals = self.signals.write().await;
        signals.push(signal);
        if signals.len() > self.max_signals {
            let overflow = signals.len() - self.max_signals;
            signals.drain(0..overflow);
        }
    }

    pub async fn evaluate(&self) -> DriftStatus {
        let signals = self.signals.read().await;
        if signals.is_empty() {
            return DriftStatus::Healthy;
        }

        let over_threshold = signals
            .iter()
            .filter(|s| s.value > s.threshold)
            .count() as f64;
        let ratio = over_threshold / (signals.len() as f64);

        if ratio >= self.critical_ratio {
            DriftStatus::Critical
        } else if ratio >= self.warning_ratio {
            DriftStatus::Warning
        } else {
            DriftStatus::Healthy
        }
    }

    /// Run a lightweight testing loop that samples a metric at a fixed interval.
    pub async fn testing_loop<F, Fut>(&self, interval: Duration, mut sampler: F) -> DriftStatus
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = DriftSignal>,
    {
        for _ in 0..3 {
            let signal = sampler().await;
            self.record_signal(signal).await;
            sleep(interval).await;
        }

        let status = self.evaluate().await;
        match status {
            DriftStatus::Healthy => info!(target: "governance::drift", "Drift testing loop: healthy"),
            DriftStatus::Warning => warn!(target: "governance::drift", "Drift testing loop: warning"),
            DriftStatus::Critical => warn!(target: "governance::drift", "Drift testing loop: critical"),
        }
        status
    }
}

impl Default for DriftDetector {
    fn default() -> Self {
        Self::new(200)
    }
}
