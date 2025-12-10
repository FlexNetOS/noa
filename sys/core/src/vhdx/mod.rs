//! VHDX integration helpers (US8)
//!
//! Provides portable stack packaging, nested support, and snapshot/rollback.

pub mod packaging;
pub mod nested;
pub mod snapshot;

pub use packaging::{VhdxPackager, VhdxPackage};
pub use nested::{NestedVhdxManager, MountedChild};
pub use snapshot::{VhdxSnapshot, VhdxSnapshotManager};
