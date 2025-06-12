//! Component Struct Definitions
//!
//! This module contains the actual component structs that are built by the builders.
//! These represent the final constructed Material Design 3 selection components.
//!
//! ## Components
//! - [`Checkbox`] - Material Design 3 checkbox component
//! - [`Radio`] - Material Design 3 radio button component
//! - [`Switch`] - Material Design 3 switch component
//! - [`Chip`] - Material Design 3 chip component

use super::super::common::{prelude::*, validate_props};
use crate::styling::material::components::selection_style::{
    SelectionSize as LegacySelectionSize, SelectionStyleBuilder, SelectionVariant,
};
use crate::styling::material::{MaterialColors, tokens::core::MaterialTokens};
use iced::{Element, Renderer, theme::Theme, widget::Radio as IcedRadio};

// Static MaterialTokens instances to avoid lifetime issues
/// Default light theme Material Design tokens for selection components.
///
/// This provides a static instance of MaterialTokens configured with light theme colors,
/// used as a fallback when no specific tokens are provided for component styling.
pub static LIGHT_TOKENS: std::sync::LazyLock<MaterialTokens> = std::sync::LazyLock::new(|| {
    MaterialTokens::default().with_colors(MaterialColors::light_default())
});

/// Default dark theme Material Design tokens for selection components.
///
/// This provides a static instance of MaterialTokens configured with dark theme colors,
/// used as a fallback when no specific tokens are provided for component styling.
pub static DARK_TOKENS: std::sync::LazyLock<MaterialTokens> = std::sync::LazyLock::new(|| {
    MaterialTokens::default().with_colors(MaterialColors::dark_default())
});

// ============================================================================
// Component Struct Definitions
// ============================================================================

/// Material Design 3 Checkbox component (modern implementation)
#[derive(Debug, Clone, Default)]
pub struct Checkbox {
    pub(crate) state: CheckboxState,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

// ============================================================================
// Phase 1: Core Trait Implementations for Checkbox
// ============================================================================

impl SelectionComponent for Checkbox {
    type State = CheckboxState;
    type Message = CheckboxState;

    fn state(&self) -> Self::State {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }

    fn validate(&self) -> Result<(), SelectionError> {
        super::super::common::validate_checkbox_state(self.state, &self.props)
    }

    fn has_error(&self) -> bool {
        self.error_state
    }
}

impl StatefulComponent for Checkbox {
    fn set_state(&mut self, new_state: Self::State) -> Result<(), SelectionError> {
        self.state = new_state;
        Ok(())
    }

    fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }
}

impl AnimatedComponent for Checkbox {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

impl Checkbox {
    /// Create a new checkbox with the specified state
    #[must_use]
    pub fn new(state: CheckboxState) -> Self {
        Self {
            state,
            props: ComponentProps::default(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        }
    }

    /// Check the checkbox
    pub fn check(&mut self) -> Result<CheckboxState, SelectionError> {
        let previous_state = self.state;
        self.state = CheckboxState::Checked;
        Ok(previous_state)
    }

    /// Uncheck the checkbox
    pub fn uncheck(&mut self) -> Result<CheckboxState, SelectionError> {
        let previous_state = self.state;
        self.state = CheckboxState::Unchecked;
        Ok(previous_state)
    }

    /// Set checkbox to indeterminate state
    pub fn set_indeterminate(&mut self) -> Result<CheckboxState, SelectionError> {
        let previous_state = self.state;
        self.state = CheckboxState::Indeterminate;
        Ok(previous_state)
    }

    /// Toggle the checkbox state
    pub fn toggle(&mut self) -> Result<(CheckboxState, CheckboxState), SelectionError> {
        let previous_state = self.state;
        self.state = self.state.toggle();
        Ok((previous_state, self.state))
    }

    /// Check if the checkbox is selected (checked or indeterminate)
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        matches!(
            self.state,
            CheckboxState::Checked | CheckboxState::Indeterminate
        )
    }

    /// Check if the checkbox is checked
    #[must_use]
    pub const fn is_checked(&self) -> bool {
        matches!(self.state, CheckboxState::Checked)
    }

    /// Check if the checkbox is unchecked
    #[must_use]
    pub const fn is_unchecked(&self) -> bool {
        matches!(self.state, CheckboxState::Unchecked)
    }

    /// Check if the checkbox is in indeterminate state
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self.state, CheckboxState::Indeterminate)
    }

    /// Convert checkbox state to boolean (checked = true, others = false)
    #[must_use]
    pub const fn to_bool(&self) -> bool {
        matches!(self.state, CheckboxState::Checked)
    }
}

impl PartialEq for Checkbox {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl Eq for Checkbox {}

// ============================================================================
// Phase 1: Core Trait Implementations for Switch
// ============================================================================

impl SelectionComponent for Switch {
    type State = SwitchState;
    type Message = SwitchState;

    fn state(&self) -> Self::State {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }

    fn validate(&self) -> Result<(), SelectionError> {
        super::super::common::validate_switch_state(self.state, &self.props)
    }

