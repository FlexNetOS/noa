use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoaPathsError {
    #[error("NOA_DATA (or XDG_DATA_HOME) is not set; cannot enforce NOA data containment")]
    NoaDataNotSet,

    #[error("NOA_DATA path is not absolute: {0}")]
    NoaDataNotAbsolute(String),

    #[error("NOA_ROOT is not set; cannot determine NOA root directory")]
    NoaRootNotSet,
}

/// Resolve the NOA root directory.
///
/// Uses `NOA_ROOT` environment variable.
pub fn noa_root() -> Result<PathBuf, NoaPathsError> {
    let raw = std::env::var("NOA_ROOT")
        .ok()
        .ok_or(NoaPathsError::NoaRootNotSet)?;

    Ok(PathBuf::from(&raw))
}

/// Resolve the NOA data directory.
///
/// We prefer `NOA_DATA`, but fall back to `XDG_DATA_HOME` since NOA sets both.
pub fn noa_data_dir() -> Result<PathBuf, NoaPathsError> {
    let raw = std::env::var("NOA_DATA")
        .ok()
        .or_else(|| std::env::var("XDG_DATA_HOME").ok())
        .ok_or(NoaPathsError::NoaDataNotSet)?;

    let path = PathBuf::from(&raw);
    if !path.is_absolute() {
        return Err(NoaPathsError::NoaDataNotAbsolute(raw));
    }

    Ok(path)
}

/// The canonical NOA UI app root: `$NOA_DATA/apps/noa-ui`.
pub fn noa_ui_root() -> Result<PathBuf, NoaPathsError> {
    Ok(noa_data_dir()?.join("apps").join("noa-ui"))
}

pub fn ensure_noa_ui_dirs() -> Result<NoaUiDirs, std::io::Error> {
    let root = noa_ui_root().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let state = root.join("state");
    let projects = root.join("projects");
    let db = root.join("db");
    let p2p = root.join("p2p");
    let sync = root.join("sync");
    let releases = root.join("releases");
    let cache = root.join("cache");
    let tmp = root.join("tmp");

    for dir in [&root, &state, &projects, &db, &p2p, &sync, &releases, &cache, &tmp] {
        std::fs::create_dir_all(dir)?;
    }

    Ok(NoaUiDirs {
        root,
        state,
        projects,
        db,
        p2p,
        sync,
        releases,
        cache,
        tmp,
    })
}

#[derive(Debug, Clone)]
pub struct NoaUiDirs {
    pub root: PathBuf,
    pub state: PathBuf,
    pub projects: PathBuf,
    pub db: PathBuf,
    pub p2p: PathBuf,
    pub sync: PathBuf,
    pub releases: PathBuf,
    pub cache: PathBuf,
    pub tmp: PathBuf,
}

pub fn join_under(base: &Path, rel: impl AsRef<Path>) -> PathBuf {
    base.join(rel)
}
