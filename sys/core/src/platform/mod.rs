//! Cross-Platform Platform Utilities
//!
//! Phase 13 (US9): Cross-platform deployment scaffolding.
//! Provides platform detection, OS-specific adaptations, path resolution, and capability helpers.

pub mod capabilities;
pub mod detect;
pub mod linux;
pub mod macos;
pub mod paths;
pub mod windows;

pub use capabilities::{CapabilitySnapshot, HardwareTier};
pub use detect::{platform_info, Architecture, Platform, PlatformInfo};
pub use paths::{PlatformPaths, RootedPaths};

/// Common adaptation hints for each supported platform.
#[derive(Debug, Clone)]
pub struct PlatformAdaptation {
    pub default_shell: String,
    pub path_separator: char,
    pub env_path_separator: char,
    pub supports_symlinks: bool,
    pub case_sensitive_paths: bool,
}

impl PlatformAdaptation {
    pub fn windows() -> Self {
        Self {
            default_shell: "pwsh.exe".to_string(),
            path_separator: '\\',
            env_path_separator: ';',
            supports_symlinks: true,
            case_sensitive_paths: false,
        }
    }

    pub fn macos() -> Self {
        Self {
            default_shell: "/bin/zsh".to_string(),
            path_separator: '/',
            env_path_separator: ':',
            supports_symlinks: true,
            case_sensitive_paths: true,
        }
    }

    pub fn linux() -> Self {
        Self {
            default_shell: "/bin/bash".to_string(),
            path_separator: '/',
            env_path_separator: ':',
            supports_symlinks: true,
            case_sensitive_paths: true,
        }
    }
}
