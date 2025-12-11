//! NOA Autonomy Module
//!
//! Implements autonomous operation capabilities including:
//! - Autonomous execution mode (§3.4)
//! - Goal generation and management (FR-066-070)
//! - 3-plane rollback safety net (FR-061-065)
//! - Activity logging (§3.5)
//! - Constitutional boundaries (§3.10)
//!
//! FR-051-055: Autonomous Continuous Operation
//! FR-061-065: Full Autonomy Operation
//! FR-066-070: Autonomous Goal Generation

// Continuous loop modules (FR-051-055)
pub mod ampk;
pub mod autonomy_loop;
pub mod continuous_loop;
pub mod decompose;
pub mod goal_queue;
pub mod resource_optimizer;
pub mod scheduler;
pub mod self_monitor;

// Full autonomy operation modules (FR-061-065)
pub mod activity_log;
pub mod autonomous_mode;
pub mod co_improve;
pub mod constitutional_boundary;
pub mod safety_net;

// Autonomous goal generation modules (FR-066-070)
pub mod goal_boundary;
pub mod goal_generator;
pub mod goal_limiter;
pub mod goal_rationale;
pub mod pattern_analyzer;
pub mod priority_queue;

#[cfg(test)]
pub mod pattern_analyzer_test;

// Re-exports
pub use ampk::{AMPKAction, AMPKMode, ResourceSnapshot};
pub use autonomy_loop::{AutonomyLoop, DecideOutcome, SenseInput};
pub use continuous_loop::ContinuousLoop;
pub use decompose::GoalDecomposer;
pub use goal_queue::GoalQueueManager;
pub use resource_optimizer::ResourceOptimizer;
pub use scheduler::SelfReinventionScheduler;
pub use self_monitor::PerformanceSelfMonitor;

pub use activity_log::ActivityLog;
pub use autonomous_mode::AutonomousMode;
pub use co_improve::CoImprovementIntake;
pub use constitutional_boundary::ConstitutionalBoundary;
pub use goal_boundary::GoalBoundaryChecker;
pub use goal_generator::GoalGenerator;
pub use goal_limiter::GoalLimiter;
pub use goal_rationale::GoalRationaleLogger;
pub use pattern_analyzer::PatternAnalyzer;
pub use priority_queue::PriorityQueue;
pub use safety_net::SafetyNet;
