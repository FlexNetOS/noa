use std::time::Instant;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub trait CompressionAlgorithm: Send + Sync {
    fn compress(&self, data: &[u8]) -> Result<CompressionResult>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn get_name(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    pub compressed_data: Vec<u8>,
    pub stats: CompressionStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub algorithm: String,
    pub original_size: usize,
    pub compressed_size: usize,
    pub compression_ratio: f64,
    pub compression_time_ms: u64,
    pub decompression_time_ms: Option<u64>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct ZstdAlgorithm {
    level: i32,
}

impl ZstdAlgorithm {
    pub fn new() -> Self {
        Self { level: 3 }
    }
    
    pub fn with_level(level: i32) -> Self {
        Self { level: level.clamp(1, 22) }
    }
}

impl CompressionAlgorithm for ZstdAlgorithm {
    fn compress(&self, data: &[u8]) -> Result<CompressionResult> {
        let start_time = Instant::now();
        
        let compressed_data = zstd::encode_all(data, self.level)?;
        let compression_time = start_time.elapsed().as_millis() as u64;
        
        let stats = CompressionStats {
            algorithm: self.get_name().to_string(),
            original_size: data.len(),
            compressed_size: compressed_data.len(),
            compression_ratio: data.len() as f64 / compressed_data.len() as f64,
            compression_time_ms: compression_time,
            decompression_time_ms: None,
            timestamp: chrono::Utc::now(),
        };
        
        Ok(CompressionResult {
            compressed_data,
            stats,
        })
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        zstd::decode_all(data)
    }
    
    fn get_name(&self) -> &'static str {
        "zstd"
    }
}

pub struct BrotliAlgorithm {
    quality: u32,
    window_size: u32,
}

impl BrotliAlgorithm {
    pub fn new() -> Self {
        Self {
            quality: 6,
            window_size: 22,
        }
    }
    
    pub fn with_quality(quality: u32) -> Self {
        Self {
            quality: quality.clamp(0, 11),
            window_size: 22,
        }
    }
}

impl CompressionAlgorithm for BrotliAlgorithm {
    fn compress(&self, data: &[u8]) -> Result<CompressionResult> {
        let start_time = Instant::now();
        
        let mut compressed_data = Vec::new();
        let mut encoder = brotli::CompressorWriter::new(
            &mut compressed_data,
            4096, // buffer size
            self.quality,
            self.window_size
        );
        
        std::io::Write::write_all(&mut encoder, data)?;
        encoder.flush()?;
        drop(encoder); // Ensure all data is written
        
        let compression_time = start_time.elapsed().as_millis() as u64;
        
        let stats = CompressionStats {
            algorithm: self.get_name().to_string(),
            original_size: data.len(),
            compressed_size: compressed_data.len(),
            compression_ratio: data.len() as f64 / compressed_data.len() as f64,
            compression_time_ms: compression_time,
            decompression_time_ms: None,
            timestamp: chrono::Utc::now(),
        };
        
        Ok(CompressionResult {
            compressed_data,
            stats,
        })
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decompressed_data = Vec::new();
        let mut decoder = brotli::Decompressor::new(data, 4096);
        std::io::Read::read_to_end(&mut decoder, &mut decompressed_data)?;
        Ok(decompressed_data)
    }
    
    fn get_name(&self) -> &'static str {
        "brotli"
    }
}

pub struct GzipAlgorithm {
    level: flate2::Compression,
}

impl GzipAlgorithm {
    pub fn new() -> Self {
        Self {
            level: flate2::Compression::default(),
        }
    }
    
    pub fn with_level(level: u32) -> Self {
        let compression_level = match level {
            0 => flate2::Compression::none(),
            1..=3 => flate2::Compression::fast(),
            4..=6 => flate2::Compression::default(),
            7..=9 => flate2::Compression::best(),
            _ => flate2::Compression::default(),
        };
        
        Self {
            level: compression_level,
        }
    }
}

impl CompressionAlgorithm for GzipAlgorithm {
    fn compress(&self, data: &[u8]) -> Result<CompressionResult> {
        let start_time = Instant::now();
        
        let mut compressed_data = Vec::new();
        {
            let mut encoder = flate2::write::GzEncoder::new(
                &mut compressed_data,
                self.level
            );
            std::io::Write::write_all(&mut encoder, data)?;
            encoder.finish()?;
        }
        
        let compression_time = start_time.elapsed().as_millis() as u64;
        
        let stats = CompressionStats {
            algorithm: self.get_name().to_string(),
            original_size: data.len(),
            compressed_size: compressed_data.len(),
            compression_ratio: data.len() as f64 / compressed_data.len() as f64,
            compression_time_ms: compression_time,
            decompression_time_ms: None,
            timestamp: chrono::Utc::now(),
        };
        
        Ok(CompressionResult {
            compressed_data,
            stats,
        })
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decompressed_data = Vec::new();
        let mut decoder = flate2::read::GzDecoder::new(data);
        std::io::Read::read_to_end(&mut decoder, &mut decompressed_data)?;
        Ok(decompressed_data)
    }
    
