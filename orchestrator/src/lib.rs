//! Coordinator Plane - Audit, Analytics, and Promotion Decisions
//!
//! This crate implements the coordinator plane functionality for the 3-plane architecture:
//! - Long-term memory persistence
//! - Analytics and evaluation
//! - Promotion policy enforcement
//! - Audit logging for all plane transitions

pub mod audit;

pub use audit::{TransitionLogger, DecisionRecorder, TransitionQuery};

