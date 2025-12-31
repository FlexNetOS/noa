use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Returns the canonical projects directory under `$NOA_DATA/apps/noa-ui/projects`.
pub fn get_project_directory() -> Result<PathBuf> {
    let dirs = noa_ui_paths::ensure_noa_ui_dirs()?;
    Ok(dirs.projects)
}

/// Returns the canonical state/config directory under `$NOA_DATA/apps/noa-ui/state`.
pub fn get_config_directory() -> Result<PathBuf> {
    let dirs = noa_ui_paths::ensure_noa_ui_dirs()?;
    Ok(dirs.state)
}

pub fn copy_directory_recursive(src: &Path, dst: &Path) -> Result<()> {
    ensure_directory_exists(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_directory_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

pub fn clean_directory(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_dir_all(path)?;
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}
