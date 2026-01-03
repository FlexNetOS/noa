//! Manual Edit Preserver
//!
//! Extracts and reinserts manually-added content between regenerations.
//! Uses markers: <!-- provider:add-manual-edit --> ... <!-- /provider:add-manual-edit -->

use std::collections::HashMap;
use std::path::Path;

use super::LithoError;

/// A preserved manual edit section
#[derive(Debug, Clone)]
pub struct ManualEditSection {
    /// Unique identifier based on position/content hash
    pub id: String,
    /// The content between markers (excluding markers)
    pub content: String,
    /// Line number where section started (for reinsertion hints)
    pub original_line: usize,
    /// Context before the section (for fuzzy matching)
    pub context_before: String,
    /// Context after the section (for fuzzy matching)
    pub context_after: String,
}

/// Manual edit preserver
pub struct ManualEditPreserver {
    marker_start: String,
    marker_end: String,
    validation_mode: ValidationMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationMode {
    /// Fail if any manual edits would be lost
    FailOnLoss,
    /// Warn but continue
    WarnOnLoss,
    /// Silently drop unmatched sections
    Silent,
}

impl Default for ManualEditPreserver {
    fn default() -> Self {
        Self {
            marker_start: "<!-- provider:add-manual-edit -->".to_string(),
            marker_end: "<!-- /provider:add-manual-edit -->".to_string(),
            validation_mode: ValidationMode::FailOnLoss,
        }
    }
}

impl ManualEditPreserver {
    /// Create with custom markers
    pub fn new(marker_start: String, marker_end: String, validation_mode: ValidationMode) -> Self {
        Self {
            marker_start,
            marker_end,
            validation_mode,
        }
    }

    /// Extract all manual edit sections from a file
    pub fn extract(&self, content: &str) -> Vec<ManualEditSection> {
        let mut sections = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            if lines[i].contains(&self.marker_start) {
                let start_line = i;
                let mut end_line = None;

                // Find the closing marker
                for j in (i + 1)..lines.len() {
                    if lines[j].contains(&self.marker_end) {
                        end_line = Some(j);
                        break;
                    }
                }

                if let Some(end) = end_line {
                    // Extract content between markers
                    let content: String = lines[(start_line + 1)..end]
                        .iter()
                        .map(|s| *s)
                        .collect::<Vec<_>>()
                        .join("\n");

                    // Get context for fuzzy matching
                    let context_before = if start_line > 0 {
                        lines[start_line.saturating_sub(3)..start_line].join("\n")
                    } else {
                        String::new()
                    };

                    let context_after = if end + 1 < lines.len() {
                        lines[(end + 1)..std::cmp::min(end + 4, lines.len())].join("\n")
                    } else {
                        String::new()
                    };

                    // Generate ID from content hash
                    let id = format!("{:x}", md5_hash(&content));

                    sections.push(ManualEditSection {
                        id,
                        content,
                        original_line: start_line,
                        context_before,
                        context_after,
                    });

                    i = end + 1;
                    continue;
                }
            }
            i += 1;
        }

        sections
    }

    /// Extract sections from a file path
    pub fn extract_from_file(&self, path: &Path) -> Result<Vec<ManualEditSection>, LithoError> {
        let content = std::fs::read_to_string(path)?;
        Ok(self.extract(&content))
    }

    /// Reinsert preserved sections into newly generated content
    pub fn reinsert(
        &self,
        generated_content: &str,
        preserved_sections: &[ManualEditSection],
    ) -> Result<String, LithoError> {
        if preserved_sections.is_empty() {
            return Ok(generated_content.to_string());
        }

        let mut result = generated_content.to_string();
        let mut inserted = HashMap::new();

        for section in preserved_sections {
            // Try to find insertion point by context matching
            if let Some(pos) = self.find_insertion_point(&result, section) {
                let insertion = format!(
                    "\n{}\n{}\n{}\n",
                    self.marker_start, section.content, self.marker_end
                );
                result.insert_str(pos, &insertion);
                inserted.insert(section.id.clone(), true);
            } else {
                match self.validation_mode {
                    ValidationMode::FailOnLoss => {
                        return Err(LithoError::ManualEditLost(format!(
                            "Could not find insertion point for section: {}",
                            section.id
                        )));
                    }
                    ValidationMode::WarnOnLoss => {
                        eprintln!(
                            "Warning: Could not find insertion point for manual edit section: {}",
                            section.id
                        );
                        // Append at end as fallback
                        result.push_str(&format!(
                            "\n{}\n{}\n{}\n",
                            self.marker_start, section.content, self.marker_end
                        ));
                    }
                    ValidationMode::Silent => {
                        // Silently append at end
                        result.push_str(&format!(
                            "\n{}\n{}\n{}\n",
                            self.marker_start, section.content, self.marker_end
                        ));
                    }
                }
            }
        }

        Ok(result)
    }

