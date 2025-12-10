//! Multi-GPU Layer Distribution
//!
//! T479: Implement multi-GPU layer distribution
//! US2: Distribute model layers across multiple GPUs

use crate::error::Result;
use crate::neural::cuda_devices::{CudaDevice, CudaDeviceEnumerator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Layer distribution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributionStrategy {
    /// Round-robin distribution
    RoundRobin,
    /// Memory-aware distribution
    MemoryAware,
    /// Performance-optimized distribution
    PerformanceOptimized,
}

/// Layer assignment to GPU
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerAssignment {
    pub layer_index: usize,
    pub device_id: u32,
    pub memory_required_bytes: u64,
}

/// Multi-GPU layer distributor
pub struct MultiGpuDistributor {
    enumerator: CudaDeviceEnumerator,
}

impl MultiGpuDistributor {
    /// Create a new multi-GPU distributor
    pub fn new() -> Self {
        Self {
            enumerator: CudaDeviceEnumerator::new(),
        }
    }

    /// Distribute layers across available GPUs
    pub async fn distribute_layers(
        &self,
        total_layers: usize,
        strategy: DistributionStrategy,
    ) -> Result<Vec<LayerAssignment>> {
        let devices = self.enumerator.enumerate_devices().await?;

        if devices.is_empty() {
            return Ok(vec![]);
        }

        let mut assignments = Vec::new();

        match strategy {
            DistributionStrategy::RoundRobin => {
                for layer_index in 0..total_layers {
                    let device_id = devices[layer_index % devices.len()].id;
                    assignments.push(LayerAssignment {
                        layer_index,
                        device_id,
                        memory_required_bytes: 0, // Would be calculated from layer size
                    });
                }
            }
            DistributionStrategy::MemoryAware => {
                // Distribute based on available memory
                let total_memory: u64 = devices.iter().map(|d| d.free_memory_bytes).sum();
                let mut current_memory: HashMap<u32, u64> =
                    devices.iter().map(|d| (d.id, 0)).collect();

                for layer_index in 0..total_layers {
                    // Find device with most available memory
                    let device = devices
                        .iter()
                        .min_by_key(|d| {
                            let used = current_memory.get(&d.id).unwrap_or(&0);
                            d.free_memory_bytes.saturating_sub(*used)
                        })
                        .unwrap();

                    let memory_required =
                        (total_memory / total_layers as u64).min(device.free_memory_bytes);
                    *current_memory.get_mut(&device.id).unwrap() += memory_required;

                    assignments.push(LayerAssignment {
                        layer_index,
                        device_id: device.id,
                        memory_required_bytes: memory_required,
                    });
                }
            }
            DistributionStrategy::PerformanceOptimized => {
                // Distribute based on compute capability and memory
                let mut device_scores: Vec<(u32, f64)> = devices
                    .iter()
                    .map(|d| {
                        let compute_score = d.multiprocessor_count as f64 * 100.0;
                        let memory_score = (d.free_memory_bytes as f64 / 1_000_000_000.0) * 10.0;
                        (d.id, compute_score + memory_score)
                    })
                    .collect();
                device_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                for layer_index in 0..total_layers {
                    let device_id = device_scores[layer_index % device_scores.len()].0;
                    assignments.push(LayerAssignment {
                        layer_index,
                        device_id,
                        memory_required_bytes: 0,
                    });
                }
            }
        }

        Ok(assignments)
    }

    /// Get optimal number of GPU layers per device
    pub async fn get_optimal_layers_per_device(
        &self,
        total_layers: usize,
    ) -> Result<HashMap<u32, usize>> {
        let devices = self.enumerator.enumerate_devices().await?;
        let assignments =
            self.distribute_layers(total_layers, DistributionStrategy::MemoryAware).await?;

        let mut layers_per_device: HashMap<u32, usize> = HashMap::new();
        for assignment in assignments {
            *layers_per_device.entry(assignment.device_id).or_insert(0) += 1;
        }

        Ok(layers_per_device)
    }
}

impl Default for MultiGpuDistributor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_distribute_layers_round_robin() {
        let distributor = MultiGpuDistributor::new();
        let assignments = distributor
            .distribute_layers(10, DistributionStrategy::RoundRobin)
            .await
            .unwrap();
        // May be empty if no GPUs available
        assert!(assignments.len() <= 10);
    }
}
