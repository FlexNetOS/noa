//! Hardware Detection and Quantization
//!
//! T110: Implement quantization detection and layer offloading
//! §3.2: Local-First & Offline-Capable
//! US2: Hardware-aware model loading

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Hardware detector for GPU/CPU capabilities
pub struct HardwareDetector;

impl HardwareDetector {
    /// Create a new hardware detector
    pub fn new() -> Self {
        Self
    }

    /// Detect available GPU devices
    pub async fn detect_gpu_devices(&self) -> Result<Vec<GpuDevice>> {
        // TODO: Implement actual GPU detection (T478-T485)
        // For now, return empty list
        Ok(vec![])
    }

    /// Detect CPU capabilities
    pub async fn detect_cpu_info(&self) -> Result<CpuInfo> {
        // Get CPU count using std::thread
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        Ok(CpuInfo {
            cores: cpu_count,
            threads: cpu_count, // Simplified: assume 1 thread per core
        })
    }

    /// Get total available memory (bytes)
    pub async fn get_available_memory(&self) -> Result<u64> {
        // TODO: Implement actual memory detection
        // For now, return a default value
        Ok(8 * 1024 * 1024 * 1024) // 8GB default
    }
}

impl Default for HardwareDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// GPU device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    pub id: u32,
    pub name: String,
    pub memory_bytes: u64,
    pub compute_capability: Option<String>,
}

/// CPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub cores: usize,
    pub threads: usize,
}

/// Quantization detector for GGUF files
pub struct QuantizationDetector;

impl QuantizationDetector {
    /// Create a new quantization detector
    pub fn new() -> Self {
        Self
    }

    /// Detect quantization type from GGUF file
    pub async fn detect_quantization(&self, model_path: &Path) -> Result<Option<QuantizationType>> {
        // Check file extension
        if !model_path.extension().map(|e| e == "gguf").unwrap_or(false) {
            return Ok(None);
        }

        // Extract quantization from filename
        // Common patterns: model-q4_0.gguf, model-q8_0.gguf, etc.
        let filename = model_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if filename.contains("q4_0") {
            Ok(Some(QuantizationType::Q4_0))
        } else if filename.contains("q4_1") {
            Ok(Some(QuantizationType::Q4_1))
        } else if filename.contains("q5_0") {
            Ok(Some(QuantizationType::Q5_0))
        } else if filename.contains("q5_1") {
            Ok(Some(QuantizationType::Q5_1))
        } else if filename.contains("q8_0") {
            Ok(Some(QuantizationType::Q8_0))
        } else if filename.contains("f16") {
            Ok(Some(QuantizationType::F16))
        } else if filename.contains("f32") {
            Ok(Some(QuantizationType::F32))
        } else {
            Ok(None)
        }
    }

    /// Get recommended GPU layers based on quantization and available memory
    pub async fn recommend_gpu_layers(
        &self,
        quantization: Option<QuantizationType>,
        available_memory_gb: f64,
    ) -> i32 {
        // Simple heuristic: more memory and lower quantization = more GPU layers
        let base_layers = match quantization {
            Some(QuantizationType::Q4_0) | Some(QuantizationType::Q4_1) => 35,
            Some(QuantizationType::Q5_0) | Some(QuantizationType::Q5_1) => 30,
            Some(QuantizationType::Q8_0) => 25,
            Some(QuantizationType::F16) => 20,
            Some(QuantizationType::F32) => 10,
            None => 20, // Default
        };

        // Scale based on available memory
        let memory_factor = (available_memory_gb / 8.0).min(2.0); // Cap at 2x for 16GB+
        (base_layers as f64 * memory_factor) as i32
    }
}

impl Default for QuantizationDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Quantization types for GGUF models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantizationType {
    Q4_0,
    Q4_1,
    Q5_0,
    Q5_1,
    Q8_0,
    F16,
    F32,
}

impl QuantizationType {
    /// Get bits per parameter
    pub fn bits_per_param(&self) -> f32 {
        match self {
            QuantizationType::Q4_0 | QuantizationType::Q4_1 => 4.0,
            QuantizationType::Q5_0 | QuantizationType::Q5_1 => 5.0,
            QuantizationType::Q8_0 => 8.0,
            QuantizationType::F16 => 16.0,
            QuantizationType::F32 => 32.0,
        }
    }

    /// Get memory efficiency (lower is better)
    pub fn memory_efficiency(&self) -> f32 {
        32.0 / self.bits_per_param()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_quantization_detection() {
        let detector = QuantizationDetector::new();
        let dir = tempdir().unwrap();

        let model_path = dir.path().join("model-q4_0.gguf");
        std::fs::write(&model_path, b"dummy").unwrap();

        let quant = detector.detect_quantization(&model_path).await.unwrap();
        assert_eq!(quant, Some(QuantizationType::Q4_0));
    }

    #[tokio::test]
    async fn test_recommend_gpu_layers() {
        let detector = QuantizationDetector::new();

        let layers = detector.recommend_gpu_layers(Some(QuantizationType::Q4_0), 16.0).await;
        assert!(layers > 0);
    }
}

