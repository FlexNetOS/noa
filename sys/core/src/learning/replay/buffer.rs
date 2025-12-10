//! Short-term Memory Buffer
//!
//! T662: Implement short-term memory buffer
//! US2: Buffer for recent experiences

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Experience entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub id: Uuid,
    pub state: serde_json::Value,
    pub action: serde_json::Value,
    pub reward: f64,
    pub next_state: Option<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Short-term memory buffer
pub struct ReplayBuffer {
    buffer: Arc<RwLock<VecDeque<Experience>>>,
    max_size: usize,
}

impl ReplayBuffer {
    /// Create a new replay buffer
    pub fn new(max_size: usize) -> Self {
        Self {
            buffer: Arc::new(RwLock::new(VecDeque::with_capacity(max_size))),
            max_size,
        }
    }

    /// Add experience to buffer
    pub async fn add(&self, experience: Experience) -> Result<()> {
        let mut buffer = self.buffer.write().await;

        if buffer.len() >= self.max_size {
            buffer.pop_front();
        }

        buffer.push_back(experience);
        Ok(())
    }

    /// Sample experiences from buffer
    pub async fn sample(&self, count: usize) -> Vec<Experience> {
        let buffer = self.buffer.read().await;
        let available = buffer.len().min(count);

        // Simple random sampling (in production, use proper RNG)
        let mut samples = Vec::new();
        for i in 0..available {
            if let Some(exp) = buffer.get(i) {
                samples.push(exp.clone());
            }
        }

        samples
    }

    /// Get buffer size
    pub async fn len(&self) -> usize {
        self.buffer.read().await.len()
    }

    /// Clear buffer
    pub async fn clear(&self) {
        let mut buffer = self.buffer.write().await;
        buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_replay_buffer() {
        let buffer = ReplayBuffer::new(10);

        let exp = Experience {
            id: Uuid::new_v4(),
            state: serde_json::json!({}),
            action: serde_json::json!({}),
            reward: 1.0,
            next_state: None,
            timestamp: chrono::Utc::now(),
        };

        buffer.add(exp).await.unwrap();
        assert_eq!(buffer.len().await, 1);

        let samples = buffer.sample(1).await;
        assert_eq!(samples.len(), 1);
    }
}
