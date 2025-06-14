//! Enhanced error pattern matching for repairs

use super::error::ValidationError;

/// Strongly typed issue patterns for better type safety
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssuePattern {
    /// Issue where an entity has an empty or missing name
    EmptyName,
    /// Issue where a referenced file does not exist on the filesystem
    FileNotExists,
    /// Issue where duration values are invalid or malformed
    InvalidDuration,
    /// Issue where an entity is orphaned (references missing parent)
    Orphaned,
    /// Issue where a value exceeds the expected duration bounds
    ExceedsDuration,
    /// Issue where a previously existing entity no longer exists
    NoLongerExists,
    /// Issue where a file or entity is too small to be valid
    TooSmall,
    /// Issue where duplicate entities are detected
    Duplicate,
    /// Unknown issue pattern that doesn't match predefined categories
    Unknown(String),
}

impl IssuePattern {
    /// Extract pattern from validation error message
    #[must_use]
    pub fn from_message(message: &str) -> Self {
        use super::repair_constants::error_patterns as patterns;

        if message.contains(patterns::EMPTY_NAME) {
            Self::EmptyName
        } else if message.contains(patterns::DOES_NOT_EXIST) {
            Self::FileNotExists
        } else if message.contains(patterns::INVALID_DURATION) {
            Self::InvalidDuration
        } else if message.contains(patterns::NON_EXISTENT) || message.contains(patterns::ORPHANED) {
            Self::Orphaned
        } else if message.contains(patterns::EXCEEDS_DURATION) {
            Self::ExceedsDuration
        } else if message.contains(patterns::NO_LONGER_EXISTS) {
            Self::NoLongerExists
        } else if message.contains(patterns::TOO_SMALL) {
            Self::TooSmall
        } else if message.contains(patterns::DUPLICATE) {
            Self::Duplicate
        } else {
            Self::Unknown(message.to_string())
        }
    }

    /// Extract pattern from a validation error
    #[must_use]
    pub fn from_validation_error(error: &ValidationError) -> Option<Self> {
        Some(Self::from_message(&error.message))
    }
}
