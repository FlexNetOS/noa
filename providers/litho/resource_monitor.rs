//! Resource Monitor
//!
//! Monitors system resources (CPU, memory) for adaptive execution decisions.
//! Polls every 500ms and provides current snapshot for parallelization checks.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::executor::ResourceSnapshot;

/// Resource monitor configsuration
#[derive(Debug, Clone)]
pub struct ResourceMonitorconfigs {
    /// Poll interval in milliseconds
    pub poll_interval_ms: u64,
    /// Threshold for parallel execution (0.0-1.0)
    pub parallel_threshold: f32,
    /// Cooldown before resuming parallel after spike (seconds)
    pub cooldown_seconds: u64,
    /// Drain timeout in seconds
    pub drain_timeout_seconds: u64,
}

impl Default for ResourceMonitorconfigs {
    fn default() -> Self {
        Self {
            poll_interval_ms: 500,
            parallel_threshold: 0.35,
            cooldown_seconds: 3,
            drain_timeout_seconds: 5,
        }
    }
}

/// Tracks resource usage over time
pub struct ResourceMonitor {
    configs: ResourceMonitorconfigs,
    is_running: Arc<AtomicBool>,
    last_snapshot: ResourceSnapshot,
    last_poll_time: Instant,
    below_threshold_since: Option<Instant>,
}

impl ResourceMonitor {
    /// Create a new resource monitor
    pub fn new(configs: ResourceMonitorconfigs) -> Self {
        Self {
            configs,
            is_running: Arc::new(AtomicBool::new(false)),
            last_snapshot: ResourceSnapshot {
                cpu_percent: 0.0,
                memory_percent: 0.0,
                gpu_percent: None,
            },
            last_poll_time: Instant::now(),
            below_threshold_since: None,
        }
    }

    /// Get the current resource snapshot
    #[cfg(feature = "sysinfo")]
    pub fn poll(&mut self) -> ResourceSnapshot {
        use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );

        // Need to wait and refresh again for accurate CPU
        std::thread::sleep(Duration::from_millis(100));
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let cpu_percent = sys.global_cpu_usage() / 100.0;
        let memory_percent = if sys.total_memory() > 0 {
            sys.used_memory() as f32 / sys.total_memory() as f32
        } else {
            0.0
        };

        self.last_snapshot = ResourceSnapshot {
            cpu_percent,
            memory_percent,
            gpu_percent: None, // GPU monitoring requires additional integration
        };
        self.last_poll_time = Instant::now();

        // Track time below threshold
        if self.last_snapshot.can_parallelize(self.configs.parallel_threshold) {
            if self.below_threshold_since.is_none() {
                self.below_threshold_since = Some(Instant::now());
            }
        } else {
            self.below_threshold_since = None;
        }

        self.last_snapshot
    }

    /// Get the current resource snapshot (stub for when sysinfo is not available)
    #[cfg(not(feature = "sysinfo"))]
    pub fn poll(&mut self) -> ResourceSnapshot {
        // Stub implementation - assume low resource usage
        self.last_snapshot = ResourceSnapshot {
            cpu_percent: 0.10,
            memory_percent: 0.15,
            gpu_percent: None,
        };
        self.last_poll_time = Instant::now();

        if self.last_snapshot.can_parallelize(self.configs.parallel_threshold) {
            if self.below_threshold_since.is_none() {
                self.below_threshold_since = Some(Instant::now());
            }
        } else {
            self.below_threshold_since = None;
        }

        self.last_snapshot
    }

    /// Get the last snapshot without polling
    pub fn last_snapshot(&self) -> ResourceSnapshot {
        self.last_snapshot
    }

    /// Check if should poll again based on interval
    pub fn should_poll(&self) -> bool {
        self.last_poll_time.elapsed() >= Duration::from_millis(self.configs.poll_interval_ms)
    }

    /// Get seconds below threshold
    pub fn seconds_below_threshold(&self) -> u64 {
        self.below_threshold_since
            .map(|t| t.elapsed().as_secs())
            .unwrap_or(0)
    }

    /// Check if can parallelize based on current resources
    pub fn can_parallelize(&self) -> bool {
        self.last_snapshot.can_parallelize(self.configs.parallel_threshold)
    }

    /// Check if can resume parallel after cooldown
    pub fn can_resume_parallel(&self) -> bool {
        self.can_parallelize() && self.seconds_below_threshold() >= self.configs.cooldown_seconds
    }

    /// Get the poll interval
    pub fn poll_interval(&self) -> Duration {
        Duration::from_millis(self.configs.poll_interval_ms)
    }

    /// Get drain timeout
    pub fn drain_timeout(&self) -> Duration {
        Duration::from_secs(self.configs.drain_timeout_seconds)
    }

    /// Start monitoring (for background polling)
    pub fn start(&self) {
        self.is_running.store(true, Ordering::SeqCst);
    }

    /// Stop monitoring
    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    /// Check if monitoring is active
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}

/// Resource spike handler
pub struct ResourceSpikeHandler {
    drain_start: Option<Instant>,
    drain_timeout: Duration,
}

impl ResourceSpikeHandler {
    /// Create a new spike handler
    pub fn new(drain_timeout: Duration) -> Self {
        Self {
            drain_start: None,
            drain_timeout,
        }
    }

    /// Start a graceful drain
    pub fn start_drain(&mut self) {
        if self.drain_start.is_none() {
            self.drain_start = Some(Instant::now());
            tracing::info!("Starting graceful drain (timeout: {:?})", self.drain_timeout);
        }
    }

    /// Check if drain has timed out
    pub fn is_drain_timed_out(&self) -> bool {
        self.drain_start
            .map(|t| t.elapsed() >= self.drain_timeout)
            .unwrap_or(false)
    }

    /// Check if currently draining
    pub fn is_draining(&self) -> bool {
        self.drain_start.is_some()
    }

    /// Complete the drain
    pub fn complete_drain(&mut self) {
        if self.drain_start.is_some() {
            let elapsed = self.drain_start.map(|t| t.elapsed()).unwrap_or_default();
            tracing::info!("Drain completed in {:?}", elapsed);
            self.drain_start = None;
        }
    }

    /// Get drain elapsed time
    pub fn drain_elapsed(&self) -> Option<Duration> {
        self.drain_start.map(|t| t.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_monitor_default() {
        let configs = ResourceMonitorconfigs::default();
        assert_eq!(configs.poll_interval_ms, 500);
        assert!((configs.parallel_threshold - 0.35).abs() < f32::EPSILON);
    }

    #[test]
    fn test_can_parallelize() {
        let mut monitor = ResourceMonitor::new(ResourceMonitorconfigs::default());
        monitor.poll();
        // Stub returns low usage, should be able to parallelize
        assert!(monitor.can_parallelize());
    }

    #[test]
    fn test_spike_handler() {
        let mut handler = ResourceSpikeHandler::new(Duration::from_secs(5));
        assert!(!handler.is_draining());

        handler.start_drain();
        assert!(handler.is_draining());
        assert!(!handler.is_drain_timed_out());

        handler.complete_drain();
        assert!(!handler.is_draining());
    }
}
