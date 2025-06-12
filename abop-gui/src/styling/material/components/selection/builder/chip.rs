//! Chip Builder Implementation
//!
//! This module provides the `ChipBuilder` for creating Material Design 3 chip
//! components with advanced validation, fluent API, and comprehensive error handling.
//!
//! ## Features
//! - Support for all chip variants (Filter, Assist, Input, Suggestion)
//! - Advanced UI features (leading/trailing icons, badges, deletion)
//! - Built-in validation with configurable rules
//! - Animation support with system preference awareness
//! - Fluent builder API with method chaining
//! - Performance optimizations with inline hints

use super::super::common::prelude::*;
use super::super::common::{
    validate_label, validate_chip_state, validate_props, system_has_reduced_motion,
    validation_config_for_chips
};
use super::super::defaults;
use super::components::Chip;
use super::patterns::*;
use super::validation::*;

// ============================================================================
// Chip Builder Implementation
// ============================================================================

/// Builder for Material Design 3 Chip with validation and fluent API
#[derive(Debug, Clone)]
pub struct ChipBuilder {
    label: String,
    state: ChipState,
    variant: ChipVariant,
    props: ComponentProps,
    error_state: bool,
    validation_config: ValidationConfig,
    animation_config: AnimationConfig,
}

impl ChipBuilder {
    /// Create a new chip builder with the specified label and variant
    #[must_use]
    pub fn new<S: Into<String>>(label: S, variant: ChipVariant) -> Self {
        let label = label.into();
        Self {
            label: label.clone(),
            state: ChipState::Unselected,
            variant,
            props: ComponentProps::new().with_label(label),
            error_state: false,
            validation_config: validation_config_for_chips(),
            animation_config: defaults::default_chip_animation_config(),
        }
    }

    /// Set the chip state
    #[must_use]
    pub const fn with_state(mut self, state: ChipState) -> Self {
        self.state = state;
        self
    }

    /// Set chip as selected
    #[must_use]
    pub const fn selected(mut self, selected: bool) -> Self {
        self.state = if selected {
            ChipState::Selected
        } else {
            ChipState::Unselected
        };
        self
    }

    // ========================================================================
    // Enhanced UI Builder Methods
    // ========================================================================

    /// Add a leading icon to the chip
    ///
    /// Leading icons appear before the chip label and are typically used
    /// to provide visual context or categorization.
    ///
    /// # Arguments
    /// * `icon_name` - Font Awesome icon name (e.g., "filter", "user")
    #[must_use]
    pub fn with_leading_icon<S: Into<String>>(mut self, icon_name: S) -> Self {
        // Store icon information in props for later use during view rendering
        self.props = self.props.with_metadata("leading_icon", icon_name.into());
        self
    }

    /// Add a trailing icon to the chip
    ///
    /// Trailing icons appear after the chip label and are typically used
    /// for actions like deletion or expansion.
    ///
    /// # Arguments
    /// * `icon_name` - Font Awesome icon name (e.g., "times", "chevron-down")
    #[must_use]
    pub fn with_trailing_icon<S: Into<String>>(mut self, icon_name: S) -> Self {
        // Store icon information in props for later use during view rendering
        self.props = self.props.with_metadata("trailing_icon", icon_name.into());
        self
    }

    /// Add a badge with count to the chip
    ///
    /// Badges display numeric counts and are useful for showing quantities
    /// or notification counts.
    ///
    /// # Arguments
    /// * `count` - The number to display in the badge
    #[must_use]
    pub fn with_badge(mut self, count: u32) -> Self {
        // Store badge information in props for later use during view rendering
        self.props = self.props.with_metadata("badge_count", count.to_string());
        self
    }

    /// Make the chip deletable with a trailing delete icon
    ///
    /// This is a convenience method that adds a trailing "times" icon
    /// commonly used for deletion functionality.
    #[must_use]
    pub fn deletable(self) -> Self {
        self.with_trailing_icon("times")
    }

    /// Convenience method to create filter chip
    #[must_use]
    pub fn filter<S: Into<String>>(label: S) -> Self {
        Self::new(label, ChipVariant::Filter)
    }

    /// Convenience method to create assist chip
    #[must_use]
    pub fn assist<S: Into<String>>(label: S) -> Self {
        Self::new(label, ChipVariant::Assist)
    }

    /// Convenience method to create input chip
    #[must_use]
    pub fn input<S: Into<String>>(label: S) -> Self {
        Self::new(label, ChipVariant::Input)
    }

    /// Convenience method to create suggestion chip
    #[must_use]
    pub fn suggestion<S: Into<String>>(label: S) -> Self {
        Self::new(label, ChipVariant::Suggestion)
    }

    /// Get the chip label
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the current state
    #[must_use]
    pub const fn state(&self) -> ChipState {
        self.state
    }

