//! Advanced builder pattern implementation for Material Design 3 selection components
//!
//! This module provides sophisticated builder patterns with built-in validation,
//! fluent APIs, and comprehensive error handling for all selection components.

use super::common::*;

// Import the constant from chip module
use super::chip::DEFAULT_ANIMATION_DURATION;

// ============================================================================
// Common Configuration Constants
// ============================================================================

/// Default constants for builder configuration
pub mod defaults {
    use super::*;
    use std::time::Duration;

    /// Default maximum label length for most components
    pub const DEFAULT_MAX_LABEL_LENGTH: usize = 200;
    
    /// Default animation duration for checkbox and switch components (milliseconds)
    pub const DEFAULT_CHECKBOX_ANIMATION_DURATION_MS: u64 = 200;
    
    /// Default animation duration for radio components (milliseconds) 
    pub const DEFAULT_RADIO_ANIMATION_DURATION_MS: u64 = 100;
    
    /// Default animation duration for switch components (milliseconds)
    pub const DEFAULT_SWITCH_ANIMATION_DURATION_MS: u64 = 200;

    /// Create default validation configuration for standard components
    #[must_use]
    pub fn default_validation_config() -> ValidationConfig {
        ValidationConfig {
            max_label_length: DEFAULT_MAX_LABEL_LENGTH,
            allow_empty_label: true,
            custom_rules: Vec::new(),
        }
    }

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

    /// Create default animation configuration for radio components
    #[must_use]
    pub fn default_radio_animation_config() -> AnimationConfig {
        AnimationConfig {
            duration: Duration::from_millis(DEFAULT_RADIO_ANIMATION_DURATION_MS),
            enabled: true,
            respect_reduced_motion: true,
            easing: EasingCurve::Standard,
        }
    }

    /// Create default animation configuration for switch components
    #[must_use]
    pub fn default_switch_animation_config() -> AnimationConfig {
        AnimationConfig {
            duration: Duration::from_millis(DEFAULT_SWITCH_ANIMATION_DURATION_MS),
            enabled: true,
            respect_reduced_motion: true,
            easing: EasingCurve::Standard,
        }
    }

    /// Create default animation configuration for chip components
    #[must_use]
    pub fn default_chip_animation_config() -> AnimationConfig {
        AnimationConfig {
            duration: DEFAULT_ANIMATION_DURATION,
            enabled: true,
            respect_reduced_motion: true,
            easing: EasingCurve::Standard,
        }
    }
}

// ============================================================================
// Common Builder Methods Macro
// ============================================================================

/// Macro to generate common builder methods that are shared across all builders
/// This eliminates code duplication for disabled, size, error, validation, and animation methods
macro_rules! impl_common_builder_methods {
    ($builder:ty) => {
        impl $builder {
            /// Set disabled state
            #[must_use]
            pub const fn disabled(mut self, disabled: bool) -> Self {
                self.props.disabled = disabled;
                self
            }

            /// Set component size
            #[must_use]
            pub const fn size(mut self, size: ComponentSize) -> Self {
                self.props.size = size;
                self
            }

            /// Set error state for validation feedback
            #[must_use]
            pub const fn error(mut self, error: bool) -> Self {
                self.error_state = error;
                self
            }

            /// Set validation configuration
            #[must_use]
            pub fn validation(mut self, config: ValidationConfig) -> Self {
                self.validation_config = config;
                self
            }

            /// Set animation configuration
            #[must_use]
            pub const fn animation(mut self, config: AnimationConfig) -> Self {
                self.animation_config = config;
                self
            }

            /// Check if error state is enabled
            #[must_use]
            pub const fn has_error(&self) -> bool {
                self.error_state
            }
        }
    };
}

// ============================================================================
// Enhanced Validation Helper
// ============================================================================

