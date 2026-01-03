//! Unified compression API for NOA
//!
//! Provides a consistent interface for compression across all NOA crates.
//! Enable the `compression` feature to use this module.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// Compression algorithm selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Algorithm {
    /// Gzip compression (good compatibility)
    Gzip,
    /// Zstandard compression (fast, good ratio) - default
    #[default]
    Zstd,
    /// LZ4 compression (very fast)
    Lz4,
    /// Brotli compression (best ratio for text)
    Brotli,
}

impl Algorithm {
    /// Get algorithm name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Algorithm::Gzip => "gzip",
            Algorithm::Zstd => "zstd",
            Algorithm::Lz4 => "lz4",
            Algorithm::Brotli => "brotli",
        }
    }

    /// Parse algorithm from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "gzip" | "gz" => Some(Algorithm::Gzip),
            "zstd" | "zstandard" => Some(Algorithm::Zstd),
            "lz4" => Some(Algorithm::Lz4),
            "brotli" | "br" => Some(Algorithm::Brotli),
            _ => None,
        }
    }
}

/// Compression level (1-22, algorithm dependent)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Level(u32);

impl Level {
    pub const FAST: Self = Level(1);
    pub const DEFAULT: Self = Level(6);
    pub const BEST: Self = Level(19);

    pub fn new(level: u32) -> Self {
        Level(level.min(22))
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Default for Level {
    fn default() -> Self {
        Level::DEFAULT
    }
}

/// Compression statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub algorithm: Algorithm,
}

impl CompressionStats {
    pub fn new(original: usize, compressed: usize, algorithm: Algorithm) -> Self {
        let ratio = if compressed > 0 {
            original as f64 / compressed as f64
        } else {
            1.0
        };
        Self {
            original_size: original as u64,
            compressed_size: compressed as u64,
            compression_ratio: ratio,
            algorithm,
        }
    }
}

/// Compress data using the specified algorithm
pub fn compress(data: &[u8], algorithm: Algorithm, level: Level) -> Result<Vec<u8>> {
    match algorithm {
        Algorithm::Gzip => compress_gzip(data, level),
        Algorithm::Zstd => compress_zstd(data, level),
        Algorithm::Lz4 => compress_lz4(data),
        Algorithm::Brotli => compress_brotli(data, level),
    }
}

/// Decompress data using the specified algorithm
pub fn decompress(data: &[u8], algorithm: Algorithm) -> Result<Vec<u8>> {
    match algorithm {
        Algorithm::Gzip => decompress_gzip(data),
        Algorithm::Zstd => decompress_zstd(data),
        Algorithm::Lz4 => decompress_lz4(data),
        Algorithm::Brotli => decompress_brotli(data),
    }
}

/// Compress data and return with stats
pub fn compress_with_stats(
    data: &[u8],
    algorithm: Algorithm,
    level: Level,
) -> Result<(Vec<u8>, CompressionStats)> {
    let compressed = compress(data, algorithm, level)?;
    let stats = CompressionStats::new(data.len(), compressed.len(), algorithm);
    Ok((compressed, stats))
}

// Gzip implementation
fn compress_gzip(data: &[u8], level: Level) -> Result<Vec<u8>> {
    use flate2::write::GzEncoder;
    use flate2::Compression;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::new(level.as_u32()));
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
    use flate2::read::GzDecoder;

    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

// Zstd implementation
fn compress_zstd(data: &[u8], level: Level) -> Result<Vec<u8>> {
    Ok(zstd::encode_all(data, level.as_u32() as i32)?)
}

fn decompress_zstd(data: &[u8]) -> Result<Vec<u8>> {
    Ok(zstd::decode_all(data)?)
}

// LZ4 implementation
fn compress_lz4(data: &[u8]) -> Result<Vec<u8>> {
    Ok(lz4_flex::compress_prepend_size(data))
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>> {
    lz4_flex::decompress_size_prepended(data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()).into())
}

// Brotli implementation
fn compress_brotli(data: &[u8], level: Level) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let params = brotli::enc::BrotliEncoderParams {
        quality: level.as_u32() as i32,
        ..Default::default()
    };
    brotli::BrotliCompress(&mut std::io::Cursor::new(data), &mut output, &params)?;
    Ok(output)
}

fn decompress_brotli(data: &[u8]) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    brotli::BrotliDecompress(&mut std::io::Cursor::new(data), &mut output)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gzip_roundtrip() {
        let data = b"Hello, World! This is test data for compression.";
        let compressed = compress(data, Algorithm::Gzip, Level::DEFAULT).unwrap();
        let decompressed = decompress(&compressed, Algorithm::Gzip).unwrap();
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[test]
    fn test_zstd_roundtrip() {
        let data = b"Hello, World! This is test data for compression.";
        let compressed = compress(data, Algorithm::Zstd, Level::DEFAULT).unwrap();
        let decompressed = decompress(&compressed, Algorithm::Zstd).unwrap();
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[test]
    fn test_lz4_roundtrip() {
        let data = b"Hello, World! This is test data for compression.";
        let compressed = compress(data, Algorithm::Lz4, Level::DEFAULT).unwrap();
        let decompressed = decompress(&compressed, Algorithm::Lz4).unwrap();
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[test]
    fn test_brotli_roundtrip() {
        let data = b"Hello, World! This is test data for compression.";
        let compressed = compress(data, Algorithm::Brotli, Level::DEFAULT).unwrap();
        let decompressed = decompress(&compressed, Algorithm::Brotli).unwrap();
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[test]
    fn test_compression_stats() {
        let data = b"Hello, World! This is test data for compression. Repeat for better ratio. Hello, World! This is test data for compression.";
        let (compressed, stats) = compress_with_stats(data, Algorithm::Gzip, Level::DEFAULT).unwrap();
        assert_eq!(stats.original_size, data.len() as u64);
        assert_eq!(stats.compressed_size, compressed.len() as u64);
        assert!(stats.compression_ratio >= 1.0);
    }
}
