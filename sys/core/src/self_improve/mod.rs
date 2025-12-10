//! Self-Improvement pipeline (US8)
//!
//! Metrics, analysis, proposals, safety controls, and human co-improvement
//! helpers that drive `noa improve` commands.

pub mod metrics;
pub mod analyzer;
pub mod proposals;
pub mod snapshot;
pub mod test_runner;
pub mod rollback;
pub mod approval;
pub mod audit;

pub use metrics::{MetricSample, PerformanceMetrics};
pub use analyzer::{EfficiencyAnalyzer, EfficiencyReport};
pub use proposals::{ImprovementProposal, ImprovementProposalGenerator};
pub use snapshot::{SnapshotDescriptor, SnapshotManager};
pub use test_runner::{TestCase, TestOutcome, TestRunResult, TestRunner};
pub use rollback::{RollbackManager, RollbackOutcome};
pub use approval::{ApprovalRequest, ApprovalStatus, ImprovementApprovalWorkflow};
pub use audit::{ImprovementAuditEntry, ImprovementAuditLog};
