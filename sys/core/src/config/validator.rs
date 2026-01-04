//! configsuration Validator
//!
//! Validates configsuration against schema and business rules.

use std::path::Path;

use crate::error::{configsError, Result, ValidationError};
use super::Noaconfigs;

/// configsuration validator
pub struct configsValidator;

impl configsValidator {
    /// Validate a configsuration
    pub fn validate(configs: &Noaconfigs) -> Result<()> {
        Self::validate_noa_root(configs)?;
        Self::validate_database(configs)?;
        Self::validate_logging(configs)?;
        Self::validate_providers(configs)?;
        Ok(())
    }

    fn validate_noa_root(configs: &Noaconfigs) -> Result<()> {
        validate_path(&configs.noa_root, true, true).map_err(|e| -> crate::error::NoaError {
            configsError::ValidationError {
                field: "noa_root".to_string(),
                message: e.to_string(),
            }
            .into()
        })?;

        // Check for required subdirectories
        let required_dirs = ["bin", "configs", "data", "logs"];
        for dir in required_dirs {
            let path = configs.noa_root.join(dir);
            validate_path(&path, true, true).map_err(|e| -> crate::error::NoaError {
                configsError::ValidationError {
                    field: "noa_root".to_string(),
                    message: format!("{} ({}): {}", dir, path.display(), e),
                }
                .into()
            })?;
        }

        Ok(())
    }

    fn validate_database(configs: &Noaconfigs) -> Result<()> {
        validate_not_empty(&configs.database.driver, "database.driver")
            .map_err(|e| -> crate::error::NoaError {
                configsError::ValidationError {
                    field: "database.driver".to_string(),
                    message: e.to_string(),
                }
                .into()
            })?;

        let valid_drivers = ["sqlite", "postgresql"];
        if !valid_drivers.contains(&configs.database.driver.as_str()) {
            return Err(configsError::InvalidValue {
                field: "database.driver".to_string(),
                value: configs.database.driver.clone(),
                expected: "sqlite or postgresql".to_string(),
            }.into());
        }

        validate_range(
            configs.database.max_connections,
            1u32,
            u32::MAX,
            "database.max_connections",
        )
        .map_err(|e| -> crate::error::NoaError {
            configsError::ValidationError {
                field: "database.max_connections".to_string(),
                message: e.to_string(),
            }
            .into()
        })?;

        // For PostgreSQL, require a URL.
        if configs.database.driver == "postgresql" {
            let url_ok = configs
                .database
                .url
                .as_ref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);

            if !url_ok {
                return Err(configsError::ValidationError {
                    field: "database.url".to_string(),
                    message: "PostgreSQL requires database.primary.url (or env DATABASE_URL/DB_CONNECTION_STRING)".to_string(),
                }
                .into());
            }
        }

        // For SQLite, ensure parent directory exists
        if configs.database.driver == "sqlite" {
            if let Some(parent) = configs.database.path.parent() {
                validate_path(parent, true, true).map_err(|e| -> crate::error::NoaError {
                    configsError::ValidationError {
                        field: "database.path".to_string(),
                        message: e.to_string(),
                    }
                    .into()
                })?;
            }
        }

        Ok(())
    }

    fn validate_logging(configs: &Noaconfigs) -> Result<()> {
        // Ensure log output parent directory exists
        if let Some(parent) = configs.logging.output.parent() {
            validate_path(parent, true, true).map_err(|e| -> crate::error::NoaError {
                configsError::ValidationError {
                    field: "logging.output".to_string(),
                    message: e.to_string(),
                }
                .into()
            })?;
        }

        validate_range(
            configs.logging.max_size_mb,
            1u64,
            u64::MAX,
            "logging.max_size_mb",
        )
        .map_err(|e| -> crate::error::NoaError {
            configsError::ValidationError {
                field: "logging.max_size_mb".to_string(),
                message: e.to_string(),
            }
            .into()
        })?;

        Ok(())
    }

    fn validate_providers(configs: &Noaconfigs) -> Result<()> {
        // Validate that at least one provider is configsured
        if configs.providers.providers.is_empty() {
            // This is a warning, not an error - we can operate without providers
            tracing::warn!("No AI providers configsured");
        }

        // Validate each provider's configs path
        for (name, settings) in &configs.providers.providers {
            if settings.enabled && !settings.configs_path.as_os_str().is_empty() {
                if !settings.configs_path.exists() {
                    tracing::warn!(
                        "Provider {} configs path does not exist: {}",
                        name,
                        settings.configs_path.display()
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
        ).into());
    }

    if must_be_dir && path.exists() && !path.is_dir() {
        return Err(ValidationError::new(
            "path",
            format!("Path is not a directory: {}", path.display()),
            "NOT_A_DIRECTORY",
        ).into());
    }

    Ok(())
}

/// Validate a string is not empty
pub fn validate_not_empty(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(ValidationError::new(
            field,
            "Value cannot be empty",
            "EMPTY_VALUE",
        ).into());
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
        ).into());
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

