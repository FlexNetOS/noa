use anyhow::{anyhow, Result};

/// Validate that a string field is non-empty.
pub fn validate_non_empty(field_name: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field_name} must not be empty"));
    }
    Ok(())
}

/// Validate that an identifier-like string only contains safe characters.
///
/// This is intentionally conservative and can be relaxed later.
pub fn validate_identifier(field_name: &str, value: &str) -> Result<()> {
    validate_non_empty(field_name, value)?;

    let ok = value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');

    if !ok {
        return Err(anyhow!(
            "{field_name} contains invalid characters (allowed: a-zA-Z0-9-_. )"
        ));
    }

    Ok(())
}
