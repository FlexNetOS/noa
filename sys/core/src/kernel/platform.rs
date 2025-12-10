//! Platform detection helpers.
//!
//! Collects basic platform metadata in a kernel-agnostic way to support
//! isolation-aware runtime decisions.

use std::path::Path;

/// Platform metadata captured at startup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformInfo {
    /// Operating system identifier (windows, linux, macos, etc.)
    pub os: String,
    /// CPU architecture identifier (x86_64, aarch64, etc.)
    pub arch: String,
    /// Hostname if available.
    pub hostname: String,
    /// Whether the process appears to be running inside a container.
    pub in_container: bool,
}

impl PlatformInfo {
    /// Detect platform information using best-effort heuristics.
    pub fn detect() -> Self {
        let os = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();
        let hostname = Self::detect_hostname();
        let in_container = Self::detect_container();

        Self {
            os,
            arch,
            hostname,
            in_container,
        }
    }

    fn detect_hostname() -> String {
        if let Ok(host) = std::env::var("COMPUTERNAME") {
            return host;
        }
        if let Ok(host) = std::env::var("HOSTNAME") {
            return host;
        }
        "unknown".to_string()
    }

    fn detect_container() -> bool {
        // Linux-style hints
        if Path::new("/.dockerenv").exists() {
            return true;
        }

        if let Ok(cgroup) = std::fs::read_to_string("/proc/1/cgroup") {
            if cgroup.contains("docker") || cgroup.contains("kubepods") {
                return true;
            }
        }

        false
    }
}

impl Default for PlatformInfo {
    fn default() -> Self {
        Self::detect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_platform_basics() {
        let info = PlatformInfo::detect();
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
    }
}
