use crate::predict::patterns::PatternRecognitionEngine;

/// Insight derived from failure analysis.
#[derive(Debug, Clone)]
pub struct FailureInsight {
    pub cause: String,
    pub frequency: usize,
    pub recommendation: String,
}

/// Analyzes failure patterns across providers and tasks.
pub struct FailureAnalyzer {
    engine: PatternRecognitionEngine,
}

impl FailureAnalyzer {
    pub fn new(engine: PatternRecognitionEngine) -> Self {
        Self { engine }
    }

    pub fn analyze(&self, failures: &[String]) -> Vec<FailureInsight> {
        self.engine
            .identify_patterns(failures)
            .into_iter()
            .map(|m| FailureInsight {
                cause: m.pattern.clone(),
                frequency: m.occurrences,
                recommendation: format!("Mitigate recurring failure: {}", m.pattern),
            })
            .collect()
    }
}

impl Default for FailureAnalyzer {
    fn default() -> Self {
        Self::new(PatternRecognitionEngine::default())
    }
}