/// Enhanced validation with better error context
pub fn validate_with_context<T>(
    _builder: &T,
    component_type: &str,
    validate_fn: impl FnOnce() -> Result<(), SelectionError>,
) -> Result<(), SelectionError> {
    validate_fn().map_err(|e| match e {
        SelectionError::ValidationError(msg) => {
            SelectionError::ValidationError(format!("{component_type}: {msg}"))
        }
        SelectionError::InvalidLabel { reason } => {
            SelectionError::InvalidLabel {
                reason: format!("{component_type}: {reason}"),
            }
        }
        SelectionError::LabelTooLong { len, max } => {
            SelectionError::LabelTooLong { len, max }
        }
        SelectionError::EmptyLabel => {
            SelectionError::EmptyLabel
        }
        SelectionError::InvalidState { details } => {
            SelectionError::InvalidState {
                details: format!("{component_type}: {details}"),
            }
        }
        SelectionError::ConflictingStates { details } => {
            SelectionError::ConflictingStates {
                details: format!("{component_type}: {details}"),
            }
        }
        SelectionError::CustomRule { rule, message } => {
            SelectionError::CustomRule {
                rule,
                message: format!("{component_type}: {message}"),
            }
        }
    })
}

// ============================================================================
// Builder Trait System
// ============================================================================

/// Core builder trait for all selection components
pub trait ComponentBuilder<T> {
    /// The component type being built
    type Component;

    /// The error type for validation failures
    type Error;

    /// Build the component with validation
    fn build(self) -> Result<Self::Component, Self::Error>;

    /// Build the component without validation (for internal use)
    fn build_unchecked(self) -> Self::Component;

    /// Validate the current builder state
    fn validate(&self) -> Result<(), Self::Error>;
}

/// Trait for builders that support conditional configuration
pub trait ConditionalBuilder<T>: ComponentBuilder<T> {
    /// Apply configuration conditionally
    fn when(self, condition: bool, f: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        if condition { f(self) } else { self }
    }

    /// Apply configuration if Some value is provided
    fn when_some<U>(self, value: Option<U>, f: impl FnOnce(Self, U) -> Self) -> Self
    where
        Self: Sized,
    {
        if let Some(val) = value {
            f(self, val)
        } else {
            self
        }
    }
}

/// Trait for builders that support batch configuration
pub trait BatchBuilder<T>: ComponentBuilder<T> {
    /// Apply multiple configurations in sequence
    fn configure(self, configs: &[impl Fn(Self) -> Self]) -> Self
    where
        Self: Sized + Clone,
    {
        configs
            .iter()
            .fold(self, |builder, config| config(builder.clone()))
    }
}

// ============================================================================
// Checkbox Builder
// ============================================================================

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
            validation_config: defaults::default_validation_config(),
            animation_config: defaults::default_checkbox_animation_config(),
        }
    }

    /// Set the checkbox label
    #[must_use]
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.props.label = Some(label.into());
        self
    }

    /// Convenience method to create checked checkbox
    #[must_use]
    pub fn checked() -> Self {
        Self::new(CheckboxState::Checked)
    }

    /// Convenience method to create unchecked checkbox
    #[must_use]
    pub fn unchecked() -> Self {
        Self::new(CheckboxState::Unchecked)
    }

    /// Convenience method to create indeterminate checkbox
    #[must_use]
    pub fn indeterminate() -> Self {
        Self::new(CheckboxState::Indeterminate)
    }

    /// Create checkbox from boolean value
    #[must_use]
    pub fn from_bool(checked: bool) -> Self {
        Self::new(CheckboxState::from_bool(checked))
    }

    /// Get the current state
    #[must_use]
    pub const fn state(&self) -> CheckboxState {
        self.state
    }

    /// Get the component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }
}

// Apply common builder methods to CheckboxBuilder
impl_common_builder_methods!(CheckboxBuilder);

impl ComponentBuilder<CheckboxState> for CheckboxBuilder {
    type Component = Checkbox;
    type Error = SelectionError;

    fn build(self) -> Result<Self::Component, Self::Error> {
        self.validate()?;
        Ok(self.build_unchecked())
    }

