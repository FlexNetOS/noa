//! Task scheduler with priority queue (Phase 9 - T281)
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
pub struct ScheduledTask {
    pub priority: i32,
    pub task: String,
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Scheduler {
    heap: BinaryHeap<ScheduledTask>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn enqueue(&mut self, priority: i32, task: String) {
        self.heap.push(ScheduledTask { priority, task });
    }

    pub fn next(&mut self) -> Option<String> {
        self.heap.pop().map(|t| t.task)
    }
}
