//! Common tests for all selection components
//!
//! This module contains shared test implementations for selection components
//! to ensure consistent behavior across different component types.

#[cfg(test)]
mod tests {

    use crate::styling::material::components::selection::common::prelude::*;
    use crate::styling::material::{Checkbox, Chip, Switch};

    #[test]
    fn test_checkbox_creation() {
        let checkbox = Checkbox::default();
        assert!(checkbox.is_unchecked());
        assert!(!checkbox.is_checked());
        assert!(!checkbox.is_indeterminate());
        assert!(!checkbox.is_selected());
        assert!(!checkbox.has_error());
    }

    #[test]
    fn test_checkbox_transitions() {
        let mut checkbox = Checkbox::default();

        // Test checking
        checkbox.check().unwrap();
        assert!(checkbox.is_checked());
        assert!(checkbox.is_selected());

        // Test unchecking
        checkbox.uncheck().unwrap();
        assert!(checkbox.is_unchecked());
        assert!(!checkbox.is_selected());

        // Test indeterminate
        checkbox.set_indeterminate().unwrap();
        assert!(checkbox.is_indeterminate());
        assert!(checkbox.is_selected());
    }

    #[test]
    fn test_checkbox_toggle() {
        let mut checkbox = Checkbox::default();
        let (prev, new) = checkbox.toggle().unwrap();
        assert!(matches!(prev, CheckboxState::Unchecked));
        assert!(matches!(new, CheckboxState::Checked));

        let (prev, new) = checkbox.toggle().unwrap();
        assert!(matches!(prev, CheckboxState::Checked));
        assert!(matches!(new, CheckboxState::Unchecked));
    }

    #[test]
    fn test_switch_creation() {
        let switch = Switch::default();
        assert!(switch.is_off());
        assert!(!switch.is_on());
        assert!(!switch.to_bool());
        assert!(!switch.has_error());
    }

    #[test]
    fn test_switch_transitions() {
        let mut switch = Switch::new(SwitchState::On);
        assert!(switch.is_on());
        assert!(switch.to_bool());

        let (prev, new) = switch.toggle().unwrap();
        assert!(matches!(prev, SwitchState::On));
        assert!(matches!(new, SwitchState::Off));
        assert!(switch.is_off());
    }

    #[test]
    fn test_chip_creation() {
        let chip = Chip::default();
        assert_eq!(chip.label(), "");
        assert!(chip.is_unselected());
    }

    #[test]
    fn test_chip_transitions() {
        let mut chip = Chip::default();
        chip.select().unwrap();
        assert!(chip.is_selected());

        chip.unselect().unwrap();
        assert!(chip.is_unselected());
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
