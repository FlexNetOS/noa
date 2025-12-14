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
pub mod continuous_loop;
pub mod decompose;
pub mod goal_queue;
pub mod resource_optimizer;
pub mod self_monitor;

// Full autonomy operation modules (FR-061-065)
pub mod autonomous_mode;
pub mod co_improve;
pub mod safety_net;
pub mod activity_log;
pub mod constitutional_boundary;

// Autonomous goal generation modules (FR-066-070)
pub mod goal_generator;
pub mod goal_boundary;
pub mod goal_rationale;
pub mod priority_queue;
pub mod pattern_analyzer;
pub mod goal_limiter;

#[cfg(test)]
pub mod pattern_analyzer_test;

// Re-exports
pub use continuous_loop::ContinuousLoop;
pub use decompose::GoalDecomposer;
pub use goal_queue::GoalQueueManager;
pub use resource_optimizer::ResourceOptimizer;
pub use self_monitor::PerformanceSelfMonitor;

pub use autonomous_mode::AutonomousMode;
pub use co_improve::CoImprovementIntake;
pub use safety_net::SafetyNet;
pub use activity_log::ActivityLog;
pub use constitutional_boundary::ConstitutionalBoundary;
pub use goal_generator::GoalGenerator;
pub use goal_boundary::GoalBoundaryChecker;
pub use goal_rationale::GoalRationaleLogger;
pub use priority_queue::PriorityQueue;
pub use pattern_analyzer::PatternAnalyzer;
pub use goal_limiter::GoalLimiter;
