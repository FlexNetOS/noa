use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::configs::{query, raw_access, Noaconfigs};
use crate::error::Result;

#[derive(Clone)]
pub struct configsAccess {
    noa_root: PathBuf,
    raw: Arc<RwLock<serde_json::Value>>,
}

impl configsAccess {
    pub fn new(noa_root: PathBuf, raw: serde_json::Value) -> Self {
        Self {
            noa_root,
            raw: Arc::new(RwLock::new(raw)),
        }
    }

    pub fn from_configs(cfg: &Noaconfigs) -> Self {
        Self::new(cfg.noa_root.clone(), cfg.raw.clone())
    }

    pub async fn raw_snapshot(&self) -> serde_json::Value {
        self.raw.read().await.clone()
    }

    pub async fn get_str(&self, pointers: &[&str]) -> Option<String> {
        let raw = self.raw.read().await;
        raw_access::get_str(&raw, pointers)
    }

    pub async fn get_u32(&self, pointers: &[&str]) -> Option<u32> {
        let raw = self.raw.read().await;
        raw_access::get_u32(&raw, pointers)
    }

    pub async fn get_f32(&self, pointers: &[&str]) -> Option<f32> {
        let raw = self.raw.read().await;
        raw_access::get_f32(&raw, pointers)
    }

    pub async fn eval_query_str(&self, q: &query::Query) -> Result<Option<String>> {
        let raw = self.raw.read().await;
        q.eval_str(&raw)
    }

    pub async fn eval_query_u32(&self, q: &query::Query) -> Result<Option<u32>> {
        let raw = self.raw.read().await;
        q.eval_u32(&raw)
    }

    pub async fn eval_query_f32(&self, q: &query::Query) -> Result<Option<f32>> {
        let raw = self.raw.read().await;
        q.eval_f32(&raw)
    }

    pub async fn set_raw(&self, raw: serde_json::Value) {
        *self.raw.write().await = raw;
    }

    pub async fn reload(&self) -> Result<()> {
        // Reload by going through the standard loading pipeline.
        let cfg = Noaconfigs::load_from_root(&self.noa_root)?;
        self.set_raw(cfg.raw).await;
        Ok(())
    }

    pub fn noa_root(&self) -> &Path {
        &self.noa_root
    }

    pub fn start_polling_reload(
        &self,
        poll_interval: std::time::Duration,
    ) -> tokio::sync::watch::Sender<bool> {
        let (tx, rx) = tokio::sync::watch::channel(false);
        let access = self.clone();
        let watcher = crate::configs::watch::configsWatch::new(&self.noa_root)
            .with_poll_interval(poll_interval);

        tokio::spawn(async move {
            let _ = watcher.start_polling(access, rx).await;
        });

        tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_str_pointer() {
        let raw = serde_json::json!({"memory": {"search": {"embedding_model": "m1"}}});
        let access = configsAccess::new(PathBuf::from("."), raw);

        let v = access
            .get_str(&["/memory/search/embedding_model", "/fallback"])
            .await
            .unwrap();

        assert_eq!(v, "m1");
    }

    #[tokio::test]
    async fn test_eval_query_coalesce() {
        let raw = serde_json::json!({"memory": {"search": {}}});
        let access = configsAccess::new(PathBuf::from("."), raw);

        let q = query::Query::Coalesce(vec![
            query::Query::Ptr("/memory/search/embedding_model".to_string()),
            query::Query::Literal(serde_json::Value::String("fallback".to_string())),
        ]);

        let v = access.eval_query_str(&q).await.unwrap().unwrap();
        assert_eq!(v, "fallback");
    }
}
