//! macOS-specific platform adaptations
//!
//! T724: Implement macOS-specific adaptations.

use std::path::PathBuf;

use super::PlatformAdaptation;

/// Default adaptation hints for macOS hosts.
pub fn adaptations() -> PlatformAdaptation {
    PlatformAdaptation::macos()
}

/// Default per-user data root: ~/Library/Application Support
pub fn default_data_root() -> Option<PathBuf> {
    user_home_dir().map(|home| home.join("Library/Application Support"))
}

fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}
