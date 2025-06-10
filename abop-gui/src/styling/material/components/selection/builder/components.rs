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

use super::super::common::*;
use crate::styling::material::colors::MaterialColors;
use crate::styling::material::components::selection_style::{
    SelectionSize as LegacySelectionSize, SelectionStyleBuilder, SelectionVariant,
};
use iced::{Element, Renderer, theme::Theme, widget::Radio as IcedRadio};

// ============================================================================
// Component Struct Definitions
// ============================================================================

/// Material Design 3 Checkbox component (modern implementation)
#[derive(Debug, Clone)]
pub struct Checkbox {
    pub(crate) state: CheckboxState,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

impl Checkbox {
    /// Get the checkbox state
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

    /// Set the checkbox state
    pub fn set_state(&mut self, state: CheckboxState) {
        self.state = state;
    }

    /// Set error state
    pub fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }

    /// Toggle the checkbox state
    pub fn toggle(&mut self) {
        self.state = self.state.toggle();
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

impl<T> Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    /// Create a new radio button with the specified value
    #[must_use]
    pub fn new(value: T) -> super::RadioBuilder<T> {
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
        color_scheme: &'a MaterialColors,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        T: Copy + 'a,
    {
        // Convert modern size to legacy size
        let legacy_size = match self.props.size {
            ComponentSize::Small => LegacySelectionSize::Small,
            ComponentSize::Medium => LegacySelectionSize::Medium,
            ComponentSize::Large => LegacySelectionSize::Large,
        };

        // Create styling function
        let style_fn = SelectionStyleBuilder::new(color_scheme.clone(), SelectionVariant::Radio)
            .size(legacy_size)
            .error(self.error_state)
            .radio_style();

        // Create the radio button label
        let default_label = String::new();
        let label = self.props.label.as_ref().unwrap_or(&default_label);

        // Create radio widget
        let radio = IcedRadio::new(label, self.value, selected_value, on_select).style(style_fn);

        // Note: Radio widget always requires on_select callback in Iced API
        // Disabled state is handled through styling and visual appearance only

        radio.into()
    }
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

impl Switch {
    /// Get the switch state
    #[must_use]
    pub const fn state(&self) -> SwitchState {
        self.state
    }

    /// Get the component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    /// Check if the switch is in error state
    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.error_state
    }

    /// Get the animation configuration
    #[must_use]
    pub const fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    /// Set the switch state
    pub fn set_state(&mut self, state: SwitchState) {
        self.state = state;
    }

    /// Set error state
    pub fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }

    /// Toggle the switch state
    pub fn toggle(&mut self) {
        self.state = self.state.toggle();
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

impl Chip {
    /// Get the chip label
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the chip state
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

    /// Check if the chip is in error state
    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.error_state
    }

    /// Get the animation configuration
    #[must_use]
    pub const fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    /// Set the chip state
    pub fn set_state(&mut self, state: ChipState) {
        self.state = state;
    }
    /// Set error state
    pub fn set_error(&mut self, error: bool) {
        self.error_state = error;
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
            && self.state == other.state
            && self.variant == other.variant
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl Eq for Chip {}

// ============================================================================
// Trait Implementations
// ============================================================================

impl SelectionWidget<CheckboxState> for Checkbox {
    type Message = CheckboxState;
    type Builder = super::CheckboxBuilder;

    fn new(state: CheckboxState) -> Self::Builder {
        super::CheckboxBuilder::new(state)
    }

    fn validate(&self) -> Result<(), SelectionError> {
        super::super::common::validate_checkbox_state(self.state, &self.props)
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
        self.state = new_state;
        Ok(())
    }

    fn transition_to(&mut self, new_state: CheckboxState) -> Result<CheckboxState, SelectionError> {
        self.state = new_state;
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

impl SelectionWidget<SwitchState> for Switch {
    type Message = SwitchState;
    type Builder = super::SwitchBuilder;

    fn new(state: SwitchState) -> Self::Builder {
        super::SwitchBuilder::new(state)
    }

    fn validate(&self) -> Result<(), SelectionError> {
        super::super::common::validate_switch_state(self.state, &self.props)
    }

    fn state(&self) -> SwitchState {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }
}

impl StatefulWidget<SwitchState> for Switch {
    fn update_state(&mut self, new_state: SwitchState) -> Result<(), SelectionError> {
        self.state = new_state;
        Ok(())
    }

    fn transition_to(&mut self, new_state: SwitchState) -> Result<SwitchState, SelectionError> {
        self.state = new_state;
        Ok(self.state)
    }
}

impl AnimatedWidget for Switch {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

impl<T> SelectionWidget<T> for Radio<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    type Message = T;
    type Builder = super::RadioBuilder<T>;

    fn new(state: T) -> Self::Builder {
        super::RadioBuilder::new(state)
    }

    fn validate(&self) -> Result<(), SelectionError> {
        // Radio validation would go here
        Ok(())
    }

    fn state(&self) -> T {
        self.value.clone()
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }
}

impl SelectionWidget<ChipState> for Chip {
    type Message = ChipState;
    type Builder = super::ChipBuilder;

    fn new(state: ChipState) -> Self::Builder {
        super::ChipBuilder::filter("Default").with_state(state)
    }

    fn validate(&self) -> Result<(), SelectionError> {
        super::super::common::validate_chip_state(self.state, self.variant, &self.props)
    }

    fn state(&self) -> ChipState {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }
}

impl StatefulWidget<ChipState> for Chip {
    fn update_state(&mut self, new_state: ChipState) -> Result<(), SelectionError> {
        self.state = new_state;
        Ok(())
    }

    fn transition_to(&mut self, new_state: ChipState) -> Result<ChipState, SelectionError> {
        self.state = new_state;
        Ok(self.state)
    }
}

impl AnimatedWidget for Chip {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

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

        checkbox.toggle();
        assert_eq!(checkbox.state(), CheckboxState::Checked);

        checkbox.toggle();
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

        switch.toggle();
        assert_eq!(switch.state(), SwitchState::On);

        switch.toggle();
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

        chip.toggle();
        assert_eq!(chip.state(), ChipState::Selected);

        chip.toggle();
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
