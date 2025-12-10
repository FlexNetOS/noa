use crate::error::Result;
use crate::governance::biblical::lexical::{LexicalAnalysis, LexicalAnalyzer};
use serde::{Deserialize, Serialize};

/// Simple embedding representation for a scripture passage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassageEmbedding {
    pub reference: String,
    pub vector: Vec<f32>,
}

/// Lightweight embedding pipeline that reuses lexical statistics.
#[derive(Debug, Clone)]
pub struct EmbeddingPipeline {
    analyzer: LexicalAnalyzer,
}

impl EmbeddingPipeline {
    pub fn new() -> Self {
        Self {
            analyzer: LexicalAnalyzer::new(),
        }
    }

    pub fn embed_text(&self, reference: &str, text: &str) -> PassageEmbedding {
        self.embed(reference, text)
    }

    /// Generate an embedding from raw text by summarizing lexical ratios.
    pub fn embed(&self, reference: &str, text: &str) -> PassageEmbedding {
        let analysis = self.analyzer.analyze(text);
        self.embed_analysis(reference, &analysis)
    }

    /// Generate an embedding from a pre-computed lexical analysis.
    pub fn embed_analysis(&self, reference: &str, analysis: &LexicalAnalysis) -> PassageEmbedding {
        let magnitude = analysis.total_tokens as f32;
        let density = if analysis.unique_tokens == 0 {
            0.0
        } else {
            magnitude / (analysis.unique_tokens as f32)
        };

        PassageEmbedding {
            reference: reference.to_string(),
            vector: vec![
                magnitude,
                analysis.unique_tokens as f32,
                analysis.greek_ratio,
                analysis.hebrew_ratio,
                density,
            ],
        }
    }

    /// Compute a basic cosine similarity between two embeddings.
    pub fn similarity(&self, a: &PassageEmbedding, b: &PassageEmbedding) -> Result<f32> {
        let dot = a
            .vector
            .iter()
            .zip(&b.vector)
            .map(|(x, y)| x * y)
            .sum::<f32>();
        let norm_a = (a.vector.iter().map(|v| v * v).sum::<f32>()).sqrt();
        let norm_b = (b.vector.iter().map(|v| v * v).sum::<f32>()).sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return Ok(0.0);
        }

        Ok(dot / (norm_a * norm_b))
    }
}

impl Default for EmbeddingPipeline {
    fn default() -> Self {
        Self::new()
    }
}
