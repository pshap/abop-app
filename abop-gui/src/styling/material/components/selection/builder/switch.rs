//! Switch Builder Implementation
//!
//! This module provides the `SwitchBuilder` for creating Material Design 3 switch
//! components with advanced validation, fluent API, and comprehensive error handling.
//!
//! ## Features
//! - State-based switch creation with validation
//! - Built-in validation with configurable rules
//! - Animation support with system preference awareness
//! - Fluent builder API with method chaining
//! - Performance optimizations with inline hints

use super::super::common::prelude::*;
use super::super::common::{validate_label, validate_props, validate_switch_state};
use super::super::defaults;
use super::components::Switch;
use super::patterns::*;
use super::validation::*;
use super::common_builder::{CommonBuilderState, CommonSelectionBuilder};
use crate::impl_common_selection_builder;

// ============================================================================
// Switch Builder Implementation
// ============================================================================

/// Builder for Material Design 3 Switch with validation and fluent API
#[derive(Debug, Clone)]
pub struct SwitchBuilder {
    state: SwitchState,
    common: CommonBuilderState,
}

// Implement the common builder functionality
impl_common_selection_builder!(SwitchBuilder, common);

impl SwitchBuilder {
    /// Create a new switch builder with the specified state
    #[must_use]
    pub fn new(state: SwitchState) -> Self {
        Self {
            state,
            common: CommonBuilderState::new(defaults::default_switch_animation_config()),
        }
    }

    /// Convenience method to create switch in on state
    #[must_use]
    #[inline]
    pub fn on() -> Self {
        Self::new(SwitchState::On)
    }

    /// Convenience method to create switch in off state
    #[must_use]
    #[inline]
    pub fn off() -> Self {
        Self::new(SwitchState::Off)
    }

    /// Create switch from boolean value
    #[must_use]
    #[inline]
    pub fn from_bool(enabled: bool) -> Self {
        Self::new(SwitchState::from_bool(enabled))
    }

    /// Toggle the switch state
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
    pub fn state_validated(mut self, state: SwitchState) -> Result<Self, SelectionError> {
        validate_switch_state(state, &self.common.props)?;
        self.state = state;
        Ok(self)
    }

    /// Clone with new state
    #[must_use]
    pub fn clone_with_state(&self, new_state: SwitchState) -> Self {
        let mut cloned = self.clone();
        cloned.state = new_state;
        cloned
    }

    /// Reset animation configuration to defaults
    #[must_use]
    pub fn reset_animation(mut self) -> Self {
        self.common.animation_config = defaults::default_switch_animation_config();
        self
    }

    /// Get the current switch state
    #[must_use]
    #[inline]
    pub const fn state(&self) -> SwitchState {
        self.state
    }

    /// Build the switch component with validation
    pub fn build(self) -> Result<Switch, SelectionError> {
        self.validate()?;
        Ok(self.build_unchecked())
    }

    /// Build the switch component without validation (faster)
    ///
    /// # Safety Warning
    ///
    /// This method bypasses all validation checks for performance reasons. It should only
    /// be used when you are confident that the builder state is valid. Invalid state may
    /// result in:
    /// - UI components that don't render correctly
    /// - Inconsistent behavior in the application
    /// - Potential panics in downstream code that assumes valid state
    ///
    /// # When to use
    ///
    /// Use this method only in scenarios where:
    /// - You have previously validated the builder using [`validate`](Self::validate)
    /// - You are constructing the builder programmatically with known-good values
    /// - Performance is critical and validation overhead needs to be avoided
    ///
    /// # Recommendation
    ///
    /// In most cases, prefer [`build`](Self::build) which includes validation and provides
    /// clearer error reporting.
    #[must_use]
    pub fn build_unchecked(self) -> Switch {
        Switch {
            state: self.state,
            props: self.common.props,
            error_state: self.common.error_state,
            animation_config: self.common.animation_config,
        }
    }

    /// Validate the switch configuration
    /// 
    /// This method performs comprehensive validation of the switch builder state,
    /// including state consistency, label validation, and common property validation.
    /// 
    /// # When to use
    /// 
    /// Use this method if you want to check the validity of the builder's configuration
    /// before attempting to build the `Switch` component. The [`build`](Self::build) method
    /// automatically calls `validate()` and returns an error if validation fails, so
    /// manual validation is only necessary if you need to check validity separately
    /// (e.g., for user feedback or conditional logic before building).
    /// 
    /// # Validation types performed
    /// 
    /// - **State consistency**: Ensures the switch state is compatible with current properties
    /// - **Label validation**: Checks label length and format according to validation rules  
    /// - **Common property validation**: Validates size, disabled state, and metadata consistency
    /// 
    /// # Errors
    /// 
    /// Returns `SelectionError` if:
    /// - The switch state is incompatible with the current properties
    /// - The label exceeds maximum length or violates validation rules
    /// - Common validation rules fail (disabled state conflicts, etc.)
    pub fn validate(&self) -> Result<(), SelectionError> {
        // Unified validation approach - collect all validation errors first
        let mut validation_errors = Vec::new();
        
        // Validate switch-specific state
        if let Err(e) = validate_switch_state(self.state, &self.common.props) {
            validation_errors.push(format!("Switch state: {e}"));
        }
        
        // Validate common properties
        if let Err(e) = self.validate_common() {
            validation_errors.push(format!("Common properties: {e}"));
        }
        
        // Return comprehensive error if any validation failed
        if validation_errors.is_empty() {
            Ok(())
        } else {
            Err(SelectionError::ValidationError(validation_errors.join("; ")))
        }
    }
}