    fn get_name(&self) -> &'static str {
        "gzip"
    }
}

pub struct Lz4Algorithm {
    level: i32,
}

impl Lz4Algorithm {
    pub fn new() -> Self {
        Self { level: 9 }
    }
    
    pub fn with_level(level: i32) -> Self {
        Self { level: level.clamp(1, 16) }
    }
}

impl CompressionAlgorithm for Lz4Algorithm {
    fn compress(&self, data: &[u8]) -> Result<CompressionResult> {
        let start_time = Instant::now();
        
        let compressed_data = lz4_flex::compress_prepend_size(data);
        let compression_time = start_time.elapsed().as_millis() as u64;
        
        let stats = CompressionStats {
            algorithm: self.get_name().to_string(),
            original_size: data.len(),
            compressed_size: compressed_data.len(),
            compression_ratio: data.len() as f64 / compressed_data.len() as f64,
            compression_time_ms: compression_time,
            decompression_time_ms: None,
            timestamp: chrono::Utc::now(),
        };
        
        Ok(CompressionResult {
            compressed_data,
            stats,
        })
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        lz4_flex::decompress_size_prepended(data)
            .map_err(|e| anyhow::anyhow!("LZ4 decompression failed: {:?}", e))
    }
    
    fn get_name(&self) -> &'static str {
        "lz4"
    }
}

pub struct Bzip2Algorithm {
    level: u32,
}

impl Bzip2Algorithm {
    pub fn new() -> Self {
        Self { level: 6 }
    }
    
    pub fn with_level(level: u32) -> Self {
        Self { level: level.clamp(1, 9) }
    }
}

impl CompressionAlgorithm for Bzip2Algorithm {
    fn compress(&self, data: &[u8]) -> Result<CompressionResult> {
        let start_time = Instant::now();
        
        let mut compressed_data = Vec::new();
        {
            let mut encoder = bzip2::write::BzEncoder::new(
                &mut compressed_data,
                bzip2::Compression::new(self.level)
            );
            std::io::Write::write_all(&mut encoder, data)?;
            encoder.finish()?;
        }
        
        let compression_time = start_time.elapsed().as_millis() as u64;
        
        let stats = CompressionStats {
            algorithm: self.get_name().to_string(),
            original_size: data.len(),
            compressed_size: compressed_data.len(),
            compression_ratio: data.len() as f64 / compressed_data.len() as f64,
            compression_time_ms: compression_time,
            decompression_time_ms: None,
            timestamp: chrono::Utc::now(),
        };
        
        Ok(CompressionResult {
            compressed_data,
            stats,
        })
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decompressed_data = Vec::new();
        let mut decoder = bzip2::read::BzDecoder::new(data);
        std::io::Read::read_to_end(&mut decoder, &mut decompressed_data)?;
        Ok(decompressed_data)
    }
    
    fn get_name(&self) -> &'static str {
        "bzip2"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_data() -> Vec<u8> {
        b"This is a test string that will be compressed and decompressed. It needs to be long enough to demonstrate compression effectiveness. The quick brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_vec()
    }
    
    #[test]
    fn test_zstd_compression() {
        let data = test_data();
        let algorithm = ZstdAlgorithm::new();
        
        let result = algorithm.compress(&data).unwrap();
        assert!(result.compressed_data.len() < data.len());
        assert!(result.stats.compression_ratio > 1.0);
        
        let decompressed = algorithm.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, data);
    }
    
    #[test]
    fn test_brotli_compression() {
        let data = test_data();
        let algorithm = BrotliAlgorithm::new();
        
        let result = algorithm.compress(&data).unwrap();
        assert!(result.compressed_data.len() < data.len());
        assert!(result.stats.compression_ratio > 1.0);
        
        let decompressed = algorithm.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, data);
    }
    
    #[test]
    fn test_gzip_compression() {
        let data = test_data();
        let algorithm = GzipAlgorithm::new();
        
        let result = algorithm.compress(&data).unwrap();
        assert!(result.compressed_data.len() < data.len());
        assert!(result.stats.compression_ratio > 1.0);
        
        let decompressed = algorithm.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, data);
    }
    
    #[test]
    fn test_lz4_compression() {
        let data = test_data();
        let algorithm = Lz4Algorithm::new();
        
        let result = algorithm.compress(&data).unwrap();
        assert!(result.compressed_data.len() < data.len());
        assert!(result.stats.compression_ratio > 1.0);
        
        let decompressed = algorithm.decompress(&result.compressed_data).unwrap();
        assert_eq!(decompressed, data);
    }
    
    #[test]
    fn test_compression_levels() {
        let data = test_data();
        
        // Test different compression levels
        let zstd_low = ZstdAlgorithm::with_level(1);
        let zstd_high = ZstdAlgorithm::with_level(22);
        
        let result_low = zstd_low.compress(&data).unwrap();
        let result_high = zstd_high.compress(&data).unwrap();
        
        // Higher compression level should generally produce smaller output
        // but this isn't guaranteed for all data, so we just check they both work
        assert!(result_low.compressed_data.len() < data.len());
        assert!(result_high.compressed_data.len() < data.len());
    }
}