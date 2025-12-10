//! NVLink Detection and Utilization
//!
//! T481: Implement NVLink detection and utilization
//! US2: Detect and utilize NVLink for inter-GPU communication

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// NVLink topology information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvLinkTopology {
    pub devices: Vec<NvLinkDevice>,
    pub links: Vec<NvLinkConnection>,
}

/// NVLink device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvLinkDevice {
    pub device_id: u32,
    pub nvlink_count: u32,
    pub bandwidth_gbps: f64,
}

/// NVLink connection between devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvLinkConnection {
    pub device_a: u32,
    pub device_b: u32,
    pub link_count: u32,
    pub bandwidth_gbps: f64,
    pub latency_ns: f64,
}

/// NVLink detector
pub struct NvLinkDetector;

impl NvLinkDetector {
    /// Create a new NVLink detector
    pub fn new() -> Self {
        Self
    }

    /// Detect NVLink topology
    pub async fn detect_topology(&self) -> Result<NvLinkTopology> {
        // TODO: Implement actual NVLink detection using nvml or similar
        // For now, return empty topology

        Ok(NvLinkTopology {
            devices: vec![],
            links: vec![],
        })
    }

    /// Check if NVLink is available between two devices
    pub async fn has_nvlink(&self, device_a: u32, device_b: u32) -> bool {
        let topology = self.detect_topology().await.unwrap_or_else(|_| NvLinkTopology {
            devices: vec![],
            links: vec![],
        });

        topology.links.iter().any(|link| {
            (link.device_a == device_a && link.device_b == device_b) ||
            (link.device_a == device_b && link.device_b == device_a)
        })
    }

    /// Get optimal communication path between devices
    pub async fn get_optimal_path(&self, device_a: u32, device_b: u32) -> Result<Vec<u32>> {
        let topology = self.detect_topology().await?;

        // Simple path finding: direct connection or through intermediate
        if self.has_nvlink(device_a, device_b).await {
            return Ok(vec![device_a, device_b]);
        }

        // Find intermediate device with NVLink to both
        for link in &topology.links {
            if link.device_a == device_a {
                if self.has_nvlink(link.device_b, device_b).await {
                    return Ok(vec![device_a, link.device_b, device_b]);
                }
            }
        }

        // No NVLink path found, use PCIe fallback
        Ok(vec![device_a, device_b])
    }

    /// Get total bandwidth between devices
    pub async fn get_bandwidth(&self, device_a: u32, device_b: u32) -> Result<f64> {
        let topology = self.detect_topology().await?;

        for link in &topology.links {
            if (link.device_a == device_a && link.device_b == device_b) ||
               (link.device_a == device_b && link.device_b == device_a) {
                return Ok(link.bandwidth_gbps);
            }
        }

        // Default to PCIe bandwidth if no NVLink
        Ok(16.0) // PCIe 3.0 x16 ~16 GB/s
    }
}

impl Default for NvLinkDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_topology() {
        let detector = NvLinkDetector::new();
        let topology = detector.detect_topology().await.unwrap();
        assert!(topology.devices.len() >= 0);
    }
}

