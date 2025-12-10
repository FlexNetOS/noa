//! Platform capability detection
//!
//! T735: Implement hardware capability detection to steer resource adaptation.

use crate::error::Result;
use crate::neural::hardware::{CpuInfo, GpuDevice, HardwareDetector};
use crate::platform::detect::{platform_info, PlatformInfo};
use serde::{Deserialize, Serialize};

/// Hardware tier buckets for resource scaling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HardwareTier {
    Low,
    Medium,
    High,
    Unknown,
}

/// Snapshot of platform + hardware capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySnapshot {
    pub platform: PlatformInfo,
    pub cpu: CpuInfo,
    pub memory_bytes: u64,
    pub gpus: Vec<GpuDevice>,
    pub tier: HardwareTier,
}

impl CapabilitySnapshot {
    pub fn has_gpu(&self) -> bool {
        !self.gpus.is_empty()
    }
}

/// Detect capabilities for the current host.
pub async fn detect_capabilities() -> Result<CapabilitySnapshot> {
    let platform = platform_info();
    let detector = HardwareDetector::new();
    let cpu = detector.detect_cpu_info().await?;
    let memory_bytes = detector.get_available_memory().await?;
    let gpus = detector.detect_gpu_devices().await?;
    let tier = classify_hardware(cpu.threads, memory_bytes, gpus.len());

    Ok(CapabilitySnapshot {
        platform,
        cpu,
        memory_bytes,
        gpus,
        tier,
    })
}

fn classify_hardware(threads: usize, memory_bytes: u64, gpu_count: usize) -> HardwareTier {
    let memory_gb = memory_bytes as f64 / 1_073_741_824.0;

    if gpu_count > 0 && threads >= 12 && memory_gb >= 32.0 {
        HardwareTier::High
    } else if threads >= 8 && memory_gb >= 16.0 {
        HardwareTier::Medium
    } else if threads > 0 {
        HardwareTier::Low
    } else {
        HardwareTier::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn builds_snapshot() {
        let snapshot = detect_capabilities().await.unwrap();
        assert!(snapshot.cpu.threads > 0);
        assert!(matches!(
            snapshot.tier,
            HardwareTier::Low | HardwareTier::Medium | HardwareTier::High | HardwareTier::Unknown
        ));
    }

    #[test]
    fn classifies_tiers() {
        assert_eq!(classify_hardware(16, 64 * 1024 * 1024 * 1024, 1), HardwareTier::High);
        assert_eq!(classify_hardware(8, 16 * 1024 * 1024 * 1024, 0), HardwareTier::Medium);
        assert_eq!(classify_hardware(4, 4 * 1024 * 1024 * 1024, 0), HardwareTier::Low);
    }
}
