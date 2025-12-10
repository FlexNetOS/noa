//! Cross-platform file helpers for kernel independence.
//!
//! Provides safe wrappers around common file operations with normalization so
//! callers do not need to worry about platform-specific path handling.

use crate::error::{NoaError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// File system utilities.
#[derive(Debug, Default)]
pub struct FileOps;

/// Lightweight file metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStat {
    /// File size in bytes.
    pub size: u64,
    /// Whether the path points to a directory.
    pub is_dir: bool,
}

impl FileOps {
    /// Ensure a directory exists, creating it if necessary.
    pub fn ensure_dir(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).map_err(NoaError::from)
    }

    /// Read a UTF-8 file into a string.
    pub fn read_to_string(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).map_err(NoaError::from)
    }

    /// Write a string to a file, creating parent directories as needed.
    pub fn write_string(&self, path: &Path, contents: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(NoaError::from)?;
        }
        fs::write(path, contents).map_err(NoaError::from)
    }

    /// Get lightweight file metadata.
    pub fn stat(&self, path: &Path) -> Result<FileStat> {
        let metadata = fs::metadata(path).map_err(NoaError::from)?;
        Ok(FileStat {
            size: metadata.len(),
            is_dir: metadata.is_dir(),
        })
    }

    /// Normalize a path by resolving `.`/`..` components when possible.
    pub fn normalize(&self, path: &Path) -> PathBuf {
        fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn writes_and_reads_files() {
        let ops = FileOps::default();
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nested").join("file.txt");

        ops.write_string(&file_path, "hello").unwrap();
        let content = ops.read_to_string(&file_path).unwrap();
        assert_eq!(content, "hello");

        let stat = ops.stat(&file_path).unwrap();
        assert_eq!(stat.is_dir, false);
        assert_eq!(stat.size, 5);
    }
}
