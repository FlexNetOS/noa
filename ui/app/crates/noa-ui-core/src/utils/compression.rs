//! Compression utilities
//!
//! Re-exports compression functionality from noa-common for UI usage.
//! For direct compression operations, use noa_common::compression.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Compression statistics for tracking performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub compression_time_ms: u64,
    pub algorithm: String,
}

/// Compression algorithm selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Algorithm {
    Gzip,
    Zstd,
    Lz4,
    Brotli,
}

impl Algorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            Algorithm::Gzip => "gzip",
            Algorithm::Zstd => "zstd",
            Algorithm::Lz4 => "lz4",
            Algorithm::Brotli => "brotli",
        }
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Zstd
    }
}

/// Compression manager for UI-layer compression needs
#[derive(Debug, Clone)]
pub struct CompressionManager {
    cache: Arc<Mutex<HashMap<String, CompressionStats>>>,
    max_cache_size: usize,
}

impl CompressionManager {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_cache_size: 1000,
        }
    }

    /// Compress data using the specified algorithm
    /// 
    /// Note: This is a thin wrapper. For production use, prefer noa_common::compression
    /// when the `compression` feature is enabled in sys/core.
    pub async fn compress_data(
        &self,
        data: &[u8],
        algorithm: Algorithm,
    ) -> Result<(Vec<u8>, CompressionStats), String> {
        let start_time = std::time::Instant::now();
        let original_size = data.len() as u64;

        // Delegate to noa_common::compression when available
        #[cfg(feature = "compression")]
        let compressed = noa_common::compression::compress(data, algorithm.into())?;

        #[cfg(not(feature = "compression"))]
        let compressed = self.compress_fallback(data, algorithm)?;

        let compressed_size = compressed.len() as u64;
        let compression_time_ms = start_time.elapsed().as_millis() as u64;
        let compression_ratio = if compressed_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        let stats = CompressionStats {
            original_size,
            compressed_size,
            compression_ratio,
            compression_time_ms,
            algorithm: algorithm.as_str().to_string(),
        };

        Ok((compressed, stats))
    }

    /// Decompress data using the specified algorithm
    pub async fn decompress_data(&self, data: &[u8], algorithm: Algorithm) -> Result<Vec<u8>, String> {
        #[cfg(feature = "compression")]
        return noa_common::compression::decompress(data, algorithm.into());

        #[cfg(not(feature = "compression"))]
        self.decompress_fallback(data, algorithm)
    }

    #[cfg(not(feature = "compression"))]
    fn compress_fallback(&self, data: &[u8], _algorithm: Algorithm) -> Result<Vec<u8>, String> {
        // Fallback: return uncompressed data when compression feature not available
        Ok(data.to_vec())
    }

    #[cfg(not(feature = "compression"))]
    fn decompress_fallback(&self, data: &[u8], _algorithm: Algorithm) -> Result<Vec<u8>, String> {
        // Fallback: return data as-is when compression feature not available
        Ok(data.to_vec())
    }

    /// Get cached compression stats
    pub async fn get_compression_stats(&self, key: &str) -> Option<CompressionStats> {
        let cache = self.cache.lock().await;
        cache.get(key).cloned()
    }

    /// Clear the stats cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.lock().await;
        cache.clear();
    }
}

impl Default for CompressionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// configsuration for compression settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compressionconfigs {
    pub enabled: bool,
    pub default_algorithm: Algorithm,
    pub compression_threshold: usize,
    pub cache_enabled: bool,
    pub max_cache_size: usize,
}

impl Default for Compressionconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            default_algorithm: Algorithm::Zstd,
            compression_threshold: 1024, // 1KB
            cache_enabled: true,
            max_cache_size: 1000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compression_manager_creation() {
        let manager = CompressionManager::new();
        assert!(manager.get_compression_stats("nonexistent").await.is_none());
    }

    #[test]
    fn test_algorithm_as_str() {
        assert_eq!(Algorithm::Gzip.as_str(), "gzip");
        assert_eq!(Algorithm::Zstd.as_str(), "zstd");
        assert_eq!(Algorithm::Lz4.as_str(), "lz4");
        assert_eq!(Algorithm::Brotli.as_str(), "brotli");
    }
}
