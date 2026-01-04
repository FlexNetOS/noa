//! Directory Path Constants
//!
//! T073: Define directory constants for NOA structure
//! §3.1: Self-Contained & Autonomous

use std::path::{Path, PathBuf};

/// NOA directory path constants
pub struct NoaPaths;

impl NoaPaths {
    /// Get the root directory path
    pub fn root(noa_root: &Path) -> PathBuf {
        noa_root.to_path_buf()
    }

    /// System directories
    pub fn sys(noa_root: &Path) -> PathBuf {
        noa_root.join("sys")
    }

    pub fn sys_core(noa_root: &Path) -> PathBuf {
        noa_root.join("sys/core")
    }

    pub fn sys_services(noa_root: &Path) -> PathBuf {
        noa_root.join("sys/services")
    }

    pub fn sys_ui(noa_root: &Path) -> PathBuf {
        noa_root.join("sys/ui")
    }

    pub fn sys_digest(noa_root: &Path) -> PathBuf {
        noa_root.join("sys/digest")
    }

    pub fn sys_kernel(noa_root: &Path) -> PathBuf {
        noa_root.join("sys/kernel")
    }

    /// P2P directory
    pub fn p2p(noa_root: &Path) -> PathBuf {
        noa_root.join("p2p")
    }

    /// Optional packages directory
    pub fn opt(noa_root: &Path) -> PathBuf {
        noa_root.join("opt")
    }

    /// Initialization directory
    pub fn init(noa_root: &Path) -> PathBuf {
        noa_root.join("init")
    }

    pub fn init_bootstrap(noa_root: &Path) -> PathBuf {
        noa_root.join("init/bootstrap")
    }

    pub fn init_migrations(noa_root: &Path) -> PathBuf {
        noa_root.join("init/migrations")
    }

    pub fn init_migrations_pg(noa_root: &Path) -> PathBuf {
        noa_root.join("init/migrations/pg")
    }

    pub fn init_seeds(noa_root: &Path) -> PathBuf {
        noa_root.join("init/seeds")
    }

    pub fn init_services(noa_root: &Path) -> PathBuf {
        noa_root.join("init/services")
    }

    /// Containers directory
    pub fn containers(noa_root: &Path) -> PathBuf {
        noa_root.join("containers")
    }

    /// configsuration directory (3-layer: base/semantic/enforcement)
    pub fn configs(noa_root: &Path) -> PathBuf {
        noa_root.join("configss")
    }

    pub fn configs_base(noa_root: &Path) -> PathBuf {
        noa_root.join("configss/base")
    }

    pub fn configs_semantic(noa_root: &Path) -> PathBuf {
        noa_root.join("configss/semantic")
    }

    pub fn configs_enforcement(noa_root: &Path) -> PathBuf {
        noa_root.join("configss/enforcement")
    }

    pub fn configs_schemas(noa_root: &Path) -> PathBuf {
        noa_root.join("configss/base/schemas")
    }

    pub fn configs_templates(noa_root: &Path) -> PathBuf {
        noa_root.join("configss/base/templates")
    }

    /// Binary directory
    pub fn bin(noa_root: &Path) -> PathBuf {
        noa_root.join("bin")
    }

    /// AI directory
    pub fn ai(noa_root: &Path) -> PathBuf {
        noa_root.join("ai")
    }

    /// Providers directory (unified from ai/providers)
    pub fn providers(noa_root: &Path) -> PathBuf {
        noa_root.join("providers")
    }

    #[deprecated(note = "Use providers() instead - ai/providers has been consolidated")]
    pub fn ai_providers(noa_root: &Path) -> PathBuf {
        noa_root.join("providers")
    }

    pub fn ai_shared(noa_root: &Path) -> PathBuf {
        noa_root.join("ai/shared")
    }

    /// Data directory
    pub fn data(noa_root: &Path) -> PathBuf {
        noa_root.join("data")
    }

    pub fn data_memory(noa_root: &Path) -> PathBuf {
        noa_root.join("data/memory")
    }

    pub fn data_knowledge(noa_root: &Path) -> PathBuf {
        noa_root.join("data/knowledge")
    }

    pub fn data_embeddings(noa_root: &Path) -> PathBuf {
        noa_root.join("data/embeddings")
    }

    pub fn data_artifacts(noa_root: &Path) -> PathBuf {
        noa_root.join("data/artifacts")
    }

    pub fn data_modules(noa_root: &Path) -> PathBuf {
        noa_root.join("data/modules")
    }

    pub fn data_state(noa_root: &Path) -> PathBuf {
        noa_root.join("data/state")
    }

    pub fn data_cache(noa_root: &Path) -> PathBuf {
        noa_root.join("data/cache")
    }

    pub fn data_backups(noa_root: &Path) -> PathBuf {
        noa_root.join("data/backups")
    }

    /// Logs directory
    pub fn logs(noa_root: &Path) -> PathBuf {
        noa_root.join("logs")
    }

    /// Temporary directory
    pub fn tmp(noa_root: &Path) -> PathBuf {
        noa_root.join("tmp")
    }

    /// Get all required directory paths
    pub fn all_directories(noa_root: &Path) -> Vec<PathBuf> {
        vec![
            Self::sys(noa_root),
            Self::sys_core(noa_root),
            Self::sys_services(noa_root),
            Self::sys_ui(noa_root),
            Self::sys_digest(noa_root),
            Self::sys_kernel(noa_root),
            Self::p2p(noa_root),
            Self::opt(noa_root),
            Self::init(noa_root),
            Self::init_bootstrap(noa_root),
            Self::init_migrations(noa_root),
            Self::init_migrations_pg(noa_root),
            Self::init_seeds(noa_root),
            Self::init_services(noa_root),
            Self::containers(noa_root),
            Self::configs(noa_root),
            Self::configs_schemas(noa_root),
            Self::configs_templates(noa_root),
            Self::bin(noa_root),
            Self::ai(noa_root),
            Self::ai_providers(noa_root),
            Self::ai_shared(noa_root),
            Self::data(noa_root),
            Self::data_memory(noa_root),
            Self::data_knowledge(noa_root),
            Self::data_embeddings(noa_root),
            Self::data_artifacts(noa_root),
            Self::data_modules(noa_root),
            Self::data_state(noa_root),
            Self::data_cache(noa_root),
            Self::data_backups(noa_root),
            Self::logs(noa_root),
            Self::tmp(noa_root),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_path_generation() {
        let root = Path::new("/tmp/noa");
        assert_eq!(NoaPaths::sys(&root), PathBuf::from("/tmp/noa/sys"));
        assert_eq!(NoaPaths::configs(&root), PathBuf::from("/tmp/noa/configs"));
        assert_eq!(NoaPaths::data(&root), PathBuf::from("/tmp/noa/data"));
    }

    #[test]
    fn test_all_directories() {
        let root = Path::new("/tmp/noa");
        let dirs = NoaPaths::all_directories(&root);
        assert!(!dirs.is_empty());
        assert!(dirs.contains(&NoaPaths::sys(&root)));
        assert!(dirs.contains(&NoaPaths::configs(&root)));
    }
}

