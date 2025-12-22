//! Embedding Cache
//!
//! T137: Implement embedding cache (model_version + input_hash + params_hash)
//! §3.7: Total Memory Sovereignty

use sha2::{Digest, Sha256};
use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;

/// Embedding cache with hash-based keys and LRU eviction
pub struct EmbeddingCache {
    cache: RwLock<HashMap<String, Vec<f32>>>,
    access_order: RwLock<VecDeque<String>>, // LRU order tracking
    model_version: String,
    max_size: usize,
}

impl EmbeddingCache {
    /// Create a new embedding cache with default max size (10000 entries)
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            access_order: RwLock::new(VecDeque::new()),
            model_version: "all-MiniLM-L6-v2".to_string(), // Default model
            max_size: 10000, // Default max size
        }
    }

    /// Create cache with specific model version
    pub fn with_model(model_version: String) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            access_order: RwLock::new(VecDeque::new()),
            model_version,
            max_size: 10000,
        }
    }

    /// Create cache with specific model version and max size
    pub fn with_model_and_size(model_version: String, max_size: usize) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            access_order: RwLock::new(VecDeque::new()),
            model_version,
            max_size,
        }
    }

    /// Get cached embedding (updates LRU order)
    pub fn get(&self, text: &str) -> Option<Vec<f32>> {
        let key = self.cache_key(text);
        let cache = self.cache.read().ok()?;
        let embedding = cache.get(&key).cloned();

        // Update access order for LRU
        if embedding.is_some() {
            if let Ok(mut order) = self.access_order.write() {
                // Remove from current position and add to end (most recently used)
                order.retain(|k| k != &key);
                order.push_back(key);
            }
        }

        embedding
    }

    /// Put embedding in cache (with LRU eviction if needed)
    pub fn put(&self, text: &str, embedding: Vec<f32>) {
        let key = self.cache_key(text);

        if let Ok(mut cache) = self.cache.write() {
            let is_new = !cache.contains_key(&key);
            cache.insert(key.clone(), embedding);

            if let Ok(mut order) = self.access_order.write() {
                if is_new {
                    // New entry - add to end
                    order.push_back(key.clone());

                    // Evict oldest entries if over limit
                    while order.len() > self.max_size {
                        if let Some(oldest_key) = order.pop_front() {
                            cache.remove(&oldest_key);
                        }
                    }
                } else {
                    // Update existing entry - move to end
                    order.retain(|k| k != &key);
                    order.push_back(key);
                }
            }
        }
    }

    /// Generate cache key: hash(model_version + input_text)
    fn cache_key(&self, text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.model_version.as_bytes());
        hasher.update(text.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Clear cache
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.read().map(|c| c.len()).unwrap_or(0)
    }
}

impl Default for EmbeddingCache {
    fn default() -> Self {
        Self::new()
    }
}

