use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod algorithms;
pub mod ml_compression;
pub mod streaming_compression;
pub mod cache_compression;

use algorithms::{CompressionAlgorithm, CompressionResult, CompressionStats};
use ml_compression::{MLEmbeddingCompressor, MLModelCompressor, MLPromptCompressor};
use streaming_compression::StreamingCompressor;
use cache_compression::CacheCompressor;

pub struct CompressionManager {
    algorithms: Arc<RwLock<HashMap<String, Box<dyn CompressionAlgorithm>>>>,
    ml_compressors: Arc<RwLock<HashMap<String, Box<dyn MLOptimizedCompressor>>>>,
    cache: Arc<RwLock<HashMap<String, CompressionStats>>>,
    config: CompressionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub default_algorithm: String,
    pub algorithms: HashMap<String, AlgorithmConfig>,
    pub ml_optimization: bool,
    pub cache_enabled: bool,
    pub max_cache_size: usize,
    pub compression_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmConfig {
    pub enabled: bool,
    pub priority: u32,
    pub settings: HashMap<String, serde_json::Value>,
    pub use_cases: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CompressionTask {
    pub data: Vec<u8>,
    pub algorithm: Option<String>,
    pub ml_type: Option<MLDataType>,
    pub priority: CompressionPriority,
    pub streaming: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompressionPriority {
    RealTime,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone)]
pub enum MLDataType {
    Embedding,
    ModelWeights,
    Prompt,
    Gradient,
    Activation,
}

pub trait MLOptimizedCompressor: Send + Sync {
    fn compress(&self, data: &[u8], data_type: &MLDataType) -> Result<CompressionResult>;
    fn decompress(&self, data: &[u8], data_type: &MLDataType) -> Result<Vec<u8>>;
    fn get_compression_stats(&self) -> CompressionStats;
}

impl CompressionManager {
    pub fn new(config: CompressionConfig) -> Self {
        let mut manager = Self {
            algorithms: Arc::new(RwLock::new(HashMap::new())),
            ml_compressors: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        };
        
        manager.initialize_algorithms();
        manager
    }

    fn initialize_algorithms(&mut self) {
        // Register standard compression algorithms
        let mut algorithms = self.algorithms.blocking_write();
        
        if self.config.algorithms.get("zstd").map_or(false, |c| c.enabled) {
            algorithms.insert("zstd".to_string(), Box::new(algorithms::ZstdAlgorithm::new()));
        }
        
        if self.config.algorithms.get("brotli").map_or(false, |c| c.enabled) {
            algorithms.insert("brotli".to_string(), Box::new(algorithms::BrotliAlgorithm::new()));
        }
        
        if self.config.algorithms.get("gzip").map_or(false, |c| c.enabled) {
            algorithms.insert("gzip".to_string(), Box::new(algorithms::GzipAlgorithm::new()));
        }
        
        if self.config.algorithms.get("lz4").map_or(false, |c| c.enabled) {
            algorithms.insert("lz4".to_string(), Box::new(algorithms::Lz4Algorithm::new()));
        }
        
        // Register ML-optimized compressors
        if self.config.ml_optimization {
            let mut ml_compressors = self.ml_compressors.blocking_write();
            ml_compressors.insert("embedding".to_string(), Box::new(MLEmbeddingCompressor::new()));
            ml_compressors.insert("model".to_string(), Box::new(MLModelCompressor::new()));
            ml_compressors.insert("prompt".to_string(), Box::new(MLPromptCompressor::new()));
        }
    }

    pub async fn compress(&self, task: CompressionTask) -> Result<CompressionResult> {
        let algorithm_name = task.algorithm.as_ref()
            .unwrap_or(&self.config.default_algorithm);
        
        // Check cache first
        if self.config.cache_enabled {
            let cache_key = self.generate_cache_key(&task.data, algorithm_name);
            if let Some(cached_stats) = self.cache.read().await.get(&cache_key) {
                return Ok(CompressionResult {
                    compressed_data: task.data, // Would need to store actual compressed data
                    stats: cached_stats.clone(),
                });
            }
        }
        
        let result = if let Some(ml_type) = &task.ml_type {
            self.compress_ml_data(&task.data, ml_type, algorithm_name).await?
        } else {
            self.compress_standard(&task.data, algorithm_name).await?
        };
        
        // Cache the result
        if self.config.cache_enabled {
            let cache_key = self.generate_cache_key(&task.data, algorithm_name);
            let mut cache = self.cache.write().await;
            cache.insert(cache_key, result.stats.clone());
            
            // Evict old cache entries if needed
            if cache.len() > self.config.max_cache_size {
                self.evict_cache_entries(&mut cache).await;
            }
        }
        
        Ok(result)
    }

    pub async fn decompress(&self, compressed_data: &[u8], algorithm_name: &str, ml_type: Option<&MLDataType>) -> Result<Vec<u8>> {
        if let Some(ml_type) = ml_type {
            let ml_compressors = self.ml_compressors.read().await;
            if let Some(compressor) = ml_compressors.get(&self.get_ml_compressor_key(ml_type)) {
                return compressor.decompress(compressed_data, ml_type);
            }
        }
        
        let algorithms = self.algorithms.read().await;
        if let Some(algorithm) = algorithms.get(algorithm_name) {
            algorithm.decompress(compressed_data)
        } else {
            Err(anyhow::anyhow!("Algorithm not found: {}", algorithm_name))
        }
    }

    pub async fn get_compression_stats(&self) -> GlobalCompressionStats {
        let cache = self.cache.read().await;
        let mut total_original_size = 0;
        let mut total_compressed_size = 0;
        let mut total_compression_time_ms = 0;
        let mut algorithm_stats = HashMap::new();
        
        for stats in cache.values() {
            total_original_size += stats.original_size;
            total_compressed_size += stats.compressed_size;
            total_compression_time_ms += stats.compression_time_ms;
            
            *algorithm_stats.entry(stats.algorithm.clone()).or_insert(0) += 1;
        }
        
        GlobalCompressionStats {
            total_operations: cache.len(),
            total_original_size,
            total_compressed_size,
            total_compression_time_ms,
            average_compression_ratio: if total_original_size > 0 {
                total_original_size as f64 / total_compressed_size as f64
            } else {
                1.0
            },
            algorithm_usage: algorithm_stats,
            cache_hit_rate: 0.0, // Would need to track hits/misses
        }
    }

    pub async fn streaming_compress(&self, data_stream: impl futures::Stream<Item = Vec<u8>> + Send + 'static) -> impl futures::Stream<Item = Result<Vec<u8>>> {
        let streaming_compressor = StreamingCompressor::new(self.config.default_algorithm.clone());
        streaming_compressor.compress_stream(data_stream)
    }

    pub async fn cache_compress(&self, cache_data: &[u8], priority: CompressionPriority) -> Result<CompressionResult> {
        let cache_compressor = CacheCompressor::new();
        cache_compressor.compress_with_priority(cache_data, priority)
    }

    async fn compress_standard(&self, data: &[u8], algorithm_name: &str) -> Result<CompressionResult> {
        let algorithms = self.algorithms.read().await;
        if let Some(algorithm) = algorithms.get(algorithm_name) {
            algorithm.compress(data)
        } else {
            Err(anyhow::anyhow!("Algorithm not found: {}", algorithm_name))
        }
    }

    async fn compress_ml_data(&self, data: &[u8], ml_type: &MLDataType, algorithm_name: &str) -> Result<CompressionResult> {
        let ml_compressors = self.ml_compressors.read().await;
        let compressor_key = self.get_ml_compressor_key(ml_type);
        
        if let Some(compressor) = ml_compressors.get(&compressor_key) {
            compressor.compress(data, ml_type)
        } else {
            // Fall back to standard compression
            self.compress_standard(data, algorithm_name).await
        }
    }

    fn get_ml_compressor_key(&self, ml_type: &MLDataType) -> String {
        match ml_type {
            MLDataType::Embedding => "embedding".to_string(),
            MLDataType::ModelWeights => "model".to_string(),
            MLDataType::Prompt => "prompt".to_string(),
            MLDataType::Gradient => "gradient".to_string(),
            MLDataType::Activation => "activation".to_string(),
        }
    }

    fn generate_cache_key(&self, data: &[u8], algorithm: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        algorithm.hash(&mut hasher);
        
        format!("{:x}_{}", hasher.finish(), algorithm)
    }

    async fn evict_cache_entries(&self, cache: &mut HashMap<String, CompressionStats>) {
        // Simple LRU eviction - remove oldest entries
        let mut entries: Vec<_> = cache.iter().map(|(k, v)| (k.clone(), v.timestamp)).collect();
        entries.sort_by_key(|(_, ts)| *ts);
        
        let to_remove = entries.len() / 4; // Remove 25% of entries
        for (key, _) in entries.into_iter().take(to_remove) {
            cache.remove(&key);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCompressionStats {
    pub total_operations: usize,
    pub total_original_size: usize,
    pub total_compressed_size: usize,
    pub total_compression_time_ms: u64,
    pub average_compression_ratio: f64,
    pub algorithm_usage: HashMap<String, usize>,
    pub cache_hit_rate: f64,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        let mut algorithms = HashMap::new();
        algorithms.insert("zstd".to_string(), AlgorithmConfig {
            enabled: true,
            priority: 1,
            settings: HashMap::new(),
            use_cases: vec!["general".to_string(), "ml".to_string()],
        });
        algorithms.insert("brotli".to_string(), AlgorithmConfig {
            enabled: true,
            priority: 2,
            settings: HashMap::new(),
            use_cases: vec!["web".to_string(), "text".to_string()],
        });
        algorithms.insert("lz4".to_string(), AlgorithmConfig {
            enabled: true,
            priority: 3,
            settings: HashMap::new(),
            use_cases: vec!["realtime".to_string(), "streaming".to_string()],
        });
        
        Self {
            default_algorithm: "zstd".to_string(),
            algorithms,
            ml_optimization: true,
            cache_enabled: true,
            max_cache_size: 10000,
            compression_level: 3,
        }
    }
}