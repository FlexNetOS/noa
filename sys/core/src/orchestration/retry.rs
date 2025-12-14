//! Retry and escalation logic (Phase 9 - T283)
use std::time::Duration;

pub fn backoff(attempt: u32) -> Duration {
    let ms = 100u64.saturating_mul(2u64.saturating_pow(attempt.min(6)));
    Duration::from_millis(ms.min(10_000))
}