    fn build_unchecked(self) -> Self::Component {
        Checkbox {
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

impl ConditionalBuilder<CheckboxState> for CheckboxBuilder {}
impl BatchBuilder<CheckboxState> for CheckboxBuilder {}

// ============================================================================
// Radio Builder
// ============================================================================

/// Builder for Material Design 3 Radio Button with validation and fluent API
#[derive(Debug, Clone)]
pub struct RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    value: T,
    props: ComponentProps,
    error_state: bool,
    validation_config: ValidationConfig,
    animation_config: AnimationConfig,
}

impl<T> RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
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
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.props.label = Some(label.into());
        self
    }

    /// Set disabled state
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    /// Set component size
    #[must_use]
    pub const fn size(mut self, size: ComponentSize) -> Self {
        self.props.size = size;
        self
    }

    /// Set error state for validation feedback
    #[must_use]
    pub const fn error(mut self, error: bool) -> Self {
        self.error_state = error;
        self
    }

    /// Set validation configuration
    #[must_use]
    pub fn validation(mut self, config: ValidationConfig) -> Self {
        self.validation_config = config;
        self
    }

    /// Set animation configuration
    #[must_use]
    pub const fn animation(mut self, config: AnimationConfig) -> Self {
        self.animation_config = config;
        self
    }

    /// Check if error state is enabled
    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.error_state
    }

    /// Get the radio value
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Get the component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }
}

// Apply common builder methods to RadioBuilder
// Note: Cannot use generic macro for RadioBuilder due to Rust macro limitations
// These methods are manually implemented in the RadioBuilder impl block above

impl<T> ComponentBuilder<T> for RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
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

impl<T> ConditionalBuilder<T> for RadioBuilder<T> where T: Clone + PartialEq + Eq + std::hash::Hash {}
impl<T> BatchBuilder<T> for RadioBuilder<T> where T: Clone + PartialEq + Eq + std::hash::Hash {}

// ============================================================================
// Switch Builder
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
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.props.label = Some(label.into());
        self
    }

    /// Convenience method to create switch in on state
    #[must_use]
    pub fn on() -> Self {
        Self::new(SwitchState::On)
    }

    /// Convenience method to create switch in off state
    #[must_use]
    pub fn off() -> Self {
        Self::new(SwitchState::Off)
    }

    /// Create switch from boolean value
    #[must_use]
    pub fn from_bool(enabled: bool) -> Self {
        Self::new(SwitchState::from_bool(enabled))
    }

    /// Get the current state
    #[must_use]
    pub const fn state(&self) -> SwitchState {
        self.state
    }

    /// Get the component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }
}

// Apply common builder methods to SwitchBuilder
impl_common_builder_methods!(SwitchBuilder);

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

impl ConditionalBuilder<SwitchState> for SwitchBuilder {}
impl BatchBuilder<SwitchState> for SwitchBuilder {}

// ============================================================================
// Chip Builder
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

    // Common builder methods (disabled, size, error, validation, animation, has_error) 
    // are provided by the impl_common_builder_methods! macro applied below

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
}

// Apply common builder methods to ChipBuilder
impl_common_builder_methods!(ChipBuilder);

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

impl ConditionalBuilder<ChipState> for ChipBuilder {}
impl BatchBuilder<ChipState> for ChipBuilder {}

// ============================================================================
// Component Structs (Forward Declarations)
// ============================================================================

