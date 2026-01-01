use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub description: String,
    pub stages: Vec<PipelineStage>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub metadata: PipelineMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub id: String,
    pub name: String,
    pub stage_type: StageType,
    pub dependencies: Vec<String>,
    pub config: HashMap<String, serde_json::Value>,
    pub retry_policy: RetryPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StageType {
    DataIngestion,
    DataPreprocessing,
    FeatureEngineering,
    ModelTraining,
    ModelValidation,
    ModelDeployment,
    Monitoring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_strategy: String,
    pub initial_delay_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMetadata {
    pub version: String,
    pub created_by: String,
    pub tags: Vec<String>,
    pub schedule: Option<String>,
    pub timeout_minutes: u32,
}

pub struct PipelineOrchestrator {
    pipelines: HashMap<String, Pipeline>,
    executions: HashMap<String, crate::ml_devops::PipelineExecution>,
}

impl PipelineOrchestrator {
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new(),
            executions: HashMap::new(),
        }
    }
    
    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub async fn create_pipeline(&mut self, pipeline: Pipeline) -> Result<String> {
        let id = pipeline.id.clone();
        self.pipelines.insert(id.clone(), pipeline);
        Ok(id)
    }
    
    pub async fn execute_pipeline(&mut self, pipeline_id: &str, parameters: HashMap<String, serde_json::Value>) -> Result<String> {
        let execution_id = uuid::Uuid::new_v4().to_string();
        
        let execution = crate::ml_devops::PipelineExecution {
            id: execution_id.clone(),
            pipeline_id: pipeline_id.to_string(),
            status: crate::ml_devops::ExecutionStatus::Running,
            parameters,
            metrics: HashMap::new(),
            artifacts: Vec::new(),
            logs: Vec::new(),
            start_time: chrono::Utc::now(),
            end_time: None,
        };
        
        self.executions.insert(execution_id.clone(), execution);
        
        // Start execution in background
        tokio::spawn(async move {
            // Simulate pipeline execution
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        });
        
        Ok(execution_id)
    }
    
    pub async fn get_execution_status(&self, execution_id: &str) -> Option<crate::ml_devops::PipelineExecution> {
        self.executions.get(execution_id).cloned()
    }
    
    pub async fn cancel_execution(&mut self, execution_id: &str) -> Result<()> {
        if let Some(execution) = self.executions.get_mut(execution_id) {
            execution.status = crate::ml_devops::ExecutionStatus::Cancelled;
            execution.end_time = Some(chrono::Utc::now());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Execution not found"))
        }
    }
    
    pub async fn get_statistics(&self) -> crate::ml_devops::PipelineStatistics {
        let total_pipelines = self.pipelines.len();
        let active_executions = self.executions.values()
            .filter(|e| matches!(e.status, crate::ml_devops::ExecutionStatus::Running))
            .count();
        
        let completed_executions: Vec<_> = self.executions.values()
            .filter(|e| matches!(e.status, crate::ml_devops::ExecutionStatus::Succeeded | crate::ml_devops::ExecutionStatus::Failed))
            .collect();
        
        let success_rate = if !completed_executions.is_empty() {
            let successful = completed_executions.iter()
                .filter(|e| matches!(e.status, crate::ml_devops::ExecutionStatus::Succeeded))
                .count();
            successful as f64 / completed_executions.len() as f64
        } else {
            0.0
        };
        
        let average_execution_time = if !completed_executions.is_empty() {
            let total_time: f64 = completed_executions.iter()
                .filter_map(|e| e.end_time.map(|end| (end - e.start_time).num_seconds() as f64))
                .sum();
            total_time / completed_executions.len() as f64
        } else {
            0.0
        };
        
        crate::ml_devops::PipelineStatistics {
            total_pipelines,
            active_executions,
            success_rate,
            average_execution_time,
        }
    }
}

impl Pipeline {
    pub fn training_pipeline(experiment_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("training_pipeline_{}", experiment_id),
            description: "ML training pipeline".to_string(),
            stages: vec![
                PipelineStage {
                    id: "data_ingestion".to_string(),
                    name: "Data Ingestion".to_string(),
                    stage_type: StageType::DataIngestion,
                    dependencies: vec![],
                    config: HashMap::new(),
                    retry_policy: RetryPolicy {
                        max_attempts: 3,
                        backoff_strategy: "exponential".to_string(),
                        initial_delay_seconds: 60,
                    },
                },
                PipelineStage {
                    id: "preprocessing".to_string(),
                    name: "Data Preprocessing".to_string(),
                    stage_type: StageType::DataPreprocessing,
                    dependencies: vec!["data_ingestion".to_string()],
                    config: HashMap::new(),
                    retry_policy: RetryPolicy {
                        max_attempts: 2,
                        backoff_strategy: "linear".to_string(),
                        initial_delay_seconds: 30,
                    },
                },
                PipelineStage {
                    id: "training".to_string(),
                    name: "Model Training".to_string(),
                    stage_type: StageType::ModelTraining,
                    dependencies: vec!["preprocessing".to_string()],
                    config: HashMap::new(),
                    retry_policy: RetryPolicy {
                        max_attempts: 1,
                        backoff_strategy: "fixed".to_string(),
                        initial_delay_seconds: 300,
                    },
                },
                PipelineStage {
                    id: "validation".to_string(),
                    name: "Model Validation".to_string(),
                    stage_type: StageType::ModelValidation,
                    dependencies: vec!["training".to_string()],
                    config: HashMap::new(),
                    retry_policy: RetryPolicy {
                        max_attempts: 2,
                        backoff_strategy: "linear".to_string(),
                        initial_delay_seconds: 60,
                    },
                },
            ],
            parameters: HashMap::new(),
            metadata: PipelineMetadata {
                version: "1.0".to_string(),
                created_by: "system".to_string(),
                tags: vec!["training".to_string(), "ml".to_string()],
                schedule: None,
                timeout_minutes: 1440, // 24 hours
            },
        }
    }
}