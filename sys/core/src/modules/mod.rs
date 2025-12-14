//! Module abstraction (Phase 20)
//! Provides registry, CAS, lifecycle management, and CLI helpers.

pub mod types;
pub mod registry;
pub mod cas;
pub mod lifecycle;
pub mod loader;
pub mod verify;
pub mod resolver;

pub use types::{ModuleMetadata, ModuleType, ModuleLifecycleState};
pub use registry::ModuleRegistry;
pub use cas::ContentAddressableStore;
