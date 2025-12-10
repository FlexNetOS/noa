//! Knowledge Capsules (KPLANE) primitives (US8)
//!
//! Provides lightweight storage, schema registry, metrics, directory, snapshot,
//! and crash forensics utilities used by self-improvement.

pub mod kidx;
pub mod kschema;
pub mod kmetrics;
pub mod kdir;
pub mod ksnap;
pub mod kcrash;

pub use kidx::{CapsuleIndex, CapsuleRecord};
pub use kschema::{SchemaDefinition, SchemaRegistry};
pub use kmetrics::{KnowledgeMetric, KnowledgeMetrics};
pub use kdir::KnowledgeDirectory;
pub use ksnap::{KnowledgeSnapshot, SnapshotStore};
pub use kcrash::{CrashForensics, CrashReport};
