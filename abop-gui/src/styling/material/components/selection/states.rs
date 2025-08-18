//! State definitions for Material Design 3 selection components
//!
//! This module contains all state enums and their implementations for selection components,
//! including checkbox, switch, and chip states with unified trait implementations.

use serde::{Deserialize, Serialize};

use super::state_traits::{ComponentState, InteractiveState, MultiLevelState};

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

// Unified state traits for CheckboxState
impl ComponentState for CheckboxState {
    fn toggle(self) -> Self {
        self.toggle()
    }

    fn is_active(self) -> bool {
        self.is_selected()
    }

    fn to_bool(self) -> bool {
        self.to_bool()
    }

    fn from_bool(value: bool) -> Self {
        CheckboxState::from_bool(value)
    }
}

impl MultiLevelState for CheckboxState {
    fn is_intermediate(self) -> bool {
        matches!(self, CheckboxState::Indeterminate)
    }

    fn all_states() -> &'static [Self] {
        &[
            CheckboxState::Unchecked,
            CheckboxState::Checked,
            CheckboxState::Indeterminate,
        ]
    }

    fn next_state(self) -> Self {
        match self {
            CheckboxState::Unchecked => CheckboxState::Checked,
            CheckboxState::Checked => CheckboxState::Indeterminate,
            CheckboxState::Indeterminate => CheckboxState::Unchecked,
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

// Unified state traits for SwitchState
impl ComponentState for SwitchState {
    fn toggle(self) -> Self {
        self.toggle()
    }

    fn is_active(self) -> bool {
        self.is_on()
    }

    fn to_bool(self) -> bool {
        self.to_bool()
    }

    fn from_bool(value: bool) -> Self {
        SwitchState::from_bool(value)
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

// Unified state traits for ChipState
impl ComponentState for ChipState {
    fn toggle(self) -> Self {
        self.toggle()
    }

    fn is_active(self) -> bool {
        self.is_selected()
    }

    fn to_bool(self) -> bool {
        matches!(self, ChipState::Selected)
    }

    fn from_bool(value: bool) -> Self {
        if value {
            ChipState::Selected
        } else {
            ChipState::Unselected
        }
    }
}

impl InteractiveState for ChipState {
    fn is_pressed(self) -> bool {
        matches!(self, ChipState::Pressed)
    }

    fn to_pressed(self) -> Self {
        match self {
            ChipState::Selected => ChipState::Pressed,
            other => other,
        }
    }

    fn to_unpressed(self) -> Self {
        match self {
            ChipState::Pressed => ChipState::Selected,
            other => other,
        }
    }
}

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
