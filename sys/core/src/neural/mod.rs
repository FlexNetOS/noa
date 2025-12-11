//! Neural Runtime Module
//!
//! Implements neural runtime for multi-SLM inference.
//! §3.2: Local-First & Offline-Capable
//! US2: Multi-SLM Neural Runtime

pub mod model_loader;
pub mod llama_backend;
pub mod context;
pub mod inference;
pub mod hardware;
pub mod benchmark;
pub mod export;
pub mod cuda_devices;
pub mod multi_gpu;
pub mod tensor_parallel;
pub mod nvlink;
pub mod gpu_pool;
pub mod cuda_tiles;
pub mod gpu_scheduler;
pub mod gpu_health;

pub use model_loader::ModelLoader;
pub use llama_backend::LlamaBackend;
pub use context::InferenceContext;
pub use inference::InferenceEngine;
pub use hardware::{HardwareDetector, QuantizationDetector};
pub use benchmark::ModelBenchmark;
pub use export::ModelExporter;
pub use cuda_devices::{CudaDeviceEnumerator, CudaDevice};
pub use multi_gpu::{MultiGpuDistributor, DistributionStrategy};
pub use tensor_parallel::{TensorParallelManager, ShardDimension};
pub use nvlink::NvLinkDetector;
pub use gpu_pool::{GpuMemoryPool, MemoryAllocation};
pub use cuda_tiles::{CudaTilesManager, CudaTilesConfig};
pub use gpu_scheduler::{GpuScheduler, LoadBalanceStrategy};
pub use gpu_health::{GpuHealthMonitor, GpuHealthStatus};

