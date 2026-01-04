//! # NOA UI MLOps
//!
//! ML DevOps platform for NOA UI providing:
//! - Pipeline orchestration
//! - Experiment tracking
//! - Model registry
//! - ML monitoring
//! - Model deployment
//! - Feature store
//! - Alert management

pub mod deployment;
pub mod experiment;
pub mod feature_store;
pub mod model_registry;
pub mod monitoring;
pub mod pipeline;

pub use deployment::{
    Deployment, DeploymentManager, DeploymentState, DeploymentStatistics, DeploymentStatus,
    DeploymentStrategy, HealthCheck as DeploymentHealthCheck, ResourceRequirements as DeploymentResourceRequirements,
};
pub use experiment::{Artifact, ArtifactType, Experiment, ExperimentStatus, ExperimentTracker};
pub use feature_store::{Feature, FeatureGroup, FeatureStore, OfflineStoreconfigs, OnlineStoreconfigs};
pub use model_registry::{ModelRegistry, ModelStage, ModelVersion};
pub use monitoring::{AlertManager, MLMonitor, Metric};
pub use pipeline::{
    Pipeline, PipelineMetadata, PipelineOrchestrator, PipelineStage, RetryPolicy, StageType,
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main MLDevOps manager coordinating all components
pub struct MLDevOpsManager {
    pipeline_orchestrator: Arc<RwLock<PipelineOrchestrator>>,
    experiment_tracker: Arc<RwLock<ExperimentTracker>>,
    model_registry: Arc<RwLock<ModelRegistry>>,
    ml_monitor: Arc<RwLock<MLMonitor>>,
    deployment_manager: Arc<RwLock<DeploymentManager>>,
    feature_store: Arc<RwLock<FeatureStore>>,
    alert_manager: Arc<RwLock<AlertManager>>,
}

/// configsuration for MLDevOps components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLDevOpsconfigs {
    pub pipeline_engine: String,
    pub experiment_tracking: String,
    pub model_registry: String,
    pub monitoring: String,
    pub deployment: String,
    pub feature_store: String,
    pub storage_backend: String,
    pub compute_providers: Vec<String>,
    pub notification_channels: Vec<NotificationChannel>,
}

/// Notification channel configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    pub name: String,
    pub channel_type: NotificationType,
    pub configs: HashMap<String, String>,
    pub enabled: bool,
}

/// Types of notification channels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    Email,
    Slack,
    Webhook,
    PagerDuty,
}

/// A pipeline execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineExecution {
    pub id: String,
    pub pipeline_id: String,
    pub status: ExecutionStatus,
    pub parameters: HashMap<String, serde_json::Value>,
    pub metrics: HashMap<String, f64>,
    pub artifacts: Vec<PipelineArtifact>,
    pub logs: Vec<LogEntry>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
}

/// A pipeline artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineArtifact {
    pub id: String,
    pub artifact_type: PipelineArtifactType,
    pub location: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub checksum: String,
    pub size_bytes: u64,
}

/// Types of pipeline artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PipelineArtifactType {
    Model,
    Dataset,
    configsuration,
    Log,
    Report,
    Image,
}

/// A log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// Experiment configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experimentconfigs {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub metrics: Vec<String>,
    pub artifacts: Vec<String>,
}

/// Deployment configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deploymentconfigs {
    pub strategy: DeploymentStrategy,
    pub environment: String,
    pub resources: ResourceRequirements,
    pub health_checks: Vec<HealthCheck>,
}

/// Resource requirements for deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_cores: f64,
    pub memory_gb: f64,
    pub gpu_count: u32,
    pub storage_gb: f64,
}

/// Health check configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub endpoint: String,
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub healthy_threshold: u32,
    pub unhealthy_threshold: u32,
}

/// Alert definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertDefinition {
    pub id: String,
    pub name: String,
    pub condition: AlertCondition,
    pub severity: AlertSeverity,
    pub notification_channels: Vec<String>,
    pub enabled: bool,
}

