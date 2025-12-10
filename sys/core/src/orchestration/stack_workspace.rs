//! Stack workspace utilities (Phase 9 - T279)
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct StackWorkspace {
    pub root: PathBuf,
}

impl StackWorkspace {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn in_dir(&self) -> PathBuf {
        self.root.join("in")
    }

    pub fn work_dir(&self) -> PathBuf {
        self.root.join("work")
    }

    pub fn out_dir(&self) -> PathBuf {
        self.root.join("out")
    }

    pub fn logs_dir(&self) -> PathBuf {
        self.root.join("logs")
    }
}
