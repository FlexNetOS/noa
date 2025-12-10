use crate::error::Result;
use crate::governance::biblical::embedding::{EmbeddingPipeline, PassageEmbedding};
use crate::governance::biblical::knowledge_graph::KnowledgeGraph;
use crate::governance::biblical::lexical::LexicalAnalyzer;
use crate::governance::engine::{DecisionVerdict, GovernanceDecision, GovernanceRule, RuleVerdict};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Core biblical principle definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiblicalPrinciple {
    pub id: String,
    pub reference: String,
    pub summary: String,
    pub boundary: String,
    pub weight: f32,
}

/// Ethical boundary guard driven by biblical principles.
pub struct EthicsGuard {
    principles: Vec<BiblicalPrinciple>,
    embedder: EmbeddingPipeline,
    lexical: LexicalAnalyzer,
    graph: KnowledgeGraph,
}

impl EthicsGuard {
    pub fn new(principles: Vec<BiblicalPrinciple>, embedder: EmbeddingPipeline) -> Self {
        let mut graph = KnowledgeGraph::new(embedder.clone());
        for principle in &principles {
            let metadata = serde_json::json!({
                "boundary": principle.boundary,
                "weight": principle.weight,
            });
            let _ = graph.upsert_passage(
                principle.id.clone(),
                principle.reference.clone(),
                &principle.summary,
                metadata,
            );
        }

        Self {
            principles,
            embedder,
            lexical: LexicalAnalyzer::new(),
            graph,
        }
    }

    /// Build a default guard with a minimal principle set.
    pub fn default_guard() -> Self {
        let principles = vec![
            BiblicalPrinciple {
                id: "GEN1_27".to_string(),
                reference: "Genesis 1:27".to_string(),
                summary: "All people bear inherent dignity; do no harm or dehumanize."
                    .to_string(),
                boundary: "Block any action that exploits, coerces, or harms people.".to_string(),
                weight: 1.0,
            },
            BiblicalPrinciple {
                id: "EX20_16".to_string(),
                reference: "Exodus 20:16".to_string(),
                summary: "Speak truthfully; do not bear false witness or hide material risk."
                    .to_string(),
                boundary: "Escalate when evidence is missing or claims cannot be verified."
                    .to_string(),
                weight: 0.8,
            },
            BiblicalPrinciple {
                id: "MIC6_8".to_string(),
                reference: "Micah 6:8".to_string(),
                summary: "Act justly, love mercy, walk humbly; protect the vulnerable."
                    .to_string(),
                boundary: "Require rollback checkpoints when touching user-facing systems."
                    .to_string(),
                weight: 0.7,
            },
        ];

        Self::new(principles, EmbeddingPipeline::default())
    }

    fn detect_risk_flags(&self, text: &str) -> Vec<String> {
        let flagged_terms = ["harm", "exploit", "deceive", "coerce", "bypass", "tamper"];
        let lower = text.to_ascii_lowercase();
        flagged_terms
            .iter()
            .filter(|term| lower.contains(*term))
            .map(|s| s.to_string())
            .collect()
    }

    fn best_principle(
        &self,
        embedding: &PassageEmbedding,
    ) -> (BiblicalPrinciple, f32) {
        let mut best = (
            self.principles
                .get(0)
                .cloned()
                .unwrap_or(BiblicalPrinciple {
                    id: "UNSET".to_string(),
                    reference: "Unconfigured principle".to_string(),
                    summary: "Configure biblical principles before enforcement.".to_string(),
                    boundary: "Escalate to human reviewer".to_string(),
                    weight: 0.0,
                }),
            0.0,
        );
        for principle in &self.principles {
            let p_embedding = self
                .embedder
                .embed_text(&principle.reference, &principle.summary);
            if let Ok(score) = self.embedder.similarity(embedding, &p_embedding) {
                if score > best.1 {
                    best = (principle.clone(), score);
                }
            }
        }
        best
    }
}

#[async_trait]
impl GovernanceRule for EthicsGuard {
    fn name(&self) -> &'static str {
        "biblical_ethics_guard"
    }

    async fn evaluate(&self, decision: &GovernanceDecision) -> Result<RuleVerdict> {
        let context_str = serde_json::to_string(&decision.context).unwrap_or_default();
        let combined_text = format!("{} {}", decision.rationale, context_str);

        let lexical = self.lexical.analyze(&combined_text);
        let decision_embedding = self
            .embedder
            .embed_text(&decision.action, &combined_text);
        let (principle, alignment_score) = self.best_principle(&decision_embedding);
        let risk_flags = self.detect_risk_flags(&combined_text);

        let mut verdict = DecisionVerdict::Allow;
        let mut rationale = format!(
            "Aligned with {} (score {:.2}); lexical greek {:.2}, hebrew {:.2}",
            principle.reference, alignment_score, lexical.greek_ratio, lexical.hebrew_ratio
        );
        let mut reward_delta = alignment_score * principle.weight;
        let mut requires_snapshot = false;

        if alignment_score < 0.15 {
            verdict = DecisionVerdict::Escalate;
            rationale = format!(
                "Low alignment ({:.2}) to biblical guardrails; escalate for human review",
                alignment_score
            );
            reward_delta = -0.2;
        }

        if !risk_flags.is_empty() {
            verdict = DecisionVerdict::Deny;
            rationale = format!(
                "Boundary hit ({}) with risk terms: {}",
                principle.boundary,
                risk_flags.join(", ")
            );
            reward_delta = -1.0 * (risk_flags.len() as f32);
            requires_snapshot = true;
        }

        // Surface related passages for audit traceability.
        let related = self
            .graph
            .related_passages(&decision_embedding, 3)
            .unwrap_or_default()
            .into_iter()
            .map(|(node, score)| serde_json::json!({"reference": node.reference, "score": score}))
            .collect::<Vec<_>>();

        info!(
            target: "governance::ethics",
            agent = %decision.agent_id,
            action = %decision.action,
            verdict = ?verdict,
            alignment = alignment_score,
            risks = %risk_flags.join(", ")
        );

        Ok(RuleVerdict {
            rule: self.name().to_string(),
            verdict,
            rationale,
            principles: vec![principle.reference.clone()],
            reward_delta,
            requires_snapshot,
            evidence: serde_json::json!({
                "alignment_score": alignment_score,
                "risk_flags": risk_flags,
                "related_passages": related,
            }),
        })
    }
}