/// Material Design 3 Checkbox component (modern implementation)
#[derive(Debug, Clone)]
pub struct Checkbox {
    pub(crate) state: CheckboxState,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

/// Material Design 3 Radio Button component (modern implementation)
#[derive(Debug, Clone)]
pub struct Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    pub(crate) value: T,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

impl<T> PartialEq for Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl<T> Eq for Radio<T> where T: Clone + PartialEq + Eq + std::hash::Hash {}

/// Material Design 3 Switch component (modern implementation)
#[derive(Debug, Clone)]
pub struct Switch {
    pub(crate) state: SwitchState,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

/// Material Design 3 Chip component (modern implementation)
#[derive(Debug, Clone)]
pub struct Chip {
    pub(crate) label: String,
    pub(crate) state: ChipState,
    pub(crate) variant: ChipVariant,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

impl PartialEq for Chip {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
            && self.state == other.state
            && self.variant == other.variant
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl Eq for Chip {}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Create a new checkbox builder
#[must_use]
pub fn checkbox(state: CheckboxState) -> CheckboxBuilder {
    CheckboxBuilder::new(state)
}

/// Create a new radio builder
#[must_use]
pub fn radio<T>(value: T) -> RadioBuilder<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    RadioBuilder::new(value)
}

/// Create a new switch builder
#[must_use]
pub fn switch(state: SwitchState) -> SwitchBuilder {
    SwitchBuilder::new(state)
}

/// Create a new chip builder
#[must_use]
pub fn chip<S: Into<String>>(label: S, variant: ChipVariant) -> ChipBuilder {
    ChipBuilder::new(label, variant)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_builder() {
        let checkbox = CheckboxBuilder::checked()
            .label("Test Checkbox")
            .size(ComponentSize::Large)
            .disabled(false)
            .error(true)
            .build()
            .expect("Should build valid checkbox");

        assert_eq!(checkbox.state, CheckboxState::Checked);
        assert_eq!(checkbox.props.label, Some("Test Checkbox".to_string()));
        assert_eq!(checkbox.props.size, ComponentSize::Large);
        assert!(!checkbox.props.disabled);
        assert!(checkbox.error_state);
    }

    #[test]
    fn test_radio_builder() {
        let radio = RadioBuilder::new("option_a")
            .label("Option A")
            .size(ComponentSize::Medium)
            .build()
            .expect("Should build valid radio");

        assert_eq!(radio.value, "option_a");
        assert_eq!(radio.props.label, Some("Option A".to_string()));
        assert_eq!(radio.props.size, ComponentSize::Medium);
    }

    #[test]
    fn test_switch_builder() {
        let switch = SwitchBuilder::on()
            .label("Enable feature")
            .disabled(false)
            .build()
            .expect("Should build valid switch");

        assert_eq!(switch.state, SwitchState::On);
        assert_eq!(switch.props.label, Some("Enable feature".to_string()));
        assert!(!switch.props.disabled);
    }

    #[test]
    fn test_chip_builder() {
        let chip = ChipBuilder::filter("Category")
            .selected(true)
            .size(ComponentSize::Small)
            .build()
            .expect("Should build valid chip");

        assert_eq!(chip.label, "Category");
        assert_eq!(chip.state, ChipState::Selected);
        assert_eq!(chip.variant, ChipVariant::Filter);
        assert_eq!(chip.props.size, ComponentSize::Small);
    }

    #[test]
    fn test_validation_failure() {
        let result = ChipBuilder::new("", ChipVariant::Filter).build();

        assert!(result.is_err());
        assert!(matches!(result, Err(SelectionError::EmptyLabel)));
    }

    #[test]
    fn test_conditional_builder() {
        let checkbox = CheckboxBuilder::unchecked()
            .when(true, |b| b.label("Conditional Label"))
            .when(false, |b| b.disabled(true))
            .build()
            .expect("Should build with conditional config");

        assert_eq!(checkbox.props.label, Some("Conditional Label".to_string()));
        assert!(!checkbox.props.disabled);
    }

    #[test]
    fn test_convenience_functions() {
        let cb = checkbox(CheckboxState::Checked)
            .label("Test")
            .build()
            .unwrap();
        let rb = radio("value").label("Test").build().unwrap();
        let sw = switch(SwitchState::On).label("Test").build().unwrap();
        let ch = chip("Test", ChipVariant::Filter).build().unwrap();

        assert_eq!(cb.state, CheckboxState::Checked);
        assert_eq!(rb.value, "value");
        assert_eq!(sw.state, SwitchState::On);
        assert_eq!(ch.label, "Test");
    }
}
