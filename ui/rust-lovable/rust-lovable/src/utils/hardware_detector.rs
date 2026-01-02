//! Hardware detection utilities
//!
//! Provides hardware detection and capability reporting for optimal resource allocation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hardware detector for system capability analysis
pub struct HardwareDetector {
    /// Cached hardware info
    cached_info: Option<HardwareInfo>,
}

/// Comprehensive hardware information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    /// CPU information
    pub cpu: CpuInfo,
    /// Memory information
    pub memory: MemoryInfo,
    /// GPU information (if available)
    pub gpu: Option<GpuInfo>,
    /// Storage information
    pub storage: StorageInfo,
    /// Network capabilities
    pub network: NetworkInfo,
}

/// CPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    /// CPU model name
    pub model: String,
    /// Number of physical cores
    pub cores: u32,
    /// Number of logical processors (threads)
    pub threads: u32,
    /// Base frequency in MHz
    pub base_frequency_mhz: u32,
    /// Architecture (x86_64, aarch64, etc.)
    pub architecture: String,
    /// Supported features (AVX, SSE, etc.)
    pub features: Vec<String>,
}

/// Memory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Total physical memory in bytes
    pub total_bytes: u64,
    /// Available memory in bytes
    pub available_bytes: u64,
    /// Memory type (DDR4, DDR5, etc.)
    pub memory_type: String,
}

/// GPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    /// GPU model name
    pub model: String,
    /// VRAM in bytes
    pub vram_bytes: u64,
    /// Whether CUDA is available
    pub cuda_available: bool,
    /// CUDA compute capability
    pub cuda_compute_capability: Option<String>,
    /// Whether ROCm is available
    pub rocm_available: bool,
    /// Whether Metal is available (macOS)
    pub metal_available: bool,
}

/// Storage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    /// Available storage in bytes
    pub available_bytes: u64,
    /// Total storage in bytes
    pub total_bytes: u64,
    /// Storage type (SSD, HDD, NVMe)
    pub storage_type: String,
}

/// Network capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// Whether network is available
    pub available: bool,
    /// Estimated bandwidth in Mbps
    pub estimated_bandwidth_mbps: Option<u32>,
}

impl HardwareDetector {
    /// Create a new hardware detector
    pub fn new() -> Self {
        Self { cached_info: None }
    }

    /// Detect hardware capabilities
    pub fn detect(&mut self) -> HardwareInfo {
        if let Some(ref info) = self.cached_info {
            return info.clone();
        }

        let info = self.detect_hardware();
        self.cached_info = Some(info.clone());
        info
    }

    /// Force re-detection of hardware
    pub fn refresh(&mut self) -> HardwareInfo {
        self.cached_info = None;
        self.detect()
    }

    fn detect_hardware(&self) -> HardwareInfo {
        HardwareInfo {
            cpu: self.detect_cpu(),
            memory: self.detect_memory(),
            gpu: self.detect_gpu(),
            storage: self.detect_storage(),
            network: self.detect_network(),
        }
    }

    fn detect_cpu(&self) -> CpuInfo {
        // Use std::thread::available_parallelism for thread count
        let threads = std::thread::available_parallelism()
            .map(|p| p.get() as u32)
            .unwrap_or(1);

        CpuInfo {
            model: std::env::var("PROCESSOR_IDENTIFIER")
                .unwrap_or_else(|_| "Unknown CPU".to_string()),
            cores: threads / 2, // Approximation
            threads,
            base_frequency_mhz: 0, // Would need platform-specific detection
            architecture: std::env::consts::ARCH.to_string(),
            features: vec![],
        }
    }

    fn detect_memory(&self) -> MemoryInfo {
        // Simplified - would need platform-specific APIs for accurate detection
        MemoryInfo {
            total_bytes: 16 * 1024 * 1024 * 1024,    // Default 16GB
            available_bytes: 8 * 1024 * 1024 * 1024, // Default 8GB available
            memory_type: "Unknown".to_string(),
        }
    }

    fn detect_gpu(&self) -> Option<GpuInfo> {
        // Check for common GPU indicators
        #[cfg(target_os = "windows")]
        {
            // Check for CUDA
            if std::path::Path::new("C:\\Program Files\\NVIDIA GPU Computing Toolkit").exists() {
                return Some(GpuInfo {
                    model: "NVIDIA GPU".to_string(),
                    vram_bytes: 0,
                    cuda_available: true,
                    cuda_compute_capability: None,
                    rocm_available: false,
                    metal_available: false,
                });
            }
        }

        #[cfg(target_os = "macos")]
        {
            return Some(GpuInfo {
                model: "Apple GPU".to_string(),
                vram_bytes: 0,
                cuda_available: false,
                cuda_compute_capability: None,
                rocm_available: false,
                metal_available: true,
            });
        }

        None
    }

    fn detect_storage(&self) -> StorageInfo {
        StorageInfo {
            available_bytes: 100 * 1024 * 1024 * 1024, // Default 100GB
            total_bytes: 500 * 1024 * 1024 * 1024,     // Default 500GB
            storage_type: "Unknown".to_string(),
        }
    }

    fn detect_network(&self) -> NetworkInfo {
        NetworkInfo {
            available: true,
            estimated_bandwidth_mbps: None,
        }
    }

    /// Check if system meets minimum requirements for ML workloads
    pub fn meets_ml_requirements(&mut self) -> bool {
        let info = self.detect();

        // Minimum: 4 threads, 8GB RAM
        info.cpu.threads >= 4 && info.memory.total_bytes >= 8 * 1024 * 1024 * 1024
    }

    /// Get recommended batch size based on available memory
    pub fn recommended_batch_size(&mut self) -> u32 {
        let info = self.detect();

        if let Some(ref gpu) = info.gpu {
            if gpu.cuda_available && gpu.vram_bytes > 8 * 1024 * 1024 * 1024 {
                return 32; // Large GPU
            } else if gpu.cuda_available {
                return 16; // Small GPU
            }
        }

        // CPU-based batch size
        if info.memory.available_bytes > 16 * 1024 * 1024 * 1024 {
            8
        } else if info.memory.available_bytes > 8 * 1024 * 1024 * 1024 {
            4
        } else {
            1
        }
    }
}

impl Default for HardwareDetector {
    fn default() -> Self {
        Self::new()
    }
}
