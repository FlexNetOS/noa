//! Initialization Service
//!
//! T085-T087: Implement InitService with full initialization flow
//! §3.1: Self-Contained & Autonomous
//! §3.6: Security & Privacy

use crate::error::{NoaError, Result};
use crate::init::{configsGenerator, DatabaseInitializer, DirectoryStructure, NoaPaths};
use std::path::Path;
use tracing::{info, warn};

/// Initialization service
pub struct InitService;

/// Tracks what was created during initialization for cleanup on failure
#[derive(Debug, Default)]
pub(crate) struct InitState {
    directories_created: Vec<std::path::PathBuf>,
    configss_created: Vec<std::path::PathBuf>,
    database_created: bool,
    marker_file_created: bool,
}

impl InitService {
    /// Perform full NOA initialization with cleanup on failure
    pub async fn initialize(noa_root: &Path, force: bool) -> Result<InitResult> {
        info!(root = %noa_root.display(), force = force, "Starting NOA initialization");

        let mut state = InitState::default();
        let mut result = InitResult {
            directories_created: 0,
            configss_generated: 0,
            database_initialized: false,
            binary_paths_registered: 0,
            errors: Vec::new(),
        };

        // Step 1: Create directory structure
        match DirectoryStructure::create_all(noa_root, force) {
            Ok(_) => {
                let dirs = NoaPaths::all_directories(noa_root);
                // Track created directories for cleanup
                for dir in &dirs {
                    if dir.exists() {
                        state.directories_created.push(dir.clone());
                    }
                }
                result.directories_created = dirs.len();
                info!("Directory structure created");
            }
            Err(e) => {
                result.errors.push(format!("Directory creation failed: {}", e));
                warn!("Directory creation failed: {}", e);
                // Cleanup on failure
                Self::cleanup(&state, noa_root);
                return Ok(result);
            }
        }

        // Step 2: Check and set permissions
        match Self::check_permissions(noa_root) {
            Ok(_) => {
                info!("Permissions verified");
            }
            Err(e) => {
                result.errors.push(format!("Permission check failed: {}", e));
                warn!("Permission check failed: {}", e);
            }
        }

        // Step 3: Generate default configsurations
        match configsGenerator::generate_all(noa_root) {
            Ok(_) => {
                // Track created configs files
                let configss = vec![
                    "ai-providers.json",
                    "noa-server.json",
                    "features.json",
                    "models.json",
                ];
                for configs in configss {
                    let path = NoaPaths::configs(noa_root).join(configs);
                    if path.exists() {
                        state.configss_created.push(path);
                    }
                }
                result.configss_generated = 4; // ai-providers, noa-server, features, models
                info!("Default configsurations generated");
            }
            Err(e) => {
                result.errors.push(format!("configs generation failed: {}", e));
                warn!("configs generation failed: {}", e);
                // Cleanup on failure
                Self::cleanup(&state, noa_root);
                return Ok(result);
            }
        }

        // Step 4: Initialize database
        match DatabaseInitializer::initialize(noa_root, force) {
            Ok(_) => {
                state.database_created = true;
                result.database_initialized = true;
                info!("Database initialized");
            }
            Err(e) => {
                result.errors.push(format!("Database initialization failed: {}", e));
                warn!("Database initialization failed: {}", e);
                // Cleanup on failure
                Self::cleanup(&state, noa_root);
                return Ok(result);
            }
        }

        // Step 5: Register binary paths
        match Self::register_binary_paths(noa_root) {
            Ok(count) => {
                result.binary_paths_registered = count;
                info!(count = count, "Binary paths registered");
            }
            Err(e) => {
                result.errors.push(format!("Binary path registration failed: {}", e));
                warn!("Binary path registration failed: {}", e);
            }
        }

        if result.errors.is_empty() {
            info!("NOA initialization completed successfully");
        } else {
            warn!(error_count = result.errors.len(), "Initialization completed with errors");
            // Cleanup on partial failure if requested
            if !result.errors.is_empty() {
                Self::cleanup(&state, noa_root);
            }
        }

        Ok(result)
    }

