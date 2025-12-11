//! Shared provider execution memory (in-memory stub)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedContext {
    pub provider: String,
    pub context_type: String,
    pub content: String,
    pub metadata: Option<String>,
}

#[derive(Clone, Default)]
pub struct SharedProviderMemory {
    inner: Arc<Mutex<HashMap<String, SharedContext>>>,
}

impl SharedProviderMemory {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn upsert(&self, context: SharedContext) -> Result<()> {
        let key = format!("{}:{}", context.provider, context.context_type);
        self.inner.lock().unwrap().insert(key, context);
        Ok(())
    }

    pub fn get(&self, provider: &str, context_type: &str) -> Option<SharedContext> {
        let key = format!("{}:{}", provider, context_type);
        self.inner.lock().unwrap().get(&key).cloned()
    }

    pub fn all(&self) -> Vec<SharedContext> {
        self.inner.lock().unwrap().values().cloned().collect()
    }
}
