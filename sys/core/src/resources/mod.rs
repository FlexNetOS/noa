//! Resource adaptation layer
//!
//! Phase 13 (US9): Dynamic resource adaptation for cross-platform deployment.

pub mod allocator;
pub mod degradation;
pub mod model_selector;

pub use allocator::{ResourceAllocationPlan, ResourceAllocator};
pub use degradation::{DegradationAdvisor, DegradationPlan};
pub use model_selector::{ModelSelection, ModelSelector, ModelSizeClass};