    /// Cleanup created resources on initialization failure
    /// VER007: Partial init failure cleans up created directories
    pub(crate) fn cleanup(state: &InitState, noa_root: &Path) {
        info!("Cleaning up partial initialization");

        // Remove created configs files
        for configs_path in &state.configss_created {
            if let Err(e) = std::fs::remove_file(configs_path) {
                warn!(path = %configs_path.display(), error = %e, "Failed to remove configs file during cleanup");
            } else {
                info!(path = %configs_path.display(), "Removed configs file");
            }
        }

        // Remove database if created
        if state.database_created {
            let db_path = NoaPaths::data(noa_root).join("noa.db");
            if let Err(e) = std::fs::remove_file(&db_path) {
                warn!(path = %db_path.display(), error = %e, "Failed to remove database during cleanup");
            } else {
                info!(path = %db_path.display(), "Removed database file");
            }
        }

        // Remove marker file if created
        if state.marker_file_created {
            let marker_path = noa_root.join(".noa-initialized");
            if let Err(e) = std::fs::remove_file(&marker_path) {
                warn!(path = %marker_path.display(), error = %e, "Failed to remove marker file during cleanup");
            } else {
                info!(path = %marker_path.display(), "Removed marker file");
            }
        }

        // Remove created directories (in reverse order to handle nested dirs)
        // Only remove if they're empty (safe cleanup)
        for dir in state.directories_created.iter().rev() {
            if dir.exists() {
                // Only remove if empty (safe cleanup)
                if let Ok(entries) = std::fs::read_dir(dir) {
                    if entries.count() == 0 {
                        if let Err(e) = std::fs::remove_dir(dir) {
                            warn!(path = %dir.display(), error = %e, "Failed to remove directory during cleanup");
                        } else {
                            info!(path = %dir.display(), "Removed empty directory");
                        }
                    } else {
                        tracing::debug!(path = %dir.display(), "Skipping non-empty directory during cleanup");
                    }
                }
            }
        }

        info!("Cleanup completed");
    }

    /// Check and set directory permissions
    fn check_permissions(noa_root: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            use std::fs;
            use std::os::unix::fs::PermissionsExt;

            // Check that we can write to the root
            if !noa_root.exists() {
                return Err(NoaError::Internal {
                    message: "NOA root directory does not exist".to_string(),
                    source: None,
                });
            }

            // Set permissions on key directories
            let key_dirs = vec![
                NoaPaths::bin(noa_root),
                NoaPaths::configs(noa_root),
                NoaPaths::data(noa_root),
            ];

            for dir in key_dirs {
                if dir.exists() {
                    let mut perms = fs::metadata(&dir)?.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&dir, perms)?;
                }
            }
        }

        #[cfg(windows)]
        {
            // Windows permissions are handled differently
            // Just verify we can write
            if !noa_root.exists() {
                return Err(NoaError::Internal {
                    message: "NOA root directory does not exist".to_string(),
                    source: None,
                });
            }
        }

        Ok(())
    }

    /// Register binary paths in PATH
    fn register_binary_paths(noa_root: &Path) -> Result<usize> {
        let bin_dir = NoaPaths::bin(noa_root);

        if !bin_dir.exists() {
            return Ok(0);
        }

        // TODO: Actually register paths in system PATH or create wrapper scripts
        // For now, just verify bin directory exists
        let count = std::fs::read_dir(&bin_dir)
            .map_err(|e| NoaError::Io(e))?
            .count();

        info!(bin_dir = %bin_dir.display(), count = count, "Binary paths available");
        Ok(count)
    }

    /// Verify initialization
    pub fn verify(noa_root: &Path) -> Result<VerificationResult> {
        let mut result = VerificationResult {
            directories_ok: false,
            configss_ok: false,
            database_ok: false,
            errors: Vec::new(),
        };

        // Verify directories
        match DirectoryStructure::verify(noa_root) {
            Ok(missing) => {
                if missing.is_empty() {
                    result.directories_ok = true;
                } else {
                    result.errors.push(format!("Missing directories: {:?}", missing));
                }
            }
            Err(e) => {
                result.errors.push(format!("Directory verification failed: {}", e));
            }
        }

        // Verify configss
        let configss = vec![
            "ai-providers.json",
            "noa-server.json",
            "features.json",
            "models.json",
        ];

        let mut missing_configss = Vec::new();
        for configs in configss {
            let path = NoaPaths::configs(noa_root).join(configs);
            if !path.exists() {
                missing_configss.push(configs);
            }
        }

        if missing_configss.is_empty() {
            result.configss_ok = true;
        } else {
            result.errors.push(format!("Missing configss: {:?}", missing_configss));
        }

        // Verify database
        match DatabaseInitializer::verify(noa_root) {
            Ok(ok) => {
                result.database_ok = ok;
                if !ok {
                    result.errors.push("Database not operational".to_string());
                }
            }
            Err(e) => {
                result.errors.push(format!("Database verification failed: {}", e));
            }
        }

        Ok(result)
    }
}

/// Initialization result
#[derive(Debug, Clone)]
pub struct InitResult {
    pub directories_created: usize,
    pub configss_generated: usize,
    pub database_initialized: bool,
    pub binary_paths_registered: usize,
    pub errors: Vec<String>,
}

/// Verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub directories_ok: bool,
    pub configss_ok: bool,
    pub database_ok: bool,
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_init_service() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        let result = InitService::initialize(root, false).await.unwrap();
        assert!(result.directories_created > 0);
        assert!(result.configss_generated > 0);
        assert!(result.database_initialized);
    }
}