/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    pub metric: String,
    pub operator: AlertOperator,
    pub threshold: f64,
    pub duration_seconds: u64,
}

/// Alert operators
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

/// An active alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub definition_id: String,
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
}

/// Comparison between experiments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentComparison {
    pub experiment_id: String,
    pub metrics: HashMap<String, f64>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub performance: ExperimentPerformance,
}

/// Experiment performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentPerformance {
    pub duration_seconds: u64,
    pub resource_usage: ResourceUsage,
    pub cost_estimate: f64,
}

/// Resource usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_hours: f64,
    pub memory_gb_hours: f64,
    pub gpu_hours: f64,
    pub storage_gb_hours: f64,
}

/// Summary of MLDevOps state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLDevOpsSummary {
    pub pipeline_stats: PipelineStatistics,
    pub experiment_stats: ExperimentStatistics,
    pub model_stats: ModelStatistics,
    pub deployment_stats: DeploymentStatistics,
    pub active_alerts: usize,
    pub feature_groups: usize,
}

/// Pipeline statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStatistics {
    pub total_pipelines: usize,
    pub active_executions: usize,
    pub success_rate: f64,
    pub average_execution_time: f64,
}

/// Experiment statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentStatistics {
    pub total_experiments: usize,
    pub completed_experiments: usize,
    pub active_experiments: usize,
    pub average_duration: f64,
}

/// Model statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStatistics {
    pub total_models: usize,
    pub models_in_production: usize,
    pub models_in_staging: usize,
    pub average_model_size: f64,
}

impl MLDevOpsManager {
    /// Create a new MLDevOpsManager with the given configsuration
    pub fn new(configs: MLDevOpsconfigs) -> Self {
        Self {
            pipeline_orchestrator: Arc::new(RwLock::new(PipelineOrchestrator::new())),
            experiment_tracker: Arc::new(RwLock::new(ExperimentTracker::new())),
            model_registry: Arc::new(RwLock::new(ModelRegistry::new())),
            ml_monitor: Arc::new(RwLock::new(MLMonitor::new())),
            deployment_manager: Arc::new(RwLock::new(DeploymentManager::new())),
            feature_store: Arc::new(RwLock::new(FeatureStore::new())),
            alert_manager: Arc::new(RwLock::new(AlertManager::new(configs.notification_channels))),
        }
    }

    /// Initialize all components
    pub async fn initialize(&self) -> Result<()> {
        self.pipeline_orchestrator
            .write()
            .await
            .initialize()
            .await?;
        self.experiment_tracker.write().await.initialize().await?;
        self.model_registry.write().await.initialize().await?;
        self.ml_monitor.write().await.initialize().await?;
        self.deployment_manager.write().await.initialize().await?;
        self.feature_store.write().await.initialize().await?;
        self.alert_manager.write().await.initialize().await?;

        Ok(())
    }

    /// Cleanup all components
    pub async fn cleanup(&self) -> Result<()> {
        self.pipeline_orchestrator.write().await.cleanup().await?;
        self.experiment_tracker.write().await.cleanup().await?;
        self.model_registry.write().await.cleanup().await?;
        self.ml_monitor.write().await.cleanup().await?;
        self.deployment_manager.write().await.cleanup().await?;
        self.feature_store.write().await.cleanup().await?;
        self.alert_manager.write().await.cleanup().await?;

        Ok(())
    }

    // Pipeline Management

    /// Create a new pipeline
    pub async fn create_pipeline(&self, pipeline: Pipeline) -> Result<String> {
        self.pipeline_orchestrator
            .write()
            .await
            .create_pipeline(pipeline)
            .await
    }

    /// Execute a pipeline with parameters
    pub async fn execute_pipeline(
        &self,
        pipeline_id: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        self.pipeline_orchestrator
            .write()
            .await
            .execute_pipeline(pipeline_id, parameters)
            .await
    }

    /// Get pipeline execution status
    pub async fn get_pipeline_status(&self, execution_id: &str) -> Option<PipelineExecution> {
        self.pipeline_orchestrator
            .read()
            .await
            .get_execution_status(execution_id)
            .await
    }

