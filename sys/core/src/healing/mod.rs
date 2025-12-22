//! Self-Healing Loop Module
//!
//! Implements FR-071 through FR-075: 5-stage self-healing loop
//! §3.4: Adaptive & Self-Improving - continuous health monitoring and auto-recovery
//!
//! Stages:
//! 1. Monitor: Continuous health monitoring
//! 2. Detect: Anomaly detection
//! 3. Diagnose: Root cause analysis
//! 4. Fix: Auto-fix executor (restart, reconfig, rollback, redistribute)
//! 5. Validate: Fix validation and retry/escalate

pub mod anomaly;
pub mod audit;
pub mod diagnose;
pub mod escalate;
pub mod fix;
pub mod monitor;
pub mod plane_swap;
pub mod retry;
pub mod validate;

pub use anomaly::AnomalyDetector;
pub use audit::HealingAuditLogger;
pub use diagnose::RootCauseAnalyzer;
pub use escalate::EscalationNotifier;
pub use fix::AutoFixExecutor;
pub use monitor::{ComponentHealth, HealthMonitor};
pub use plane_swap::PlaneSwapExecutor;
pub use retry::RetryCounter;
pub use validate::FixValidator;

use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

/// Healing event status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealingStatus {
    /// Monitoring detected issue
    Detected,
    /// Root cause analysis in progress
    Diagnosing,
    /// Auto-fix being applied
    Fixing,
    /// Fix validation in progress
    Validating,
    /// Issue resolved
    Resolved,
    /// Escalated to user after retry limit
    Escalated,
    /// Failed to resolve
    Failed,
}

/// Healing event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingEvent {
    pub id: Uuid,
    pub component_id: String,
    pub component_type: String,
    pub detected_at: DateTime<Utc>,
    pub status: HealingStatus,
    pub health_before: ComponentHealth,
    pub anomaly_type: Option<String>,
    pub root_cause: Option<String>,
    pub fix_applied: Option<String>,
    pub fix_attempts: u32,
    pub validated: bool,
    pub escalated: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
}

/// Self-healing orchestrator
pub struct SelfHealingOrchestrator {
    monitor: Arc<HealthMonitor>,
    anomaly_detector: Arc<Mutex<AnomalyDetector>>,
    root_cause_analyzer: Arc<RootCauseAnalyzer>,
    fix_executor: Arc<AutoFixExecutor>,
    fix_validator: Arc<FixValidator>,
    retry_counter: Arc<RetryCounter>,
    escalation_notifier: Arc<EscalationNotifier>,
    audit_logger: Arc<HealingAuditLogger>,
    plane_swap: Arc<PlaneSwapExecutor>,
    active_events: Arc<RwLock<Vec<HealingEvent>>>,
}

impl SelfHealingOrchestrator {
    /// Create a new self-healing orchestrator
    pub fn new(
        monitor: Arc<HealthMonitor>,
        anomaly_detector: Arc<AnomalyDetector>,
        root_cause_analyzer: Arc<RootCauseAnalyzer>,
        fix_executor: Arc<AutoFixExecutor>,
        fix_validator: Arc<FixValidator>,
        retry_counter: Arc<RetryCounter>,
        escalation_notifier: Arc<EscalationNotifier>,
        audit_logger: Arc<HealingAuditLogger>,
        plane_swap: Arc<PlaneSwapExecutor>,
    ) -> Self {
        Self {
            monitor,
            anomaly_detector: Arc::new(Mutex::new((*anomaly_detector).clone())),
            root_cause_analyzer,
            fix_executor,
            fix_validator,
            retry_counter,
            escalation_notifier,
            audit_logger,
            plane_swap,
            active_events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Run the self-healing loop
    pub async fn run_healing_loop(&self) -> Result<()> {
        loop {
            // Stage 1: Monitor - continuous health monitoring
            let health_snapshots = self.monitor.get_all_health_snapshots().await;

            // Stage 2: Detect - anomaly detection
            if let Some(anomaly) = {
                let mut detector = self.anomaly_detector.lock().await;
                detector.detect(&health_snapshots).await?
            } {
                let anomaly_type = anomaly.anomaly_type.clone();
                let metadata_value = serde_json::Value::Object(anomaly.metadata.clone().into_iter().collect());

                let event = HealingEvent {
                    id: Uuid::new_v4(),
                    component_id: anomaly.component_id.clone(),
                    component_type: anomaly.component_type.clone(),
                    detected_at: Utc::now(),
                    status: HealingStatus::Detected,
                    health_before: anomaly.health_status,
                    anomaly_type: Some(anomaly_type),
                    root_cause: None,
                    fix_applied: None,
                    fix_attempts: 0,
                    validated: false,
                    escalated: false,
                    resolved_at: None,
                    metadata: metadata_value,
                };

                // Log detection
                self.audit_logger.log_event(&event).await?;

                // Add to active events
                {
                    let mut events = self.active_events.write().await;
                    events.push(event.clone());
                }

                // Stage 3: Diagnose - root cause analysis
                let root_cause = self
                    .root_cause_analyzer
                    .analyze(&anomaly, &health_snapshots)
                    .await?;

                // Update event with root cause
                {
                    let mut events = self.active_events.write().await;
                    if let Some(e) = events.iter_mut().find(|e| e.id == event.id) {
                        e.root_cause = Some(root_cause.clone());
                        e.status = HealingStatus::Diagnosing;
                    }
                }

                // Stage 4: Fix - auto-fix executor
                let fix_result = self
                    .fix_executor
                    .apply_fix(&anomaly, &root_cause)
                    .await?;

                // Update event with fix
                {
                    let mut events = self.active_events.write().await;
                    if let Some(e) = events.iter_mut().find(|e| e.id == event.id) {
                        e.fix_applied = Some(fix_result.fix_type.clone());
                        e.fix_attempts += 1;
                        e.status = HealingStatus::Fixing;
                    }
                }

                // Stage 5: Validate - fix validation
                let validation_result = self
                    .fix_validator
                    .validate(&anomaly, &fix_result)
                    .await?;

                if validation_result.success {
                    // Fix successful
                    {
                        let mut events = self.active_events.write().await;
                        if let Some(e) = events.iter_mut().find(|e| e.id == event.id) {
                            e.validated = true;
                            e.status = HealingStatus::Resolved;
                            e.resolved_at = Some(Utc::now());
                        }
                    }
                    self.audit_logger.log_resolution(&event.id).await?;
                } else {
                    // Check retry counter
                    let retry_count = self.retry_counter.increment(&event.id).await?;

                    if retry_count >= 3 {
                        // Escalate to user
                        self.escalation_notifier
                            .notify(&event, &root_cause, retry_count)
                            .await?;

                        {
                            let mut events = self.active_events.write().await;
                            if let Some(e) = events.iter_mut().find(|e| e.id == event.id) {
                                e.escalated = true;
                                e.status = HealingStatus::Escalated;
                            }
                        }
                    } else {
                        // Retry fix
                        // Continue loop to retry
                    }
                }
            }

            // Sleep before next monitoring cycle
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    /// Get active healing events
    pub async fn get_active_events(&self) -> Vec<HealingEvent> {
        self.active_events.read().await.clone()
    }
}

