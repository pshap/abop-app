//! Radio Builder Implementation
//!
//! This module provides the `RadioBuilder` for creating Material Design 3 radio button
//! components with advanced validation, fluent API, and comprehensive error handling.
//!
//! ## Features
//! - Generic value type support for type-safe radio groups
//! - Built-in validation with configurable rules
//! - Animation support with system preference awareness
//! - Fluent builder API with method chaining
//! - Performance optimizations with inline hints

use super::super::common::*;
use super::super::defaults;
use super::components::Radio;
use super::patterns::*;
use super::validation::*;

// ============================================================================
// Radio Builder Implementation
// ============================================================================

/// Builder for Material Design 3 Radio Button with validation and fluent API
#[derive(Debug, Clone)]
pub struct RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    value: T,
    props: ComponentProps,
    error_state: bool,
    validation_config: ValidationConfig,
    animation_config: AnimationConfig,
}

impl<T> RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    /// Create a new radio builder with the specified value
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            value,
            props: ComponentProps::new(),
            error_state: false,
            validation_config: defaults::default_validation_config(),
            animation_config: defaults::default_radio_animation_config(),
        }
    }

    /// Set the radio button label
    #[must_use]
    #[inline]
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.props.label = Some(label.into());
        self
    }

    /// Set disabled state
    #[must_use]
    #[inline]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    /// Set component size
    #[must_use]
    #[inline]
    pub const fn size(mut self, size: ComponentSize) -> Self {
        self.props.size = size;
        self
    }

    /// Set error state for validation feedback
    #[must_use]
    #[inline]
    pub const fn error(mut self, error: bool) -> Self {
        self.error_state = error;
        self
    }

    /// Set validation configuration
    #[must_use]
    #[inline]
    pub fn validation(mut self, config: ValidationConfig) -> Self {
        self.validation_config = config;
        self
    }

    /// Set animation configuration
    #[must_use]
    #[inline]
    pub const fn animation(mut self, config: AnimationConfig) -> Self {
        self.animation_config = config;
        self
    }

    /// Check if error state is enabled
    #[must_use]
    #[inline]
    pub const fn has_error(&self) -> bool {
        self.error_state
    }

    /// Get the radio value
    #[must_use]
    #[inline]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Get the component properties
    #[must_use]
    #[inline]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    // ========================================================================
    // Phase 2: Advanced Radio Builder Methods
    // ========================================================================

    /// Set label with validation
    ///
    /// Validates the label according to the current validation configuration
    /// before applying it to the radio button.
    pub fn label_validated<S: Into<String>>(mut self, label: S) -> Result<Self, SelectionError> {
        let label_str: String = label.into();
        validate_label(&label_str, &self.validation_config)?;
        self.props.label = Some(label_str);
        Ok(self)
    }

    /// Set value with validation
    pub fn value_validated(mut self, value: T) -> Result<Self, SelectionError> {
        // No specific validation needed for radio values by default
        self.value = value;
        Ok(self)
    }

    /// Apply configuration based on system preferences
    #[must_use]
    pub fn with_system_preferences(mut self) -> Self {
        if system_has_reduced_motion() {
            self.animation_config.enabled = false;
        }
        self
    }

    /// Clone with value modification
    #[must_use]
    pub fn clone_with_value(&self, new_value: T) -> Self {
        let mut cloned = self.clone();
        cloned.value = new_value;
        cloned
    }

    /// Build with detailed validation result
    pub fn build_with_detailed_validation(self) -> Result<Radio<T>, ValidationResult> {
        let result = self.validate_detailed();

        if result.is_valid() {
            Ok(self.build_unchecked())
        } else {
            Err(result)
        }
    }

    /// Create a radio optimized for performance (minimal validation)
    #[must_use]
    pub fn build_fast(self) -> Radio<T> {
        self.build_unchecked()
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl<T> ComponentBuilder<T> for RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    type Component = Radio<T>;
    type Error = SelectionError;

    fn build(self) -> Result<Self::Component, Self::Error> {
        self.validate()?;
        Ok(self.build_unchecked())
    }

    fn build_unchecked(self) -> Self::Component {
        Radio {
            value: self.value,
            props: self.props,
            error_state: self.error_state,
            animation_config: self.animation_config,
        }
    }

    fn validate(&self) -> Result<(), Self::Error> {
        validate_with_context(self, "RadioBuilder", || {
            validate_props(&self.props, &self.validation_config)?;
            Ok(())
        })
    }
}

