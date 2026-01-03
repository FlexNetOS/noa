//! Litho Executor
//!
//! Orchestrates multi-pass documentation generation with adaptive parallelism.
//! Delegates to Rust Pack subagents and handles graceful drain on resource spikes.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;

use super::{FallbackChain, LithoConfig, LithoError, LithoPass, ManualEditPreserver};

/// Execution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Run passes sequentially
    Sequential,
    /// Run parallelizable passes concurrently
    Parallel,
    /// Dynamically switch based on resource usage
    Adaptive,
}

/// Current execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutorState {
    /// Not started
    Idle,
    /// Currently executing passes
    Running,
    /// Draining: completing current work before switching to sequential
    Draining,
    /// Paused due to foreground activity
    Paused,
    /// Completed successfully
    Completed,
    /// Failed with error
    Failed,
    /// Cancelled by user
    Cancelled,
}

/// Pass execution status
#[derive(Debug, Clone)]
pub struct PassStatus {
    pub pass_id: u8,
    pub name: String,
    pub state: PassState,
    pub progress_percent: u8,
    pub files_processed: usize,
    pub files_total: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassState {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Resource snapshot for adaptive execution
#[derive(Debug, Clone, Copy)]
pub struct ResourceSnapshot {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub gpu_percent: Option<f32>,
}

impl ResourceSnapshot {
    /// Check if resources are below threshold for parallel execution
    pub fn can_parallelize(&self, threshold: f32) -> bool {
        let total = self.cpu_percent.max(self.memory_percent);
        total < threshold
    }

    /// Get combined resource usage
    pub fn total_usage(&self) -> f32 {
        self.cpu_percent.max(self.memory_percent)
    }
}

/// Litho executor for multi-pass documentation generation
pub struct LithoExecutor {
    config: LithoConfig,
    fallback_chain: FallbackChain,
    passes: Vec<LithoPass>,
    manual_edit_preserver: ManualEditPreserver,
    noa_root: PathBuf,

    // Execution state (atomic for thread-safe status checks)
    state: Arc<AtomicU8>,
    should_cancel: Arc<AtomicBool>,
    is_draining: Arc<AtomicBool>,

    // Pass status tracking
    pass_status: Vec<PassStatus>,
}

impl LithoExecutor {
    /// Create a new executor
    pub fn new(
        config: LithoConfig,
        fallback_chain: FallbackChain,
        passes: Vec<LithoPass>,
        noa_root: PathBuf,
    ) -> Self {
        let pass_status = passes
            .iter()
            .map(|p| PassStatus {
                pass_id: p.id,
                name: p.name.clone(),
                state: PassState::Pending,
                progress_percent: 0,
                files_processed: 0,
                files_total: 0,
                error: None,
            })
            .collect();

        let manual_edit_preserver = ManualEditPreserver::new(
            config.manual_edits.marker_start.clone(),
            config.manual_edits.marker_end.clone(),
            if config.manual_edits.validation == "fail_on_loss" {
                super::manual_edit::ValidationMode::FailOnLoss
            } else {
                super::manual_edit::ValidationMode::WarnOnLoss
            },
        );

        Self {
            config,
            fallback_chain,
            passes,
            manual_edit_preserver,
            noa_root,
            state: Arc::new(AtomicU8::new(ExecutorState::Idle as u8)),
            should_cancel: Arc::new(AtomicBool::new(false)),
            is_draining: Arc::new(AtomicBool::new(false)),
            pass_status,
        }
    }

    /// Get current execution state
    pub fn state(&self) -> ExecutorState {
        match self.state.load(Ordering::SeqCst) {
            0 => ExecutorState::Idle,
            1 => ExecutorState::Running,
            2 => ExecutorState::Draining,
            3 => ExecutorState::Paused,
            4 => ExecutorState::Completed,
            5 => ExecutorState::Failed,
            6 => ExecutorState::Cancelled,
            _ => ExecutorState::Idle,
        }
    }

    /// Set execution state
    fn set_state(&self, state: ExecutorState) {
        self.state.store(state as u8, Ordering::SeqCst);
    }

    /// Request cancellation
    pub fn cancel(&self) {
        self.should_cancel.store(true, Ordering::SeqCst);
    }

    /// Check if cancellation was requested
    pub fn is_cancelled(&self) -> bool {
        self.should_cancel.load(Ordering::SeqCst)
    }

    /// Trigger graceful drain
    pub fn start_drain(&self) {
        self.is_draining.store(true, Ordering::SeqCst);
        self.set_state(ExecutorState::Draining);
    }

    /// Check if draining
    pub fn is_draining(&self) -> bool {
        self.is_draining.load(Ordering::SeqCst)
    }

    /// Complete drain and switch to sequential
    pub fn complete_drain(&self) {
        self.is_draining.store(false, Ordering::SeqCst);
        self.set_state(ExecutorState::Running);
    }

    /// Get pass status
    pub fn pass_status(&self) -> &[PassStatus] {
        &self.pass_status
    }

    /// Determine execution mode based on resource usage
    pub fn determine_mode(&self, resources: &ResourceSnapshot) -> ExecutionMode {
        let execution_mode = self.config.runtime.execution.as_str();

        match execution_mode {
            "sequential" => ExecutionMode::Sequential,
            "parallel" => ExecutionMode::Parallel,
            "adaptive" | _ => {
                if self.is_draining() {
                    ExecutionMode::Sequential
                } else if resources.can_parallelize(self.config.runtime.parallel_threshold) {
                    ExecutionMode::Parallel
                } else {
                    ExecutionMode::Sequential
                }
            }
        }
    }

    /// Handle resource spike - initiate graceful drain
    pub fn handle_resource_spike(&self, resources: &ResourceSnapshot) {
        if resources.total_usage() > self.config.runtime.parallel_threshold
            && !self.is_draining()
            && self.state() == ExecutorState::Running
        {
            tracing::info!(
                "Resource spike detected ({}%), initiating graceful drain",
                (resources.total_usage() * 100.0) as u8
            );
            self.start_drain();
        }
    }

    /// Check if can resume parallel execution after cooldown
    pub fn can_resume_parallel(&self, resources: &ResourceSnapshot, seconds_below_threshold: u64) -> bool {
        resources.can_parallelize(self.config.runtime.parallel_threshold)
            && seconds_below_threshold >= self.config.runtime.transitions.cooldown_before_parallel
    }

    /// Get passes that can run in parallel (after pass 1 completes)
    pub fn parallelizable_passes(&self) -> Vec<&LithoPass> {
        self.passes.iter().filter(|p| p.parallelizable).collect()
    }

    /// Get the structure pass (pass 1, must run first)
    pub fn structure_pass(&self) -> Option<&LithoPass> {
        self.passes.iter().find(|p| p.id == 1)
    }

    /// Update pass status
    pub fn update_pass_status(&mut self, pass_id: u8, state: PassState, progress: u8) {
        if let Some(status) = self.pass_status.iter_mut().find(|s| s.pass_id == pass_id) {
            status.state = state;
            status.progress_percent = progress;
        }
    }

    /// Mark pass as failed
    pub fn mark_pass_failed(&mut self, pass_id: u8, error: String) {
        if let Some(status) = self.pass_status.iter_mut().find(|s| s.pass_id == pass_id) {
            status.state = PassState::Failed;
            status.error = Some(error);
        }
    }

    /// Get overall progress percentage
    pub fn overall_progress(&self) -> u8 {
        if self.pass_status.is_empty() {
            return 0;
        }
        let total: u32 = self.pass_status.iter().map(|s| s.progress_percent as u32).sum();
        (total / self.pass_status.len() as u32) as u8
    }

    /// Get the fallback chain
    pub fn fallback_chain(&self) -> &FallbackChain {
        &self.fallback_chain
    }

    /// Get mutable fallback chain
    pub fn fallback_chain_mut(&mut self) -> &mut FallbackChain {
        &mut self.fallback_chain
    }

    /// Get manual edit preserver
    pub fn manual_edit_preserver(&self) -> &ManualEditPreserver {
        &self.manual_edit_preserver
    }

    /// Get configuration
    pub fn config(&self) -> &LithoConfig {
        &self.config
    }

    /// Get NOA root path
    pub fn noa_root(&self) -> &PathBuf {
        &self.noa_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> LithoConfig {
        LithoConfig::default()
    }

    fn test_passes() -> Vec<LithoPass> {
        super::super::default_passes()
    }

    #[test]
    fn test_determine_mode_adaptive() {
        let executor = LithoExecutor::new(
            test_config(),
            FallbackChain::default(),
            test_passes(),
            PathBuf::from("/test"),
        );

        let low_resources = ResourceSnapshot {
            cpu_percent: 0.20,
            memory_percent: 0.15,
            gpu_percent: None,
        };
        assert_eq!(executor.determine_mode(&low_resources), ExecutionMode::Parallel);

        let high_resources = ResourceSnapshot {
            cpu_percent: 0.50,
            memory_percent: 0.40,
            gpu_percent: None,
        };
        assert_eq!(executor.determine_mode(&high_resources), ExecutionMode::Sequential);
    }

    #[test]
    fn test_graceful_drain() {
        let executor = LithoExecutor::new(
            test_config(),
            FallbackChain::default(),
            test_passes(),
            PathBuf::from("/test"),
        );

        assert!(!executor.is_draining());
        executor.start_drain();
        assert!(executor.is_draining());
        assert_eq!(executor.state(), ExecutorState::Draining);

        executor.complete_drain();
        assert!(!executor.is_draining());
    }

    #[test]
    fn test_cancel() {
        let executor = LithoExecutor::new(
            test_config(),
            FallbackChain::default(),
            test_passes(),
            PathBuf::from("/test"),
        );

        assert!(!executor.is_cancelled());
        executor.cancel();
        assert!(executor.is_cancelled());
    }
}
