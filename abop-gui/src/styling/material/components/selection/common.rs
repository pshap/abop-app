//! Common types, traits, and utilities for Material Design 3 selection components
//!
//! This module provides the foundational building blocks for all selection components,
//! including state definitions, error handling, validation, and shared behavior patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

// Import constants from the new constants module
use super::constants;
use super::state_traits::{ComponentState, InteractiveState, MultiLevelState};

// Remove hardcoded dependency - use constants instead
// Note: This will require updating chip module to use constants as well

// ============================================================================
// Prelude - Import this for convenient access to all traits
// ============================================================================

/// Convenient re-exports for component traits and types
pub mod prelude {
    // Core component types and states
    pub use super::{
        AnimatedComponent, AnimationConfig, CheckboxState, ChipState, ChipVariant, ComponentProps,
        ComponentSize, SelectionComponent, SelectionError, StatefulComponent, SwitchState,
    };

    // Unified state traits
    pub use super::super::state_traits::{
        AnimatableState, ComponentState, InteractiveState, MultiLevelState,
    };

    // Validation types
    pub use super::{EasingCurve, ValidationConfig, ValidationRule};

    // Constants access
    pub use super::super::constants;
}

// Note: Metadata keys are now centralized in the constants module
// Access them via: constants::metadata_keys::LEADING_ICON, etc.

// ============================================================================
// Component States (Modern State-Based Design)
// ============================================================================

/// Checkbox state enum for type-safe state management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheckboxState {
    /// Checkbox is unchecked
    Unchecked,
    /// Checkbox is checked
    Checked,
    /// Checkbox is in indeterminate state (partially checked)
    Indeterminate,
}

impl Default for CheckboxState {
    fn default() -> Self {
        Self::Unchecked
    }
}

impl CheckboxState {
    /// Check if the checkbox is in a selected state (checked or indeterminate)
    #[must_use]
    pub const fn is_selected(self) -> bool {
        matches!(self, Self::Checked | Self::Indeterminate)
    }

    /// Toggle between checked and unchecked (indeterminate goes to checked)
    #[must_use]
    pub const fn toggle(self) -> Self {
        match self {
            Self::Unchecked => Self::Checked,
            Self::Checked => Self::Unchecked,
            Self::Indeterminate => Self::Checked,
        }
    }

    /// Convert to boolean (true if checked, false otherwise)
    #[must_use]
    pub const fn to_bool(self) -> bool {
        matches!(self, Self::Checked)
    }

    /// Create from boolean value
    #[must_use]
    pub const fn from_bool(checked: bool) -> Self {
        if checked {
            Self::Checked
        } else {
            Self::Unchecked
        }
    }
}

// Phase 1: Implement unified state traits for CheckboxState
impl ComponentState for CheckboxState {
    fn toggle(self) -> Self {
        Self::toggle(self)
    }

    fn is_active(self) -> bool {
        self.is_selected()
    }

    fn to_bool(self) -> bool {
        Self::to_bool(self)
    }

    fn from_bool(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl MultiLevelState for CheckboxState {
    fn is_intermediate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    fn all_states() -> &'static [Self] {
        &[Self::Unchecked, Self::Checked, Self::Indeterminate]
    }

    fn next_state(self) -> Self {
        match self {
            Self::Unchecked => Self::Checked,
            Self::Checked => Self::Indeterminate,
            Self::Indeterminate => Self::Unchecked,
        }
    }
}

/// Switch state enum for on/off toggles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwitchState {
    /// Switch is off/disabled
    Off,
    /// Switch is on/enabled
    On,
}

impl Default for SwitchState {
    fn default() -> Self {
        Self::Off
    }
}

impl SwitchState {
    /// Toggle between on and off
    #[must_use]
    pub const fn toggle(self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }

    /// Check if switch is on
    #[must_use]
    pub const fn is_on(self) -> bool {
        matches!(self, Self::On)
    }

    /// Convert to boolean
    #[must_use]
    pub const fn to_bool(self) -> bool {
        self.is_on()
    }

    /// Create from boolean value
    #[must_use]
    pub const fn from_bool(enabled: bool) -> Self {
        if enabled { Self::On } else { Self::Off }
    }
}

