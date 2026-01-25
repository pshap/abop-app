//! Error handling and conversions for audio validation

use crate::error::AppError;

/// Enhanced error context for validation failures
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// The field or parameter that failed validation
    pub field: String,
    /// Human-readable error message
    pub message: String,
    /// Optional context information
    pub context: Option<String>,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            context: None,
        }
    }

    /// Create a validation error with additional context
    pub fn with_context(
        field: impl Into<String>,
        message: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            context: Some(context.into()),
        }
    }

    /// Convert to AppError::Audio
    pub fn into_app_error(self) -> AppError {
        let base_msg = format!("Validation failed for '{}': {}", self.field, self.message);
        if let Some(context) = self.context {
            AppError::Audio(format!("{} (context: {})", base_msg, context))
        } else {
            AppError::Audio(base_msg)
        }
    }
}

/// Result type for validation operations
pub type ValidationResult<T> = std::result::Result<T, ValidationError>;

/// Convert ValidationError to AppError
impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        err.into_app_error()
    }
}

/// Common validation error messages
#[allow(missing_docs)]
pub mod error_messages {
    pub const INVALID_SAMPLE_RATE: &str = "Sample rate is outside supported range (8000-192000 Hz)";
    pub const INVALID_BIT_DEPTH: &str = "Bit depth must be 16, 24, or 32 bits";
    pub const INVALID_CHANNELS: &str = "Channel count must be between 1 and 8";
    pub const FILE_NOT_FOUND: &str = "File does not exist or is not accessible";
    pub const FILE_NOT_READABLE: &str = "File cannot be read (permissions issue)";
    pub const UNSUPPORTED_FORMAT: &str = "Audio format is not supported";
    pub const INVALID_DURATION: &str = "Duration is invalid (zero or negative)";
    pub const METADATA_MISSING: &str = "Required metadata is missing";
}

/// Validation result utilities
pub mod result_utils {
    use super::*;

    /// Combine multiple validation results
    pub fn combine_results<T>(results: Vec<ValidationResult<T>>) -> ValidationResult<Vec<T>> {
        let mut valid_values = Vec::new();
        let mut errors = Vec::new();

        for result in results {
            match result {
                Ok(value) => valid_values.push(value),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(valid_values)
        } else {
            Err(ValidationError::new(
                "multiple_validations",
                format!("{} validation errors occurred: {:?}", errors.len(), errors),
            ))
        }
    }

    /// Return first error from multiple validations, or Ok(())
    pub fn first_error_or_ok(results: Vec<ValidationResult<()>>) -> ValidationResult<()> {
        for result in results {
            result?;
        }
        Ok(())
    }
}
