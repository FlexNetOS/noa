//! Virtual package detection and registry integration.
//!
//! Uses `rattler_virtual_packages` to detect conda-style virtual packages.
//! Intended to support cross-platform conda-forge environments for notebooks.

use crate::error::Result;
use crate::modules::types::{ModuleMetadata, ModuleType};

pub fn detect_as_module_metadata() -> Result<ModuleMetadata> {
    let report = noa_virtual_packages::detect_virtual_packages()?;
    let json = serde_json::to_string(&report).unwrap_or_else(|_| "{}".to_string());

    // Hash represents detected platform virtual packages, not an artifact file.
    let hash = blake3::hash(json.as_bytes()).to_hex().to_string();

    let mut meta = ModuleMetadata::new(
        "conda-virtual-packages",
        ModuleType::Microkernel,
        env!("CARGO_PKG_VERSION"),
        hash,
    );

    meta.capabilities = vec!["conda".into(), "virtual-packages".into()];
    meta.dependencies = vec![];
    meta.path = None;

    Ok(meta)
}

pub fn detect_report_json() -> Result<String> {
    Ok(noa_virtual_packages::detect_virtual_packages_json()?)
}
