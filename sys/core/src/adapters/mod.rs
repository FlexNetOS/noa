//! Secondary layer adapters and toggle framework.
//!
//! Provides lightweight adapter implementations for external surfaces such as
//! Docker, GitHub, and MinIO along with a toggle registry to enable or disable
//! adapters at runtime.

pub mod docker;
pub mod github;
pub mod minio;
pub mod toggle;

pub use docker::DockerAdapter;
pub use github::GithubAdapter;
pub use minio::MinioAdapter;
pub use toggle::{Adapter, AdapterHealth, AdapterRegistry, AdapterStatus};
