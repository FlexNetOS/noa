//! Key rotation mechanism (FR-108).

use crate::auth::device_identity::DeviceIdentity;
use crate::error::Result;
use crate::init::paths::NoaPaths;
use chrono::Utc;
use std::fs;
use std::path::Path;

fn identity_path(noa_root: &Path) -> std::path::PathBuf {
    NoaPaths::data_state(noa_root).join("device-identity.json")
}

/// Rotate device keys by archiving the current identity and issuing a new one.
pub fn rotate_keys(noa_root: &Path) -> Result<DeviceIdentity> {
    let path = identity_path(noa_root);
    if path.exists() {
        let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
        let backup = NoaPaths::data_state(noa_root)
            .join(format!("device-identity-{}.bak.json", timestamp));
        fs::rename(&path, backup)?;
    }

    DeviceIdentity::generate(noa_root)
}
