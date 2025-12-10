//! Knowledge Capsules (KPLANE) primitives (US8)
//!
//! Provides lightweight storage, schema registry, metrics, directory, snapshot,
//! and crash forensics utilities used by self-improvement.

pub mod kcrash;
pub mod kdir;
pub mod kidx;
pub mod kmetrics;
pub mod kschema;
pub mod ksnap;

pub use kcrash::{CrashForensics, CrashReport};
pub use kdir::KnowledgeDirectory;
pub use kidx::{CapsuleIndex, CapsuleRecord};
pub use kmetrics::{KnowledgeMetric, KnowledgeMetrics};
pub use kschema::{SchemaDefinition, SchemaRegistry};
pub use ksnap::{KnowledgeSnapshot, SnapshotStore};
