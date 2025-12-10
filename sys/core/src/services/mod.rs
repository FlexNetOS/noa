//! Services Module
//!
//! Business logic services for NOA core functionality.

pub mod digest;
pub mod memory_service;
pub mod search_service;
pub mod device_service;
pub mod neural_service;
pub mod model_download;
pub mod init_service;
pub mod agent_service;
pub mod task_service;
pub mod orchestration_service;
pub mod crm_strangler;

pub use memory_service::MemoryService;
pub use device_service::DeviceService;
pub use search_service::{SearchService, SearchResult};
pub use neural_service::NeuralService;
pub use model_download::{ModelDownloadService, DownloadProgress, DownloadStatus};
pub use init_service::{InitService, InitResult, VerificationResult};
pub use agent_service::AgentService;
pub use task_service::TaskService;
pub use orchestration_service::OrchestrationService;
pub use crm_strangler::{CrmStranglerService, StranglerMode};
