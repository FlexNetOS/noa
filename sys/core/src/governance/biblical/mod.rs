//! Biblical governance pipeline (FR-026)
//!
//! Provides ingestion, lexical analysis, embeddings, knowledge graph wiring,
//! and ethics guards rooted in biblical principles.

pub mod embedding;
pub mod ethics;
pub mod ingest;
pub mod knowledge_graph;
pub mod lexical;

pub use embedding::{EmbeddingPipeline, PassageEmbedding};
pub use ethics::{BiblicalPrinciple, EthicsGuard};
pub use ingest::{BiblicalIngestor, BiblicalSource, IngestionReport, ScriptureLanguage};
pub use knowledge_graph::{KnowledgeGraph, KnowledgeGraphEdge, KnowledgeGraphNode};
pub use lexical::{LexicalAnalysis, LexicalAnalyzer, TokenStat};
