//! Compression configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub default_algorithm: String,
    pub algorithms: HashMap<String, AlgorithmConfig>,
    pub thresholds: CompressionThresholds,
    pub streaming: StreamingCompressionConfig,
}

/// Algorithm-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmConfig {
    pub enabled: bool,
    pub level: CompressionLevel,
    pub use_cases: Vec<String>,
}

/// Compression level settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionLevel {
    pub min: u32,
    pub max: u32,
    pub default: u32,
}

/// Thresholds for compression decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionThresholds {
    pub min_size_bytes: usize,
    pub max_size_bytes: Option<usize>,
    pub compression_ratio_threshold: f64,
    pub time_limit_ms: u64,
    pub memory_limit_mb: u64,
}

/// Streaming compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingCompressionConfig {
    pub enabled: bool,
    pub buffer_size: usize,
    pub chunk_size: usize,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        let mut algorithms = HashMap::new();
        
        algorithms.insert("zstd".to_string(), AlgorithmConfig {
            enabled: true,
            level: CompressionLevel { min: 1, max: 22, default: 3 },
            use_cases: vec!["general".to_string(), "embeddings".to_string()],
        });
        
        algorithms.insert("lz4".to_string(), AlgorithmConfig {
            enabled: true,
            level: CompressionLevel { min: 1, max: 12, default: 1 },
            use_cases: vec!["fast".to_string(), "streaming".to_string()],
        });
        
        algorithms.insert("gzip".to_string(), AlgorithmConfig {
            enabled: true,
            level: CompressionLevel { min: 1, max: 9, default: 6 },
            use_cases: vec!["compatibility".to_string(), "http".to_string()],
        });
        
        algorithms.insert("brotli".to_string(), AlgorithmConfig {
            enabled: true,
            level: CompressionLevel { min: 0, max: 11, default: 4 },
            use_cases: vec!["text".to_string(), "web".to_string()],
        });

        Self {
            enabled: true,
            default_algorithm: "zstd".to_string(),
            algorithms,
            thresholds: CompressionThresholds::default(),
            streaming: StreamingCompressionConfig::default(),
        }
    }
}

impl Default for CompressionThresholds {
    fn default() -> Self {
        Self {
            min_size_bytes: 1024,
            max_size_bytes: Some(1024 * 1024 * 100), // 100MB
            compression_ratio_threshold: 1.1,
            time_limit_ms: 5000,
            memory_limit_mb: 512,
        }
    }
}

impl Default for StreamingCompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            buffer_size: 64 * 1024,     // 64KB
            chunk_size: 16 * 1024,      // 16KB
        }
    }
}
