//! Configuration validation utilities
//!
//! This module provides validation logic for configuration settings,
//! ensuring that loaded configurations are valid and providing helpful
//! error messages for invalid settings.

use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Validation error with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Field that failed validation
    pub field: String,
    /// Error message
    pub message: String,
    /// Suggested fix
    pub suggestion: Option<String>,
}

/// Validation result containing all errors and warnings
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Validation errors that prevent the config from being used
    pub errors: Vec<ValidationError>,
    /// Validation warnings that indicate potential issues
    pub warnings: Vec<ValidationError>,
    /// Whether the configuration is valid overall
    pub is_valid: bool,
}

impl ValidationResult {
    /// Create a new validation result
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            is_valid: true,
        }
    }

    /// Add an error to the validation result
    pub fn add_error(&mut self, field: &str, message: &str, suggestion: Option<&str>) {
        self.errors.push(ValidationError {
            field: field.to_string(),
            message: message.to_string(),
            suggestion: suggestion.map(String::from),
        });
        self.is_valid = false;
    }

    /// Add a warning to the validation result
    pub fn add_warning(&mut self, field: &str, message: &str, suggestion: Option<&str>) {
        self.warnings.push(ValidationError {
            field: field.to_string(),
            message: message.to_string(),
            suggestion: suggestion.map(String::from),
        });
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for validating configuration structs
pub trait ConfigValidation {
    /// Validate the configuration and return any errors or warnings
    fn validate(&self) -> ValidationResult;

    /// Validate and fix common issues automatically
    fn validate_and_fix(&mut self) -> ValidationResult {
        self.validate()
    }
}

/// Validate a file path exists and is accessible
pub fn validate_path_exists(path: &Path, field_name: &str) -> Result<()> {
    if !path.exists() {
        return Err(AppError::Config(format!(
            "Path '{}' for field '{}' does not exist",
            path.display(),
            field_name
        )));
    }
    Ok(())
}

/// Validate a directory path and create it if it doesn't exist
pub fn validate_or_create_directory(path: &Path, field_name: &str) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path).map_err(|e| {
            AppError::Config(format!(
                "Failed to create directory '{}' for field '{}': {}",
                path.display(),
                field_name,
                e
            ))
        })?;
    } else if !path.is_dir() {
        return Err(AppError::Config(format!(
            "Path '{}' for field '{}' exists but is not a directory",
            path.display(),
            field_name
        )));
    }
    Ok(())
}

/// Validate a numeric value is within a reasonable range
pub fn validate_range<T: PartialOrd + std::fmt::Display>(
    value: T,
    min: T,
    max: T,
    field_name: &str,
) -> Result<()> {
    if value < min || value > max {
        return Err(AppError::Config(format!(
            "Value '{}' for field '{}' is out of range [{}, {}]",
            value, field_name, min, max
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_validation_result() {
        let mut result = ValidationResult::new();
        assert!(result.is_valid);
        assert!(!result.has_errors());

        result.add_error("test_field", "Test error", Some("Fix suggestion"));
        assert!(!result.is_valid);
        assert!(result.has_errors());
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_validate_path_exists() {
        let temp_dir = tempdir().unwrap();
        let existing_path = temp_dir.path();
        let non_existing_path = temp_dir.path().join("nonexistent");

        assert!(validate_path_exists(existing_path, "test").is_ok());
        assert!(validate_path_exists(&non_existing_path, "test").is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, 1, 10, "test").is_ok());
        assert!(validate_range(0, 1, 10, "test").is_err());
        assert!(validate_range(15, 1, 10, "test").is_err());
    }
}