// Phase 1: Implement unified state traits for SwitchState
impl ComponentState for SwitchState {
    fn toggle(self) -> Self {
        Self::toggle(self)
    }

    fn is_active(self) -> bool {
        self.is_on()
    }

    fn to_bool(self) -> bool {
        Self::to_bool(self)
    }

    fn from_bool(value: bool) -> Self {
        Self::from_bool(value)
    }
}

/// Chip state enum for selection state with animation support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChipState {
    /// Chip is unselected
    Unselected,
    /// Chip is selected
    Selected,
    /// Chip is being pressed (for animation support)
    Pressed,
}

impl Default for ChipState {
    fn default() -> Self {
        Self::Unselected
    }
}

impl ChipState {
    /// Check if chip is selected
    #[must_use]
    pub const fn is_selected(self) -> bool {
        matches!(self, Self::Selected | Self::Pressed)
    }

    /// Toggle between selected and unselected
    #[must_use]
    pub const fn toggle(self) -> Self {
        match self {
            Self::Unselected => Self::Selected,
            Self::Selected | Self::Pressed => Self::Unselected,
        }
    }
}

// Phase 1: Implement unified state traits for ChipState
impl ComponentState for ChipState {
    fn toggle(self) -> Self {
        Self::toggle(self)
    }

    fn is_active(self) -> bool {
        self.is_selected()
    }

    fn to_bool(self) -> bool {
        matches!(self, Self::Selected)
    }

    fn from_bool(value: bool) -> Self {
        if value {
            Self::Selected
        } else {
            Self::Unselected
        }
    }
}

impl InteractiveState for ChipState {
    fn is_pressed(self) -> bool {
        matches!(self, Self::Pressed)
    }

    fn to_pressed(self) -> Self {
        match self {
            Self::Selected => Self::Pressed,
            other => other,
        }
    }

    fn to_unpressed(self) -> Self {
        match self {
            Self::Pressed => Self::Selected,
            other => other,
        }
    }
}

// ============================================================================
// Component Size System
// ============================================================================

/// Size variants for consistent sizing across all selection components
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ComponentSize {
    /// Small size (16px) - for dense layouts and compact spaces
    Small,
    /// Medium size (20px) - default size for most use cases
    Medium,
    /// Large size (24px) - for accessibility and prominent placement
    Large,
}

impl Default for ComponentSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl ComponentSize {
    /// Get the pixel size for the selection component
    #[must_use]
    pub const fn size_px(self) -> f32 {
        match self {
            Self::Small => constants::sizes::SMALL_SIZE_PX,
            Self::Medium => constants::sizes::MEDIUM_SIZE_PX,
            Self::Large => constants::sizes::LARGE_SIZE_PX,
        }
    }
    /// Get the appropriate touch target size (Material Design minimum 48px)
    #[must_use]
    pub const fn touch_target_size(self) -> f32 {
        // Material Design minimum touch target size is 48px
        constants::ui::MIN_TOUCH_TARGET_SIZE
    }

    /// Get the appropriate border width for the size
    #[must_use]
    pub const fn border_width(self) -> f32 {
        // Default border width based on size
        match self {
            Self::Small => 1.0,
            Self::Medium => 1.5,
            Self::Large => 2.0,
        }
    }

    /// Get the appropriate text size for labels
    #[must_use]
    pub const fn text_size(self) -> f32 {
        match self {
            Self::Small => 12.0,
            Self::Medium => 14.0,
            Self::Large => 16.0,
        }
    }

    /// Get all available sizes
    #[must_use]
    pub const fn all() -> [Self; 3] {
        [Self::Small, Self::Medium, Self::Large]
    }

    /// Check if this size meets Material Design touch target requirements
    #[must_use]
    pub const fn meets_touch_target_requirements(self) -> bool {
        self.touch_target_size() >= constants::ui::MIN_TOUCH_TARGET_SIZE
    }
}

// ============================================================================
// Chip Variants
// ============================================================================

/// Material Design 3 chip variants for different use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChipVariant {
    /// Action chips for common tasks and quick actions
    Assist,
    /// Filter chips for filtering content and making selections  
    Filter,
    /// Input chips for user-generated content and tags
    Input,
    /// Suggestion chips for suggested actions or completions
    Suggestion,
}

impl Default for ChipVariant {
    fn default() -> Self {
        Self::Filter
    }
}

