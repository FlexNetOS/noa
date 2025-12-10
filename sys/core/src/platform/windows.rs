//! Windows-specific platform adaptations
//!
//! T723: Implement Windows-specific adaptations.

use std::path::PathBuf;

use super::PlatformAdaptation;

/// Default adaptation hints for Windows hosts.
pub fn adaptations() -> PlatformAdaptation {
    PlatformAdaptation::windows()
}

/// Resolve %LOCALAPPDATA% or %APPDATA% for per-user data.
pub fn default_data_root() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("APPDATA").map(PathBuf::from))
        .or_else(user_home_dir)
}

/// Resolve %PROGRAMDATA% for shared state if available.
pub fn program_data_root() -> Option<PathBuf> {
    std::env::var_os("PROGRAMDATA").map(PathBuf::from)
}

fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE").map(PathBuf::from)
}