// Phase 2: Enhanced RadioBuilder Trait Implementations
impl<T> BuilderValidation for RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    fn validate_detailed(&self) -> ValidationResult {
        let context = ValidationContext::new("RadioBuilder".to_string(), "validation".to_string());
        let mut result = ValidationResult::new(context);

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
        ValidationContext::new("RadioBuilder".to_string(), "validation".to_string())
    }
}

impl<T> AdvancedConditionalBuilder<T> for RadioBuilder<T> where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy
{
}

impl<T> StatefulBuilder<T> for RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    fn validate_state_transition(&self, new_value: T) -> Result<(), SelectionError> {
        // All value changes are valid for radio buttons
        let _ = new_value; // Suppress unused variable warning
        Ok(())
    }

    fn apply_state_validated(mut self, value: T) -> Result<Self, SelectionError> {
        self.validate_state_transition(value.clone())?;
        self.value = value;
        Ok(self)
    }
}

impl<T> ConditionalBuilder<T> for RadioBuilder<T> where T: Clone + PartialEq + Eq + std::hash::Hash + Copy {}
impl<T> BatchBuilder<T> for RadioBuilder<T> where T: Clone + PartialEq + Eq + std::hash::Hash + Copy {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestOption {
        A,
        B,
        C,
    }

    #[test]
    fn test_radio_builder_creation() {
        let builder = RadioBuilder::new(TestOption::A);
        assert_eq!(builder.value(), &TestOption::A);
        assert!(!builder.has_error());
        assert!(builder.props().label.is_none());
    }

    #[test]
    fn test_radio_builder_chaining() {
        let builder = RadioBuilder::new(TestOption::B)
            .label("Option B")
            .size(ComponentSize::Large)
            .disabled(false)
            .error(true);

        assert_eq!(builder.value(), &TestOption::B);
        assert_eq!(builder.props().label, Some("Option B".to_string()));
        assert_eq!(builder.props().size, ComponentSize::Large);
        assert!(!builder.props().disabled);
        assert!(builder.has_error());
    }

    #[test]
    fn test_radio_builder_build() {
        let radio = RadioBuilder::new(TestOption::C)
            .label("Option C")
            .build()
            .expect("Should build valid radio");

        assert_eq!(radio.value, TestOption::C);
        assert_eq!(radio.props.label, Some("Option C".to_string()));
        assert!(!radio.error_state);
    }

    #[test]
    fn test_radio_builder_validation() {
        let builder = RadioBuilder::new(TestOption::A).label("Valid Label");

        let result = builder.validate_detailed();
        assert!(result.is_valid());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_radio_builder_clone_with_value() {
        let original = RadioBuilder::new(TestOption::A).label("Original");

        let cloned = original.clone_with_value(TestOption::B);

        assert_eq!(original.value(), &TestOption::A);
        assert_eq!(cloned.value(), &TestOption::B);
        assert_eq!(original.props().label, cloned.props().label);
    }

    #[test]
    fn test_radio_builder_system_preferences() {
        let builder = RadioBuilder::new(TestOption::A).with_system_preferences();

        // System preference behavior will depend on actual system state
        // This test mainly ensures the method doesn't panic
        assert_eq!(builder.value(), &TestOption::A);
    }

    #[test]
    fn test_radio_builder_performance_build() {
        let radio = RadioBuilder::new(TestOption::B)
            .label("Fast Build")
            .build_fast();

        assert_eq!(radio.value, TestOption::B);
        assert_eq!(radio.props.label, Some("Fast Build".to_string()));
    }
}