    /// Cancel a pipeline execution
    pub async fn cancel_pipeline(&self, execution_id: &str) -> Result<()> {
        self.pipeline_orchestrator
            .write()
            .await
            .cancel_execution(execution_id)
            .await
    }

    // Experiment Management

    /// Create a new experiment
    pub async fn create_experiment(&self, experiment: Experiment) -> Result<String> {
        self.experiment_tracker
            .write()
            .await
            .create_experiment(experiment)
            .await
    }

    /// Log a metric for an experiment
    pub async fn log_experiment_metric(
        &self,
        experiment_id: &str,
        metric: String,
        value: f64,
    ) -> Result<()> {
        self.experiment_tracker
            .write()
            .await
            .log_metric(experiment_id, metric, value)
            .await
    }

    /// Log a parameter for an experiment
    pub async fn log_experiment_parameter(
        &self,
        experiment_id: &str,
        parameter: String,
        value: serde_json::Value,
    ) -> Result<()> {
        self.experiment_tracker
            .write()
            .await
            .log_parameter(experiment_id, parameter, value)
            .await
    }

    /// Complete an experiment
    pub async fn complete_experiment(
        &self,
        experiment_id: &str,
        status: ExecutionStatus,
    ) -> Result<()> {
        self.experiment_tracker
            .write()
            .await
            .complete_experiment(experiment_id, status)
            .await
    }

    /// Get an experiment by ID
    pub async fn get_experiment(&self, experiment_id: &str) -> Option<Experiment> {
        self.experiment_tracker
            .read()
            .await
            .get_experiment(experiment_id)
            .await
    }

    /// Compare multiple experiments
    pub async fn compare_experiments(
        &self,
        experiment_ids: Vec<String>,
    ) -> Vec<ExperimentComparison> {
        self.experiment_tracker
            .read()
            .await
            .compare_experiments(experiment_ids)
            .await
    }

    // Model Registry Management

    /// Register a new model
    pub async fn register_model(&self, model: ModelVersion) -> Result<String> {
        self.model_registry
            .write()
            .await
            .register_model(model)
            .await
    }

    /// Promote a model to a stage
    pub async fn promote_model(&self, model_id: &str, stage: String) -> Result<()> {
        self.model_registry
            .write()
            .await
            .promote_model(model_id, stage)
            .await
    }

    /// Get a model by ID
    pub async fn get_model(&self, model_id: &str) -> Option<ModelVersion> {
        self.model_registry.read().await.get_model(model_id).await
    }

    /// List models with filters
    pub async fn list_models(&self, filters: HashMap<String, String>) -> Vec<ModelVersion> {
        self.model_registry.read().await.list_models(filters).await
    }

    // Monitoring

    /// Log a metric
    pub async fn log_metric(
        &self,
        metric: String,
        value: f64,
        tags: HashMap<String, String>,
    ) -> Result<()> {
        self.ml_monitor
            .write()
            .await
            .log_metric(metric, value, tags)
            .await
    }

    /// Create an alert
    pub async fn create_alert(&self, alert: AlertDefinition) -> Result<String> {
        self.alert_manager.write().await.create_alert(alert).await
    }

    /// Check alerts
    pub async fn check_alerts(&self) -> Vec<Alert> {
        self.alert_manager.read().await.check_alerts().await
    }

    // Deployment Management

    /// Deploy a model
    pub async fn deploy_model(
        &self,
        model_id: &str,
        strategy: DeploymentStrategy,
    ) -> Result<String> {
        self.deployment_manager
            .write()
            .await
            .deploy_model(model_id, strategy)
            .await
    }

    /// Get deployment status
    pub async fn get_deployment_status(&self, deployment_id: &str) -> Option<DeploymentStatus> {
        self.deployment_manager
            .read()
            .await
            .get_deployment_status(deployment_id)
    }

    /// Rollback a deployment
    pub async fn rollback_deployment(&self, deployment_id: &str) -> Result<()> {
        self.deployment_manager
            .write()
            .await
            .rollback_deployment(deployment_id)
            .await
    }