    fn has_error(&self) -> bool {
        self.error_state
    }
}

impl StatefulComponent for Switch {
    fn set_state(&mut self, new_state: Self::State) -> Result<(), SelectionError> {
        self.state = new_state;
        Ok(())
    }

    fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }
}

impl AnimatedComponent for Switch {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

/// Material Design 3 Radio Button component (modern implementation)
#[derive(Debug, Clone)]
pub struct Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    pub(crate) value: T,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

impl<T> Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    /// Create a new radio button with the specified value
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            value,
            props: ComponentProps::default(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        }
    }

    /// Create a new radio button with the specified value
    #[must_use]
    pub fn builder(value: T) -> super::RadioBuilder<T> {
        super::RadioBuilder::new(value)
    }

    /// Get the radio button value
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get the component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    /// Check if the radio button is in error state
    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.error_state
    }

    /// Get the animation configuration
    #[must_use]
    pub const fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    /// Set error state
    pub fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }
    /// Check if this radio button is selected based on the current group value
    #[must_use]
    pub fn is_selected(&self, selected_value: Option<&T>) -> bool {
        selected_value == Some(&self.value)
    }

    /// Create the Iced widget element for this radio button
    ///
    /// # Arguments
    /// * `selected_value` - The currently selected value in the radio group
    /// * `on_select` - Callback function called when this radio button is selected
    /// * `color_scheme` - Material Design color scheme to use for styling
    ///
    /// # Returns
    /// An Iced Element that can be added to the UI
    pub fn view<'a, Message: Clone + 'a>(
        &self,
        selected_value: Option<T>,
        on_select: impl FnOnce(T) -> Message + Copy + 'a,
        _color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        T: Copy + 'a,
    {
        // Convert modern size to legacy size
        let legacy_size = match self.props.size {
            ComponentSize::Small => LegacySelectionSize::Small,
            ComponentSize::Medium => LegacySelectionSize::Medium,
            ComponentSize::Large => LegacySelectionSize::Large,
        }; // Create the radio button label
        let default_label = String::new();
        let label = self.props.label.as_ref().unwrap_or(&default_label);

        // Create tokens outside the closure to ensure they live long enough
        let tokens = &*LIGHT_TOKENS; // Default to light tokens for now

        // Create the style function with the tokens
        let style_fn = {
            let builder = SelectionStyleBuilder::new(tokens, SelectionVariant::Radio)
                .size(legacy_size)
                .error(self.error_state);

            // Create the style function
            builder.radio_style()
        };

        // Create radio widget with the style function
        let radio = IcedRadio::new(label, self.value, selected_value, on_select).style(style_fn);

        // Note: Radio widget always requires on_select callback in Iced API
        // Disabled state is handled through styling and visual appearance only

        radio.into()
    }
}

impl<T> PartialEq for Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl<T> Eq for Radio<T> where T: Clone + PartialEq + Eq + std::hash::Hash + Copy {}

// ============================================================================
// Phase 1: Core Trait Implementations for Radio
// ============================================================================

impl<T> SelectionComponent for Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    type State = T;
    type Message = T;

    fn state(&self) -> Self::State {
        self.value
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }

    fn validate(&self) -> Result<(), SelectionError> {
        // Radio validation would go here
        Ok(())
    }

    fn has_error(&self) -> bool {
        self.error_state
    }
}

impl<T> StatefulComponent for Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    fn set_state(&mut self, new_state: Self::State) -> Result<(), SelectionError> {
        self.value = new_state;
        Ok(())
    }

    fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }
}

impl<T> AnimatedComponent for Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Copy,
{
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

/// Material Design 3 Switch component (modern implementation)
#[derive(Debug, Clone, Default)]
pub struct Switch {
    pub(crate) state: SwitchState,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

impl Switch {
    /// Create a new switch with the specified state
    #[must_use]
    pub fn new(state: SwitchState) -> Self {
        Self {
            state,
            props: ComponentProps::default(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        }
    }

    /// Toggle the switch state
    pub fn toggle(&mut self) -> Result<(SwitchState, SwitchState), SelectionError> {
        let previous_state = self.state;
        self.state = self.state.toggle();
        Ok((previous_state, self.state))
    }

    /// Check if the switch is on
    #[must_use]
    pub const fn is_on(&self) -> bool {
        matches!(self.state, SwitchState::On)
    }

    /// Check if the switch is off
    #[must_use]
    pub const fn is_off(&self) -> bool {
        matches!(self.state, SwitchState::Off)
    }

    /// Convert switch state to boolean (on = true, off = false)
    #[must_use]
    pub const fn to_bool(&self) -> bool {
        matches!(self.state, SwitchState::On)
    }
}

impl PartialEq for Switch {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl Eq for Switch {}

// ============================================================================
// Phase 1: Core Trait Implementations for Chip
// ============================================================================

impl SelectionComponent for Chip {
    type State = ChipState;
    type Message = ChipState;

