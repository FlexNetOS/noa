use crate::error::{NoaError, Result, ValidationError};

/// Result of boundary output verification.
#[derive(Debug, Clone)]
pub struct VerifiedOutput {
    pub accepted: bool,
    pub message: String,
}

/// Verifier for responses leaving the NKAL boundary.
pub struct OutputVerifier;

impl OutputVerifier {
    /// Verify textual output before releasing across the boundary.
    pub fn verify_text(output: &str) -> VerifiedOutput {
        if output.trim().is_empty() {
            return VerifiedOutput {
                accepted: false,
                message: "Output is empty after trimming".to_string(),
            };
        }

        // Very lightweight secret guardrail to avoid accidental key leakage.
        if output.contains("BEGIN PRIVATE KEY")
            || output.contains("BEGIN RSA PRIVATE KEY")
            || output.contains("PRIVATE KEY-----")
        {
            return VerifiedOutput {
                accepted: false,
                message: "Output resembles a private key; blocked".to_string(),
            };
        }

        VerifiedOutput {
            accepted: true,
            message: "Output verified".to_string(),
        }
    }

    /// Convenience helper to produce a Result for callers that expect errors.
    pub fn assert_text(output: &str, context: &str) -> Result<()> {
        let verified = Self::verify_text(output);
        if verified.accepted {
            Ok(())
        } else {
            Err(NoaError::Validation(ValidationError::new(
                context,
                verified.message,
                "NKAL_OUTPUT_BLOCKED",
            )))
        }
    }
}
