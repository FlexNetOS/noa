//! Device revocation flow (FR-107).

use crate::error::Result;
use crate::init::paths::NoaPaths;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct RevocationList {
    revoked: HashSet<String>,
}

fn state_path(noa_root: &Path) -> std::path::PathBuf {
    NoaPaths::data_state(noa_root).join("revoked-devices.json")
}

fn load(noa_root: &Path) -> Result<RevocationList> {
    let path = state_path(noa_root);
    if path.exists() {
        let data = fs::read_to_string(path)?;
        let list: RevocationList = serde_json::from_str(&data)?;
        Ok(list)
    } else {
        Ok(RevocationList {
            revoked: HashSet::new(),
        })
    }
}

fn save(noa_root: &Path, list: &RevocationList) -> Result<()> {
    fs::create_dir_all(NoaPaths::data_state(noa_root))?;
    let path = state_path(noa_root);
    let data = serde_json::to_string_pretty(list)?;
    fs::write(path, data)?;
    Ok(())
}

/// Mark a device as revoked and persist the state.
pub fn revoke_device(noa_root: &Path, device_id: &str) -> Result<()> {
    let mut list = load(noa_root)?;
    list.revoked.insert(device_id.to_string());
    save(noa_root, &list)
}

/// Check whether a device id has been revoked.
pub fn is_revoked(noa_root: &Path, device_id: &str) -> Result<bool> {
    let list = load(noa_root)?;
    Ok(list.revoked.contains(device_id))
}

/// Return all revoked device ids.
pub fn list_revoked(noa_root: &Path) -> Result<Vec<String>> {
    let list = load(noa_root)?;
    Ok(list.revoked.into_iter().collect())
}
