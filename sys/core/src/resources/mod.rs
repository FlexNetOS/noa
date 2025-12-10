//! Resource adaptation layer
//!
//! Phase 13 (US9): Dynamic resource adaptation for cross-platform deployment.

pub mod allocator;
pub mod model_selector;
pub mod degradation;

pub use allocator::{ResourceAllocationPlan, ResourceAllocator};
pub use model_selector::{ModelSelector, ModelSelection, ModelSizeClass};
pub use degradation::{DegradationAdvisor, DegradationPlan};
