use crate::compression::{CompressionPriority, CompressionResult, CompressionStats};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct CacheCompressor {
    priority_algorithms: std::collections::HashMap<CompressionPriority, &'static str>,
}

impl CacheCompressor {
    pub fn new() -> Self {
        let mut priority_algorithms = std::collections::HashMap::new();
        priority_algorithms.insert(CompressionPriority::RealTime, "lz4");
        priority_algorithms.insert(CompressionPriority::High, "zstd");
        priority_algorithms.insert(CompressionPriority::Normal, "zstd");
        priority_algorithms.insert(CompressionPriority::Low, "brotli");

        Self {
            priority_algorithms,
        }
    }

    pub fn compress_with_priority(
        &self,
        data: &[u8],
        priority: CompressionPriority,
    ) -> Result<CompressionResult> {
        let algorithm = self.priority_algorithms.get(&priority).unwrap_or(&"zstd");

        match *algorithm {
            "lz4" => self.compress_lz4(data),
            "zstd" => self.compress_zstd(data),
            "brotli" => self.compress_brotli(data),
            _ => Err(anyhow::anyhow!("Unknown algorithm: {}", algorithm)),
        }
    }

    fn compress_lz4(&self, data: &[u8]) -> Result<CompressionResult> {
        let start_time = std::time::Instant::now();

        let compressed_data = lz4_flex::compress_prepend_size(data);
        let compression_time = start_time.elapsed().as_millis() as u64;

        let stats = CompressionStats {
            algorithm: "lz4_cache".to_string(),
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

    fn compress_zstd(&self, data: &[u8]) -> Result<CompressionResult> {
        let start_time = std::time::Instant::now();

        let compressed_data = zstd::encode_all(data, 3)?;
        let compression_time = start_time.elapsed().as_millis() as u64;

        let stats = CompressionStats {
            algorithm: "zstd_cache".to_string(),
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

    fn compress_brotli(&self, data: &[u8]) -> Result<CompressionResult> {
        let start_time = std::time::Instant::now();

        let mut compressed_data = Vec::new();
        let mut encoder = brotli::CompressorWriter::new(&mut compressed_data, 4096, 6, 22);

        std::io::Write::write_all(&mut encoder, data)?;
        std::io::Write::flush(&mut encoder)?;
        drop(encoder); // Ensure all data is written

        let compression_time = start_time.elapsed().as_millis() as u64;

        let stats = CompressionStats {
            algorithm: "brotli_cache".to_string(),
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

    pub fn should_compress(&self, data: &[u8], min_size_kb: usize) -> bool {
        data.len() > min_size_kb * 1024
    }

    pub fn estimate_compression_benefit(&self, data: &[u8], algorithm: &str) -> f64 {
        // Simple heuristic based on data patterns
        let entropy = self.calculate_entropy(data);
        let size = data.len();

        match algorithm {
            "lz4" => {
                if entropy < 0.5 && size > 1024 {
                    2.0 // 2x compression expected
                } else {
                    1.2 // 1.2x compression expected
                }
            }
            "zstd" => {
                if entropy < 0.7 {
                    3.0 // 3x compression expected
                } else {
                    1.5 // 1.5x compression expected
                }
            }
            "brotli" => {
                if entropy < 0.6 && size > 2048 {
                    4.0 // 4x compression expected
                } else {
                    1.8 // 1.8x compression expected
                }
            }
            _ => 1.0,
        }
    }

    fn calculate_entropy(&self, data: &[u8]) -> f64 {
        let mut frequency = [0u32; 256];

        for &byte in data {
            frequency[byte as usize] += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &frequency {
            if count > 0 {
                let probability = count as f64 / len;
                entropy -= probability * probability.log2();
            }
        }

        entropy / 8.0 // Normalize to 0-1 range
    }
}

pub struct AdaptiveCacheCompressor {
    base_compressor: CacheCompressor,
    performance_history: std::collections::VecDeque<CompressionPerformance>,
    max_history_size: usize,
}

#[derive(Debug, Clone)]
pub struct CompressionPerformance {
    pub algorithm: String,
    pub input_size: usize,
    pub output_size: usize,
    pub compression_time_ms: u64,
    pub decompression_time_ms: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AdaptiveCacheCompressor {
    pub fn new() -> Self {
        Self {
            base_compressor: CacheCompressor::new(),
            performance_history: std::collections::VecDeque::new(),
            max_history_size: 1000,
        }
    }

    pub fn compress_adaptive(
        &mut self,
        data: &[u8],
        priority: CompressionPriority,
    ) -> Result<CompressionResult> {
        // Based on performance history, choose the best algorithm
        let best_algorithm = self.select_best_algorithm(data, &priority);

        let start_time = std::time::Instant::now();
        let result = self
            .base_compressor
            .compress_with_priority(data, priority)?;
        let compression_time = start_time.elapsed().as_millis() as u64;

        // Record performance
        self.record_performance(CompressionPerformance {
            algorithm: result.stats.algorithm.clone(),
            input_size: data.len(),
            output_size: result.compressed_data.len(),
            compression_time_ms: compression_time,
            decompression_time_ms: 0, // Would measure actual decompression
            timestamp: chrono::Utc::now(),
        });

        Ok(result)
    }

    fn select_best_algorithm(&self, _data: &[u8], priority: &CompressionPriority) -> &'static str {
        // Analyze recent performance history
        let recent_performance: Vec<_> = self
            .performance_history
            .iter()
            .filter(|p| p.timestamp > chrono::Utc::now() - chrono::Duration::hours(1))
            .collect();

        if recent_performance.is_empty() {
            return self
                .base_compressor
                .priority_algorithms
                .get(priority)
                .unwrap_or(&"zstd");
        }

        // Calculate average performance metrics for each algorithm
        let mut algorithm_scores = std::collections::HashMap::<String, f64>::new();

        for perf in recent_performance {
            let score = self.calculate_algorithm_score(perf, priority);
            *algorithm_scores
                .entry(perf.algorithm.clone())
                .or_insert(0.0) += score;
        }

        // Choose the algorithm with the best score
        let best = algorithm_scores
            .into_iter()
            .max_by(|(_, score_a), (_, score_b)| score_a.partial_cmp(score_b).unwrap())
            .map(|(algorithm, _)| algorithm);

        match best.as_deref() {
            Some("lz4") => "lz4",
            Some("brotli") => "brotli",
            Some("gzip") => "gzip",
            _ => "zstd",
        }
    }

    fn calculate_algorithm_score(
        &self,
        perf: &CompressionPerformance,
        priority: &CompressionPriority,
    ) -> f64 {
        let compression_ratio = perf.input_size as f64 / perf.output_size as f64;
        let speed = perf.input_size as f64 / perf.compression_time_ms as f64;

        match priority {
            CompressionPriority::RealTime => speed * 2.0 + compression_ratio * 0.5,
            CompressionPriority::High => speed * 1.5 + compression_ratio * 1.0,
            CompressionPriority::Normal => speed * 1.0 + compression_ratio * 1.5,
            CompressionPriority::Low => speed * 0.5 + compression_ratio * 2.0,
        }
    }

    fn record_performance(&mut self, performance: CompressionPerformance) {
        self.performance_history.push_back(performance);

        if self.performance_history.len() > self.max_history_size {
            self.performance_history.pop_front();
        }
    }

    pub fn get_performance_stats(&self) -> CacheCompressionStats {
        let total_operations = self.performance_history.len();
        let mut total_compression_ratio = 0.0;
        let mut total_compression_time = 0.0;
        let mut algorithm_usage = std::collections::HashMap::new();

        for perf in &self.performance_history {
            let compression_ratio = perf.input_size as f64 / perf.output_size as f64;
            total_compression_ratio += compression_ratio;
            total_compression_time += perf.compression_time_ms as f64;

            *algorithm_usage.entry(perf.algorithm.clone()).or_insert(0) += 1;
        }

        CacheCompressionStats {
            total_operations,
            average_compression_ratio: if total_operations > 0 {
                total_compression_ratio / total_operations as f64
            } else {
                1.0
            },
            average_compression_time_ms: if total_operations > 0 {
                total_compression_time / total_operations as f64
            } else {
                0.0
            },
            algorithm_usage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheCompressionStats {
    pub total_operations: usize,
    pub average_compression_ratio: f64,
    pub average_compression_time_ms: f64,
    pub algorithm_usage: std::collections::HashMap<String, usize>,
}
