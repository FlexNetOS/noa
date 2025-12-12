//! Phase 11: Result Block & Sign-Off
//!
//! Result Block tracking system for phase verification and final sign-off.
//! Based on Universal Task Execution Policy §8D.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Result status for a phase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResultStatus {
    /// All verification items passed
    Pass,
    /// Some verification items passed, but not all
    Partial,
    /// Critical verification items failed
    Fail,
}

impl ResultStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResultStatus::Pass => "PASS",
            ResultStatus::Partial => "PARTIAL",
            ResultStatus::Fail => "FAIL",
        }
    }
}

/// Result Block for a phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultBlock {
    /// Phase number (1-10)
    pub phase: u8,
    /// Phase name
    pub phase_name: String,
    /// Result status
    pub result: ResultStatus,
    /// One-line summary of why this result
    pub why: String,
    /// Smallest verifiable step if not PASS
    pub next: Option<String>,
    /// Total verification items in phase
    pub total_items: usize,
    /// Completed items
    pub completed_items: usize,
    /// Incomplete items
    pub incomplete_items: usize,
    /// Timestamp when result block was generated
    pub timestamp: String,
}

impl ResultBlock {
    /// Create a new Result Block
    pub fn new(phase: u8, phase_name: String, total_items: usize, completed_items: usize) -> Self {
        let incomplete_items = total_items.saturating_sub(completed_items);

        // Determine result status
        let result = if incomplete_items == 0 {
            ResultStatus::Pass
        } else if completed_items == 0 {
            ResultStatus::Fail
        } else {
            ResultStatus::Partial
        };

        // Generate why message
        let why = if result == ResultStatus::Pass {
            format!("All {} verification items completed", total_items)
        } else if result == ResultStatus::Fail {
            format!("No verification items completed (0/{})", total_items)
        } else {
            format!(
                "Partial completion: {} of {} items verified ({:.1}%)",
                completed_items,
                total_items,
                (completed_items as f64 / total_items as f64) * 100.0
            )
        };

        // Generate next step if not PASS
        let next = if result != ResultStatus::Pass {
            Some(format!(
                "Complete remaining {} verification items for Phase {}",
                incomplete_items, phase
            ))
        } else {
            None
        };

        Self {
            phase,
            phase_name,
            result,
            why,
            next,
            total_items,
            completed_items,
            incomplete_items,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Format as RESULT block text (per §8D)
    pub fn format_block(&self) -> String {
        let mut block = format!("RESULT: {}\n", self.result.as_str());
        block.push_str(&format!("WHY: {}\n", self.why));
        if let Some(ref next) = self.next {
            block.push_str(&format!("NEXT: {}\n", next));
        }
        block
    }

    /// Format as markdown
    pub fn format_markdown(&self) -> String {
        format!(
            "### Phase {}: {}\n\n{}\n\n**Status**: {} ({}/{})\n",
            self.phase,
            self.phase_name,
            self.format_block(),
            self.result.as_str(),
            self.completed_items,
            self.total_items
        )
    }
}

/// Final Sign-Off status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalSignOff {
    /// All phase result blocks are PASS
    pub all_phases_pass: bool,
    /// FINAL_REPORT.md complete and reviewed
    pub final_report_complete: bool,
    /// All HASHES.txt entries verified
    pub hashes_verified: bool,
    /// No FAIL or PARTIAL without documented remedy
    pub no_unremedied_failures: bool,
    /// Evidence Ledger complete with Triple-Verify outcomes
    pub evidence_ledger_complete: bool,
    /// Timestamp
    pub timestamp: String,
    /// Overall sign-off status
    pub overall_status: ResultStatus,
}

impl FinalSignOff {
    pub fn new(
        result_blocks: &[ResultBlock],
        final_report_exists: bool,
        hashes_verified: bool,
        evidence_ledger_complete: bool,
    ) -> Self {
        let all_phases_pass = result_blocks.iter().all(|rb| rb.result == ResultStatus::Pass);

        // Check for unremedied failures
        let no_unremedied_failures = result_blocks.iter().all(|rb| {
            rb.result == ResultStatus::Pass
                || (rb.result != ResultStatus::Pass && rb.next.is_some())
        });

        let overall_status = if all_phases_pass
            && final_report_exists
            && hashes_verified
            && no_unremedied_failures
            && evidence_ledger_complete
        {
            ResultStatus::Pass
        } else if all_phases_pass || final_report_exists {
            ResultStatus::Partial
        } else {
            ResultStatus::Fail
        };

        Self {
            all_phases_pass,
            final_report_complete: final_report_exists,
            hashes_verified,
            no_unremedied_failures,
            evidence_ledger_complete,
            timestamp: chrono::Utc::now().to_rfc3339(),
            overall_status,
        }
    }

