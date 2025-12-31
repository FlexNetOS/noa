use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

// Compression and decompression utilities for managing capacity constraints

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub compression_time_ms: u64,
    pub algorithm: String,
}

#[derive(Debug, Clone)]
pub struct CompressionManager {
    cache: Arc<Mutex<HashMap<String, CompressionStats>>>,
    max_cache_size: usize,
    compression_algorithms: Vec<String>,
}

impl CompressionManager {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_cache_size: 1000,
            compression_algorithms: vec![
                "gzip".to_string(),
                "brotli".to_string(),
                "zstd".to_string(),
                "lz4".to_string(),
            ],
        }
    }
    
    pub async fn compress_data(&self, data: &[u8], algorithm: &str) -> Result<(Vec<u8>, CompressionStats), String> {
        let start_time = std::time::Instant::now();
        let original_size = data.len() as u64;
        
        let compressed = match algorithm {
            "gzip" => self.compress_gzip(data)?,
            "brotli" => self.compress_brotli(data)?,
            "zstd" => self.compress_zstd(data)?,
            "lz4" => self.compress_lz4(data)?,
            _ => return Err(format!("Unsupported compression algorithm: {}", algorithm)),
        };
        
        let compressed_size = compressed.len() as u64;
        let compression_time_ms = start_time.elapsed().as_millis() as u64;
        let compression_ratio = original_size as f64 / compressed_size as f64;
        
        let stats = CompressionStats {
            original_size,
            compressed_size,
            compression_ratio,
            compression_time_ms,
            algorithm: algorithm.to_string(),
        };
        
        // Cache the stats
        let cache_key = format!("{}_{}", algorithm, xxhash_rust::xxh3::xxh3_64(data));
        {
            let mut cache = self.cache.lock().await;
            if cache.len() >= self.max_cache_size {
                // Remove oldest entries
                let keys: Vec<String> = cache.keys().cloned().collect();
                for key in keys.iter().take(100) {
                    cache.remove(key);
                }
            }
            cache.insert(cache_key, stats.clone());
        }
        
        Ok((compressed, stats))
    }
    
    pub async fn decompress_data(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>, String> {
        match algorithm {
            "gzip" => self.decompress_gzip(data),
            "brotli" => self.decompress_brotli(data),
            "zstd" => self.decompress_zstd(data),
            "lz4" => self.decompress_lz4(data),
            _ => Err(format!("Unsupported decompression algorithm: {}", algorithm)),
        }
    }
    
    fn compress_gzip(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).map_err(|e| e.to_string())?;
        encoder.finish().map_err(|e| e.to_string())
    }
    
    fn decompress_gzip(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        use flate2::read::GzDecoder;
        use std::io::Read;
        
        let mut decoder = GzDecoder::new(data);
        let mut output = Vec::new();
        decoder.read_to_end(&mut output).map_err(|e| e.to_string())?;
        Ok(output)
    }
    
    fn compress_brotli(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        use brotli::enc::BrotliEncoderParams;
        
        let params = BrotliEncoderParams::default();
        let mut output = Vec::new();
        brotli::BrotliCompress(&mut std::io::Cursor::new(data), &mut output, &params)
            .map_err(|e| e.to_string())?;
        Ok(output)
    }
    
    fn decompress_brotli(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        brotli::BrotliDecompress(&mut std::io::Cursor::new(data), &mut output)
            .map_err(|e| e.to_string())?;
        Ok(output)
    }
    
    fn compress_zstd(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        use zstd::stream::encode_all;
        
        let output = encode_all(std::io::Cursor::new(data), 3).map_err(|e| e.to_string())?;
        Ok(output)
    }
    
    fn decompress_zstd(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        use zstd::stream::decode_all;
        
        let output = decode_all(std::io::Cursor::new(data)).map_err(|e| e.to_string())?;
        Ok(output)
    }
    
    fn compress_lz4(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        use lz4_flex::frame::FrameEncoder;
        use std::io::Write;
        
        let mut encoder = FrameEncoder::new(Vec::new());
        encoder.write_all(data).map_err(|e| e.to_string())?;
        Ok(encoder.finish().map_err(|e| e.to_string())?)
    }
    
    fn decompress_lz4(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        use lz4_flex::frame::FrameDecoder;
        use std::io::Read;
        
        let mut decoder = FrameDecoder::new(std::io::Cursor::new(data));
        let mut output = Vec::new();
        decoder.read_to_end(&mut output).map_err(|e| e.to_string())?;
        Ok(output)
    }
    
    pub async fn get_best_algorithm(&self, data: &[u8]) -> String {
        let algorithms = vec!["gzip", "brotli", "zstd", "lz4"];
        let mut best_algorithm = "gzip".to_string();
        let mut best_ratio = 0.0;
        
        for algorithm in &algorithms {
            if let Ok((_, stats)) = self.compress_data(data, algorithm).await {
                if stats.compression_ratio > best_ratio {
                    best_ratio = stats.compression_ratio;
                    best_algorithm = algorithm.to_string();
                }
            }
        }
        
        best_algorithm
    }
    
    pub async fn get_compression_stats(&self, key: &str) -> Option<CompressionStats> {
        let cache = self.cache.lock().await;
        cache.get(key).cloned()
    }
    
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.lock().await;
        (cache.len(), cache.capacity())
    }
    
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.lock().await;
        cache.clear();
    }
}

