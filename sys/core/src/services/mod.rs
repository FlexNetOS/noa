//! Services Module
//!
//! Business logic services for NOA core functionality.

pub mod agent_service;
pub mod crm_strangler;
pub mod device_service;
pub mod digest;
pub mod digest_service;
pub mod init_service;
pub mod memory_service;
pub mod model_download;
pub mod neural_service;
pub mod orchestration_service;
pub mod search_service;
pub mod task_service;

pub use agent_service::AgentService;
pub use crm_strangler::{CrmStranglerService, StranglerMode};
pub use device_service::DeviceService;
pub use digest_service::DigestService;
pub use init_service::{InitResult, InitService, VerificationResult};
pub use memory_service::MemoryService;
pub use model_download::{DownloadProgress, DownloadStatus, ModelDownloadService};
pub use neural_service::NeuralService;
pub use orchestration_service::OrchestrationService;
pub use search_service::{SearchResult, SearchService};
pub use task_service::TaskService;
