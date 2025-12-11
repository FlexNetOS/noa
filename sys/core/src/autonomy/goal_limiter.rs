//! Goal limiter to cap self-generated goals (FR-099)

use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct GoalLimiter {
    max_per_window: usize,
    window: Duration,
    events: VecDeque<Instant>,
}

impl GoalLimiter {
    pub fn new(max_per_hour: usize) -> Self {
        Self {
            max_per_window: max_per_hour,
            window: Duration::from_secs(3600),
            events: VecDeque::new(),
        }
    }

    pub fn allow(&mut self) -> bool {
        let now = Instant::now();
        while let Some(front) = self.events.front() {
            if now.duration_since(*front) > self.window {
                self.events.pop_front();
            } else {
                break;
            }
        }

        if self.events.len() < self.max_per_window {
            self.events.push_back(now);
            true
        } else {
            false
        }
    }
}