    fn state(&self) -> Self::State {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }

    fn validate(&self) -> Result<(), SelectionError> {
        validate_props(&self.props, &ValidationConfig::default())
    }

    fn has_error(&self) -> bool {
        self.error_state
    }
}

impl StatefulComponent for Chip {
    fn set_state(&mut self, state: ChipState) -> Result<(), SelectionError> {
        self.state = state;
        Ok(())
    }

    fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }
}

impl AnimatedComponent for Chip {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

/// Material Design 3 Chip component (modern implementation)
#[derive(Debug, Clone, Default)]
pub struct Chip {
    pub(crate) label: String,
    pub(crate) state: ChipState,
    pub(crate) variant: ChipVariant,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

impl Chip {
    /// Get the chip label
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the chip variant
    #[must_use]
    pub const fn variant(&self) -> ChipVariant {
        self.variant
    }

    /// Select the chip
    pub fn select(&mut self) -> Result<ChipState, SelectionError> {
        let previous_state = self.state;
        self.state = ChipState::Selected;
        Ok(previous_state)
    }

    /// Unselect the chip
    pub fn unselect(&mut self) -> Result<ChipState, SelectionError> {
        let previous_state = self.state;
        self.state = ChipState::Unselected;
        Ok(previous_state)
    }

    /// Toggle the chip selection state
    pub fn toggle(&mut self) -> Result<(ChipState, ChipState), SelectionError> {
        let previous_state = self.state;
        self.state = self.state.toggle();
        Ok((previous_state, self.state))
    }

    /// Check if the chip is selected
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        matches!(self.state, ChipState::Selected)
    }

    /// Check if the chip is unselected
    #[must_use]
    pub const fn is_unselected(&self) -> bool {
        matches!(self.state, ChipState::Unselected)
    }

    /// Set the chip label
    pub fn set_label<S: Into<String>>(&mut self, label: S) {
        self.label = label.into();
    }
}

impl PartialEq for Chip {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
            && self.variant == other.variant
            && self.state == other.state
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl Eq for Chip {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_component() {
        let checkbox = Checkbox {
            state: CheckboxState::Checked,
            props: ComponentProps::new(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        };

        assert_eq!(checkbox.state(), CheckboxState::Checked);
        assert!(!checkbox.has_error());
    }

    #[test]
    fn test_checkbox_toggle() {
        let mut checkbox = Checkbox {
            state: CheckboxState::Unchecked,
            props: ComponentProps::new(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        };

        let _ = checkbox.toggle();
        assert_eq!(checkbox.state(), CheckboxState::Checked);

        let _ = checkbox.toggle();
        assert_eq!(checkbox.state(), CheckboxState::Unchecked);
    }

    #[test]
    fn test_radio_component() {
        let radio = Radio {
            value: "option_a",
            props: ComponentProps::new(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        };

        assert_eq!(radio.value(), &"option_a");
        assert!(!radio.has_error());
        assert!(radio.is_selected(Some(&"option_a")));
        assert!(!radio.is_selected(Some(&"option_b")));
    }

    #[test]
    fn test_switch_component() {
        let switch = Switch {
            state: SwitchState::On,
            props: ComponentProps::new(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        };

        assert_eq!(switch.state(), SwitchState::On);
        assert!(switch.is_on());
        assert!(!switch.is_off());
        assert!(!switch.has_error());
    }

    #[test]
    fn test_switch_toggle() {
        let mut switch = Switch {
            state: SwitchState::Off,
            props: ComponentProps::new(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        };

        let _ = switch.toggle();
        assert_eq!(switch.state(), SwitchState::On);

        let _ = switch.toggle();
        assert_eq!(switch.state(), SwitchState::Off);
    }

    #[test]
    fn test_chip_component() {
        let chip = Chip {
            label: "Test Chip".to_string(),
            state: ChipState::Selected,
            variant: ChipVariant::Filter,
            props: ComponentProps::new(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        };

        assert_eq!(chip.label(), "Test Chip");
        assert_eq!(chip.state(), ChipState::Selected);
        assert_eq!(chip.variant(), ChipVariant::Filter);
        assert!(chip.is_selected());
        assert!(!chip.is_unselected());
        assert!(!chip.has_error());
    }

    #[test]
    fn test_chip_toggle() {
        let mut chip = Chip {
            label: "Toggle Chip".to_string(),
            state: ChipState::Unselected,
            variant: ChipVariant::Filter,
            props: ComponentProps::new(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        };

        let _ = chip.toggle();
        assert_eq!(chip.state(), ChipState::Selected);

        let _ = chip.toggle();
        assert_eq!(chip.state(), ChipState::Unselected);
    }

    #[test]
    fn test_component_equality() {
        let checkbox1 = Checkbox {
            state: CheckboxState::Checked,
            props: ComponentProps::new(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        };

        let checkbox2 = Checkbox {
            state: CheckboxState::Checked,
            props: ComponentProps::new(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        };

        assert_eq!(checkbox1, checkbox2);
    }
}
