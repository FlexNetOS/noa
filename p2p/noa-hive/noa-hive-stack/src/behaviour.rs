//! Network behaviour composition for NOA-Hive.

// This module would contain the behaviour composition logic.
// For now, we re-export from swarm.rs where the behaviour is defined.

pub use super::swarm::{HiveBehaviour, HiveRequest, HiveResponse};
