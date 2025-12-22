use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::error::Result;

pub struct ConfigWatch {
    noa_root: PathBuf,
    poll_interval: Duration,
}

impl ConfigWatch {
    pub fn new(noa_root: &Path) -> Self {
        Self {
            noa_root: noa_root.to_path_buf(),
            poll_interval: Duration::from_secs(2),
        }
    }

    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    pub fn config_paths(&self) -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = super::merge_map::CORE_MERGE_SPECS
            .iter()
            .map(|s| self.noa_root.join(s.relative_path))
            .collect();

        // Legacy / optional aliases (still referenced by some docs/older setups)
        paths.extend([
            self.noa_root.join("config/noa-instance.yaml"),
            self.noa_root.join("config/noa.yaml"),
            self.noa_root.join("config/noa.json"),
        ]);

        paths
    }

    pub async fn start_polling(
        self,
        access: crate::config::access::ConfigAccess,
        shutdown: tokio::sync::watch::Receiver<bool>,
    ) -> Result<()> {
        let mut last = std::collections::HashMap::<PathBuf, std::time::SystemTime>::new();

        for p in self.config_paths() {
            if let Ok(m) = std::fs::metadata(&p).and_then(|m| m.modified()) {
                last.insert(p, m);
            }
        }

        let mut shutdown = shutdown;

        loop {
            if *shutdown.borrow() {
                break;
            }

            let mut changed = false;
            for p in self.config_paths() {
                if let Ok(m) = std::fs::metadata(&p).and_then(|m| m.modified()) {
                    match last.get(&p) {
                        Some(prev) if *prev >= m => {}
                        _ => {
                            last.insert(p, m);
                            changed = true;
                        }
                    }
                }
            }

            if changed {
                let _ = access.reload().await;
            }

            tokio::select! {
                _ = tokio::time::sleep(self.poll_interval) => {},
                _ = shutdown.changed() => {},
            }
        }

        Ok(())
    }
}
