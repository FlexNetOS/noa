//! Governance & Safety (FR-025)
//!
//! Implements constitutional governance primitives, including the decision
//! engine and audit trail used to keep every agent action aligned with
//! NOA's guardrails.

pub mod audit;
pub mod biblical;
pub mod correction;
pub mod drift_detection;
pub mod engine;
pub mod harness;
pub mod rewards;
pub mod rollback_executor;
pub mod rollback_validator;
pub mod snapshot;

pub use audit::{AuditEvent, GovernanceAuditTrail};
pub use biblical::{
    BiblicalIngestor, BiblicalPrinciple, BiblicalSource, EmbeddingPipeline, EthicsGuard,
    IngestionReport, KnowledgeGraph, KnowledgeGraphEdge, KnowledgeGraphNode, LexicalAnalysis,
    LexicalAnalyzer, PassageEmbedding, ScriptureLanguage, TokenStat,
};
pub use correction::{CorrectionAction, CorrectionEngine, CorrectionPlan};
pub use drift_detection::{DriftDetector, DriftSignal, DriftStatus};
pub use engine::{
    DecisionVerdict, GovernanceDecision, GovernanceEngine, GovernanceOutcome, GovernanceRule,
    RuleVerdict,
};
pub use harness::GovernanceHarness;
pub use rewards::{RewardEvent, RewardSystem};
pub use rollback_executor::{RollbackExecutor, RollbackResult};
pub use rollback_validator::{RollbackAssessment, RollbackValidator};
pub use snapshot::{SnapshotArtifact, SnapshotRecord, SnapshotService};
