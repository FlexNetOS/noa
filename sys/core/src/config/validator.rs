//! Configuration Validator
//!
//! Validates configuration against schema and business rules.

use std::path::Path;

use super::NoaConfig;
use crate::error::{ConfigError, Result, ValidationError};

/// Configuration validator
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate a configuration
    pub fn validate(config: &NoaConfig) -> Result<()> {
        Self::validate_noa_root(config)?;
        Self::validate_database(config)?;
        Self::validate_logging(config)?;
        Self::validate_providers(config)?;
        Ok(())
    }

    fn validate_noa_root(config: &NoaConfig) -> Result<()> {
        if !config.noa_root.exists() {
            return Err(ConfigError::ValidationError {
                field: "noa_root".to_string(),
                message: format!("Directory does not exist: {}", config.noa_root.display()),
            }
            .into());
        }

        // Check for required subdirectories
        let required_dirs = ["bin", "config", "data", "logs"];
        for dir in required_dirs {
            let path = config.noa_root.join(dir);
            if !path.exists() {
                return Err(ConfigError::ValidationError {
                    field: "noa_root".to_string(),
                    message: format!("Required directory missing: {}", dir),
                }
                .into());
            }
        }

        Ok(())
    }

    fn validate_database(config: &NoaConfig) -> Result<()> {
        let valid_drivers = ["sqlite", "postgresql"];
        if !valid_drivers.contains(&config.database.driver.as_str()) {
            return Err(ConfigError::InvalidValue {
                field: "database.driver".to_string(),
                value: config.database.driver.clone(),
                expected: "sqlite or postgresql".to_string(),
            }
            .into());
        }

        if config.database.max_connections == 0 {
            return Err(ConfigError::ValidationError {
                field: "database.max_connections".to_string(),
                message: "Must be greater than 0".to_string(),
            }
            .into());
        }

        // For SQLite, ensure parent directory exists
        if config.database.driver == "sqlite" {
            if let Some(parent) = config.database.path.parent() {
                if !parent.exists() {
                    return Err(ConfigError::ValidationError {
                        field: "database.path".to_string(),
                        message: format!("Parent directory does not exist: {}", parent.display()),
                    }
                    .into());
                }
            }
        }

        Ok(())
    }

    fn validate_logging(config: &NoaConfig) -> Result<()> {
        // Ensure log output parent directory exists
        if let Some(parent) = config.logging.output.parent() {
            if !parent.exists() {
                return Err(ConfigError::ValidationError {
                    field: "logging.output".to_string(),
                    message: format!("Parent directory does not exist: {}", parent.display()),
                }
                .into());
            }
        }

        if config.logging.max_size_mb == 0 {
            return Err(ConfigError::ValidationError {
                field: "logging.max_size_mb".to_string(),
                message: "Must be greater than 0".to_string(),
            }
            .into());
        }

        Ok(())
    }

    fn validate_providers(config: &NoaConfig) -> Result<()> {
        // Validate that at least one provider is configured
        if config.providers.providers.is_empty() {
            // This is a warning, not an error - we can operate without providers
            tracing::warn!("No AI providers configured");
        }

        // Validate each provider's config path
        for (name, settings) in &config.providers.providers {
            if settings.enabled && !settings.config_path.as_os_str().is_empty() {
                if !settings.config_path.exists() {
                    tracing::warn!(
                        "Provider {} config path does not exist: {}",
                        name,
                        settings.config_path.display()
                    );
                }
            }
        }

        Ok(())
    }
}

/// Validate a file path against constraints
pub fn validate_path(path: &Path, must_exist: bool, must_be_dir: bool) -> Result<()> {
    if must_exist && !path.exists() {
        return Err(ValidationError::new(
            "path",
            format!("Path does not exist: {}", path.display()),
            "PATH_NOT_FOUND",
        )
        .into());
    }

    if must_be_dir && path.exists() && !path.is_dir() {
        return Err(ValidationError::new(
            "path",
            format!("Path is not a directory: {}", path.display()),
            "NOT_A_DIRECTORY",
        )
        .into());
    }

    Ok(())
}

/// Validate a string is not empty
pub fn validate_not_empty(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(ValidationError::new(field, "Value cannot be empty", "EMPTY_VALUE").into());
    }
    Ok(())
}

/// Validate a numeric value is in range
pub fn validate_range<T: PartialOrd + std::fmt::Display>(
    value: T,
    min: T,
    max: T,
    field: &str,
) -> Result<()> {
    if value < min || value > max {
        return Err(ValidationError::new(
            field,
            format!("Value {} is out of range [{}, {}]", value, min, max),
            "OUT_OF_RANGE",
        )
        .into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_not_empty() {
        assert!(validate_not_empty("test", "field").is_ok());
        assert!(validate_not_empty("", "field").is_err());
        assert!(validate_not_empty("   ", "field").is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, 1, 10, "field").is_ok());
        assert!(validate_range(0, 1, 10, "field").is_err());
        assert!(validate_range(11, 1, 10, "field").is_err());
    }
}
