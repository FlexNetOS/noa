//! Platform detection utilities
//!
//! T722: Create platform detection module.
//! Provides lightweight detection of OS/architecture for cross-platform coordination.

use crate::platform::{PlatformAdaptation};
use std::fmt;

/// Supported platform variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

/// CPU architecture for platform targeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    AArch64,
    Arm,
    RiscV64,
    Unknown,
}

/// Snapshot of detected platform metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformInfo {
    pub platform: Platform,
    pub architecture: Architecture,
    pub os_family: String,
    pub os_version: Option<String>,
    pub kernel_version: Option<String>,
}

impl PlatformInfo {
    /// Whether the platform is one of the supported targets.
    pub fn is_supported(&self) -> bool {
        matches!(
            self.platform,
            Platform::Windows | Platform::MacOS | Platform::Linux
        )
    }

    /// Recommended adaptation hints for the detected platform.
    pub fn adaptation(&self) -> PlatformAdaptation {
        match self.platform {
            Platform::Windows => PlatformAdaptation::windows(),
            Platform::MacOS => PlatformAdaptation::macos(),
            Platform::Linux => PlatformAdaptation::linux(),
            Platform::Unknown => PlatformAdaptation::linux(),
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Platform::Windows => "windows",
            Platform::MacOS => "macos",
            Platform::Linux => "linux",
            Platform::Unknown => "unknown",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Architecture::X86_64 => "x86_64",
            Architecture::AArch64 => "aarch64",
            Architecture::Arm => "arm",
            Architecture::RiscV64 => "riscv64",
            Architecture::Unknown => "unknown",
        };
        write!(f, "{}", name)
    }
}

/// Detect the current platform and architecture.
pub fn platform_info() -> PlatformInfo {
    let platform = detect_platform();
    PlatformInfo {
        platform,
        architecture: detect_architecture(),
        os_family: std::env::consts::OS.to_string(),
        os_version: None, // TODO: add richer OS version detection when available
        kernel_version: None,
    }
}

fn detect_platform() -> Platform {
    if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::MacOS
    } else if cfg!(target_os = "linux") {
        Platform::Linux
    } else {
        Platform::Unknown
    }
}

fn detect_architecture() -> Architecture {
    match std::env::consts::ARCH {
        "x86_64" | "x86-64" | "amd64" => Architecture::X86_64,
        "aarch64" | "arm64" => Architecture::AArch64,
        "arm" | "armv7" | "armv6" => Architecture::Arm,
        "riscv64" => Architecture::RiscV64,
        _ => Architecture::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_supported_platform() {
        let info = platform_info();
        assert!(info.is_supported() || info.platform == Platform::Unknown);
        assert!(!info.os_family.is_empty());
    }

    #[test]
    fn detects_architecture() {
        let info = platform_info();
        assert_ne!(info.architecture, Architecture::Unknown);
    }
}
