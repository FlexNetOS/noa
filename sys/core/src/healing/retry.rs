//! Retry Counter
//!
//! T617: Implement retry counter (≥3 attempts before escalate)
//! FR-074: System MUST retry fixes up to 3 times before escalating
//! §3.4: Adaptive & Self-Improving

use crate::error::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Retry counter configuration
#[derive(Debug, Clone)]
pub struct RetryCounterConfig {
    pub max_attempts: u32,
    pub retry_backoff_secs: u64,
}

impl Default for RetryCounterConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            retry_backoff_secs: 5,
        }
    }
}

/// Retry counter
pub struct RetryCounter {
    config: RetryCounterConfig,
    event_attempts: Arc<RwLock<HashMap<Uuid, u32>>>,
}

impl RetryCounter {
    /// Create a new retry counter
    pub fn new(config: RetryCounterConfig) -> Self {
        Self {
            config,
            event_attempts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Increment retry count for an event
    pub async fn increment(&self, event_id: &Uuid) -> Result<u32> {
        let mut attempts = self.event_attempts.write().await;
        let count = attempts.entry(*event_id).or_insert(0);
        *count += 1;

        let current_count = *count;
        drop(attempts);

        info!(
            event_id = %event_id,
            attempt = current_count,
            max_attempts = self.config.max_attempts,
            "Incremented retry counter"
        );

        if current_count >= self.config.max_attempts {
            warn!(
                event_id = %event_id,
                attempts = current_count,
                "Retry limit reached, should escalate"
            );
        }

        Ok(current_count)
    }

    /// Get retry count for an event
    pub async fn get_count(&self, event_id: &Uuid) -> u32 {
        let attempts = self.event_attempts.read().await;
        attempts.get(event_id).copied().unwrap_or(0)
    }

    /// Check if retry limit reached
    pub async fn is_limit_reached(&self, event_id: &Uuid) -> bool {
        let count = self.get_count(event_id).await;
        count >= self.config.max_attempts
    }

    /// Reset retry count for an event
    pub async fn reset(&self, event_id: &Uuid) -> Result<()> {
        let mut attempts = self.event_attempts.write().await;
        attempts.remove(event_id);
        debug!(event_id = %event_id, "Reset retry counter");
        Ok(())
    }

    /// Clear all retry counts
    pub async fn clear_all(&self) -> Result<()> {
        let mut attempts = self.event_attempts.write().await;
        attempts.clear();
        debug!("Cleared all retry counters");
        Ok(())
    }
}

impl Default for RetryCounter {
    fn default() -> Self {
        Self::new(RetryCounterConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_counter() {
        let counter = RetryCounter::default();
        let event_id = Uuid::new_v4();

        assert_eq!(counter.get_count(&event_id).await, 0);
        assert!(!counter.is_limit_reached(&event_id).await);

        assert_eq!(counter.increment(&event_id).await.unwrap(), 1);
        assert_eq!(counter.get_count(&event_id).await, 1);
        assert!(!counter.is_limit_reached(&event_id).await);

        assert_eq!(counter.increment(&event_id).await.unwrap(), 2);
        assert_eq!(counter.increment(&event_id).await.unwrap(), 3);
        assert!(counter.is_limit_reached(&event_id).await);

        counter.reset(&event_id).await.unwrap();
        assert_eq!(counter.get_count(&event_id).await, 0);
    }
}
