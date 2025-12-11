//! Shared memory persistence helpers

use crate::error::Result;
use crate::providers::shared_memory::SharedProviderMemory;
use std::fs;
use std::path::Path;

/// Persist shared contexts to a JSON file for auditing.
pub fn persist(memory: &SharedProviderMemory, path: &Path) -> Result<()> {
    let data = serde_json::to_string_pretty(&memory.all())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, data)?;
    Ok(())
}
