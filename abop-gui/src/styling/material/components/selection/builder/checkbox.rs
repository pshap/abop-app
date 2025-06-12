//! Checkbox builder implementation with Material Design 3 support
//!
//! This module provides a sophisticated checkbox builder with:
//! - State-based design using CheckboxState enum
//! - Built-in validation and error handling
//! - Animation support for smooth transitions
//! - Performance optimizations and advanced patterns

use super::super::common::prelude::*;
use super::super::common::{
    system_has_reduced_motion, validate_checkbox_state, validate_label, validate_props,
};
use super::patterns::*;
use super::validation::*;

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
    props: ComponentProps,
    error_state: bool,
    validation_config: ValidationConfig,
    animation_config: AnimationConfig,
}

impl CheckboxBuilder {
    /// Create a new checkbox builder with the specified state
    #[must_use]
    pub fn new(state: CheckboxState) -> Self {
        Self {
            state,
            props: ComponentProps::new(),
            error_state: false,
            validation_config: super::super::defaults::default_validation_config(),
            animation_config: defaults::default_checkbox_animation_config(),
        }
    }

    /// Set the checkbox label
    #[must_use]
    #[inline]
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.props.label = Some(label.into());
        self
    }

    /// Convenience method to create checked checkbox
    #[must_use]
    #[inline]
    pub fn checked() -> Self {
        Self::new(CheckboxState::Checked)
    }

    /// Convenience method to create unchecked checkbox
    #[must_use]
    #[inline]
    pub fn unchecked() -> Self {
        Self::new(CheckboxState::Unchecked)
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

    /// Get the current state
    #[must_use]
    #[inline]
    pub const fn state(&self) -> CheckboxState {
        self.state
    }

    /// Get the component properties
    #[must_use]
    #[inline]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    // ========================================================================
    // Phase 2: Advanced Builder Methods
    // ========================================================================

    /// Set label with validation
    pub fn label_validated<S: Into<String>>(mut self, label: S) -> Result<Self, SelectionError> {
        let label_str = label.into();
        validate_label(label_str.as_str(), &self.validation_config)?;
        self.props.label = Some(label_str);
        Ok(self)
    }

    /// Set state with validation
    pub fn state_validated(mut self, state: CheckboxState) -> Result<Self, SelectionError> {
        validate_checkbox_state(state, &self.props)?;
        self.state = state;
        Ok(self)
    }

    /// Toggle the state
    #[must_use]
    pub fn toggled(mut self) -> Self {
        self.state = self.state.toggle();
        self
    }

    /// Apply a custom validation rule
    pub fn with_custom_validation<F>(self, validator: F) -> Result<Self, SelectionError>
    where
        F: FnOnce(&Self) -> Result<(), SelectionError>,
    {
        validator(&self)?;
        Ok(self)
    }

    /// Build with detailed validation result
    pub fn build_with_detailed_validation(
        self,
    ) -> Result<super::components::Checkbox, ValidationResult> {
        let _context = ValidationContext::new("CheckboxBuilder".to_string(), "build".to_string());
        let result = self.validate_detailed();

        if result.is_valid() {
            Ok(self.build_unchecked())
        } else {
            Err(result)
        }
    }

    /// Create a checkbox optimized for performance (minimal validation)
    #[must_use]
    pub fn build_fast(self) -> super::components::Checkbox {
        self.build_unchecked()
    }

    /// Apply configuration based on system preferences
    #[must_use]
    pub fn with_system_preferences(mut self) -> Self {
        if system_has_reduced_motion() {
            self.animation_config.enabled = false;
        }
        self
    }

    /// Clone with state modification
    #[must_use]
    pub fn clone_with_state(&self, new_state: CheckboxState) -> Self {
        let mut cloned = self.clone();
        cloned.state = new_state;
        cloned
    }

    /// Reset animation configuration to defaults
    #[must_use]
    pub fn reset_animation(mut self) -> Self {
        self.animation_config = defaults::default_checkbox_animation_config();
        self
    }
}

// Apply common builder methods to CheckboxBuilder
impl_common_builder_methods!(CheckboxBuilder);

impl ComponentBuilder<CheckboxState> for CheckboxBuilder {
    type Component = super::components::Checkbox;
    type Error = SelectionError;

    fn build(self) -> Result<Self::Component, Self::Error> {
        self.validate()?;
        Ok(self.build_unchecked())
    }

    fn build_unchecked(self) -> Self::Component {
        super::components::Checkbox {
            state: self.state,
            props: self.props,
            error_state: self.error_state,
            animation_config: self.animation_config,
        }
    }

    fn validate(&self) -> Result<(), Self::Error> {
        validate_with_context(self, "CheckboxBuilder", || {
            validate_checkbox_state(self.state, &self.props)?;
            validate_props(&self.props, &self.validation_config)?;
            Ok(())
        })
    }
}

// Phase 2: Enhanced Trait Implementations
impl BuilderValidation for CheckboxBuilder {
    fn validate_detailed(&self) -> ValidationResult {
        let context =
            ValidationContext::new("CheckboxBuilder".to_string(), "validation".to_string());
        let mut result = ValidationResult::new(context);

        // Validate state
        if let Err(error) = validate_checkbox_state(self.state, &self.props) {
            result.add_error(error);
        }

        // Validate props
        if let Err(error) = validate_props(&self.props, &self.validation_config) {
            result.add_error(error);
        }

        // Add warnings for potential issues
        if let Some(ref label) = self.props.label
            && label.len() > 100
        {
            result.add_warning("Label is very long, consider shortening for better UX");
        }

        if self.animation_config.enabled && system_has_reduced_motion() {
            result.add_warning("Animations enabled but system has reduced motion preference");
        }

        result
    }

    fn validation_context(&self) -> ValidationContext {
        ValidationContext::new("CheckboxBuilder".to_string(), "validation".to_string())
    }
}

impl AdvancedConditionalBuilder<CheckboxState> for CheckboxBuilder {}

impl StatefulBuilder<CheckboxState> for CheckboxBuilder {
    fn validate_state_transition(&self, new_state: CheckboxState) -> Result<(), SelectionError> {
        // All state transitions are valid for checkboxes
        validate_checkbox_state(new_state, &self.props)
    }

    fn apply_state_validated(mut self, state: CheckboxState) -> Result<Self, SelectionError> {
        self.validate_state_transition(state)?;
        self.state = state;
        Ok(self)
    }
}

impl ConditionalBuilder<CheckboxState> for CheckboxBuilder {}
impl BatchBuilder<CheckboxState> for CheckboxBuilder {}
