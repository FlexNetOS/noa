use std::collections::HashMap;

/// Recognized pattern details.
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern: String,
    pub occurrences: usize,
    pub confidence: f64,
}

/// Pattern recognition engine using frequency analysis.
pub struct PatternRecognitionEngine;

impl PatternRecognitionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn identify_patterns(&self, signals: &[String]) -> Vec<PatternMatch> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for signal in signals {
            *counts.entry(signal.clone()).or_insert(0) += 1;
        }

        let total = signals.len().max(1) as f64;
        counts
            .into_iter()
            .map(|(pattern, occurrences)| PatternMatch {
                confidence: occurrences as f64 / total,
                pattern,
                occurrences,
            })
            .collect()
    }
}

impl Default for PatternRecognitionEngine {
    fn default() -> Self {
        Self::new()
    }
}