// Enhanced trait implementations for backward compatibility
impl ComponentBuilder<SwitchState> for SwitchBuilder {
    type Component = Switch;
    type Error = SelectionError;

    fn build(self) -> Result<Self::Component, Self::Error> {
        self.validate()?;
        Ok(self.build_unchecked())
    }

    fn build_unchecked(self) -> Self::Component {
        Switch {
            state: self.state,
            props: self.common.props,
            error_state: self.common.error_state,
            animation_config: self.common.animation_config,
        }
    }

    fn validate(&self) -> Result<(), Self::Error> {
        validate_with_context(self, "SwitchBuilder", || {
            validate_switch_state(self.state, &self.common.props)
                .map_err(|e| SelectionError::InvalidState { 
                    details: format!("Switch validation failed: {e}") 
                })?;
            
            self.validate_common()
                .map_err(|e| SelectionError::ValidationError(
                    format!("Common validation failed: {e}")
                ))?;
            
            Ok(())
        })
    }
}

impl BuilderValidation for SwitchBuilder {
    fn validate_detailed(&self) -> ValidationResult {
        let context =
            ValidationContext::new("SwitchBuilder".to_string(), "validation".to_string());
        let mut result = ValidationResult::new(context);

        // Validate state
        if let Err(error) = validate_switch_state(self.state, &self.common.props) {
            result.add_error(error);
        }

        // Validate props
        if let Err(error) = validate_props(&self.common.props, &self.common.validation_config) {
            result.add_error(error);
        }

        result
    }

    fn validation_context(&self) -> ValidationContext {
        ValidationContext::new("SwitchBuilder".to_string(), "validation".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_builder_with_common() {
        let switch = SwitchBuilder::new(SwitchState::Off)
            .label("Test Switch")
            .disabled(false)
            .size(ComponentSize::Medium)
            .build_unchecked();

        assert_eq!(switch.state, SwitchState::Off);
        assert_eq!(switch.props.label, Some("Test Switch".to_string()));
        assert!(!switch.props.disabled);
        assert_eq!(switch.props.size, ComponentSize::Medium);
    }

    #[test]
    fn test_switch_convenience_methods() {
        let on_switch = SwitchBuilder::on().build_unchecked();
        assert_eq!(on_switch.state, SwitchState::On);

        let off_switch = SwitchBuilder::off().build_unchecked();
        assert_eq!(off_switch.state, SwitchState::Off);

        let from_bool = SwitchBuilder::from_bool(true).build_unchecked();
        assert_eq!(from_bool.state, SwitchState::On);
    }

    #[test]
    fn test_switch_toggle() {
        let toggled = SwitchBuilder::off().toggled().build_unchecked();
        assert_eq!(toggled.state, SwitchState::On);

        let toggled_back = SwitchBuilder::on().toggled().build_unchecked();
        assert_eq!(toggled_back.state, SwitchState::Off);
    }

    #[test]
    fn test_switch_validation() {
        let valid_builder = SwitchBuilder::off().label("Valid");
        assert!(valid_builder.validate().is_ok());

        let invalid_builder = SwitchBuilder::off().label("x".repeat(300));
        assert!(invalid_builder.validate().is_err());
    }

    #[test]
    fn test_switch_clone_with_state() {
        let original = SwitchBuilder::off().label("Test");
        let cloned = original.clone_with_state(SwitchState::On);
        
        let original_switch = original.build_unchecked();
        let cloned_switch = cloned.build_unchecked();
        
        assert_eq!(original_switch.state, SwitchState::Off);
        assert_eq!(cloned_switch.state, SwitchState::On);
        assert_eq!(original_switch.props.label, cloned_switch.props.label);
    }

    #[test]
    fn test_switch_animation_config() {
        let switch = SwitchBuilder::off()
            .animations_enabled(false)
            .build_unchecked();
            
        assert!(!switch.animation_config.enabled);
    }

    #[test]
    fn test_common_builder_trait() {
        let builder = SwitchBuilder::off()
            .with_metadata("color", "primary")
            .error(true);

        assert_eq!(builder.props().get_metadata("color"), Some("primary"));
        assert!(builder.has_error());
    }
}