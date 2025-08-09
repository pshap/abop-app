//! Checkbox builder implementation with Material Design 3 support
//!
//! This module provides a sophisticated checkbox builder with:
//! - State-based design using CheckboxState enum
//! - Built-in validation and error handling
//! - Animation support for smooth transitions
//! - Performance optimizations and advanced patterns

use super::super::common::prelude::*;
use super::super::common::{validate_checkbox_state, validate_label, validate_props};
use super::patterns::*;
use super::validation::*;
use super::common_builder::{CommonBuilderState, CommonSelectionBuilder};
use crate::impl_common_selection_builder;

/// Default constants for checkbox configuration
pub mod defaults {
    use super::*;
    use std::time::Duration;

    /// Default animation duration for checkbox components (milliseconds)
    pub const DEFAULT_CHECKBOX_ANIMATION_DURATION_MS: u64 = 200;

    /// Create default animation configuration for checkbox components
    #[must_use]
    pub fn default_checkbox_animation_config() -> AnimationConfig {
        AnimationConfig {
            duration: Duration::from_millis(DEFAULT_CHECKBOX_ANIMATION_DURATION_MS),
            enabled: true,
            respect_reduced_motion: true,
            easing: EasingCurve::Standard,
        }
    }
}

/// Builder for Material Design 3 Checkbox with validation and fluent API
#[derive(Debug, Clone)]
pub struct CheckboxBuilder {
    state: CheckboxState,
    common: CommonBuilderState,
}

// Implement the common builder functionality
impl_common_selection_builder!(CheckboxBuilder, common);

impl CheckboxBuilder {
    /// Create a new checkbox builder with the specified state
    #[must_use]
    pub fn new(state: CheckboxState) -> Self {
        Self {
            state,
            common: CommonBuilderState::new(defaults::default_checkbox_animation_config()),
        }
    }

    /// Convenience method to create unchecked checkbox
    #[must_use]
    #[inline]
    pub fn unchecked() -> Self {
        Self::new(CheckboxState::Unchecked)
    }

    /// Convenience method to create checked checkbox
    #[must_use]
    #[inline]
    pub fn checked() -> Self {
        Self::new(CheckboxState::Checked)
    }

    /// Convenience method to create indeterminate checkbox
    #[must_use]
    #[inline]
    pub fn indeterminate() -> Self {
        Self::new(CheckboxState::Indeterminate)
    }

    /// Create checkbox from boolean value
    #[must_use]
    #[inline]
    pub fn from_bool(checked: bool) -> Self {
        Self::new(CheckboxState::from_bool(checked))
    }

    /// Toggle the checkbox state
    #[must_use]
    #[inline]
    pub fn toggled(mut self) -> Self {
        self.state = self.state.toggle();
        self
    }

    /// Set label with validation
    pub fn label_validated<S: Into<String>>(mut self, label: S) -> Result<Self, SelectionError> {
        let label_str = label.into();
        validate_label(label_str.as_str(), &self.common.validation_config)?;
        self.common.props.label = Some(label_str);
        Ok(self)
    }

    /// Set state with validation
    pub fn state_validated(mut self, state: CheckboxState) -> Result<Self, SelectionError> {
        validate_checkbox_state(state, &self.common.props)?;
        self.state = state;
        Ok(self)
    }

    /// Clone with new state (for radio group patterns)
    #[must_use]
    pub fn clone_with_state(&self, new_state: CheckboxState) -> Self {
        let mut cloned = self.clone();
        cloned.state = new_state;
        cloned
    }

    /// Reset animation configuration to defaults
    #[must_use]
    pub fn reset_animation(mut self) -> Self {
        self.common.animation_config = defaults::default_checkbox_animation_config();
        self
    }

    /// Get the current checkbox state
    #[must_use]
    #[inline]
    pub const fn state(&self) -> CheckboxState {
        self.state
    }

    /// Build the checkbox component with validation
    pub fn build(self) -> Result<super::components::Checkbox, SelectionError> {
        self.validate()?;
        Ok(self.build_unchecked())
    }

    /// Build the checkbox component without validation (faster)
    #[must_use]
    pub fn build_unchecked(self) -> super::components::Checkbox {
        super::components::Checkbox {
            state: self.state,
            props: self.common.props,
            error_state: self.common.error_state,
            animation_config: self.common.animation_config,
        }
    }

    /// Validate the checkbox configuration
    pub fn validate(&self) -> Result<(), SelectionError> {
        validate_checkbox_state(self.state, &self.common.props)?;
        self.validate_common()?;
        Ok(())
    }
}