    /// Get the chip variant
    #[must_use]
    pub const fn variant(&self) -> ChipVariant {
        self.variant
    }

    /// Get the component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    // ========================================================================
    // Phase 2: Advanced Chip Builder Methods
    // ========================================================================

    /// Set label with validation
    ///
    /// Validates the label according to the current validation configuration
    /// before applying it to the chip component.
    pub fn label_validated<S: Into<String>>(mut self, label: S) -> Result<Self, SelectionError> {
        let label_str: String = label.into();
        validate_label(&label_str, &self.validation_config)?;
        self.label = label_str.clone();
        self.props.label = Some(label_str);
        Ok(self)
    }

    /// Set state with validation
    pub fn state_validated(mut self, state: ChipState) -> Result<Self, SelectionError> {
        validate_chip_state(state, self.variant, &self.props)?;
        self.state = state;
        Ok(self)
    }

    /// Toggle the selection state
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
    pub fn clone_with_state(&self, new_state: ChipState) -> Self {
        let mut cloned = self.clone();
        cloned.state = new_state;
        cloned
    }

    /// Clone with variant modification
    #[must_use]
    pub fn clone_with_variant(&self, new_variant: ChipVariant) -> Self {
        let mut cloned = self.clone();
        cloned.variant = new_variant;
        cloned
    }

    /// Build with detailed validation result
    pub fn build_with_detailed_validation(self) -> Result<Chip, ValidationResult> {
        let result = self.validate_detailed();

        if result.is_valid() {
            Ok(self.build_unchecked())
        } else {
            Err(result)
        }
    }

    /// Create a chip optimized for performance (minimal validation)
    #[must_use]
    pub fn build_fast(self) -> Chip {
        self.build_unchecked()
    }

    /// Add custom metadata for advanced use cases
    #[must_use]
    pub fn with_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.props = self.props.with_metadata(key, value);
        self
    }
}

// ============================================================================
// Common Builder Methods
// ============================================================================

// Apply common builder methods to ChipBuilder
impl_common_builder_methods!(ChipBuilder);

// ============================================================================
// Trait Implementations
// ============================================================================

impl ComponentBuilder<ChipState> for ChipBuilder {
    type Component = Chip;
    type Error = SelectionError;

    fn build(self) -> Result<Self::Component, Self::Error> {
        self.validate()?;
        Ok(self.build_unchecked())
    }

    fn build_unchecked(self) -> Self::Component {
        Chip {
            label: self.label,
            state: self.state,
            variant: self.variant,
            props: self.props,
            error_state: self.error_state,
            animation_config: self.animation_config,
        }
    }

    fn validate(&self) -> Result<(), Self::Error> {
        validate_with_context(self, "ChipBuilder", || {
            validate_chip_state(self.state, self.variant, &self.props)?;
            validate_props(&self.props, &self.validation_config)?;
            Ok(())
        })
    }
}

// Phase 2: Enhanced ChipBuilder Trait Implementations
impl BuilderValidation for ChipBuilder {
    fn validate_detailed(&self) -> ValidationResult {
        let context = ValidationContext::new("ChipBuilder".to_string(), "validation".to_string());
        let mut result = ValidationResult::new(context);

        // Validate chip-specific state
        if let Err(error) = validate_chip_state(self.state, self.variant, &self.props) {
            result.add_error(error);
        }

        // Validate props
        if let Err(error) = validate_props(&self.props, &self.validation_config) {
            result.add_error(error);
        }

        // Add warnings for potential issues
        if self.label.len() > 50 {
            result.add_warning("Chip label is very long, consider shortening for better UX");
        }

        if self.animation_config.enabled && system_has_reduced_motion() {
            result.add_warning("Animations enabled but system has reduced motion preference");
        }

        // Check for empty label (more strict for chips)
        if self.label.is_empty() {
            result.add_error(SelectionError::EmptyLabel);
        }

        result
    }

    fn validation_context(&self) -> ValidationContext {
        ValidationContext::new("ChipBuilder".to_string(), "validation".to_string())
    }
}

impl AdvancedConditionalBuilder<ChipState> for ChipBuilder {}

impl StatefulBuilder<ChipState> for ChipBuilder {
    fn validate_state_transition(&self, new_state: ChipState) -> Result<(), SelectionError> {
        validate_chip_state(new_state, self.variant, &self.props)
    }

    fn apply_state_validated(mut self, state: ChipState) -> Result<Self, SelectionError> {
        self.validate_state_transition(state)?;
        self.state = state;
        Ok(self)
    }
}

impl ConditionalBuilder<ChipState> for ChipBuilder {}
impl BatchBuilder<ChipState> for ChipBuilder {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip_builder_creation() {
        let builder = ChipBuilder::new("Test", ChipVariant::Filter);
        assert_eq!(builder.label(), "Test");
        assert_eq!(builder.variant(), ChipVariant::Filter);
        assert_eq!(builder.state(), ChipState::Unselected);
    }

