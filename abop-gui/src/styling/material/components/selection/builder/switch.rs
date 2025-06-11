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

use super::super::common::*;
use super::super::defaults;
use super::components::Switch;
use super::patterns::*;
use super::validation::*;

// ============================================================================
// Switch Builder Implementation
// ============================================================================

/// Builder for Material Design 3 Switch with validation and fluent API
#[derive(Debug, Clone)]
pub struct SwitchBuilder {
    state: SwitchState,
    props: ComponentProps,
    error_state: bool,
    validation_config: ValidationConfig,
    animation_config: AnimationConfig,
}

impl SwitchBuilder {
    /// Create a new switch builder with the specified state
    #[must_use]
    pub fn new(state: SwitchState) -> Self {
        Self {
            state,
            props: ComponentProps::new(),
            error_state: false,
            validation_config: defaults::default_validation_config(),
            animation_config: defaults::default_switch_animation_config(),
        }
    }

    /// Set the switch label
    #[must_use]
    #[inline]
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.props.label = Some(label.into());
        self
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

    /// Get the current state
    #[must_use]
    #[inline]
    pub const fn state(&self) -> SwitchState {
        self.state
    }

    /// Get the component properties
    #[must_use]
    #[inline]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    // ========================================================================
    // Phase 2: Advanced Switch Builder Methods
    // ========================================================================

    /// Set label with validation
    ///
    /// Validates the label according to the current validation configuration
    /// before applying it to the switch component.
    #[must_use]
    pub fn label_validated<S: Into<String>>(mut self, label: S) -> Result<Self, SelectionError> {
        let label_str: String = label.into();
        validate_label(&label_str, &self.validation_config)?;
        self.props.label = Some(label_str);
        Ok(self)
    }

    /// Set state with validation
    #[must_use]
    pub fn state_validated(mut self, state: SwitchState) -> Result<Self, SelectionError> {
        validate_switch_state(state, &self.props)?;
        self.state = state;
        Ok(self)
    }

    /// Toggle the state
    #[must_use]
    pub fn toggled(mut self) -> Self {
        self.state = self.state.toggle();
        self
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
    pub fn clone_with_state(&self, new_state: SwitchState) -> Self {
        let mut cloned = self.clone();
        cloned.state = new_state;
        cloned
    }

    /// Build with detailed validation result
    pub fn build_with_detailed_validation(self) -> Result<Switch, ValidationResult> {
        let result = self.validate_detailed();

        if result.is_valid() {
            Ok(self.build_unchecked())
        } else {
            Err(result)
        }
    }

    /// Create a switch optimized for performance (minimal validation)
    #[must_use]
    pub fn build_fast(self) -> Switch {
        self.build_unchecked()
    }
}

// ============================================================================
// Common Builder Methods
// ============================================================================

// Apply common builder methods to SwitchBuilder
impl_common_builder_methods!(SwitchBuilder);

// ============================================================================
// Trait Implementations
// ============================================================================

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
            props: self.props,
            error_state: self.error_state,
            animation_config: self.animation_config,
        }
    }

    fn validate(&self) -> Result<(), Self::Error> {
        validate_with_context(self, "SwitchBuilder", || {
            validate_switch_state(self.state, &self.props)?;
            validate_props(&self.props, &self.validation_config)?;
            Ok(())
        })
    }
}

// Phase 2: Enhanced SwitchBuilder Trait Implementations
impl BuilderValidation for SwitchBuilder {
    fn validate_detailed(&self) -> ValidationResult {
        let context = ValidationContext::new("SwitchBuilder".to_string(), "validation".to_string());
        let mut result = ValidationResult::new(context);

        // Validate state
        if let Err(error) = validate_switch_state(self.state, &self.props) {
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
        ValidationContext::new("SwitchBuilder".to_string(), "validation".to_string())
    }
}

impl AdvancedConditionalBuilder<SwitchState> for SwitchBuilder {}

impl StatefulBuilder<SwitchState> for SwitchBuilder {
    fn validate_state_transition(&self, new_state: SwitchState) -> Result<(), SelectionError> {
        // All state transitions are valid for switches
        validate_switch_state(new_state, &self.props)
    }

