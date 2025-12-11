use crate::self_improve::analyzer::EfficiencyReport;

/// Proposed improvement with expected outcome.
#[derive(Debug, Clone)]
pub struct ImprovementProposal {
    pub id: String,
    pub summary: String,
    pub rationale: String,
    pub estimated_gain: f64,
    pub actions: Vec<String>,
}

pub struct ImprovementProposalGenerator;

impl ImprovementProposalGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn from_report(&self, report: &EfficiencyReport) -> Vec<ImprovementProposal> {
        let mut proposals = Vec::new();

        for bottleneck in &report.bottlenecks {
            proposals.push(ImprovementProposal {
                id: uuid::Uuid::new_v4().to_string(),
                summary: format!("Address bottleneck: {}", bottleneck),
                rationale: bottleneck.clone(),
                estimated_gain: 0.15,
                actions: vec![
                    "capture_before_metrics".to_string(),
                    "apply_fix".to_string(),
                    "compare_after_metrics".to_string(),
                ],
            });
        }

        if proposals.is_empty() {
            proposals.push(ImprovementProposal {
                id: uuid::Uuid::new_v4().to_string(),
                summary: "Maintain current posture".to_string(),
                rationale: "No bottlenecks detected".to_string(),
                estimated_gain: 0.02,
                actions: vec!["continue_monitoring".to_string()],
            });
        }

        proposals
    }
}
