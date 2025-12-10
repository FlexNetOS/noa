use crate::error::{NoaError, Result, ValidationError};

/// Sanitized representation of an input crossing the NKAL boundary.
#[derive(Debug, Clone)]
pub struct SanitizedInput {
    pub original: String,
    pub sanitized: String,
    pub warnings: Vec<String>,
    pub blocked: bool,
}

/// Simple input sanitizer focused on boundary-crossing values.
pub struct Sanitizer;

impl Sanitizer {
    /// Sanitize a command or action string (newline stripping, control removal).
    pub fn sanitize_command(input: &str) -> SanitizedInput {
        let mut warnings = Vec::new();
        let mut blocked = false;

        let mut sanitized: String = input
            .chars()
            .map(|c| if c.is_control() { ' ' } else { c })
            .collect();

        if sanitized.contains("..") {
            warnings.push("Path traversal marker detected ('..')".to_string());
        }

        if sanitized.contains('|') || sanitized.contains(';') {
            warnings.push("Command chaining characters detected".to_string());
            blocked = true;
        }

        if sanitized.contains('$') && sanitized.contains('(') && sanitized.contains(')') {
            warnings.push("Potential variable/command substitution detected".to_string());
        }

        sanitized = sanitized.trim().to_string();

        SanitizedInput {
            original: input.to_string(),
            sanitized,
            warnings,
            blocked,
        }
    }

    /// Sanitize filesystem paths to avoid traversal and drive escapes.
    pub fn sanitize_path(input: &str) -> SanitizedInput {
        let mut sanitized: String = input.replace('\\', "/");
        let mut warnings = Vec::new();
        let mut blocked = false;

        if sanitized.contains("../") || sanitized.starts_with("..") {
            warnings.push("Relative parent segments are not allowed".to_string());
            blocked = true;
        }

        if sanitized.starts_with("//") {
            warnings.push("UNC paths are blocked at NKAL boundary".to_string());
            blocked = true;
        }

        sanitized = sanitized.trim_matches('"').to_string();

        SanitizedInput {
            original: input.to_string(),
            sanitized,
            warnings,
            blocked,
        }
    }

    /// Fail fast if sanitization flagged the value as blocked.
    pub fn assert_not_blocked(input: SanitizedInput, field: &str) -> Result<String> {
        if input.blocked {
            return Err(NoaError::Validation(ValidationError::new(
                field,
                "Rejected by NKAL sanitization",
                "NKAL_SANITIZE_BLOCKED",
            )));
        }

        if input.sanitized.is_empty() {
            return Err(NoaError::Validation(ValidationError::new(
                field,
                "Empty value after sanitization",
                "NKAL_SANITIZE_EMPTY",
            )));
        }

        Ok(input.sanitized)
    }
}
