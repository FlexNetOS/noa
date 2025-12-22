use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::config::NoaConfig;
use crate::db::{ConnectionPool, PoolConfig};
use crate::error::Result;

#[derive(Clone)]
pub struct CliContext {
    pub noa_root: PathBuf,
    pub config_path: PathBuf,
    pub config: Arc<NoaConfig>,
    pub db: Arc<ConnectionPool>,
}

impl CliContext {
    pub fn new(noa_root: PathBuf, config_path: PathBuf, config: NoaConfig) -> Result<Self> {
        let db_path = PathBuf::from(&config.database.path);
        let db = ConnectionPool::new(&db_path, PoolConfig::default())?;

        Ok(Self {
            noa_root,
            config_path,
            config: Arc::new(config),
            db: Arc::new(db),
        })
    }

    pub fn from_config_file(noa_root: PathBuf, _config_path: &Path) -> Result<Self> {
        // Current config loader resolves from NOA root.
        let config = NoaConfig::load_from_root(&noa_root)?;
        Self::new(noa_root, _config_path.to_path_buf(), config)
    }
}