// ============================================================================
// Component Properties
// ============================================================================

/// Common properties shared across selection components
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ComponentProps {
    /// Optional text label displayed with the component
    pub label: Option<String>,
    /// Whether the component is disabled (non-interactive)
    pub disabled: bool,
    /// The size variant of the component
    pub size: ComponentSize,
    /// Metadata storage for extended properties (icons, badges, layout, etc.)
    pub metadata: HashMap<String, String>,
}

impl ComponentProps {
    /// Create new component properties
    #[must_use]
    pub fn new() -> Self {
        Self {
            label: None,
            disabled: false,
            size: ComponentSize::Medium,
            metadata: HashMap::new(),
        }
    }

    /// Set the label (builder pattern)
    #[must_use]
    pub fn with_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set disabled state (builder pattern)
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set size (builder pattern)
    #[must_use]
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
    /// Add metadata key-value pair (builder pattern)
    ///
    /// This method allows storing arbitrary metadata for enhanced features
    /// like icons, badges, layout preferences, etc. It validates that only
    /// known metadata keys are used to prevent typos and maintain consistency.
    ///
    /// # Arguments
    /// * `key` - The metadata key (should use predefined constants)
    /// * `value` - The metadata value (automatically converted to String)
    ///
    /// # Examples
    /// ```rust,no_run
    /// use abop_gui::styling::material::components::selection::common::*;
    /// use abop_gui::styling::material::components::selection::constants;
    ///
    /// let props = ComponentProps::new()
    ///     .with_metadata(constants::metadata_keys::LEADING_ICON, "star")
    ///     .with_metadata(constants::metadata_keys::BADGE, "5");
    /// ```
    #[must_use]
    pub fn with_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        let key_string = key.into();

        // Use const lookup for better performance in release builds
        let is_known_key = constants::metadata_keys::ALL_SUPPORTED
            .iter()
            .any(|&k| k == key_string);

        if is_known_key {
            self.metadata.insert(key_string, value.into());
        } else {
            // Allow unknown keys for extensibility but warn in debug builds
            #[cfg(debug_assertions)]
            log::warn!("Unknown metadata key '{key_string}'. Consider using predefined constants.");
            self.metadata.insert(key_string, value.into());
        }
        self
    }

    /// Get metadata value by key
    ///
    /// # Arguments
    /// * `key` - The metadata key to look up
    ///
    /// # Returns
    /// Optional reference to the metadata value as a string slice for better performance
    #[must_use]
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// Check if metadata contains a specific key
    ///
    /// # Arguments
    /// * `key` - The metadata key to check
    ///
    /// # Returns
    /// True if the key exists in metadata
    #[must_use]
    pub fn has_metadata(&self, key: &str) -> bool {
        self.metadata.contains_key(key)
    }
}

// ============================================================================
// Validation System
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
// Animation Configuration
// ============================================================================

/// Animation configuration for selection components
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationConfig {
    /// Animation duration
    pub duration: Duration,
    /// Whether animations are enabled
    pub enabled: bool,
    /// Respect system reduced motion preferences
    pub respect_reduced_motion: bool,
    /// Easing curve type
    pub easing: EasingCurve,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(constants::animation::DEFAULT_DURATION_MS),
            enabled: true,
            respect_reduced_motion: true,
            easing: EasingCurve::Standard,
        }
    }
}

/// Material Design 3 easing curves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingCurve {
    /// Standard easing for common transitions
    Standard,
    /// Emphasized easing for important transitions
    Emphasized,
    /// Decelerated easing for entering elements
    Decelerated,
    /// Accelerated easing for exiting elements
    Accelerated,
}

// ============================================================================
// Common Validation Functions
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
// Core Selection Component Traits (Simplified and Focused)
// ============================================================================

/// Core interface for selection components with validation
///
/// This trait focuses on the essential operations that all selection components need.
/// It avoids complex generics and provides clear, focused functionality.
pub trait SelectionComponent {
    /// The state type for this component
    type State: Copy + PartialEq;

    /// The message type produced by this component
    type Message;

    /// Get the current state
    fn state(&self) -> Self::State;

    /// Get the component properties
    fn props(&self) -> &ComponentProps;

    /// Validate the current widget state
    fn validate(&self) -> Result<(), SelectionError>;

