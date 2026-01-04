//! EWC (Elastic Weight Consolidation) Module
//!
//! Prevent catastrophic forgetting

pub mod fisher;
pub mod consolidate;
pub mod adapters;

pub use fisher::{FisherInfo, FisherComputer};
pub use consolidate::{EwcTrainer, Ewcconfigs};
pub use adapters::{AdapterManager, TaskAdapter};

