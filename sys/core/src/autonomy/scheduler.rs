use chrono::{DateTime, Duration, Utc};

/// Hourly self-reinvention scheduler.
pub struct SelfReinventionScheduler {
    interval: Duration,
    last_run: Option<DateTime<Utc>>,
}

impl SelfReinventionScheduler {
    pub fn new_hours(hours: i64) -> Self {
        Self {
            interval: Duration::hours(hours),
            last_run: None,
        }
    }

    pub fn is_due(&self, now: DateTime<Utc>) -> bool {
        match self.last_run {
            Some(last) => now - last >= self.interval,
            None => true,
        }
    }

    pub fn mark_run(&mut self, when: DateTime<Utc>) {
        self.last_run = Some(when);
    }
}

impl Default for SelfReinventionScheduler {
    fn default() -> Self {
        Self::new_hours(1)
    }
}