    fn apply_state_validated(mut self, state: SwitchState) -> Result<Self, SelectionError> {
        self.validate_state_transition(state)?;
        self.state = state;
        Ok(self)
    }
}

impl ConditionalBuilder<SwitchState> for SwitchBuilder {}
impl BatchBuilder<SwitchState> for SwitchBuilder {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_builder_creation() {
        let builder = SwitchBuilder::new(SwitchState::Off);
        assert_eq!(builder.state(), SwitchState::Off);
        assert!(builder.props().label.is_none());
    }

    #[test]
    fn test_switch_builder_convenience_methods() {
        let on_builder = SwitchBuilder::on();
        assert_eq!(on_builder.state(), SwitchState::On);

        let off_builder = SwitchBuilder::off();
        assert_eq!(off_builder.state(), SwitchState::Off);

        let bool_builder = SwitchBuilder::from_bool(true);
        assert_eq!(bool_builder.state(), SwitchState::On);

        let bool_builder_false = SwitchBuilder::from_bool(false);
        assert_eq!(bool_builder_false.state(), SwitchState::Off);
    }

    #[test]
    fn test_switch_builder_chaining() {
        let builder = SwitchBuilder::on()
            .label("Enable feature")
            .size(ComponentSize::Large)
            .disabled(false)
            .error(false);

        assert_eq!(builder.state(), SwitchState::On);
        assert_eq!(builder.props().label, Some("Enable feature".to_string()));
        assert_eq!(builder.props().size, ComponentSize::Large);
        assert!(!builder.props().disabled);
    }

    #[test]
    fn test_switch_builder_build() {
        let switch = SwitchBuilder::off()
            .label("Toggle setting")
            .build()
            .expect("Should build valid switch");

        assert_eq!(switch.state, SwitchState::Off);
        assert_eq!(switch.props.label, Some("Toggle setting".to_string()));
        assert!(!switch.error_state);
    }

    #[test]
    fn test_switch_builder_toggle() {
        let builder = SwitchBuilder::off().toggled();
        assert_eq!(builder.state(), SwitchState::On);

        let builder = SwitchBuilder::on().toggled();
        assert_eq!(builder.state(), SwitchState::Off);
    }

    #[test]
    fn test_switch_builder_validation() {
        let builder = SwitchBuilder::on().label("Valid Switch");

        let result = builder.validate_detailed();
        assert!(result.is_valid());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_switch_builder_clone_with_state() {
        let original = SwitchBuilder::off().label("Original");

        let cloned = original.clone_with_state(SwitchState::On);

        assert_eq!(original.state(), SwitchState::Off);
        assert_eq!(cloned.state(), SwitchState::On);
        assert_eq!(original.props().label, cloned.props().label);
    }

    #[test]
    fn test_switch_builder_system_preferences() {
        let builder = SwitchBuilder::on().with_system_preferences();

        // System preference behavior will depend on actual system state
        // This test mainly ensures the method doesn't panic
        assert_eq!(builder.state(), SwitchState::On);
    }

    #[test]
    fn test_switch_builder_performance_build() {
        let switch = SwitchBuilder::on().label("Fast Build").build_fast();

        assert_eq!(switch.state, SwitchState::On);
        assert_eq!(switch.props.label, Some("Fast Build".to_string()));
    }

    #[test]
    fn test_switch_builder_validation_methods() {
        let builder = SwitchBuilder::off();

        // Test state validation
        let validated_builder = builder
            .state_validated(SwitchState::On)
            .expect("Should validate state");
        assert_eq!(validated_builder.state(), SwitchState::On);

        // Test label validation
        let labeled_builder = SwitchBuilder::off()
            .label_validated("Valid Label")
            .expect("Should validate label");
        assert_eq!(
            labeled_builder.props().label,
            Some("Valid Label".to_string())
        );
    }
}
