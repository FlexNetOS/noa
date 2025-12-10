//! Tensor Parallelism for Model Sharding
//!
//! T480: Implement tensor parallelism for model sharding
//! US2: Shard model tensors across multiple GPUs

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Tensor sharding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorShardConfig {
    pub shard_count: usize,
    pub shard_dimension: ShardDimension,
    pub replication_factor: usize,
}

/// Dimension to shard along
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShardDimension {
    /// Shard along feature dimension
    Feature,
    /// Shard along sequence dimension
    Sequence,
    /// Shard along batch dimension
    Batch,
}

/// Tensor parallelism manager
pub struct TensorParallelManager {
    shard_count: usize,
}

impl TensorParallelManager {
    /// Create a new tensor parallelism manager
    pub fn new(shard_count: usize) -> Self {
        Self { shard_count }
    }

    /// Shard a tensor across devices
    pub fn shard_tensor(
        &self,
        tensor_shape: &[usize],
        shard_dim: ShardDimension,
    ) -> Result<Vec<TensorShard>> {
        let mut shards = Vec::new();

        match shard_dim {
            ShardDimension::Feature => {
                if tensor_shape.len() >= 2 {
                    let feature_dim = tensor_shape[tensor_shape.len() - 1];
                    let shard_size = feature_dim / self.shard_count;
                    let remainder = feature_dim % self.shard_count;

                    let mut offset = 0;
                    for shard_id in 0..self.shard_count {
                        let size = shard_size + if shard_id < remainder { 1 } else { 0 };
                        shards.push(TensorShard {
                            shard_id,
                            offset,
                            size,
                            shape: {
                                let mut shape = tensor_shape.to_vec();
                                shape[tensor_shape.len() - 1] = size;
                                shape
                            },
                        });
                        offset += size;
                    }
                }
            }
            ShardDimension::Sequence => {
                if tensor_shape.len() >= 1 {
                    let seq_dim = tensor_shape[0];
                    let shard_size = seq_dim / self.shard_count;
                    let remainder = seq_dim % self.shard_count;

                    let mut offset = 0;
                    for shard_id in 0..self.shard_count {
                        let size = shard_size + if shard_id < remainder { 1 } else { 0 };
                        shards.push(TensorShard {
                            shard_id,
                            offset,
                            size,
                            shape: {
                                let mut shape = tensor_shape.to_vec();
                                shape[0] = size;
                                shape
                            },
                        });
                        offset += size;
                    }
                }
            }
            ShardDimension::Batch => {
                if tensor_shape.len() >= 1 {
                    let batch_dim = tensor_shape[0];
                    let shard_size = batch_dim / self.shard_count;
                    let remainder = batch_dim % self.shard_count;

                    let mut offset = 0;
                    for shard_id in 0..self.shard_count {
                        let size = shard_size + if shard_id < remainder { 1 } else { 0 };
                        shards.push(TensorShard {
                            shard_id,
                            offset,
                            size,
                            shape: {
                                let mut shape = tensor_shape.to_vec();
                                shape[0] = size;
                                shape
                            },
                        });
                        offset += size;
                    }
                }
            }
        }

        Ok(shards)
    }

    /// Gather shards back into full tensor
    pub fn gather_shards(&self, shards: &[TensorShard]) -> Result<Vec<usize>> {
        if shards.is_empty() {
            return Ok(vec![]);
        }

        // Reconstruct full shape from shards
        let mut full_shape = shards[0].shape.clone();

        // Sum the sharded dimension
        if !full_shape.is_empty() {
            full_shape[0] = shards.iter().map(|s| s.size).sum();
        }

        Ok(full_shape)
    }
}

/// Tensor shard information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorShard {
    pub shard_id: usize,
    pub offset: usize,
    pub size: usize,
    pub shape: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shard_tensor_feature() {
        let manager = TensorParallelManager::new(2);
        let shards = manager.shard_tensor(&[10, 100], ShardDimension::Feature).unwrap();
        assert_eq!(shards.len(), 2);
        assert_eq!(shards[0].size, 50);
        assert_eq!(shards[1].size, 50);
    }

    #[test]
    fn test_shard_tensor_sequence() {
        let manager = TensorParallelManager::new(3);
        let shards = manager.shard_tensor(&[100, 10], ShardDimension::Sequence).unwrap();
        assert_eq!(shards.len(), 3);
    }
}
