//! Cross-platform path resolver
//!
//! T726: Create cross-platform path resolver.
//! Provides consistent directories for config, data, cache, logs, and runtime artifacts.

use crate::error::Result;
use crate::platform::detect::{platform_info, Platform, PlatformInfo};
use std::path::{Path, PathBuf};

/// Normalized set of important NOA paths.
#[derive(Debug, Clone)]
pub struct PlatformPaths {
    pub root: PathBuf,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub log_dir: PathBuf,
    pub runtime_dir: PathBuf,
}

impl PlatformPaths {
    /// Build a path set from an explicit NOA root.
    pub fn from_root(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref().to_path_buf();
        Self {
            config_dir: root.join("config"),
            data_dir: root.join("data"),
            cache_dir: root.join("data/cache"),
            log_dir: root.join("logs"),
            runtime_dir: root.join("tmp/runtime"),
            root,
        }
    }

    /// Resolve paths for the current platform using NOA_ROOT when provided.
    pub fn resolve(noa_root: Option<impl AsRef<Path>>) -> Self {
        match noa_root {
            Some(root) => Self::from_root(root),
            None => Self::default_for(platform_info()),
        }
    }

    /// Resolve platform-aware defaults.
    pub fn default_for(info: PlatformInfo) -> Self {
        let base = resolve_default_root(&info);
        Self::from_root(base)
    }

    /// Ensure all directories exist on disk.
    pub fn ensure(&self) -> Result<()> {
        for path in [
            &self.root,
            &self.config_dir,
            &self.data_dir,
            &self.cache_dir,
            &self.log_dir,
            &self.runtime_dir,
        ] {
            std::fs::create_dir_all(path)?;
        }
        Ok(())
    }
}

/// Convenience wrapper for consumers that want typed access to rooted paths.
#[derive(Debug, Clone)]
pub struct RootedPaths {
    pub platform: PlatformInfo,
    pub paths: PlatformPaths,
}

impl RootedPaths {
    pub fn new(noa_root: Option<impl AsRef<Path>>) -> Self {
        let platform = platform_info();
        let paths = PlatformPaths::resolve(noa_root);
        Self { platform, paths }
    }
}

fn resolve_default_root(info: &PlatformInfo) -> PathBuf {
    let base = match info.platform {
        Platform::Windows => super::windows::default_data_root(),
        Platform::MacOS => super::macos::default_data_root(),
        Platform::Linux => super::linux::default_data_root(),
        Platform::Unknown => None,
    };

    if let Some(root) = base {
        return root.join("noa");
    }

    home_dir().map(|home| home.join(".noa")).unwrap_or_else(|| PathBuf::from("noa"))
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_paths_from_root() {
        let root = PathBuf::from("/tmp/noa");
        let paths = PlatformPaths::from_root(&root);
        assert_eq!(paths.config_dir, root.join("config"));
        assert_eq!(paths.runtime_dir, root.join("tmp/runtime"));
    }

    #[test]
    fn resolves_platform_paths() {
        let resolved = PlatformPaths::resolve(None);
        // Ensure we produce non-empty defaults even without NOA_ROOT
        assert!(!resolved.root.as_os_str().is_empty());
        assert!(resolved.config_dir.ends_with("config"));
    }
}
