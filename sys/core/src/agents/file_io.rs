use crate::agents::base::BaseAgent;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// File operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum FileOperation {
    Read { path: String },
    Write { path: String, content: String },
    Append { path: String, content: String },
    Delete { path: String },
    List { path: String },
    Exists { path: String },
    Mkdir { path: String },
    Copy { from: String, to: String },
    Move { from: String, to: String },
}

/// File operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationResult {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

pub struct FileIOAgent {
    base_path: Option<PathBuf>,
}

impl FileIOAgent {
    pub fn new() -> Self {
        Self { base_path: None }
    }

    pub fn with_base_path(base_path: PathBuf) -> Self {
        Self {
            base_path: Some(base_path),
        }
    }

    /// Execute a file operation
    pub fn execute_operation(&self, operation: FileOperation) -> Result<FileOperationResult> {
        match operation {
            FileOperation::Read { path } => self.read_file(&path),
            FileOperation::Write { path, content } => self.write_file(&path, &content),
            FileOperation::Append { path, content } => self.append_file(&path, &content),
            FileOperation::Delete { path } => self.delete_file(&path),
            FileOperation::List { path } => self.list_directory(&path),
            FileOperation::Exists { path } => self.check_exists(&path),
            FileOperation::Mkdir { path } => self.create_directory(&path),
            FileOperation::Copy { from, to } => self.copy_file(&from, &to),
            FileOperation::Move { from, to } => self.move_file(&from, &to),
        }
    }

    fn resolve_path(&self, path: &str) -> PathBuf {
        let p = PathBuf::from(path);
        if let Some(base) = &self.base_path {
            if p.is_relative() {
                base.join(p)
            } else {
                p
            }
        } else {
            p
        }
    }

    fn read_file(&self, path: &str) -> Result<FileOperationResult> {
        let resolved = self.resolve_path(path);
        match fs::read_to_string(&resolved) {
            Ok(content) => Ok(FileOperationResult {
                success: true,
                data: Some(content),
                error: None,
            }),
            Err(e) => Ok(FileOperationResult {
                success: false,
                data: None,
                error: Some(format!("Failed to read {}: {}", path, e)),
            }),
        }
    }

    fn write_file(&self, path: &str, content: &str) -> Result<FileOperationResult> {
        let resolved = self.resolve_path(path);
        
        // Create parent directories if needed
        if let Some(parent) = resolved.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        match fs::write(&resolved, content) {
            Ok(_) => Ok(FileOperationResult {
                success: true,
                data: Some(format!("Wrote {} bytes to {}", content.len(), path)),
                error: None,
            }),
            Err(e) => Ok(FileOperationResult {
                success: false,
                data: None,
                error: Some(format!("Failed to write {}: {}", path, e)),
            }),
        }
    }

