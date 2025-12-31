use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub sharing: ResourceSharingConfig,
    pub cache: ResourceCacheConfig,
    pub storage: StorageConfig,
    pub logs: LogConfig,
    pub prompts: SharedPromptsConfig,
    pub skills: SharedSkillsConfig,
    pub agents: SharedAgentsConfig,
    pub commands: SharedCommandsConfig,
    pub data: DataSharingConfig,
    pub embeddings: EmbeddingsSharingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSharingConfig {
    pub enabled: bool,
    pub mode: SharingMode,
    pub providers: Vec<SharingProvider>,
    pub encryption: EncryptionConfig,
    pub synchronization: SynchronizationConfig,
    pub access_control: AccessControlConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SharingMode {
    LocalOnly,
    Cluster,
    Distributed,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharingProvider {
    pub name: String,
    pub provider_type: ProviderType,
    pub config: HashMap<String, serde_json::Value>,
    pub priority: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    FileSystem,
    Redis,
    PostgreSQL,
    S3,
    AzureBlob,
    GoogleCloudStorage,
    CustomAPI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub key_rotation_days: u32,
    pub master_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizationConfig {
    pub enabled: bool,
    pub strategy: SyncStrategy,
    pub interval_seconds: u64,
    pub conflict_resolution: ConflictResolutionStrategy,
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncStrategy {
    RealTime,
    Periodic,
    OnDemand,
    EventDriven,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConflictResolutionStrategy {
    LastWriteWins,
    TimestampBased,
    VectorClock,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub enabled: bool,
    pub default_permission: PermissionLevel,
    pub user_permissions: HashMap<String, Vec<Permission>>,
    pub role_based_access: bool,
    pub api_key_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionLevel {
    None,
    Read,
    Write,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub resource: String,
    pub level: PermissionLevel,
    pub conditions: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCacheConfig {
    pub enabled: bool,
    pub max_size_mb: u64,
    pub ttl_seconds: u64,
    pub eviction_policy: EvictionPolicy,
    pub compression: CompressionConfig,
    pub metrics: CacheMetricsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EvictionPolicy {
    LRU,
    LFU,
    FIFO,
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub level: u32,
    pub threshold_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetricsConfig {
    pub enabled: bool,
    pub collection_interval_seconds: u64,
    pub metrics_to_collect: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub providers: Vec<StorageProvider>,
    pub default_provider: String,
    pub backup: BackupConfig,
    pub retention: RetentionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProvider {
    pub name: String,
    pub provider_type: StorageProviderType,
    pub config: HashMap<String, serde_json::Value>,
    pub enabled: bool,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageProviderType {
    LocalFileSystem,
    S3,
    AzureBlob,
    GoogleCloudStorage,
    PostgreSQL,
    MongoDB,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub enabled: bool,
    pub schedule: String,
    pub retention_days: u32,
    pub compression: bool,
    pub encryption: bool,
    pub providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionConfig {
    pub logs_days: u32,
    pub cache_days: u32,
    pub user_data_days: u32,
    pub analytics_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub enabled: bool,
    pub level: String,
    pub outputs: Vec<LogOutput>,
    pub format: LogFormat,
    pub rotation: LogRotationConfig,
    pub filters: LogFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogOutput {
    pub output_type: OutputType,
    pub config: HashMap<String, serde_json::Value>,
    pub filters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Console,
    File,
    Syslog,
    Elasticsearch,
    Loki,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Pretty,
    Structured,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    pub enabled: bool,
    pub max_size_mb: u64,
    pub max_files: u32,
    pub compress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilters {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub min_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedPromptsConfig {
    pub enabled: bool,
    pub sharing_mode: SharingMode,
    pub categories: Vec<PromptCategory>,
    pub versioning: PromptVersioningConfig,
    pub access_control: PromptAccessControlConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptCategory {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub access_level: PermissionLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptVersioningConfig {
    pub enabled: bool,
    pub max_versions: u32,
    pub auto_cleanup: bool,
    pub backup_old_versions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptAccessControlConfig {
    pub public_categories: Vec<String>,
    private_categories: Vec<String>,
    pub require_approval: bool,
    pub approval_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSkillsConfig {
    pub enabled: bool,
    pub registry: SkillsRegistryConfig,
    pub execution: SkillsExecutionConfig,
    pub sharing: SkillsSharingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsRegistryConfig {
    pub storage_provider: String,
    pub indexing_enabled: bool,
    pub validation_enabled: bool,
    pub auto_discovery: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsExecutionConfig {
    pub sandbox_enabled: bool,
    pub timeout_seconds: u64,
    pub memory_limit_mb: u64,
    pub allowed_imports: Vec<String>,
    pub security_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsSharingConfig {
    pub public_skills: Vec<String>,
    pub sharing_permissions: HashMap<String, Vec<String>>,
    pub collaboration_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedAgentsConfig {
    pub enabled: bool,
    pub registry: AgentRegistryConfig,
    pub orchestration: AgentOrchestrationConfig,
    pub communication: AgentCommunicationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistryConfig {
    pub storage_provider: String,
    pub lifecycle_management: bool,
    pub health_check_interval_seconds: u64,
    pub auto_scaling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOrchestrationConfig {
    pub enabled: bool,
    pub strategy: String,
    pub load_balancing: String,
    pub failover_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCommunicationConfig {
    pub protocol: String,
    pub message_queue: String,
    pub encryption_enabled: bool,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedCommandsConfig {
    pub enabled: bool,
    pub registry: CommandRegistryConfig,
    pub execution: CommandExecutionConfig,
    pub permissions: CommandPermissionsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRegistryConfig {
    pub storage_provider: String,
    pub versioning_enabled: bool,
    pub validation_enabled: bool,
    pub documentation_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionConfig {
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub retry_delay_seconds: u64,
    pub sandbox_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPermissionsConfig {
    pub require_admin_approval: bool,
    pub allowed_commands: Vec<String>,
    pub blocked_commands: Vec<String>,
    pub user_permissions: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSharingConfig {
    pub enabled: bool,
    pub categories: Vec<DataCategory>,
    pub anonymization: AnonymizationConfig,
    pub privacy: PrivacyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCategory {
    pub name: String,
    pub sensitivity: SensitivityLevel,
    pub retention_days: u32,
    pub sharing_allowed: bool,
    pub encryption_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SensitivityLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizationConfig {
    pub enabled: bool,
    pub techniques: Vec<String>,
    pub fields_to_anonymize: Vec<String>,
    pub irreversible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub gdpr_compliant: bool,
    pub ccpa_compliant: bool,
    pub data_residency: String,
    pub consent_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsSharingConfig {
    pub enabled: bool,
    pub vector_database: VectorDatabaseSharingConfig,
    pub model_sharing: ModelSharingConfig,
    pub index_sharing: IndexSharingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseSharingConfig {
    pub provider: String,
    pub sharing_mode: SharingMode,
    pub access_control: bool,
    pub federation_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSharingConfig {
    pub enabled: bool,
    public_models: Vec<String>,
    pub model_registry: String,
    pub version_control: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSharingConfig {
    pub enabled: bool,
    pub index_types: Vec<String>,
    pub sharing_permissions: HashMap<String, Vec<String>>,
    pub auto_sync: bool,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            sharing: ResourceSharingConfig {
                enabled: true,
                mode: SharingMode::LocalOnly,
                providers: vec![],
                encryption: EncryptionConfig {
                    enabled: true,
                    algorithm: "AES-256-GCM".to_string(),
                    key_rotation_days: 30,
                    master_key: None,
                },
                synchronization: SynchronizationConfig {
                    enabled: false,
                    strategy: SyncStrategy::Periodic,
                    interval_seconds: 300,
                    conflict_resolution: ConflictResolutionStrategy::TimestampBased,
                    batch_size: 100,
                },
                access_control: AccessControlConfig {
                    enabled: true,
                    default_permission: PermissionLevel::Read,
                    user_permissions: HashMap::new(),
                    role_based_access: true,
                    api_key_required: true,
                },
            },
            cache: ResourceCacheConfig {
                enabled: true,
                max_size_mb: 1024,
                ttl_seconds: 3600,
                eviction_policy: EvictionPolicy::LRU,
                compression: CompressionConfig {
                    enabled: true,
                    algorithm: "zstd".to_string(),
                    level: 3,
                    threshold_bytes: 1024,
                },
                metrics: CacheMetricsConfig {
                    enabled: true,
                    collection_interval_seconds: 60,
                    metrics_to_collect: vec![
                        "hits".to_string(),
                        "misses".to_string(),
                        "evictions".to_string(),
                        "size".to_string(),
                    ],
                },
            },
            storage: StorageConfig {
                providers: vec![StorageProvider {
                    name: "local".to_string(),
                    provider_type: StorageProviderType::LocalFileSystem,
                    config: HashMap::new(),
                    enabled: true,
                    read_only: false,
                }],
                default_provider: "local".to_string(),
                backup: BackupConfig {
                    enabled: true,
                    schedule: "0 2 * * *".to_string(),
                    retention_days: 30,
                    compression: true,
                    encryption: true,
                    providers: vec!["local".to_string()],
                },
                retention: RetentionConfig {
                    logs_days: 30,
                    cache_days: 7,
                    user_data_days: 365,
                    analytics_days: 90,
                },
            },
            logs: LogConfig {
                enabled: true,
                level: "info".to_string(),
                outputs: vec![LogOutput {
                    output_type: OutputType::Console,
                    config: HashMap::new(),
                    filters: vec![],
                }],
                format: LogFormat::Json,
                rotation: LogRotationConfig {
                    enabled: true,
                    max_size_mb: 100,
                    max_files: 10,
                    compress: true,
                },
                filters: LogFilters {
                    include: vec![],
                    exclude: vec![],
                    min_level: "info".to_string(),
                },
            },
            prompts: SharedPromptsConfig {
                enabled: true,
                sharing_mode: SharingMode::LocalOnly,
                categories: vec![
                    PromptCategory {
                        name: "ui_generation".to_string(),
                        description: "UI component generation prompts".to_string(),
                        tags: vec!["ui".to_string(), "dioxus".to_string()],
                        access_level: PermissionLevel::Write,
                    },
                ],
                versioning: PromptVersioningConfig {
                    enabled: true,
                    max_versions: 10,
                    auto_cleanup: true,
                    backup_old_versions: true,
                },
                access_control: PromptAccessControlConfig {
                    public_categories: vec!["examples".to_string()],
                    private_categories: vec!["internal".to_string()],
                    require_approval: false,
                    approval_process: "manual".to_string(),
                },
            },
            skills: SharedSkillsConfig {
                enabled: true,
                registry: SkillsRegistryConfig {
                    storage_provider: "local".to_string(),
                    indexing_enabled: true,
                    validation_enabled: true,
                    auto_discovery: true,
                },
                execution: SkillsExecutionConfig {
                    sandbox_enabled: true,
                    timeout_seconds: 300,
                    memory_limit_mb: 512,
                    allowed_imports: vec![
                        "std::".to_string(),
                        "tokio::".to_string(),
                        "serde::".to_string(),
                    ],
                    security_level: "strict".to_string(),
                },
                sharing: SkillsSharingConfig {
                    public_skills: vec!["utility".to_string()],
                    sharing_permissions: HashMap::new(),
                    collaboration_enabled: true,
                },
            },
            agents: SharedAgentsConfig {
                enabled: true,
                registry: AgentRegistryConfig {
                    storage_provider: "local".to_string(),
                    lifecycle_management: true,
                    health_check_interval_seconds: 60,
                    auto_scaling: true,
                },
                orchestration: AgentOrchestrationConfig {
                    enabled: true,
                    strategy: "round_robin".to_string(),
                    load_balancing: "least_connections".to_string(),
                    failover_enabled: true,
                },
                communication: AgentCommunicationConfig {
                    protocol: "grpc".to_string(),
                    message_queue: "redis".to_string(),
                    encryption_enabled: true,
                    compression_enabled: true,
                },
            },
            commands: SharedCommandsConfig {
                enabled: true,
                registry: CommandRegistryConfig {
                    storage_provider: "local".to_string(),
                    versioning_enabled: true,
                    validation_enabled: true,
                    documentation_required: true,
                },
                execution: CommandExecutionConfig {
                    timeout_seconds: 300,
                    retry_attempts: 3,
                    retry_delay_seconds: 5,
                    sandbox_enabled: true,
                },
                permissions: CommandPermissionsConfig {
                    require_admin_approval: false,
                    allowed_commands: vec!["cargo".to_string(), "rustc".to_string()],
                    blocked_commands: vec!["rm -rf /".to_string()],
                    user_permissions: HashMap::new(),
                },
            },
            data: DataSharingConfig {
                enabled: true,
                categories: vec![
                    DataCategory {
                        name: "user_data".to_string(),
                        sensitivity: SensitivityLevel::Confidential,
                        retention_days: 365,
                        sharing_allowed: false,
                        encryption_required: true,
                    },
                    DataCategory {
                        name: "analytics".to_string(),
                        sensitivity: SensitivityLevel::Internal,
                        retention_days: 90,
                        sharing_allowed: true,
                        encryption_required: false,
                    },
                ],
                anonymization: AnonymizationConfig {
                    enabled: true,
                    techniques: vec!["hashing".to_string(), "pseudonymization".to_string()],
                    fields_to_anonymize: vec!["email".to_string(), "username".to_string()],
                    irreversible: true,
                },
                privacy: PrivacyConfig {
                    gdpr_compliant: true,
                    ccpa_compliant: true,
                    data_residency: "EU".to_string(),
                    consent_required: true,
                },
            },
            embeddings: EmbeddingsSharingConfig {
                enabled: true,
                vector_database: VectorDatabaseSharingConfig {
                    provider: "pgvector".to_string(),
                    sharing_mode: SharingMode::LocalOnly,
                    access_control: true,
                    federation_enabled: false,
                },
                model_sharing: ModelSharingConfig {
                    enabled: false,
                    public_models: vec![],
                    model_registry: "huggingface".to_string(),
                    version_control: true,
                },
                index_sharing: IndexSharingConfig {
                    enabled: false,
                    index_types: vec!["hnsw".to_string()],
                    sharing_permissions: HashMap::new(),
                    auto_sync: false,
                },
            },
        }
    }
}