// Project compression manager for handling large projects
#[derive(Debug, Clone)]
pub struct ProjectCompressionManager {
    compression_manager: CompressionManager,
    project_cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    compression_threshold: usize,
}

impl ProjectCompressionManager {
    pub fn new(compression_threshold: usize) -> Self {
        Self {
            compression_manager: CompressionManager::new(),
            project_cache: Arc::new(Mutex::new(HashMap::new())),
            compression_threshold,
        }
    }
    
    pub async fn compress_project(&self, project_id: &str, project_data: &[u8]) -> Result<(Vec<u8>, CompressionStats), String> {
        if project_data.len() < self.compression_threshold {
            return Ok((project_data.to_vec(), CompressionStats {
                original_size: project_data.len() as u64,
                compressed_size: project_data.len() as u64,
                compression_ratio: 1.0,
                compression_time_ms: 0,
                algorithm: "none".to_string(),
            }));
        }
        
        let algorithm = self.compression_manager.get_best_algorithm(project_data).await;
        let (compressed, stats) = self.compression_manager.compress_data(project_data, &algorithm).await?;
        
        // Cache the compressed data
        {
            let mut cache = self.project_cache.lock().await;
            if cache.len() >= 100 {
                // Remove oldest entries
                let keys: Vec<String> = cache.keys().cloned().collect();
                for key in keys.iter().take(10) {
                    cache.remove(key);
                }
            }
            cache.insert(project_id.to_string(), compressed.clone());
        }
        
        Ok((compressed, stats))
    }
    
    pub async fn decompress_project(&self, project_id: &str, compressed_data: &[u8], algorithm: &str) -> Result<Vec<u8>, String> {
        // Check cache first
        {
            let cache = self.project_cache.lock().await;
            if let Some(cached) = cache.get(project_id) {
                return Ok(cached.clone());
            }
        }
        
        let decompressed = self.compression_manager.decompress_data(compressed_data, algorithm).await?;
        
        // Cache the decompressed data
        {
            let mut cache = self.project_cache.lock().await;
            cache.insert(project_id.to_string(), decompressed.clone());
        }
        
        Ok(decompressed)
    }
    
    pub async fn get_project_size(&self, project_id: &str) -> Option<(u64, u64)> {
        let cache = self.project_cache.lock().await;
        if let Some(data) = cache.get(project_id) {
            let original_size = data.len() as u64;
            
            // Try to get compression stats
            let stats = self.compression_manager.get_compression_stats(project_id).await;
            
            if let Some(stats) = stats {
                Some((original_size, stats.compressed_size))
            } else {
                Some((original_size, original_size))
            }
        } else {
            None
        }
    }
    
    pub async fn cleanup_old_projects(&self, max_age_hours: u64) {
        // Implementation would clean up projects older than specified hours
        let mut cache = self.project_cache.lock().await;
        // This is a simplified implementation
        // In production, you'd track creation time and remove old entries
        if cache.len() > 50 {
            let keys: Vec<String> = cache.keys().cloned().collect();
            for key in keys.iter().take(cache.len() - 50) {
                cache.remove(key);
            }
        }
    }
}

// Streaming compression for large data
#[derive(Debug)]
pub struct StreamingCompressor {
    algorithm: String,
    buffer_size: usize,
}

impl StreamingCompressor {
    pub fn new(algorithm: String, buffer_size: usize) -> Self {
        Self {
            algorithm,
            buffer_size,
        }
    }
    
    pub fn compress_stream<S>(&self, input: S) -> impl futures::Stream<Item = Result<Vec<u8>, String>>
    where
        S: futures::Stream<Item = Vec<u8>> + Send + 'static,
    {
        // This would implement streaming compression
        // For now, return a simple implementation  
        futures::stream::unfold(Box::pin(input), |mut stream| async move {
            use futures::StreamExt;
            stream.next().await.map(|chunk| (Ok(chunk), stream))
        })
    }
    
    pub fn decompress_stream<S>(&self, input: S) -> impl futures::Stream<Item = Result<Vec<u8>, String>>
    where
        S: futures::Stream<Item = Vec<u8>> + Send + 'static,
    {
        // This would implement streaming decompression
        // For now, return a simple implementation
        futures::stream::unfold(Box::pin(input), |mut stream| async move {
            use futures::StreamExt;
            stream.next().await.map(|chunk| (Ok(chunk), stream))
        })
    }
}

// Compression utilities for AI/ML workloads
pub struct MLCompressionUtils;

