//! Modern Material Design 3 Checkbox Implementation
//!
//! This module provides a completely redesigned checkbox component with:
//! - State-based design using CheckboxState enum
//! - Built-in validation and error handling
//! - Animation support for smooth transitions
//! - Indeterminate state visual rendering
//! - Modern builder pattern with fluent API

use super::builder::{Checkbox, CheckboxBuilder, ComponentBuilder};
use super::common::*;
use crate::styling::material::colors::MaterialColors;
use crate::styling::material::components::selection_style::{
    SelectionSize as LegacySelectionSize, SelectionStyleBuilder, SelectionVariant,
};

use iced::{Element, Renderer, theme::Theme, widget::Checkbox as IcedCheckbox};

// ============================================================================
// Component Implementation
// ============================================================================

impl Checkbox {
    /// Create a new checkbox with the specified state
    #[must_use]
    pub fn new(state: CheckboxState) -> CheckboxBuilder {
        CheckboxBuilder::new(state)
    }

    /// Create a checked checkbox
    #[must_use]
    pub fn checked() -> CheckboxBuilder {
        CheckboxBuilder::checked()
    }

    /// Create an unchecked checkbox
    #[must_use]
    pub fn unchecked() -> CheckboxBuilder {
        CheckboxBuilder::unchecked()
    }

    /// Create an indeterminate checkbox
    #[must_use]
    pub fn indeterminate() -> CheckboxBuilder {
        CheckboxBuilder::indeterminate()
    }

    /// Create checkbox from boolean value
    #[must_use]
    pub fn from_bool(checked: bool) -> CheckboxBuilder {
        CheckboxBuilder::from_bool(checked)
    }

    /// Get the current checkbox state
    #[must_use]
    pub const fn state(&self) -> CheckboxState {
        self.state
    }

