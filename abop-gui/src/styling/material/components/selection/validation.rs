//! Validation system for Material Design 3 selection components
//!
//! This module provides comprehensive validation for component properties,
//! states, and configurations to ensure robust component behavior.

use thiserror::Error;

use super::constants;
use super::properties::{ChipVariant, ComponentProps};
use super::states::{CheckboxState, ChipState, SwitchState};

// ============================================================================
// Validation Configuration
// ============================================================================

/// Validation configuration for components
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationConfig {
    /// Maximum label length (characters)
    pub max_label_length: usize,
    /// Whether empty labels are allowed
    pub allow_empty_label: bool,
    /// Custom validation rules
    pub custom_rules: Vec<ValidationRule>,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_label_length: constants::ui::MAX_LABEL_LENGTH,
            allow_empty_label: true,
            custom_rules: Vec::new(),
        }
    }
}

/// Custom validation rule
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationRule {
    /// Rule name for error reporting
    pub name: String,
    /// Error message if rule fails
    pub error_message: String,
}

// ============================================================================
// Validation Errors
// ============================================================================

/// Validation errors for selection components
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SelectionError {
    /// Invalid component state combination
    #[error("Invalid component state: {details}")]
    InvalidState {
        /// Detailed description of the invalid state
        details: String,
    },

    /// Label validation error
    #[error("Label validation failed: {reason}")]
    InvalidLabel {
        /// Reason why the label is invalid
        reason: String,
    },

    /// Label too long
    #[error("Label too long: {len} characters (max {max})")]
    LabelTooLong {
        /// Actual length of the label
        len: usize,
        /// Maximum allowed length
        max: usize,
    },

    /// Empty label when not allowed
    #[error("Label cannot be empty")]
    EmptyLabel,

    /// Conflicting states
    #[error("Conflicting states: {details}")]
    ConflictingStates {
        /// Description of the state conflict
        details: String,
    },

    /// Custom validation rule failed
    #[error("Validation rule '{rule}' failed: {message}")]
    CustomRule {
        /// Name of the validation rule that failed
        rule: String,
        /// Error message describing the failure
        message: String,
    },

    /// General validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Generic validation helper for selection components
fn validate_selection_component_props(props: &ComponentProps) -> Result<(), SelectionError> {
    validate_props(props, &ValidationConfig::default())
}

/// Validate component properties
pub fn validate_props(
    props: &ComponentProps,
    config: &ValidationConfig,
) -> Result<(), SelectionError> {
    if let Some(ref label) = props.label {
        validate_label(label, config)?;
    }
    Ok(())
}

/// Validate a label string
pub fn validate_label(label: &str, config: &ValidationConfig) -> Result<(), SelectionError> {
    if label.is_empty() && !config.allow_empty_label {
        return Err(SelectionError::EmptyLabel);
    }

    if label.len() > config.max_label_length {
        return Err(SelectionError::LabelTooLong {
            len: label.len(),
            max: config.max_label_length,
        });
    }

    Ok(())
}

/// Validate checkbox state consistency
pub fn validate_checkbox_state(
    _state: CheckboxState,
    props: &ComponentProps,
) -> Result<(), SelectionError> {
    validate_selection_component_props(props)
}

/// Validate switch state consistency
pub fn validate_switch_state(
    _state: SwitchState,
    props: &ComponentProps,
) -> Result<(), SelectionError> {
    validate_selection_component_props(props)
}

/// Validate chip state and configuration
pub fn validate_chip_state(
    _state: ChipState,
    _variant: ChipVariant,
    props: &ComponentProps,
) -> Result<(), SelectionError> {
    validate_props(props, &ValidationConfig::default())?;

    // Check chip-specific requirements
    if props.label.is_none() {
        return Err(SelectionError::InvalidLabel {
            reason: "Chips must have a label".to_string(),
        });
    }

    // Validate label length for chips (stricter than other components)
    if let Some(ref label) = props.label
        && label.len() > constants::chips::MAX_LABEL_LENGTH
    {
        return Err(SelectionError::LabelTooLong {
            len: label.len(),
            max: constants::chips::MAX_LABEL_LENGTH,
        });
    }

    Ok(())
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper function to create validation config for specific use cases
#[must_use]
pub const fn validation_config_for_chips() -> ValidationConfig {
    ValidationConfig {
        max_label_length: constants::chips::MAX_LABEL_LENGTH,
        allow_empty_label: false,
        custom_rules: Vec::new(),
    }
}

/// Helper function to create validation config for checkbox/radio/switch
#[must_use]
pub const fn validation_config_for_toggles() -> ValidationConfig {
    ValidationConfig {
        max_label_length: constants::ui::MAX_LABEL_LENGTH,
        allow_empty_label: true,
        custom_rules: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        let props = ComponentProps::new().with_label("Valid label");
        assert!(validate_props(&props, &ValidationConfig::default()).is_ok());

        let long_label = "x".repeat(201);
        let invalid_props = ComponentProps::new().with_label(long_label);
        assert!(validate_props(&invalid_props, &ValidationConfig::default()).is_err());
    }

    #[test]
    fn test_validation_helper_consistency() {
        let props = ComponentProps::new().with_label("Test");

        // All component validation should use the same base validation
        assert!(validate_checkbox_state(CheckboxState::Unchecked, &props).is_ok());
        assert!(validate_switch_state(SwitchState::Off, &props).is_ok());

        // Test with invalid props
        let invalid_props = ComponentProps::new().with_label("x".repeat(201));
        assert!(validate_checkbox_state(CheckboxState::Unchecked, &invalid_props).is_err());
        assert!(validate_switch_state(SwitchState::Off, &invalid_props).is_err());
    }
}