    // Feature Store Management

    /// Create a feature group
    pub async fn create_feature_group(&self, group: FeatureGroup) -> Result<String> {
        self.feature_store
            .write()
            .await
            .create_feature_group(group)
            .await
    }

    /// Get a feature group
    pub async fn get_feature_group(&self, group_id: &str) -> Option<FeatureGroup> {
        self.feature_store
            .read()
            .await
            .get_feature_group(group_id)
            .await
    }

    /// Ingest features into a feature group
    pub async fn ingest_features(
        &self,
        group_id: &str,
        features: Vec<HashMap<String, serde_json::Value>>,
    ) -> Result<()> {
        self.feature_store
            .write()
            .await
            .ingest_features(group_id, features)
            .await
    }

    /// Get online features
    pub async fn get_online_features(
        &self,
        group_id: &str,
        entity_id: &str,
    ) -> Option<HashMap<String, serde_json::Value>> {
        self.feature_store
            .read()
            .await
            .get_online_features(group_id, entity_id)
            .await
    }

    // ML DevOps Workflow

    /// Run a training pipeline
    pub async fn run_training_pipeline(
        &self,
        experiment_configs: Experimentconfigs,
    ) -> Result<String> {
        // Create experiment
        let experiment =
            Experiment::new(experiment_configs.name.clone(), experiment_configs.description.clone());
        let experiment_id = self.create_experiment(experiment).await?;

        // Create training pipeline
        let pipeline = Pipeline::training_pipeline(experiment_id.clone());
        let pipeline_id = self.create_pipeline(pipeline).await?;

        // Execute pipeline
        let _execution_id = self
            .execute_pipeline(&pipeline_id, experiment_configs.parameters)
            .await?;

        Ok(experiment_id)
    }

    /// Deploy a trained model
    pub async fn deploy_trained_model(
        &self,
        experiment_id: &str,
        deployment_configs: Deploymentconfigs,
    ) -> Result<String> {
        // Get experiment results
        let experiment = self
            .get_experiment(experiment_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Experiment not found"))?;

        // Register model
        let model = ModelVersion::from_experiment(&experiment);
        let model_id = self.register_model(model).await?;

        // Deploy model
        let deployment_id = self
            .deploy_model(&model_id, deployment_configs.strategy)
            .await?;

        Ok(deployment_id)
    }

    /// Get MLDevOps summary
    pub async fn get_ml_devops_summary(&self) -> MLDevOpsSummary {
        let pipeline_stats = self
            .pipeline_orchestrator
            .read()
            .await
            .get_statistics()
            .await;
        let experiment_stats = self.experiment_tracker.read().await.get_statistics().await;
        let model_stats = self.model_registry.read().await.get_statistics().await;
        let deployment_stats = self.deployment_manager.read().await.get_statistics();

        MLDevOpsSummary {
            pipeline_stats,
            experiment_stats,
            model_stats,
            deployment_stats,
            active_alerts: self
                .alert_manager
                .read()
                .await
                .get_active_alert_count()
                .await,
            feature_groups: self
                .feature_store
                .read()
                .await
                .get_feature_group_count()
                .await,
        }
    }
}

impl Default for MLDevOpsconfigs {
    fn default() -> Self {
        Self {
            pipeline_engine: "argo".to_string(),
            experiment_tracking: "mlflow".to_string(),
            model_registry: "mlflow".to_string(),
            monitoring: "prometheus".to_string(),
            deployment: "kubernetes".to_string(),
            feature_store: "feast".to_string(),
            storage_backend: "s3".to_string(),
            compute_providers: vec!["local".to_string()],
            notification_channels: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mldevops_manager_creation() {
        let configs = MLDevOpsconfigs::default();
        let manager = MLDevOpsManager::new(configs);
        assert!(manager.initialize().await.is_ok());
    }

    #[test]
    fn test_configs_default() {
        let configs = MLDevOpsconfigs::default();
        assert_eq!(configs.pipeline_engine, "argo");
        assert_eq!(configs.experiment_tracking, "mlflow");
    }
}
