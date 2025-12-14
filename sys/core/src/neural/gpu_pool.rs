//! GPU Memory Pooling Across Devices
//!
//! T482: Add GPU memory pooling across devices
//! US2: Pool GPU memory across multiple devices

use crate::error::Result;
use crate::neural::cuda_devices::CudaDevice;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GPU memory pool entry
#[derive(Debug, Clone)]
struct MemoryPoolEntry {
    device_id: u32,
    size_bytes: u64,
    allocated: bool,
}

/// GPU memory pool manager
pub struct GpuMemoryPool {
    pools: Arc<RwLock<HashMap<u32, Vec<MemoryPoolEntry>>>>,
}

impl GpuMemoryPool {
    /// Create a new GPU memory pool
    pub fn new() -> Self {
        Self {
            pools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize memory pool for a device
    pub async fn initialize_pool(&self, device_id: u32, total_memory_bytes: u64, chunk_size_bytes: u64) -> Result<()> {
        let mut pools = self.pools.write().await;
        let chunk_count = (total_memory_bytes / chunk_size_bytes) as usize;

        let mut entries = Vec::with_capacity(chunk_count);
        for _ in 0..chunk_count {
            entries.push(MemoryPoolEntry {
                device_id,
                size_bytes: chunk_size_bytes,
                allocated: false,
            });
        }

        pools.insert(device_id, entries);
        Ok(())
    }

    /// Allocate memory from pool
    pub async fn allocate(&self, device_id: u32, size_bytes: u64) -> Result<Option<MemoryAllocation>> {
        let mut pools = self.pools.write().await;

        if let Some(entries) = pools.get_mut(&device_id) {
            // Find contiguous chunks that can satisfy the request
            let chunks_needed = ((size_bytes as f64 / entries[0].size_bytes as f64).ceil()) as usize;

            let mut start_index = None;
            let mut contiguous_count = 0;

            for (i, entry) in entries.iter().enumerate() {
                if !entry.allocated {
                    if start_index.is_none() {
                        start_index = Some(i);
                    }
                    contiguous_count += 1;
                    if contiguous_count >= chunks_needed {
                        break;
                    }
                } else {
                    start_index = None;
                    contiguous_count = 0;
                }
            }

            if let Some(start) = start_index {
                if contiguous_count >= chunks_needed {
                    // Mark chunks as allocated
                    for i in start..(start + chunks_needed) {
                        entries[i].allocated = true;
                    }

                    return Ok(Some(MemoryAllocation {
                        device_id,
                        start_chunk: start,
                        chunk_count: chunks_needed,
                        total_bytes: chunks_needed as u64 * entries[0].size_bytes,
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Deallocate memory back to pool
    pub async fn deallocate(&self, allocation: MemoryAllocation) -> Result<()> {
        let mut pools = self.pools.write().await;

        if let Some(entries) = pools.get_mut(&allocation.device_id) {
            for i in allocation.start_chunk..(allocation.start_chunk + allocation.chunk_count) {
                if i < entries.len() {
                    entries[i].allocated = false;
                }
            }
        }

        Ok(())
    }

    /// Get pool status
    pub async fn get_status(&self, device_id: u32) -> Result<Option<PoolStatus>> {
        let pools = self.pools.read().await;

        if let Some(entries) = pools.get(&device_id) {
            let total_chunks = entries.len();
            let allocated_chunks = entries.iter().filter(|e| e.allocated).count();
            let free_chunks = total_chunks - allocated_chunks;
            let chunk_size = entries.first().map(|e| e.size_bytes).unwrap_or(0);

            Ok(Some(PoolStatus {
                device_id,
                total_chunks,
                allocated_chunks,
                free_chunks,
                chunk_size_bytes: chunk_size,
                total_bytes: total_chunks as u64 * chunk_size,
                allocated_bytes: allocated_chunks as u64 * chunk_size,
                free_bytes: free_chunks as u64 * chunk_size,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Default for GpuMemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory allocation from pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAllocation {
    pub device_id: u32,
    pub start_chunk: usize,
    pub chunk_count: usize,
    pub total_bytes: u64,
}

/// Pool status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStatus {
    pub device_id: u32,
    pub total_chunks: usize,
    pub allocated_chunks: usize,
    pub free_chunks: usize,
    pub chunk_size_bytes: u64,
    pub total_bytes: u64,
    pub allocated_bytes: u64,
    pub free_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_pool() {
        let pool = GpuMemoryPool::new();
        pool.initialize_pool(0, 8 * 1024 * 1024 * 1024, 64 * 1024 * 1024).await.unwrap();

        let allocation = pool.allocate(0, 128 * 1024 * 1024).await.unwrap();
        assert!(allocation.is_some());

        if let Some(alloc) = allocation {
            pool.deallocate(alloc).await.unwrap();
        }
    }
}

