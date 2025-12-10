//! Module abstraction (Phase 20)
//! Provides registry, CAS, lifecycle management, and CLI helpers.

pub mod cas;
pub mod lifecycle;
pub mod loader;
pub mod registry;
pub mod resolver;
pub mod types;
pub mod verify;

pub use cas::ContentAddressableStore;
pub use registry::ModuleRegistry;
pub use types::{ModuleLifecycleState, ModuleMetadata, ModuleType};
