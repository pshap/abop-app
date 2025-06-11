//! Common types, traits, and utilities for Material Design 3 selection components
//!
//! This module provides the foundational building blocks for all selection components,
//! including state definitions, error handling, validation, and shared behavior patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

// Import the constant from chip module
use super::chip::MAX_CHIP_LABEL_LENGTH;

// ============================================================================
// Prelude - Import this for convenient access to all traits
// ============================================================================

/// Convenient re-exports for component traits and types
pub mod prelude {
    pub use super::{
        AnimatedComponent, AnimationConfig, CheckboxState, ChipState, ChipVariant, ComponentProps,
        ComponentSize, SelectionComponent, SelectionError, StatefulComponent, SwitchState,
    };
}

// ============================================================================
// Metadata Constants
// ============================================================================

/// Metadata key for leading icon configuration
pub const LEADING_ICON_KEY: &str = "leading_icon";
/// Metadata key for trailing icon configuration
pub const TRAILING_ICON_KEY: &str = "trailing_icon";
/// Metadata key for badge configuration
pub const BADGE_KEY: &str = "badge";
/// Metadata key for badge color configuration
pub const BADGE_COLOR_KEY: &str = "badge_color";
/// Metadata key for layout configuration
pub const LAYOUT_KEY: &str = "layout";
/// Metadata key for spacing configuration
pub const SPACING_KEY: &str = "spacing";

/// All supported metadata keys for validation
pub const SUPPORTED_METADATA_KEYS: &[&str] = &[
    LEADING_ICON_KEY,
    TRAILING_ICON_KEY,
    BADGE_KEY,
    BADGE_COLOR_KEY,
    LAYOUT_KEY,
    SPACING_KEY,
];

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
            Self::Small => 16.0,
            Self::Medium => 20.0,
            Self::Large => 24.0,
        }
    }

    /// Get the appropriate touch target size (Material Design minimum 48px)
    #[must_use]
    pub const fn touch_target_size(self) -> f32 {
        match self {
            Self::Small => 32.0,  // Compact but still accessible
            Self::Medium => 40.0, // Standard touch target
            Self::Large => 48.0,  // Full Material Design touch target
        }
    }

    /// Get the appropriate border width for the size
    #[must_use]
    pub const fn border_width(self) -> f32 {
        match self {
            Self::Small => 1.5,
            Self::Medium => 2.0,
            Self::Large => 2.5,
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
        self.touch_target_size() >= constants::MIN_TOUCH_TARGET_SIZE
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
    ///
    /// let props = ComponentProps::new()
    ///     .with_metadata(LEADING_ICON_KEY, "star")
    ///     .with_metadata(BADGE_KEY, "5");
    /// ```
    #[must_use]
    pub fn with_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        let key_string = key.into();

        // Use const lookup for better performance in release builds
        let is_known_key = SUPPORTED_METADATA_KEYS.iter().any(|&k| k == key_string);

        if is_known_key {
            self.metadata.insert(key_string, value.into());
        } else {
            // Allow unknown keys for extensibility but warn in debug builds
            #[cfg(debug_assertions)]
            eprintln!(
                "Warning: Unknown metadata key '{key_string}'. Consider using predefined constants."
            );
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
            max_label_length: constants::MAX_LABEL_LENGTH,
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
            duration: Duration::from_millis(constants::DEFAULT_ANIMATION_DURATION_MS),
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
    validate_props(props, &ValidationConfig::default())
}

/// Validate switch state consistency
pub fn validate_switch_state(
    _state: SwitchState,
    props: &ComponentProps,
) -> Result<(), SelectionError> {
    validate_props(props, &ValidationConfig::default())
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
        && label.len() > MAX_CHIP_LABEL_LENGTH
    {
        return Err(SelectionError::LabelTooLong {
            len: label.len(),
            max: MAX_CHIP_LABEL_LENGTH,
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

/// Check if the system has reduced motion enabled (placeholder implementation)
/// In a real implementation, this would check OS accessibility settings
#[must_use]
pub fn system_has_reduced_motion() -> bool {
    // TODO: Implement actual system check
    false
}

/// Helper function to create validation config for specific use cases
#[must_use]
pub const fn validation_config_for_chips() -> ValidationConfig {
    ValidationConfig {
        max_label_length: constants::MAX_CHIP_LABEL_LENGTH,
        allow_empty_label: false,
        custom_rules: Vec::new(),
    }
}

/// Helper function to create validation config for checkbox/radio/switch
#[must_use]
pub const fn validation_config_for_toggles() -> ValidationConfig {
    ValidationConfig {
        max_label_length: constants::MAX_LABEL_LENGTH,
        allow_empty_label: true,
        custom_rules: Vec::new(),
    }
}

// ============================================================================
// Common Constants
// ============================================================================

/// Common constants to avoid duplication
pub mod constants {
    /// Default empty label for components that don't require labels
    pub const DEFAULT_LABEL: &str = "";

    /// Default animation duration for state transitions (matches Material Design 3)
    pub const DEFAULT_ANIMATION_DURATION_MS: u64 = 200;

    /// Minimum touch target size per Material Design guidelines
    pub const MIN_TOUCH_TARGET_SIZE: f32 = 48.0;

    /// Maximum recommended label length for accessibility
    pub const MAX_LABEL_LENGTH: usize = 200;

    /// Maximum label length for chip components (stricter requirement)
    pub const MAX_CHIP_LABEL_LENGTH: usize = 100;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
}
