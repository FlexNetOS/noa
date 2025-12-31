use std::path::{Path, PathBuf};
use anyhow::Result;

pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_project_directory() -> Result<PathBuf> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    
    let projects_dir = home_dir.join(".rust-lovable").join("projects");
    ensure_directory_exists(&projects_dir)?;
    
    Ok(projects_dir)
}

pub fn get_config_directory() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    
    let rust_lovable_dir = config_dir.join("rust-lovable");
    ensure_directory_exists(&rust_lovable_dir)?;
    
    Ok(rust_lovable_dir)
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