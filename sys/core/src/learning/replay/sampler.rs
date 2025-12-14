//! Experience Replay Sampling
//!
//! T664: Implement experience replay sampling
//! US2: Sample experiences for training

use crate::error::Result;
use crate::learning::replay::{ReplayBuffer, Experience};
use serde::{Deserialize, Serialize};

/// Sampling strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SamplingStrategy {
    Uniform,
    Prioritized,
    Recent,
}

/// Experience sampler
pub struct ExperienceSampler {
    buffer: ReplayBuffer,
    strategy: SamplingStrategy,
}

impl ExperienceSampler {
    /// Create a new sampler
    pub fn new(buffer: ReplayBuffer, strategy: SamplingStrategy) -> Self {
        Self { buffer, strategy }
    }

    /// Sample experiences
    pub async fn sample(&self, count: usize) -> Result<Vec<Experience>> {
        match self.strategy {
            SamplingStrategy::Uniform => {
                Ok(self.buffer.sample(count).await)
            }
            SamplingStrategy::Prioritized => {
                // Prioritize high-reward experiences
                let all = self.buffer.sample(self.buffer.len().await).await;
                let mut prioritized: Vec<_> = all.into_iter().collect();
                prioritized.sort_by(|a, b| b.reward.partial_cmp(&a.reward).unwrap());
                Ok(prioritized.into_iter().take(count).collect())
            }
            SamplingStrategy::Recent => {
                // Return most recent experiences
                let all = self.buffer.sample(self.buffer.len().await).await;
                let mut recent: Vec<_> = all.into_iter().collect();
                recent.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                Ok(recent.into_iter().take(count).collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sampler() {
        let buffer = ReplayBuffer::new(10);
        let sampler = ExperienceSampler::new(buffer, SamplingStrategy::Uniform);
        let samples = sampler.sample(5).await.unwrap();
        assert!(samples.len() <= 5);
    }
}

