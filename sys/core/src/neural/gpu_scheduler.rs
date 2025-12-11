//! Multi-GPU Load Balancing
//!
//! T484: Add multi-GPU load balancing
//! US2: Balance inference load across multiple GPUs

use crate::error::Result;
use crate::neural::cuda_devices::CudaDevice;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Load balancing strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalanceStrategy {
    /// Round-robin scheduling
    RoundRobin,
    /// Least loaded device
    LeastLoaded,
    /// Performance-based (fastest device first)
    PerformanceBased,
    /// Memory-aware (most free memory)
    MemoryAware,
}

/// Device load information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceLoad {
    pub device_id: u32,
    pub active_inferences: usize,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub utilization_percent: f64,
}

/// Multi-GPU scheduler
pub struct GpuScheduler {
    loads: Arc<RwLock<HashMap<u32, DeviceLoad>>>,
    strategy: LoadBalanceStrategy,
    round_robin_index: Arc<RwLock<usize>>,
}

impl GpuScheduler {
    /// Create a new GPU scheduler
    pub fn new(strategy: LoadBalanceStrategy) -> Self {
        Self {
            loads: Arc::new(RwLock::new(HashMap::new())),
            strategy,
            round_robin_index: Arc::new(RwLock::new(0)),
        }
    }

    /// Register a device for scheduling
    pub async fn register_device(&self, device: &CudaDevice) -> Result<()> {
        let mut loads = self.loads.write().await;
        loads.insert(
            device.id,
            DeviceLoad {
                device_id: device.id,
                active_inferences: 0,
                memory_used_bytes: device.total_memory_bytes - device.free_memory_bytes,
                memory_total_bytes: device.total_memory_bytes,
                utilization_percent: 0.0,
            },
        );
        Ok(())
    }

    /// Select best device for inference
    pub async fn select_device(&self, available_devices: &[u32]) -> Result<Option<u32>> {
        if available_devices.is_empty() {
            return Ok(None);
        }

        let loads = self.loads.read().await;

        match self.strategy {
            LoadBalanceStrategy::RoundRobin => {
                let mut index = self.round_robin_index.write().await;
                let device_id = available_devices[*index % available_devices.len()];
                *index = (*index + 1) % available_devices.len();
                Ok(Some(device_id))
            }
            LoadBalanceStrategy::LeastLoaded => {
                let device = available_devices
                    .iter()
                    .filter_map(|&id| loads.get(&id))
                    .min_by(|a, b| a.active_inferences.cmp(&b.active_inferences))
                    .map(|load| load.device_id);
                Ok(device)
            }
            LoadBalanceStrategy::PerformanceBased => {
                // Select device with lowest utilization
                let device = available_devices
                    .iter()
                    .filter_map(|&id| loads.get(&id))
                    .min_by(|a, b| {
                        a.utilization_percent.partial_cmp(&b.utilization_percent).unwrap()
                    })
                    .map(|load| load.device_id);
                Ok(device)
            }
            LoadBalanceStrategy::MemoryAware => {
                // Select device with most free memory
                let device = available_devices
                    .iter()
                    .filter_map(|&id| loads.get(&id))
                    .max_by_key(|load| load.memory_total_bytes - load.memory_used_bytes)
                    .map(|load| load.device_id);
                Ok(device)
            }
        }
    }

    /// Update device load
    pub async fn update_load(
        &self,
        device_id: u32,
        active_inferences: usize,
        memory_used_bytes: u64,
    ) -> Result<()> {
        let mut loads = self.loads.write().await;
        if let Some(load) = loads.get_mut(&device_id) {
            load.active_inferences = active_inferences;
            load.memory_used_bytes = memory_used_bytes;
            load.utilization_percent =
                (memory_used_bytes as f64 / load.memory_total_bytes as f64) * 100.0;
        }
        Ok(())
    }

    /// Get device load
    pub async fn get_load(&self, device_id: u32) -> Option<DeviceLoad> {
        let loads = self.loads.read().await;
        loads.get(&device_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neural::cuda_devices::CudaDevice;

    #[tokio::test]
    async fn test_round_robin_scheduling() {
        let scheduler = GpuScheduler::new(LoadBalanceStrategy::RoundRobin);
        let devices = vec![0, 1, 2];

        let device1 = scheduler.select_device(&devices).await.unwrap();
        let device2 = scheduler.select_device(&devices).await.unwrap();
        let device3 = scheduler.select_device(&devices).await.unwrap();

        assert!(device1.is_some());
        assert!(device2.is_some());
        assert!(device3.is_some());
    }
}