    /// Find the best insertion point for a section based on context matching
    fn find_insertion_point(&self, content: &str, section: &ManualEditSection) -> Option<usize> {
        // Try exact context_before match
        if !section.context_before.is_empty() {
            if let Some(pos) = content.find(&section.context_before) {
                return Some(pos + section.context_before.len());
            }
        }

        // Try exact context_after match
        if !section.context_after.is_empty() {
            if let Some(pos) = content.find(&section.context_after) {
                return Some(pos);
            }
        }

        // Fallback: try to find similar line patterns
        let lines: Vec<&str> = content.lines().collect();
        let original_ratio = section.original_line as f32 / 100.0; // Rough position estimate

        // Try to insert at similar relative position
        let target_line = (lines.len() as f32 * original_ratio.min(1.0)) as usize;
        let target_line = target_line.min(lines.len());

        // Find byte position of target line
        let mut pos = 0;
        for (i, line) in lines.iter().enumerate() {
            if i == target_line {
                return Some(pos);
            }
            pos += line.len() + 1; // +1 for newline
        }

        Some(content.len()) // Append at end
    }

    /// Validate that all sections were preserved
    pub fn validate(
        &self,
        original: &str,
        regenerated: &str,
    ) -> Result<(), LithoError> {
        let original_sections = self.extract(original);
        let new_sections = self.extract(regenerated);

        let original_ids: std::collections::HashSet<_> =
            original_sections.iter().map(|s| &s.id).collect();
        let new_ids: std::collections::HashSet<_> =
            new_sections.iter().map(|s| &s.id).collect();

        let missing: Vec<_> = original_ids.difference(&new_ids).collect();

        if !missing.is_empty() && self.validation_mode == ValidationMode::FailOnLoss {
            return Err(LithoError::ManualEditLost(format!(
                "Lost {} manual edit sections: {:?}",
                missing.len(),
                missing
            )));
        }

        Ok(())
    }
}

/// Simple MD5-like hash for section IDs (not cryptographic, just for identification)
fn md5_hash(content: &str) -> u64 {
    let mut hash: u64 = 0;
    for (i, byte) in content.bytes().enumerate() {
        hash = hash.wrapping_add((byte as u64).wrapping_mul((i as u64).wrapping_add(1)));
        hash = hash.rotate_left(5);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_sections() {
        let content = r#"# Header

Some generated content.

<!-- provider:add-manual-edit -->
This is manually added content.
It should be preserved.
<!-- /provider:add-manual-edit -->

More generated content.
"#;

        let preserver = ManualEditPreserver::default();
        let sections = preserver.extract(content);

        assert_eq!(sections.len(), 1);
        assert!(sections[0].content.contains("manually added content"));
    }

    #[test]
    fn test_reinsert() {
        let original = r#"# Header

<!-- provider:add-manual-edit -->
Custom note here.
<!-- /provider:add-manual-edit -->

## Section
"#;

        let generated = r#"# Header

## Section
"#;

        let preserver = ManualEditPreserver::new(
            "<!-- provider:add-manual-edit -->".to_string(),
            "<!-- /provider:add-manual-edit -->".to_string(),
            ValidationMode::WarnOnLoss,
        );

        let sections = preserver.extract(original);
        let result = preserver.reinsert(generated, &sections).unwrap();

        assert!(result.contains("Custom note here."));
    }
}
