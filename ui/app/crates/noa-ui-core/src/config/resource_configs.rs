//! Resource configsuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resource configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resourceconfigs {
    pub memory: Memoryconfigs,
    pub storage: Storageconfigs,
    pub compute: Computeconfigs,
    pub limits: ResourceLimits,
}

/// Memory configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memoryconfigs {
    pub max_heap_mb: u64,
    pub cache_size_mb: u64,
    pub gc_threshold_mb: u64,
    pub enable_compression: bool,
}

/// Storage configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storageconfigs {
    pub data_path: String,
    pub temp_path: String,
    pub max_disk_usage_gb: u64,
    pub cleanup_interval_hours: u64,
}

/// Compute configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Computeconfigs {
    pub max_threads: usize,
    pub thread_pool_size: usize,
    pub async_runtime_threads: usize,
    pub enable_gpu: bool,
    pub gpu_memory_fraction: f64,
}

/// Resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_concurrent_requests: usize,
    pub max_request_size_mb: u64,
    pub max_response_size_mb: u64,
    pub request_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
}

impl Default for Resourceconfigs {
    fn default() -> Self {
        Self {
            memory: Memoryconfigs::default(),
            storage: Storageconfigs::default(),
            compute: Computeconfigs::default(),
            limits: ResourceLimits::default(),
        }
    }
}

impl Default for Memoryconfigs {
    fn default() -> Self {
        Self {
            max_heap_mb: 4096,
            cache_size_mb: 512,
            gc_threshold_mb: 256,
            enable_compression: true,
        }
    }
}

impl Default for Storageconfigs {
    fn default() -> Self {
        Self {
            data_path: "./data".to_string(),
            temp_path: "./tmp".to_string(),
            max_disk_usage_gb: 100,
            cleanup_interval_hours: 24,
        }
    }
}

impl Default for Computeconfigs {
    fn default() -> Self {
        let cpu_count = num_cpus::get();
        Self {
            max_threads: cpu_count,
            thread_pool_size: cpu_count,
            async_runtime_threads: cpu_count.min(4),
            enable_gpu: false,
            gpu_memory_fraction: 0.8,
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 100,
            max_request_size_mb: 50,
            max_response_size_mb: 100,
            request_timeout_seconds: 300,
            idle_timeout_seconds: 60,
        }
    }
}