// Enhanced trait implementations for backward compatibility
impl ComponentBuilder<CheckboxState> for CheckboxBuilder {
    type Component = super::components::Checkbox;
    type Error = SelectionError;

    fn build(self) -> Result<Self::Component, Self::Error> {
        CheckboxBuilder::build(self)
    }

    fn build_unchecked(self) -> Self::Component {
        CheckboxBuilder::build_unchecked(self)
    }

    fn validate(&self) -> Result<(), Self::Error> {
        validate_with_context(self, "CheckboxBuilder", || {
            CheckboxBuilder::validate(self)
        })
    }
}

impl BuilderValidation for CheckboxBuilder {
    fn validate_detailed(&self) -> ValidationResult {
        let context =
            ValidationContext::new("CheckboxBuilder".to_string(), "validation".to_string());
        let mut result = ValidationResult::new(context);

        // Validate state
        if let Err(error) = validate_checkbox_state(self.state, &self.common.props) {
            result.add_error(error);
        }

        // Validate props
        if let Err(error) = validate_props(&self.common.props, &self.common.validation_config) {
            result.add_error(error);
        }

        result
    }

    fn validation_context(&self) -> ValidationContext {
        ValidationContext::new("CheckboxBuilder".to_string(), "validation".to_string())
    }
}

// The common methods are provided by the CommonSelectionBuilder trait
// which is automatically implemented via the impl_common_selection_builder! macro

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_builder_with_common() {
        let checkbox = CheckboxBuilder::new(CheckboxState::Unchecked)
            .label("Test Checkbox")
            .disabled(false)
            .size(ComponentSize::Medium)
            .build_unchecked();

        assert_eq!(checkbox.state, CheckboxState::Unchecked);
        assert_eq!(checkbox.props.label, Some("Test Checkbox".to_string()));
        assert!(!checkbox.props.disabled);
        assert_eq!(checkbox.props.size, ComponentSize::Medium);
    }

    #[test]
    fn test_checkbox_convenience_methods() {
        let unchecked = CheckboxBuilder::unchecked().build_unchecked();
        assert_eq!(unchecked.state, CheckboxState::Unchecked);

        let checked = CheckboxBuilder::checked().build_unchecked();
        assert_eq!(checked.state, CheckboxState::Checked);

        let indeterminate = CheckboxBuilder::indeterminate().build_unchecked();
        assert_eq!(indeterminate.state, CheckboxState::Indeterminate);

        let from_bool = CheckboxBuilder::from_bool(true).build_unchecked();
        assert_eq!(from_bool.state, CheckboxState::Checked);
    }

    #[test]
    fn test_checkbox_toggle() {
        let toggled = CheckboxBuilder::unchecked().toggled().build_unchecked();
        assert_eq!(toggled.state, CheckboxState::Checked);

        let toggled_back = CheckboxBuilder::checked().toggled().build_unchecked();
        assert_eq!(toggled_back.state, CheckboxState::Unchecked);
    }

    #[test]
    fn test_checkbox_validation() {
        let valid_builder = CheckboxBuilder::unchecked().label("Valid");
        assert!(valid_builder.validate().is_ok());

        let invalid_builder = CheckboxBuilder::unchecked().label("x".repeat(300));
        assert!(invalid_builder.validate().is_err());
    }

    #[test]
    fn test_checkbox_clone_with_state() {
        let original = CheckboxBuilder::unchecked().label("Test");
        let cloned = original.clone_with_state(CheckboxState::Checked);
        
        let original_checkbox = original.build_unchecked();
        let cloned_checkbox = cloned.build_unchecked();
        
        assert_eq!(original_checkbox.state, CheckboxState::Unchecked);
        assert_eq!(cloned_checkbox.state, CheckboxState::Checked);
        assert_eq!(original_checkbox.props.label, cloned_checkbox.props.label);
    }

    #[test]
    fn test_checkbox_animation_config() {
        let checkbox = CheckboxBuilder::unchecked()
            .animations_enabled(false)
            .build_unchecked();
            
        assert!(!checkbox.animation_config.enabled);
    }

    #[test]
    fn test_common_builder_trait() {
        let builder = CheckboxBuilder::unchecked()
            .with_metadata("icon", "check")
            .error(true);

        assert_eq!(builder.props().get_metadata("icon"), Some("check"));
        assert!(builder.has_error());
    }
}