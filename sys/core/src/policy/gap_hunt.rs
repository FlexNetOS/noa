use std::collections::HashMap;

/// Gap hunt finding details.
#[derive(Debug, Clone)]
pub struct GapHuntFinding {
    pub area: String,
    pub missing_items: Vec<String>,
}

/// Gap hunt rule scans coverage map for missing artifacts.
pub struct GapHuntRule {
    required: Vec<String>,
}

impl GapHuntRule {
    pub fn new(required: Vec<String>) -> Self {
        Self { required }
    }

    pub fn find_gaps(&self, coverage: &HashMap<String, bool>) -> Vec<GapHuntFinding> {
        let mut missing = Vec::new();
        for area in &self.required {
            if !coverage.get(area).copied().unwrap_or(false) {
                missing.push(area.clone());
            }
        }

        if missing.is_empty() {
            Vec::new()
        } else {
            vec![GapHuntFinding {
                area: "coverage".to_string(),
                missing_items: missing,
            }]
        }
    }
}

impl Default for GapHuntRule {
    fn default() -> Self {
        Self::new(vec![
            "tests".to_string(),
            "docs".to_string(),
            "benchmarks".to_string(),
        ])
    }
}