    fn append_file(&self, path: &str, content: &str) -> Result<FileOperationResult> {
        let resolved = self.resolve_path(path);
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&resolved)
        {
            Ok(mut file) => {
                use std::io::Write;
                match file.write_all(content.as_bytes()) {
                    Ok(_) => Ok(FileOperationResult {
                        success: true,
                        data: Some(format!("Appended {} bytes to {}", content.len(), path)),
                        error: None,
                    }),
                    Err(e) => Ok(FileOperationResult {
                        success: false,
                        data: None,
                        error: Some(format!("Failed to append to {}: {}", path, e)),
                    }),
                }
            }
            Err(e) => Ok(FileOperationResult {
                success: false,
                data: None,
                error: Some(format!("Failed to open {}: {}", path, e)),
            }),
        }
    }

    fn delete_file(&self, path: &str) -> Result<FileOperationResult> {
        let resolved = self.resolve_path(path);
        match fs::remove_file(&resolved) {
            Ok(_) => Ok(FileOperationResult {
                success: true,
                data: Some(format!("Deleted {}", path)),
                error: None,
            }),
            Err(e) => Ok(FileOperationResult {
                success: false,
                data: None,
                error: Some(format!("Failed to delete {}: {}", path, e)),
            }),
        }
    }

    fn list_directory(&self, path: &str) -> Result<FileOperationResult> {
        let resolved = self.resolve_path(path);
        match fs::read_dir(&resolved) {
            Ok(entries) => {
                let mut items = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(name) = entry.file_name().to_str() {
                            let is_dir = entry.path().is_dir();
                            items.push(format!("{}{}", name, if is_dir { "/" } else { "" }));
                        }
                    }
                }
                Ok(FileOperationResult {
                    success: true,
                    data: Some(items.join("\n")),
                    error: None,
                })
            }
            Err(e) => Ok(FileOperationResult {
                success: false,
                data: None,
                error: Some(format!("Failed to list {}: {}", path, e)),
            }),
        }
    }

    fn check_exists(&self, path: &str) -> Result<FileOperationResult> {
        let resolved = self.resolve_path(path);
        let exists = resolved.exists();
        Ok(FileOperationResult {
            success: true,
            data: Some(exists.to_string()),
            error: None,
        })
    }

    fn create_directory(&self, path: &str) -> Result<FileOperationResult> {
        let resolved = self.resolve_path(path);
        match fs::create_dir_all(&resolved) {
            Ok(_) => Ok(FileOperationResult {
                success: true,
                data: Some(format!("Created directory {}", path)),
                error: None,
            }),
            Err(e) => Ok(FileOperationResult {
                success: false,
                data: None,
                error: Some(format!("Failed to create directory {}: {}", path, e)),
            }),
        }
    }

    fn copy_file(&self, from: &str, to: &str) -> Result<FileOperationResult> {
        let from_path = self.resolve_path(from);
        let to_path = self.resolve_path(to);
        
        // Create parent directories if needed
        if let Some(parent) = to_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        match fs::copy(&from_path, &to_path) {
            Ok(bytes) => Ok(FileOperationResult {
                success: true,
                data: Some(format!("Copied {} bytes from {} to {}", bytes, from, to)),
                error: None,
            }),
            Err(e) => Ok(FileOperationResult {
                success: false,
                data: None,
                error: Some(format!("Failed to copy {} to {}: {}", from, to, e)),
            }),
        }
    }

    fn move_file(&self, from: &str, to: &str) -> Result<FileOperationResult> {
        let from_path = self.resolve_path(from);
        let to_path = self.resolve_path(to);
        
        // Create parent directories if needed
        if let Some(parent) = to_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        match fs::rename(&from_path, &to_path) {
            Ok(_) => Ok(FileOperationResult {
                success: true,
                data: Some(format!("Moved {} to {}", from, to)),
                error: None,
            }),
            Err(e) => Ok(FileOperationResult {
                success: false,
                data: None,
                error: Some(format!("Failed to move {} to {}: {}", from, to, e)),
            }),
        }
    }
}

impl Default for FileIOAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseAgent for FileIOAgent {
    fn name(&self) -> &str {
        "file-io"
    }

    fn description(&self) -> &str {
        "Handles filesystem operations: read, write, delete, list, copy, move"
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "read".into(),
            "write".into(),
            "append".into(),
            "delete".into(),
            "list".into(),
            "exists".into(),
            "mkdir".into(),
            "copy".into(),
            "move".into(),
        ]
    }

    fn execute(&self, task: &str) -> Result<String> {
        // Try to parse task as JSON operation
        match serde_json::from_str::<FileOperation>(task) {
            Ok(operation) => {
                let result = self.execute_operation(operation)?;
                Ok(serde_json::to_string(&result)?)
            }
            Err(_) => {
                // Fallback: treat as simple read operation
                let result = self.read_file(task)?;
                Ok(serde_json::to_string(&result)?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_io_agent_operations() {
        let agent = FileIOAgent::new();
        
        // Test write
        let write_op = FileOperation::Write {
            path: "test_file.txt".into(),
            content: "Hello, World!".into(),
        };
        let result = agent.execute_operation(write_op).unwrap();
        assert!(result.success);
        
        // Test read
        let read_op = FileOperation::Read {
            path: "test_file.txt".into(),
        };
        let result = agent.execute_operation(read_op).unwrap();
        assert!(result.success);
        assert_eq!(result.data.as_deref(), Some("Hello, World!"));
        
        // Cleanup
        let delete_op = FileOperation::Delete {
            path: "test_file.txt".into(),
        };
        let _ = agent.execute_operation(delete_op);
    }
}