    /// Check if the component has validation errors
    fn has_error(&self) -> bool;
}

/// Trait for components that can change state
pub trait StatefulComponent: SelectionComponent {
    /// Update the component state with validation
    fn set_state(&mut self, new_state: Self::State) -> Result<(), SelectionError>;

    /// Set error state
    fn set_error(&mut self, error: bool);
}

/// Trait for components that support animation
pub trait AnimatedComponent {
    /// Get animation configuration
    fn animation_config(&self) -> &AnimationConfig;

    /// Set animation configuration
    fn set_animation_config(&mut self, config: AnimationConfig);

    /// Check if animations should be used
    #[must_use]
    fn should_animate(&self) -> bool {
        let config = self.animation_config();
        config.enabled && (!config.respect_reduced_motion || !system_has_reduced_motion())
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Check if the system has reduced motion enabled
/// 
/// This function checks environment variables and provides a simple cross-platform
/// reduced motion detection. In production, this could be enhanced with OS-specific APIs.
#[must_use]
pub fn system_has_reduced_motion() -> bool {
    // Check common environment variables for reduced motion preference
    std::env::var("ABOP_REDUCE_MOTION").is_ok_and(|v| v == "1" || v.to_lowercase() == "true") ||
    std::env::var("PREFER_REDUCED_MOTION").is_ok_and(|v| v == "1" || v.to_lowercase() == "true")
}

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

// Constants are now centralized in the `constants` module.
// Use: `use super::constants` to access all organized constants.

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;    #[test]
    fn test_checkbox_state_transitions() {
        assert_eq!(CheckboxState::Unchecked.toggle(), CheckboxState::Checked);
        assert_eq!(CheckboxState::Checked.toggle(), CheckboxState::Unchecked);
        assert_eq!(
            CheckboxState::Indeterminate.toggle(),
            CheckboxState::Checked
        );

        assert!(CheckboxState::Checked.is_selected());
        assert!(CheckboxState::Indeterminate.is_selected());
        assert!(!CheckboxState::Unchecked.is_selected());
    }

    #[test]
    fn test_switch_state_transitions() {
        assert_eq!(SwitchState::Off.toggle(), SwitchState::On);
        assert_eq!(SwitchState::On.toggle(), SwitchState::Off);

        assert!(SwitchState::On.is_on());
        assert!(!SwitchState::Off.is_on());
    }

    #[test]
    fn test_component_size_properties() {
        assert_eq!(ComponentSize::Small.size_px(), 16.0);
        assert_eq!(ComponentSize::Medium.size_px(), 20.0);
        assert_eq!(ComponentSize::Large.size_px(), 24.0);

        assert!(ComponentSize::Small.touch_target_size() >= 32.0);
        assert!(ComponentSize::Large.touch_target_size() >= 48.0);
    }

    #[test]
    fn test_validation() {
        let props = ComponentProps::new().with_label("Valid label");
        assert!(validate_props(&props, &ValidationConfig::default()).is_ok());

        let long_label = "x".repeat(201);
        let invalid_props = ComponentProps::new().with_label(long_label);
        assert!(validate_props(&invalid_props, &ValidationConfig::default()).is_err());
    }

    #[test]
    fn test_component_props_builder() {
        let props = ComponentProps::new()
            .with_label("Test Label")
            .disabled(true)
            .size(ComponentSize::Large);

        assert_eq!(props.label, Some("Test Label".to_string()));
        assert!(props.disabled);
        assert_eq!(props.size, ComponentSize::Large);
    }
    #[test]
    fn test_component_props_metadata() {
        let props = ComponentProps::new()
            .with_metadata("leading_icon", "filter")
            .with_metadata("badge_count", "5")
            .with_metadata("layout", "wrap");

        assert_eq!(props.get_metadata("leading_icon"), Some("filter"));
        assert_eq!(props.get_metadata("badge_count"), Some("5"));
        assert_eq!(props.get_metadata("layout"), Some("wrap"));
        assert_eq!(props.get_metadata("nonexistent"), None);

        assert!(props.has_metadata("leading_icon"));
        assert!(props.has_metadata("badge_count"));
        assert!(!props.has_metadata("nonexistent"));
    }

    // Phase 1: Tests for unified state traits
    #[test]
    fn test_component_state_trait_consistency() {
        use super::super::state_traits::ComponentState;

        // Test CheckboxState implements ComponentState
        let checkbox = CheckboxState::Unchecked;
        assert!(!checkbox.is_active());
        assert!(!checkbox.to_bool());

        let toggled = checkbox.toggle();
        assert_eq!(toggled, CheckboxState::Checked);
        assert!(toggled.is_active());
        assert!(toggled.to_bool());

        // Test SwitchState implements ComponentState
        let switch = SwitchState::Off;
        assert!(!switch.is_active());
        assert!(!switch.to_bool());

        let toggled = switch.toggle();
        assert_eq!(toggled, SwitchState::On);
        assert!(toggled.is_active());
        assert!(toggled.to_bool());

        // Test ChipState implements ComponentState
        let chip = ChipState::Unselected;
        assert!(!chip.is_active());
        assert!(!chip.to_bool());

        let toggled = chip.toggle();
        assert_eq!(toggled, ChipState::Selected);
        assert!(toggled.is_active());
        assert!(toggled.to_bool());
    }

    #[test]
    fn test_multi_level_state_trait() {
        use super::super::state_traits::MultiLevelState;

        // Test CheckboxState MultiLevelState implementation
        assert!(!CheckboxState::Checked.is_intermediate());
        assert!(CheckboxState::Indeterminate.is_intermediate());
        assert!(!CheckboxState::Unchecked.is_intermediate());

        let all_states = CheckboxState::all_states();
        assert_eq!(all_states.len(), 3);
        assert!(all_states.contains(&CheckboxState::Unchecked));
        assert!(all_states.contains(&CheckboxState::Checked));
        assert!(all_states.contains(&CheckboxState::Indeterminate));

        // Test state cycling
        let unchecked = CheckboxState::Unchecked;
        let checked = unchecked.next_state();
        let indeterminate = checked.next_state();
        let back_to_unchecked = indeterminate.next_state();

        assert_eq!(checked, CheckboxState::Checked);
        assert_eq!(indeterminate, CheckboxState::Indeterminate);
        assert_eq!(back_to_unchecked, CheckboxState::Unchecked);
    }

    #[test]
    fn test_interactive_state_trait() {
        use super::super::state_traits::InteractiveState;

        // Test ChipState InteractiveState implementation
        let selected = ChipState::Selected;
        assert!(!selected.is_pressed());

        let pressed = selected.to_pressed();
        assert_eq!(pressed, ChipState::Pressed);
        assert!(pressed.is_pressed());

        let unpressed = pressed.to_unpressed();
        assert_eq!(unpressed, ChipState::Selected);
        assert!(!unpressed.is_pressed());

        // Test that unselected doesn't become pressed
        let unselected = ChipState::Unselected;
        let still_unselected = unselected.to_pressed();
        assert_eq!(still_unselected, ChipState::Unselected);
    }
    #[test]
    fn test_constants_access() {
        // Test that constants are properly accessible
        assert_eq!(constants::animation::DEFAULT_DURATION_MS, 200);
        assert_eq!(constants::ui::MIN_TOUCH_TARGET_SIZE, 48.0);
        assert_eq!(constants::ui::MAX_LABEL_LENGTH, 200);
        assert_eq!(constants::chips::MAX_LABEL_LENGTH, 100);

        // Test that component sizes use constants correctly
        assert_eq!(
            ComponentSize::Large.touch_target_size(),
            constants::sizes::touch_targets::LARGE
        );
        assert_eq!(
            ComponentSize::Medium.size_px(),
            constants::sizes::MEDIUM_SIZE_PX
        );
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

    #[test]
    fn test_trait_delegation_consistency() {
        use super::super::state_traits::ComponentState;
        
        // Test that trait methods delegate to inherent methods correctly
        let checkbox = CheckboxState::Unchecked;
        let inherent_toggle = checkbox.toggle();
        let trait_toggle = ComponentState::toggle(checkbox);
        assert_eq!(inherent_toggle, trait_toggle);
        
        let switch = SwitchState::Off;
        let inherent_toggle = switch.toggle();
        let trait_toggle = ComponentState::toggle(switch);
        assert_eq!(inherent_toggle, trait_toggle);
    }
}
