//! NOA Agent System
//!
//! Autonomous agent orchestration for NOA.

pub mod permanent;
pub mod stack;
pub mod orchestrator;
pub mod circuit_breaker;
pub mod loop_detection;
pub mod constitutional;

pub use orchestrator::Orchestrator;
pub use circuit_breaker::{CircuitBreaker, CircuitState, CircuitBreakerConfig};
pub use loop_detection::{LoopDetector, LoopDetectedError, LoopDetectionConfig};
pub use constitutional::{ConstitutionalEnforcer, ConstitutionalPrinciple, AgentOperation};

