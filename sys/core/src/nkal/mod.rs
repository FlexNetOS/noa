//! NKAL boundary support modules
//!
//! Provides trust-boundary validation, sanitization, verification, and
//! checkpoint helpers for kernel mode transitions (Phase 18).

pub mod boundary;
pub mod checkpoint;
pub mod sanitize;
pub mod shutdown;
pub mod state;
pub mod verify;

pub use boundary::{BoundaryDecision, BoundaryValidator, CapabilityPolicy};
pub use checkpoint::{CheckpointWriter, KernelCheckpoint, MountSpec};
pub use sanitize::{SanitizedInput, Sanitizer};
pub use shutdown::{ShutdownGuard, ShutdownState};
pub use state::{ModeState, StateVerifier};
pub use verify::{OutputVerifier, VerifiedOutput};
