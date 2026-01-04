use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::configs::Noaconfigs;
use crate::db::{ConnectionPool, Poolconfigs};
use crate::error::Result;

#[derive(Clone)]
pub struct CliContext {
    pub noa_root: PathBuf,
    pub configs_path: PathBuf,
    pub configs: Arc<Noaconfigs>,
    pub db: Arc<ConnectionPool>,
}

impl CliContext {
    pub fn new(noa_root: PathBuf, configs_path: PathBuf, configs: Noaconfigs) -> Result<Self> {
        let db_path = PathBuf::from(&configs.database.path);
        let db = ConnectionPool::new(&db_path, Poolconfigs::default())?;

        Ok(Self {
            noa_root,
            configs_path,
            configs: Arc::new(configs),
            db: Arc::new(db),
        })
    }

    pub fn from_configs_file(noa_root: PathBuf, _configs_path: &Path) -> Result<Self> {
        // Current configs loader resolves from NOA root.
        let configs = Noaconfigs::load_from_root(&noa_root)?;
        Self::new(noa_root, _configs_path.to_path_buf(), configs)
    }
}
