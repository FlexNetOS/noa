//! Neural Runtime Module
//!
//! Implements neural runtime for multi-SLM inference.
//! §3.2: Local-First & Offline-Capable
//! US2: Multi-SLM Neural Runtime

pub mod benchmark;
pub mod context;
pub mod cuda_devices;
pub mod cuda_tiles;
pub mod export;
pub mod gpu_health;
pub mod gpu_pool;
pub mod gpu_scheduler;
pub mod hardware;
pub mod inference;
pub mod llama_backend;
pub mod model_loader;
pub mod multi_gpu;
pub mod nvlink;
pub mod tensor_parallel;

#[cfg(test)]
mod context_test;
#[cfg(test)]
mod inference_test;
#[cfg(test)]
mod model_loader_test;
#[cfg(test)]
mod phase10_verification_test;

pub use benchmark::ModelBenchmark;
pub use context::InferenceContext;
pub use cuda_devices::{CudaDevice, CudaDeviceEnumerator};
pub use cuda_tiles::{CudaTilesConfig, CudaTilesManager};
pub use export::ModelExporter;
pub use gpu_health::{GpuHealthMonitor, GpuHealthStatus};
pub use gpu_pool::{GpuMemoryPool, MemoryAllocation};
pub use gpu_scheduler::{GpuScheduler, LoadBalanceStrategy};
pub use hardware::{HardwareDetector, QuantizationDetector};
pub use inference::InferenceEngine;
pub use llama_backend::LlamaBackend;
pub use model_loader::ModelLoader;
pub use multi_gpu::{DistributionStrategy, MultiGpuDistributor};
pub use nvlink::NvLinkDetector;
pub use tensor_parallel::{ShardDimension, TensorParallelManager};
