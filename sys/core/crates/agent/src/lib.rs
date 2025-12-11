//! NOA Agent System
//!
//! Autonomous agent orchestration for NOA.

pub mod circuit_breaker;
pub mod constitutional;
pub mod loop_detection;
pub mod orchestrator;
pub mod permanent;
pub mod stack;

pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState};
pub use constitutional::{AgentOperation, ConstitutionalEnforcer, ConstitutionalPrinciple};
pub use loop_detection::{LoopDetectedError, LoopDetectionConfig, LoopDetector};
pub use orchestrator::Orchestrator;
