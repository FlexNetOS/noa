//! Experiment tracking

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    ExecutionStatus, ExperimentComparison, ExperimentPerformance, ExperimentStatistics,
    ResourceUsage,
};

/// An ML experiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ExperimentStatus,
    pub parameters: HashMap<String, serde_json::Value>,
    pub metrics: HashMap<String, f64>,
    pub artifacts: Vec<Artifact>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Experiment {
    /// Create a new experiment
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            status: ExperimentStatus::Pending,
            parameters: HashMap::new(),
            metrics: HashMap::new(),
            artifacts: Vec::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            start_time: None,
            end_time: None,
        }
    }
}

/// Experiment status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExperimentStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// An experiment artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub artifact_type: ArtifactType,
    pub location: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub size_bytes: u64,
    pub checksum: String,
}

/// Artifact types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactType {
    Model,
    Dataset,
    configsuration,
    Log,
    Report,
    Image,
    Video,
}

/// Experiment tracker
pub struct ExperimentTracker {
    experiments: HashMap<String, Experiment>,
    active_experiments: HashMap<String, DateTime<Utc>>,
}

impl ExperimentTracker {
    /// Create a new ExperimentTracker
    pub fn new() -> Self {
        Self {
            experiments: HashMap::new(),
            active_experiments: HashMap::new(),
        }
    }

    /// Initialize the tracker
    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Create a new experiment
    pub async fn create_experiment(&mut self, experiment: Experiment) -> Result<String> {
        let id = experiment.id.clone();
        self.experiments.insert(id.clone(), experiment);
        Ok(id)
    }

    /// Log a metric for an experiment
    pub async fn log_metric(
        &mut self,
        experiment_id: &str,
        metric: String,
        value: f64,
    ) -> Result<()> {
        if let Some(experiment) = self.experiments.get_mut(experiment_id) {
            experiment.metrics.insert(metric, value);
            experiment.updated_at = Utc::now();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Experiment not found"))
        }
    }

    /// Log a parameter for an experiment
    pub async fn log_parameter(
        &mut self,
        experiment_id: &str,
        parameter: String,
        value: serde_json::Value,
    ) -> Result<()> {
        if let Some(experiment) = self.experiments.get_mut(experiment_id) {
            experiment.parameters.insert(parameter, value);
            experiment.updated_at = Utc::now();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Experiment not found"))
        }
    }

    /// Complete an experiment
    pub async fn complete_experiment(
        &mut self,
        experiment_id: &str,
        status: ExecutionStatus,
    ) -> Result<()> {
        if let Some(experiment) = self.experiments.get_mut(experiment_id) {
            experiment.status = match status {
                ExecutionStatus::Succeeded => ExperimentStatus::Completed,
                ExecutionStatus::Failed => ExperimentStatus::Failed,
                ExecutionStatus::Cancelled => ExperimentStatus::Cancelled,
                _ => ExperimentStatus::Failed,
            };
            experiment.end_time = Some(Utc::now());
            experiment.updated_at = Utc::now();

            self.active_experiments.remove(experiment_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Experiment not found"))
        }
    }

    /// Get an experiment by ID
    pub async fn get_experiment(&self, experiment_id: &str) -> Option<Experiment> {
        self.experiments.get(experiment_id).cloned()
    }

    /// Compare experiments
    pub async fn compare_experiments(
        &self,
        experiment_ids: Vec<String>,
    ) -> Vec<ExperimentComparison> {
        let mut comparisons = Vec::new();

        for experiment_id in experiment_ids {
            if let Some(experiment) = self.experiments.get(&experiment_id) {
                let comparison = ExperimentComparison {
                    experiment_id: experiment_id.clone(),
                    metrics: experiment.metrics.clone(),
                    parameters: experiment.parameters.clone(),
                    performance: ExperimentPerformance {
                        duration_seconds: experiment
                            .start_time
                            .and_then(|start| {
                                experiment
                                    .end_time
                                    .map(|end| (end - start).num_seconds() as u64)
                            })
                            .unwrap_or(0),
                        resource_usage: ResourceUsage {
                            cpu_hours: 0.0,
                            memory_gb_hours: 0.0,
                            gpu_hours: 0.0,
                            storage_gb_hours: 0.0,
                        },
                        cost_estimate: 0.0,
                    },
                };
                comparisons.push(comparison);
            }
        }

        comparisons
    }

    /// Get experiment statistics
    pub async fn get_statistics(&self) -> ExperimentStatistics {
        let total_experiments = self.experiments.len();
        let completed_experiments = self
            .experiments
            .values()
            .filter(|e| {
                matches!(
                    e.status,
                    ExperimentStatus::Completed
                        | ExperimentStatus::Failed
                        | ExperimentStatus::Cancelled
                )
            })
            .count();
        let active_experiments = self
            .experiments
            .values()
            .filter(|e| matches!(e.status, ExperimentStatus::Running))
            .count();

        let completed_with_duration: Vec<_> = self
            .experiments
            .values()
            .filter(|e| e.start_time.is_some() && e.end_time.is_some())
            .collect();

        let average_duration = if !completed_with_duration.is_empty() {
            let total_duration: i64 = completed_with_duration
                .iter()
                .filter_map(|e| {
                    e.start_time
                        .and_then(|start| e.end_time.map(|end| (end - start).num_seconds()))
                })
                .sum();
            total_duration as f64 / completed_with_duration.len() as f64
        } else {
            0.0
        };

        ExperimentStatistics {
            total_experiments,
            completed_experiments,
            active_experiments,
            average_duration,
        }
    }
}

impl Default for ExperimentTracker {
    fn default() -> Self {
        Self::new()
    }
}
