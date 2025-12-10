//! EWC (Elastic Weight Consolidation) Module
//!
//! Prevent catastrophic forgetting

pub mod adapters;
pub mod consolidate;
pub mod fisher;

pub use adapters::{AdapterManager, TaskAdapter};
pub use consolidate::{EwcConfig, EwcTrainer};
pub use fisher::{FisherComputer, FisherInfo};
