use crate::error::Result;
use crate::governance::biblical::embedding::{EmbeddingPipeline, PassageEmbedding};
use serde::{Deserialize, Serialize};

/// Knowledge graph node derived from scripture analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraphNode {
    pub id: String,
    pub reference: String,
    pub embedding: PassageEmbedding,
    pub metadata: serde_json::Value,
}

/// Edge between two nodes capturing relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraphEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
    pub weight: f32,
}

/// Lightweight in-memory knowledge graph for biblical governance artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    #[serde(skip)]
    embedder: EmbeddingPipeline,
    pub nodes: Vec<KnowledgeGraphNode>,
    pub edges: Vec<KnowledgeGraphEdge>,
}

impl KnowledgeGraph {
    pub fn new(embedder: EmbeddingPipeline) -> Self {
        Self {
            embedder,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn upsert_passage(
        &mut self,
        id: String,
        reference: String,
        text: &str,
        metadata: serde_json::Value,
    ) -> Result<()> {
        let embedding = self.embedder.embed(&reference, text);
        if let Some(existing) = self.nodes.iter_mut().find(|n| n.id == id) {
            existing.reference = reference;
            existing.embedding = embedding;
            existing.metadata = metadata;
        } else {
            self.nodes.push(KnowledgeGraphNode {
                id,
                reference,
                embedding,
                metadata,
            });
        }
        Ok(())
    }

    pub fn related_passages(
        &self,
        embedding: &PassageEmbedding,
        limit: usize,
    ) -> Result<Vec<(KnowledgeGraphNode, f32)>> {
        let mut scored: Vec<(KnowledgeGraphNode, f32)> = self
            .nodes
            .iter()
            .filter_map(|node| {
                self.embedder
                    .similarity(&node.embedding, embedding)
                    .ok()
                    .map(|score| (node.clone(), score))
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(limit);
        Ok(scored)
    }

    pub fn add_edge(&mut self, edge: KnowledgeGraphEdge) {
        self.edges.push(edge);
    }
}