    #[test]
    fn test_chip_builder_convenience_methods() {
        let filter_chip = ChipBuilder::filter("Filter");
        assert_eq!(filter_chip.variant(), ChipVariant::Filter);

        let assist_chip = ChipBuilder::assist("Assist");
        assert_eq!(assist_chip.variant(), ChipVariant::Assist);

        let input_chip = ChipBuilder::input("Input");
        assert_eq!(input_chip.variant(), ChipVariant::Input);

        let suggestion_chip = ChipBuilder::suggestion("Suggestion");
        assert_eq!(suggestion_chip.variant(), ChipVariant::Suggestion);
    }

    #[test]
    fn test_chip_builder_chaining() {
        let builder = ChipBuilder::filter("Category")
            .selected(true)
            .size(ComponentSize::Large)
            .disabled(false)
            .with_leading_icon("filter")
            .with_trailing_icon("chevron-down")
            .with_badge(5);

        assert_eq!(builder.label(), "Category");
        assert_eq!(builder.state(), ChipState::Selected);
        assert_eq!(builder.variant(), ChipVariant::Filter);
        assert_eq!(builder.props().size, ComponentSize::Large);
        assert!(!builder.props().disabled);
    }

    #[test]
    fn test_chip_builder_build() {
        let chip = ChipBuilder::filter("Test Chip")
            .selected(true)
            .build()
            .expect("Should build valid chip");

        assert_eq!(chip.label, "Test Chip");
        assert_eq!(chip.state, ChipState::Selected);
        assert_eq!(chip.variant, ChipVariant::Filter);
        assert!(!chip.error_state);
    }

    #[test]
    fn test_chip_builder_toggle() {
        let builder = ChipBuilder::filter("Toggle").toggled();
        assert_eq!(builder.state(), ChipState::Selected);

        let builder = ChipBuilder::filter("Toggle").selected(true).toggled();
        assert_eq!(builder.state(), ChipState::Unselected);
    }

    #[test]
    fn test_chip_builder_validation() {
        let builder = ChipBuilder::filter("Valid Chip");

        let result = builder.validate_detailed();
        assert!(result.is_valid());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_chip_builder_empty_label_validation() {
        let builder = ChipBuilder::filter("");

        let result = builder.validate_detailed();
        assert!(!result.is_valid());
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_chip_builder_clone_methods() {
        let original = ChipBuilder::filter("Original").selected(true);

        let cloned_state = original.clone_with_state(ChipState::Unselected);
        assert_eq!(original.state(), ChipState::Selected);
        assert_eq!(cloned_state.state(), ChipState::Unselected);

        let cloned_variant = original.clone_with_variant(ChipVariant::Assist);
        assert_eq!(original.variant(), ChipVariant::Filter);
        assert_eq!(cloned_variant.variant(), ChipVariant::Assist);
    }

    #[test]
    fn test_chip_builder_deletable() {
        let chip = ChipBuilder::input("Deletable")
            .deletable()
            .build()
            .expect("Should build deletable chip");

        // Check if trailing icon metadata is set
        assert!(chip.props.metadata.contains_key("trailing_icon"));
        assert_eq!(
            chip.props.metadata.get("trailing_icon"),
            Some(&"times".to_string())
        );
    }

    #[test]
    fn test_chip_builder_with_metadata() {
        let chip = ChipBuilder::filter("Metadata Test")
            .with_metadata("custom_key", "custom_value")
            .build()
            .expect("Should build chip with metadata");

        assert_eq!(
            chip.props.metadata.get("custom_key"),
            Some(&"custom_value".to_string())
        );
    }

    #[test]
    fn test_chip_builder_system_preferences() {
        let builder = ChipBuilder::filter("System Prefs").with_system_preferences();

        // System preference behavior will depend on actual system state
        // This test mainly ensures the method doesn't panic
        assert_eq!(builder.label(), "System Prefs");
    }

    #[test]
    fn test_chip_builder_performance_build() {
        let chip = ChipBuilder::assist("Fast Build").build_fast();

        assert_eq!(chip.label, "Fast Build");
        assert_eq!(chip.variant, ChipVariant::Assist);
    }

    #[test]
    fn test_chip_builder_validation_methods() {
        let builder = ChipBuilder::filter("Test");

        // Test state validation
        let validated_builder = builder
            .state_validated(ChipState::Selected)
            .expect("Should validate state");
        assert_eq!(validated_builder.state(), ChipState::Selected);

        // Test label validation
        let labeled_builder = ChipBuilder::filter("")
            .label_validated("Valid Label")
            .expect("Should validate label");
        assert_eq!(labeled_builder.label(), "Valid Label");
    }
}
