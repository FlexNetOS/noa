//! Self-Improvement pipeline (US8)
//!
//! Metrics, analysis, proposals, safety controls, and human co-improvement
//! helpers that drive `noa improve` commands.

pub mod analyzer;
pub mod approval;
pub mod audit;
pub mod metrics;
pub mod proposals;
pub mod rollback;
pub mod snapshot;
pub mod test_runner;

pub use analyzer::{EfficiencyAnalyzer, EfficiencyReport};
pub use approval::{ApprovalRequest, ApprovalStatus, ImprovementApprovalWorkflow};
pub use audit::{ImprovementAuditEntry, ImprovementAuditLog};
pub use metrics::{MetricSample, PerformanceMetrics};
pub use proposals::{ImprovementProposal, ImprovementProposalGenerator};
pub use rollback::{RollbackManager, RollbackOutcome};
pub use snapshot::{SnapshotDescriptor, SnapshotManager};
pub use test_runner::{TestCase, TestOutcome, TestRunResult, TestRunner};
