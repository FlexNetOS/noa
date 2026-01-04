//! Directory Structure Creation
//!
//! T072, T074-T078: Implement directory structure creation
//! §3.1: Self-Contained & Autonomous

use crate::error::Result;
use crate::init::paths::NoaPaths;
use std::fs;
use std::path::Path;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use tracing::{debug, info, warn};

/// Directory structure creator
pub struct DirectoryStructure;

impl DirectoryStructure {
    /// Create all NOA directories
    pub fn create_all(noa_root: &Path, force: bool) -> Result<()> {
        info!(root = %noa_root.display(), "Creating NOA directory structure");

        let directories = NoaPaths::all_directories(noa_root);

        for dir in &directories {
            Self::create_directory(dir, force)?;
        }

        // Create sys/ subdirectories
        Self::create_sys_subdirectories(noa_root, force)?;

        info!("Directory structure created successfully");
        Ok(())
    }

    /// Create sys/core/ subdirectory structure
    pub fn create_sys_core(noa_root: &Path, force: bool) -> Result<()> {
        let core_dirs = vec![
            "sys/core/src",
            "sys/core/crates",
            "sys/core/data",
            "sys/core/target",
        ];

        for dir in core_dirs {
            let path = noa_root.join(dir);
            Self::create_directory(&path, force)?;
        }

        Ok(())
    }

    /// Create sys/services/ subdirectory structure
    pub fn create_sys_services(noa_root: &Path, force: bool) -> Result<()> {
        let service_dirs = vec![
            "sys/services/init",
            "sys/services/api",
            "sys/services/agent",
        ];

        for dir in service_dirs {
            let path = noa_root.join(dir);
            Self::create_directory(&path, force)?;
        }

        Ok(())
    }

    /// Create sys/ui/ subdirectory structure
    pub fn create_sys_ui(noa_root: &Path, force: bool) -> Result<()> {
        let ui_dirs = vec![
            "sys/ui/src",
            "sys/ui/public",
            "sys/ui/.next",
        ];

        for dir in ui_dirs {
            let path = noa_root.join(dir);
            Self::create_directory(&path, force)?;
        }

        Ok(())
    }

    /// Create sys/digest/ subdirectory structure
    pub fn create_sys_digest(noa_root: &Path, force: bool) -> Result<()> {
        let digest_dirs = vec![
            "sys/digest/src",
            "sys/digest/tests",
        ];

        for dir in digest_dirs {
            let path = noa_root.join(dir);
            Self::create_directory(&path, force)?;
        }

        Ok(())
    }

    /// Create sys/kernel/ subdirectory structure
    pub fn create_sys_kernel(noa_root: &Path, force: bool) -> Result<()> {
        let kernel_dirs = vec![
            "sys/kernel/configs",
            "sys/kernel/images",
            "sys/kernel/modules",
            "sys/kernel/params",
            "sys/kernel/linux",
            "sys/kernel/windows",
            "sys/kernel/macos",
        ];

        for dir in kernel_dirs {
            let path = noa_root.join(dir);
            Self::create_directory(&path, force)?;
        }

        Ok(())
    }

    /// Create all sys/ subdirectories
    fn create_sys_subdirectories(noa_root: &Path, force: bool) -> Result<()> {
        Self::create_sys_core(noa_root, force)?;
        Self::create_sys_services(noa_root, force)?;
        Self::create_sys_ui(noa_root, force)?;
        Self::create_sys_digest(noa_root, force)?;
        Self::create_sys_kernel(noa_root, force)?;
        Ok(())
    }

    /// Create a single directory with proper permissions
    fn create_directory(path: &Path, force: bool) -> Result<()> {
        if path.exists() {
            if force {
                debug!(path = %path.display(), "Directory exists, force mode enabled");
            } else {
                debug!(path = %path.display(), "Directory already exists");
                return Ok(());
            }
        } else {
            fs::create_dir_all(path)?;

            // Set permissions (755 for directories)
            #[cfg(unix)]
            {
                let mut perms = fs::metadata(path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(path, perms)?;
            }

            info!(path = %path.display(), "Created directory");
        }

        Ok(())
    }

    /// Verify directory structure
    pub fn verify(noa_root: &Path) -> Result<Vec<String>> {
        let mut missing = Vec::new();
        let directories = NoaPaths::all_directories(noa_root);

        for dir in &directories {
            if !dir.exists() {
                missing.push(dir.display().to_string());
            }
        }

        if missing.is_empty() {
            info!("All directories verified");
        } else {
            warn!(missing_count = missing.len(), "Some directories are missing");
        }

        Ok(missing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_directory_structure() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        DirectoryStructure::create_all(root, false).unwrap();

        // Verify some directories were created
        assert!(NoaPaths::sys(root).exists());
        assert!(NoaPaths::configs(root).exists());
        assert!(NoaPaths::data(root).exists());
    }

    #[test]
    fn test_create_sys_subdirectories() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        DirectoryStructure::create_sys_core(root, false).unwrap();
        assert!(NoaPaths::sys_core(root).join("src").exists());
    }
}