impl MLCompressionUtils {
    pub fn compress_embeddings(embeddings: &[f32]) -> Result<Vec<u8>, String> {
        // Convert float embeddings to compressed format
        let bytes: Vec<u8> = embeddings.iter()
            .flat_map(|&f| f.to_le_bytes())
            .collect();
        
        // Apply quantization for further compression
        Self::quantize_embeddings(&bytes)
    }
    
    pub fn decompress_embeddings(compressed: &[u8]) -> Result<Vec<f32>, String> {
        // Decompress and convert back to float embeddings
        let decompressed = Self::dequantize_embeddings(compressed)?;
        
        let embeddings: Vec<f32> = decompressed
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        
        Ok(embeddings)
    }
    
    fn quantize_embeddings(data: &[u8]) -> Result<Vec<u8>, String> {
        // Simple quantization - in production, use more sophisticated methods
        Ok(data.to_vec())
    }
    
    fn dequantize_embeddings(data: &[u8]) -> Result<Vec<u8>, String> {
        Ok(data.to_vec())
    }
    
    pub fn compress_prompt_cache(cache: &HashMap<String, String>) -> Result<Vec<u8>, String> {
        // Serialize and compress prompt cache
        let serialized = serde_json::to_vec(cache).map_err(|e| e.to_string())?;
        
        // Use zstd for good compression ratio
        let compressed = zstd::stream::encode_all(std::io::Cursor::new(serialized), 3)
            .map_err(|e| e.to_string())?;
        
        Ok(compressed)
    }
    
    pub fn decompress_prompt_cache(compressed: &[u8]) -> Result<HashMap<String, String>, String> {
        // Decompress and deserialize prompt cache
        let decompressed = zstd::stream::decode_all(std::io::Cursor::new(compressed))
            .map_err(|e| e.to_string())?;
        
        let cache: HashMap<String, String> = serde_json::from_slice(&decompressed)
            .map_err(|e| e.to_string())?;
        
        Ok(cache)
    }
}

// Configuration for compression settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub default_algorithm: String,
    pub compression_threshold: usize,
    pub cache_enabled: bool,
    pub max_cache_size: usize,
    pub ml_compression_enabled: bool,
    pub streaming_enabled: bool,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_algorithm: "zstd".to_string(),
            compression_threshold: 1024, // 1KB
            cache_enabled: true,
            max_cache_size: 1000,
            ml_compression_enabled: true,
            streaming_enabled: false,
        }
    }
}

// Compression middleware for HTTP responses
pub struct CompressionMiddleware {
    config: CompressionConfig,
    manager: CompressionManager,
}

impl CompressionMiddleware {
    pub fn new(config: CompressionConfig) -> Self {
        Self {
            config,
            manager: CompressionManager::new(),
        }
    }
    
    pub async fn compress_response(&self, data: Vec<u8>) -> Result<(Vec<u8>, String), String> {
        if !self.config.enabled || data.len() < self.config.compression_threshold {
            return Ok((data, "none".to_string()));
        }
        
        let algorithm = &self.config.default_algorithm;
        let (compressed, stats) = self.manager.compress_data(&data, algorithm).await?;
        
        // Only use compression if it provides benefit
        if stats.compression_ratio > 1.1 {
            Ok((compressed, algorithm.clone()))
        } else {
            Ok((data, "none".to_string()))
        }
    }
}

// Test utilities for compression
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_compression_algorithms() {
        let manager = CompressionManager::new();
        let test_data = b"This is a test string that will be compressed using various algorithms. The goal is to ensure that all compression methods work correctly and provide reasonable compression ratios.";
        
        let algorithms = vec!["gzip", "brotli", "zstd", "lz4"];
        
        for algorithm in &algorithms {
            let (compressed, stats) = manager.compress_data(test_data, algorithm).await.unwrap();
            let decompressed = manager.decompress_data(&compressed, algorithm).await.unwrap();
            
            assert_eq!(test_data.to_vec(), decompressed);
            assert!(stats.compression_ratio > 1.0);
            assert!(stats.compression_time_ms < 100);
        }
    }
    
    #[tokio::test]
    async fn test_best_algorithm_selection() {
        let manager = CompressionManager::new();
        let test_data = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        
        let best_algorithm = manager.get_best_algorithm(test_data).await;
        assert!(vec!["gzip", "brotli", "zstd", "lz4"].contains(&best_algorithm.as_str()));
    }
    
    #[tokio::test]
    async fn test_project_compression() {
        let project_manager = ProjectCompressionManager::new(1024);
        let project_data = b"This is a sample project data that represents a typical web application with multiple components and assets.";
        
        let (compressed, stats) = project_manager.compress_project("test_project", project_data).await.unwrap();
        
        assert!(stats.compression_ratio > 1.0);
        assert!(stats.compression_time_ms < 50);
        assert!(compressed.len() < project_data.len());
        
        let decompressed = project_manager.decompress_project("test_project", &compressed, &stats.algorithm).await.unwrap();
        assert_eq!(project_data.to_vec(), decompressed);
    }
}