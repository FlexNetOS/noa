use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Read a JSON file into a strongly-typed value.
pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

/// Write a value to a JSON file using pretty formatting.
pub fn write_json_pretty<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let text = serde_json::to_string_pretty(value)?;
    fs::write(path, text)?;
    Ok(())
}
