//! Exponential backoff handler (FR-096)

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Backoff {
    current: Duration,
    max: Duration,
    factor: f64,
}

impl Backoff {
    pub fn new(initial: Duration, max: Duration, factor: f64) -> Self {
        Self {
            current: initial,
            max,
            factor,
        }
    }

    pub fn next_delay(&mut self) -> Duration {
        let delay = self.current;
        let next = (self.current.as_millis() as f64 * self.factor) as u64;
        self.current = Duration::from_millis(next.min(self.max.as_millis() as u64));
        delay
    }

    pub fn reset(&mut self, initial: Duration) {
        self.current = initial;
    }
}

impl Default for Backoff {
    fn default() -> Self {
        Backoff::new(Duration::from_secs(1), Duration::from_secs(60), 2.0)
    }
}