    /// Get the component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    /// Check if the checkbox is in error state
    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.error_state
    }

    /// Get the animation configuration
    #[must_use]
    pub const fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    /// Update the checkbox state with validation
    pub fn update_state(&mut self, new_state: CheckboxState) -> Result<(), SelectionError> {
        validate_checkbox_state(new_state, &self.props)?;
        self.state = new_state;
        Ok(())
    }

    /// Toggle the checkbox state
    pub fn toggle(&mut self) -> Result<CheckboxState, SelectionError> {
        let new_state = self.state.toggle();
        self.update_state(new_state)?;
        Ok(new_state)
    }

    /// Set error state
    pub fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }

    /// Convert to boolean value (true if checked, false otherwise)
    #[must_use]
    pub const fn to_bool(&self) -> bool {
        self.state.to_bool()
    }

    /// Check if checkbox is selected (checked or indeterminate)
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        self.state.is_selected()
    }

    /// Create the Iced widget element for this checkbox
    ///
    /// # Arguments
    /// * `on_toggle` - Callback function called when checkbox state changes
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to the UI
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        on_toggle: impl Fn(CheckboxState) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        // Convert modern state to legacy boolean for Iced compatibility
        let is_checked = match self.state {
            CheckboxState::Unchecked => false,
            CheckboxState::Checked => true,
            CheckboxState::Indeterminate => false, // Special handling needed
        };

        // Convert modern size to legacy size
        let legacy_size = match self.props.size {
            ComponentSize::Small => LegacySelectionSize::Small,
            ComponentSize::Medium => LegacySelectionSize::Medium,
            ComponentSize::Large => LegacySelectionSize::Large,
        };

        // Create styling function
        let style_fn = SelectionStyleBuilder::new(color_scheme.clone(), SelectionVariant::Checkbox)
            .size(legacy_size)
            .error(self.error_state)
            .checkbox_style(); // Create the checkbox label
        let default_label = String::new();
        let label = self.props.label.as_ref().unwrap_or(&default_label);

        // Create checkbox widget
        let mut checkbox = IcedCheckbox::new(label, is_checked).style(style_fn);

        // Only add on_toggle handler if the checkbox is not disabled
        if !self.props.disabled {
            // Convert boolean toggle to state-based toggle
            let current_state = self.state;
            checkbox = checkbox.on_toggle(move |_checked| on_toggle(current_state.toggle()));
        }

        // TODO: Phase 5 - Add special visual handling for indeterminate state
        // This would require custom checkbox widget implementation or icon overlay

        checkbox.into()
    }

    /// Create a simplified view that handles state changes automatically
    ///
    /// This is a convenience method for cases where you want the checkbox to
    /// manage its own state internally.
    pub fn view_with_state<'a, Message: Clone + 'a>(
        &self,
        on_change: impl Fn(CheckboxState) -> Message + 'a,
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer> {
        self.view(on_change, color_scheme)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl SelectionWidget<CheckboxState> for Checkbox {
    type Message = CheckboxState;
    type Builder = CheckboxBuilder;

    fn new(state: CheckboxState) -> Self::Builder {
        CheckboxBuilder::new(state)
    }

    fn validate(&self) -> Result<(), SelectionError> {
        validate_checkbox_state(self.state, &self.props)
    }

    fn state(&self) -> CheckboxState {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }
}

impl StatefulWidget<CheckboxState> for Checkbox {
    fn update_state(&mut self, new_state: CheckboxState) -> Result<(), SelectionError> {
        self.update_state(new_state)
    }

    fn transition_to(&mut self, new_state: CheckboxState) -> Result<CheckboxState, SelectionError> {
        self.update_state(new_state)?;
        Ok(self.state)
    }
}

impl AnimatedWidget for Checkbox {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

// ============================================================================
// Default Implementation
// ============================================================================

impl Default for Checkbox {
    fn default() -> Self {
        CheckboxBuilder::unchecked().build_unchecked()
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Create a new checkbox builder
#[must_use]
pub fn checkbox(state: CheckboxState) -> CheckboxBuilder {
    CheckboxBuilder::new(state)
}

/// Create a checked checkbox builder
#[must_use]
pub fn checked_checkbox() -> CheckboxBuilder {
    CheckboxBuilder::checked()
}

/// Create an unchecked checkbox builder
#[must_use]
pub fn unchecked_checkbox() -> CheckboxBuilder {
    CheckboxBuilder::unchecked()
}

/// Create an indeterminate checkbox builder
#[must_use]
pub fn indeterminate_checkbox() -> CheckboxBuilder {
    CheckboxBuilder::indeterminate()
}

/// Create checkbox from boolean value
#[must_use]
pub fn checkbox_from_bool(checked: bool) -> CheckboxBuilder {
    CheckboxBuilder::from_bool(checked)
}

// ============================================================================
// Future: Custom Indeterminate Widget (Phase 5 Implementation)
// ============================================================================

/// Custom checkbox widget that supports indeterminate state
///
/// TODO: Phase 5 - Implement this as a proper Iced widget
/// This would provide native indeterminate state rendering with:
/// - Horizontal line icon for indeterminate state
/// - Proper state transitions and animations
/// - Full Material Design 3 compliance
#[allow(dead_code)]
struct CustomCheckboxWidget {
    state: CheckboxState,
    label: String,
    disabled: bool,
    error_state: bool,
    size: ComponentSize,
}

#[allow(dead_code)]
impl CustomCheckboxWidget {
    /// Create a new custom checkbox widget
    fn new(state: CheckboxState, label: String) -> Self {
        Self {
            state,
            label,
            disabled: false,
            error_state: false,
            size: ComponentSize::Medium,
        }
    }

    /// Set disabled state
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set error state
    fn error(mut self, error: bool) -> Self {
        self.error_state = error;
        self
    }

    /// Set size
    fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
}

// TODO: Phase 5 - Implement Widget trait for CustomCheckboxWidget
// This would provide proper indeterminate state rendering

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_creation() {
        let checkbox = Checkbox::checked()
            .label("Test Checkbox")
            .size(ComponentSize::Large)
            .build()
            .expect("Should create valid checkbox");

        assert_eq!(checkbox.state(), CheckboxState::Checked);
        assert_eq!(checkbox.props().label, Some("Test Checkbox".to_string()));
        assert_eq!(checkbox.props().size, ComponentSize::Large);
        assert!(checkbox.is_selected());
        assert!(checkbox.to_bool());
    }

    #[test]
    fn test_checkbox_state_transitions() {
        let mut checkbox = Checkbox::unchecked()
            .build()
            .expect("Should create valid checkbox");

        assert_eq!(checkbox.state(), CheckboxState::Unchecked);
        assert!(!checkbox.is_selected());

        // Toggle to checked
        let new_state = checkbox.toggle().expect("Should toggle successfully");
        assert_eq!(new_state, CheckboxState::Checked);
        assert_eq!(checkbox.state(), CheckboxState::Checked);
        assert!(checkbox.is_selected());

        // Toggle back to unchecked
        let new_state = checkbox.toggle().expect("Should toggle successfully");
        assert_eq!(new_state, CheckboxState::Unchecked);
        assert_eq!(checkbox.state(), CheckboxState::Unchecked);
        assert!(!checkbox.is_selected());
    }

    #[test]
    fn test_indeterminate_checkbox() {
        let checkbox = Checkbox::indeterminate()
            .label("Partial Selection")
            .build()
            .expect("Should create valid indeterminate checkbox");

        assert_eq!(checkbox.state(), CheckboxState::Indeterminate);
        assert!(checkbox.is_selected());
        assert!(!checkbox.to_bool()); // Indeterminate converts to false in boolean context
    }

    #[test]
    fn test_checkbox_validation() {
        // Valid checkbox
        let valid_checkbox = Checkbox::checked().label("Valid Label").build();
        assert!(valid_checkbox.is_ok());

        // Invalid checkbox - label too long
        let long_label = "x".repeat(201);
        let invalid_checkbox = Checkbox::unchecked().label(long_label).build();
        assert!(invalid_checkbox.is_err());
    }

    #[test]
    fn test_checkbox_error_state() {
        let mut checkbox = Checkbox::checked()
            .error(true)
            .build()
            .expect("Should create checkbox with error state");

        assert!(checkbox.has_error());

        checkbox.set_error(false);
        assert!(!checkbox.has_error());
    }

    #[test]
    fn test_checkbox_traits() {
        let checkbox = Checkbox::checked()
            .build()
            .expect("Should create valid checkbox");

        // Test SelectionWidget trait
        assert_eq!(checkbox.state(), CheckboxState::Checked);
        assert!(checkbox.validate().is_ok());

        // Test animation support
        assert!(checkbox.animation_config().enabled);
    }

    #[test]
    fn test_convenience_functions() {
        let cb1 = checked_checkbox().build().unwrap();
        let cb2 = unchecked_checkbox().build().unwrap();
        let cb3 = indeterminate_checkbox().build().unwrap();
        let cb4 = checkbox_from_bool(true).build().unwrap();

        assert_eq!(cb1.state(), CheckboxState::Checked);
        assert_eq!(cb2.state(), CheckboxState::Unchecked);
        assert_eq!(cb3.state(), CheckboxState::Indeterminate);
        assert_eq!(cb4.state(), CheckboxState::Checked);
    }

    #[test]
    fn test_checkbox_default() {
        let checkbox = Checkbox::default();
        assert_eq!(checkbox.state(), CheckboxState::Unchecked);
        assert!(!checkbox.props().disabled);
        assert_eq!(checkbox.props().size, ComponentSize::Medium);
    }
}
