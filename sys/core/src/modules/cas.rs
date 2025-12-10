use crate::error::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct ContentAddressableStore {
    base: PathBuf,
}

#[derive(Default, Serialize, Deserialize)]
struct RefCounts {
    counts: HashMap<String, u64>,
}

impl ContentAddressableStore {
    pub fn new(base: impl AsRef<Path>) -> Result<Self> {
        let base = base.as_ref().to_path_buf();
        fs::create_dir_all(&base)?;
        Ok(Self { base })
    }

    pub fn store_bytes(&self, data: &[u8]) -> Result<String> {
        let hash = hash_bytes(data);
        let path = self.object_path(&hash);
        if path.exists() {
            return Ok(hash);
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let tmp = path.with_extension("tmp");
        {
            let mut file = fs::File::create(&tmp)?;
            file.write_all(data)?;
            file.sync_all()?;
        }
        fs::rename(&tmp, &path)?;
        Ok(hash)
    }

    pub fn store_file(&self, path: &Path) -> Result<String> {
        let mut file = fs::File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        self.store_bytes(&buf)
    }

    pub fn load_bytes(&self, hash: &str) -> Result<Vec<u8>> {
        let path = self.object_path(hash);
        let bytes = fs::read(&path)?;
        Ok(bytes)
    }

    pub fn object_path(&self, hash: &str) -> PathBuf {
        let shard1 = &hash[0..2];
        let shard2 = &hash[2..4];
        self.base.join(shard1).join(shard2).join(hash)
    }

    pub fn exists(&self, hash: &str) -> bool {
        self.object_path(hash).exists()
    }

    pub fn increment_ref(&self, hash: &str) -> Result<()> {
        let mut refs = self.load_refs()?;
        *refs.counts.entry(hash.to_string()).or_insert(0) += 1;
        self.save_refs(&refs)
    }

    pub fn decrement_ref(&self, hash: &str) -> Result<()> {
        let mut refs = self.load_refs()?;
        if let Some(count) = refs.counts.get_mut(hash) {
            if *count > 0 {
                *count -= 1;
            }
        }
        self.save_refs(&refs)
    }

    /// Remove CAS objects with zero references. Returns the hashes removed.
    pub fn garbage_collect(&self) -> Result<Vec<String>> {
        let mut refs = self.load_refs()?;
        let mut removed = Vec::new();
        refs.counts.retain(|hash, count| {
            if *count == 0 {
                let path = self.object_path(hash);
                if path.exists() {
                    let _ = fs::remove_file(&path);
                }
                removed.push(hash.clone());
                false
            } else {
                true
            }
        });
        self.save_refs(&refs)?;
        Ok(removed)
    }

    fn load_refs(&self) -> Result<RefCounts> {
        let path = self.base.join("refs.json");
        if !path.exists() {
            return Ok(RefCounts::default());
        }
        let data = fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&data).unwrap_or_default())
    }

    fn save_refs(&self, refs: &RefCounts) -> Result<()> {
        if let Some(parent) = self.base.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(refs)?;
        fs::write(self.base.join("refs.json"), json)?;
        Ok(())
    }
}

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
