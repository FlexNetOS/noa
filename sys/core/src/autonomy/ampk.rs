use chrono::{DateTime, Utc};

/// Snapshot of current resource utilization.
#[derive(Debug, Clone)]
pub struct ResourceSnapshot {
    pub cpu_load: f64,
    pub memory_used: f64,
    pub memory_capacity: f64,
    pub disk_used: f64,
    pub disk_capacity: f64,
    pub captured_at: DateTime<Utc>,
}

impl ResourceSnapshot {
    pub fn utilization(&self) -> (f64, f64, f64) {
        let mem = if self.memory_capacity > 0.0 {
            self.memory_used / self.memory_capacity
        } else {
            0.0
        };
        let disk = if self.disk_capacity > 0.0 {
            self.disk_used / self.disk_capacity
        } else {
            0.0
        };
        (self.cpu_load, mem, disk)
    }
}

/// AMPK actions during scarcity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AMPKAction {
    Throttle,
    Quiesce,
    Resume,
}

/// AMPK mode detection and control logic.
pub struct AMPKMode {
    pub throttle_threshold: f64,
    pub quiesce_threshold: f64,
}

impl AMPKMode {
    pub fn new(throttle_threshold: f64, quiesce_threshold: f64) -> Self {
        Self {
            throttle_threshold,
            quiesce_threshold,
        }
    }

    pub fn evaluate(&self, snapshot: &ResourceSnapshot) -> AMPKAction {
        let (cpu, mem, disk) = snapshot.utilization();
        let max_util = cpu.max(mem).max(disk);

        if max_util >= self.quiesce_threshold {
            AMPKAction::Quiesce
        } else if max_util >= self.throttle_threshold {
            AMPKAction::Throttle
        } else {
            AMPKAction::Resume
        }
    }
}

impl Default for AMPKMode {
    fn default() -> Self {
        Self::new(0.75, 0.9)
    }
}