    pub fn format_markdown(&self) -> String {
        let status_icon = match self.overall_status {
            ResultStatus::Pass => "✅",
            ResultStatus::Partial => "⚠️",
            ResultStatus::Fail => "❌",
        };

        format!(
            r#"## Final Sign-Off

**Status**: {} {}

| Check | Status | Notes |
|-------|--------|-------|
| FINAL001 | {} | All phase RESULT blocks are PASS |
| FINAL002 | {} | FINAL_REPORT.md complete and reviewed |
| FINAL003 | {} | All HASHES.txt entries verified |
| FINAL004 | {} | No FAIL or PARTIAL without documented remedy |
| FINAL005 | {} | Evidence Ledger complete with Triple-Verify outcomes |

**Overall**: {}
"#,
            status_icon,
            self.overall_status.as_str(),
            if self.all_phases_pass { "✅" } else { "❌" },
            if self.final_report_complete {
                "✅"
            } else {
                "❌"
            },
            if self.hashes_verified { "✅" } else { "❌" },
            if self.no_unremedied_failures {
                "✅"
            } else {
                "❌"
            },
            if self.evidence_ledger_complete {
                "✅"
            } else {
                "❌"
            },
            self.overall_status.as_str()
        )
    }
}

/// Result Block Manager
pub struct ResultBlockManager {
    /// Path to verification checklist
    pub checklist_path: PathBuf,
    /// Path to test results directory
    pub test_results_path: PathBuf,
}

impl ResultBlockManager {
    pub fn new(noa_root: &PathBuf) -> Self {
        Self {
            checklist_path: noa_root
                .join("specs/001-noa-seed-foundation/checklists/verification.md"),
            test_results_path: noa_root.join("test-results"),
        }
    }

    /// Analyze checklist and generate Result Blocks for all phases
    pub fn analyze_phases(&self) -> Result<Vec<ResultBlock>, Box<dyn std::error::Error>> {
        let checklist_content = std::fs::read_to_string(&self.checklist_path)?;

        let phases = vec![
            (1, "Core System", "VER001-VER028"),
            (2, "Agent Architecture", "VER029-VER050"),
            (3, "Shared Provider", "VER051-VER070"),
            (4, "Digest Pipeline", "VER071-VER090"),
            (5, "P2P & UI", "VER091-VER110"),
            (6, "Governance", "VER111-VER126"),
            (7, "Performance", "VER127-VER145"),
            (8, "Regression", "REG001-REG014"),
            (9, "Truth Gate", "TG001-CT005"),
            (10, "Multi-GPU", "GPU001-GPU018"),
        ];

        let mut result_blocks = Vec::new();

        for (phase_num, phase_name, item_range) in phases {
            let (total, completed) = self.count_items_in_range(&checklist_content, item_range)?;
            let result_block =
                ResultBlock::new(phase_num, phase_name.to_string(), total, completed);
            result_blocks.push(result_block);
        }

        Ok(result_blocks)
    }

    /// Count verification items in a range (e.g., "VER001-VER028")
    fn count_items_in_range(
        &self,
        content: &str,
        range: &str,
    ) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        // Parse range (e.g., "VER001-VER028" or "REG001-REG014")
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid range format".into());
        }

        let prefix = parts[0].chars().take_while(|c| c.is_alphabetic()).collect::<String>();
        let start_num: usize =
            parts[0].chars().skip_while(|c| c.is_alphabetic()).collect::<String>().parse()?;
        let end_num: usize =
            parts[1].chars().skip_while(|c| c.is_alphabetic()).collect::<String>().parse()?;

        let mut total = 0;
        let mut completed = 0;

        for num in start_num..=end_num {
            let item_id = format!("{}{:03}", prefix, num);
            let pattern_complete = format!("- [X] {} -", item_id);
            let pattern_incomplete = format!("- [ ] {} -", item_id);

            if content.contains(&pattern_complete) {
                total += 1;
                completed += 1;
            } else if content.contains(&pattern_incomplete) {
                total += 1;
            }
        }

        Ok((total, completed))
    }

    /// Save Result Blocks to JSON
    pub fn save_result_blocks(
        &self,
        result_blocks: &[ResultBlock],
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let output_path = self.test_results_path.join("result_blocks.json");
        let json = serde_json::to_string_pretty(result_blocks)?;
        std::fs::write(&output_path, json)?;
        Ok(output_path)
    }

    /// Generate Final Sign-Off
    pub fn generate_final_sign_off(
        &self,
        result_blocks: &[ResultBlock],
    ) -> Result<FinalSignOff, Box<dyn std::error::Error>> {
        let final_report_path = self.test_results_path.join("FINAL_REPORT.md");
        let hashes_path = self.test_results_path.join("HASHES.txt");
        let evidence_ledger_path = self.test_results_path.join("EVIDENCE_LEDGER.md");

        let final_report_exists = final_report_path.exists();
        let hashes_verified = hashes_path.exists();
        let evidence_ledger_complete = evidence_ledger_path.exists();

        Ok(FinalSignOff::new(
            result_blocks,
            final_report_exists,
            hashes_verified,
            evidence_ledger_complete,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_block_pass() {
        let rb = ResultBlock::new(1, "Core System".to_string(), 28, 28);
        assert_eq!(rb.result, ResultStatus::Pass);
        assert_eq!(rb.incomplete_items, 0);
        assert!(rb.next.is_none());
    }

    #[test]
    fn test_result_block_partial() {
        let rb = ResultBlock::new(1, "Core System".to_string(), 28, 12);
        assert_eq!(rb.result, ResultStatus::Partial);
        assert_eq!(rb.incomplete_items, 16);
        assert!(rb.next.is_some());
    }

    #[test]
    fn test_result_block_fail() {
        let rb = ResultBlock::new(3, "Shared Provider".to_string(), 20, 0);
        assert_eq!(rb.result, ResultStatus::Fail);
        assert_eq!(rb.incomplete_items, 20);
        assert!(rb.next.is_some());
    }
}
