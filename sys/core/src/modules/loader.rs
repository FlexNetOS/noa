use crate::error::{NoaError, Result, ValidationError};
use crate::modules::cas::ContentAddressableStore;
use crate::modules::lifecycle::transition;
use crate::modules::types::{ModuleLifecycleState, ModuleMetadata};
use std::path::{Path, PathBuf};

pub struct ModuleLoader {
    cas: ContentAddressableStore,
    modules_dir: PathBuf,
}

impl ModuleLoader {
    pub fn new(cas: ContentAddressableStore, modules_dir: impl AsRef<Path>) -> Self {
        Self {
            cas,
            modules_dir: modules_dir.as_ref().to_path_buf(),
        }
    }

    /// Resolve the module location, downloading from CAS if necessary.
    pub fn load(&self, meta: &ModuleMetadata) -> Result<(ModuleLifecycleState, PathBuf)> {
        let state = transition(ModuleLifecycleState::Verified, ModuleLifecycleState::Loaded)
            .map_err(|e| {
                NoaError::Validation(ValidationError::new("module.lifecycle", e, "INVALID_STATE"))
            })?;

        let path = if let Some(path) = &meta.path {
            path.clone()
        } else {
            let target = self.modules_dir.join(&meta.name).join(&meta.version);
            std::fs::create_dir_all(&target)?;
            let blob = self.cas.object_path(&meta.hash);
            if blob.exists() {
                let dest = target.join("artifact.bin");
                std::fs::copy(&blob, &dest)?;
            }
            target
        };

        Ok((state, path))
    }

    pub fn unload(&self, _meta: &ModuleMetadata) -> Result<ModuleLifecycleState> {
        transition(
            ModuleLifecycleState::Loaded,
            ModuleLifecycleState::Unloading,
        )
        .map_err(|e| {
            NoaError::Validation(ValidationError::new("module.lifecycle", e, "INVALID_STATE"))
        })
    }
}
