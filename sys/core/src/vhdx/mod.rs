//! VHDX integration helpers (US8)
//!
//! Provides portable stack packaging, nested support, and snapshot/rollback.

pub mod nested;
pub mod packaging;
pub mod snapshot;

pub use nested::{MountedChild, NestedVhdxManager};
pub use packaging::{VhdxPackage, VhdxPackager};
pub use snapshot::{VhdxSnapshot, VhdxSnapshotManager};
