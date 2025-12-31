pub mod app;
pub mod components;
pub mod core;
pub mod utils;

// Advanced modules - feature gated
#[cfg(feature = "compression")]
pub mod compression;

#[cfg(feature = "ml-devops")]
pub mod config;

#[cfg(feature = "ml-devops")]
pub mod resources;

#[cfg(feature = "ml-devops")]
pub mod metadata;

#[cfg(feature = "ml-devops")]
pub mod ml_devops;

#[cfg(feature = "ml-devops")]
pub mod vibe_coding;

// Re-export main app
pub use app::App;