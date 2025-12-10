//! Linux-specific platform adaptations
//!
//! T725: Implement Linux-specific adaptations.

use std::path::PathBuf;

use super::PlatformAdaptation;

/// Default adaptation hints for Linux hosts.
pub fn adaptations() -> PlatformAdaptation {
    PlatformAdaptation::linux()
}

/// Determine XDG-compliant data root.
pub fn default_data_root() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
        return Some(PathBuf::from(xdg));
    }
    user_home_dir().map(|home| home.join(".local/share"))
}

fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}
