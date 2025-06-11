//! Comprehensive tests for the switch component
//!
//! Tests cover switch state management, Material Design 3 compliance,
//! custom widget preparation, and builder patterns.

use super::super::builder::{ComponentBuilder, Switch};
use super::super::common::{StatefulWidget, *};

#[cfg(test)]
mod switch_tests {
    use super::*;

    #[test]
    fn test_switch_creation() {
        let switch = Switch::off().build_unchecked();
        assert_eq!(switch.state(), SwitchState::Off);
        assert!(!switch.props().disabled);
        assert_eq!(switch.props().size, ComponentSize::Medium);
    }

    #[test]
    fn test_switch_with_label() {
        let switch = Switch::off()
            .label("Enable notifications")
            .build_unchecked();
        assert_eq!(
            switch.props().label,
            Some("Enable notifications".to_string())
        );
    }

    #[test]
    fn test_switch_state_changes() {
        let mut switch = Switch::off().build_unchecked();
        assert_eq!(switch.state(), SwitchState::Off);

        switch.update_state(SwitchState::On).unwrap();
        assert_eq!(switch.state(), SwitchState::On);

        let (_prev, _new) = switch.toggle().unwrap();
        assert_eq!(switch.state(), SwitchState::Off);

        let (_prev, _new) = switch.toggle().unwrap();
        assert_eq!(switch.state(), SwitchState::On);
    }

    #[test]
    fn test_switch_builder_pattern() {
        let switch = Switch::on()
            .label("Dark mode")
            .disabled(true)
            .size(ComponentSize::Large)
            .build_unchecked();

        assert_eq!(switch.state(), SwitchState::On);
        assert!(switch.props().disabled);
        assert_eq!(switch.props().size, ComponentSize::Large);
        assert_eq!(switch.props().label, Some("Dark mode".to_string()));
    }

    #[test]
    fn test_switch_convenience_methods() {
        let switch_on = Switch::on().build_unchecked();
        assert_eq!(switch_on.state(), SwitchState::On);
        assert!(switch_on.is_on());
        assert!(!switch_on.is_off());

        let switch_off = Switch::off().build_unchecked();
        assert_eq!(switch_off.state(), SwitchState::Off);
        assert!(switch_off.is_off());
        assert!(!switch_off.is_on());
    }

    #[test]
    fn test_switch_validation() {
        let valid_switch = Switch::off().label("Valid").build_unchecked();
        assert!(valid_switch.validate().is_ok());

        let long_label = "x".repeat(201);
        let invalid_switch = Switch::off().label(&long_label).build();
        assert!(invalid_switch.is_err());
    }

    #[test]
    fn test_switch_trait_implementations() {
        let switch = Switch::on().build_unchecked();

        // Test SelectionWidget trait
        assert!(switch.validate().is_ok());
        assert_eq!(switch.state(), SwitchState::On);

        // Test Clone
        let cloned = switch.clone();
        assert_eq!(switch.state(), cloned.state());
    }

    #[test]
    fn test_switch_disabled_behavior() {
        let disabled_switch = Switch::off().disabled(true).build_unchecked();
        assert!(disabled_switch.props().disabled);

        // Disabled state should still allow programmatic changes
        let mut switch = disabled_switch.clone();
        let (_prev, _new) = switch.toggle().unwrap();
        assert_eq!(switch.state(), SwitchState::On);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_switch_in_settings_form() {
        // Simulate a settings form with multiple switches
        let mut dark_mode = Switch::off().label("Dark mode").build_unchecked();

        let mut notifications = Switch::on().label("Enable notifications").build_unchecked();

        let auto_save = Switch::on()
            .label("Auto-save changes")
            .disabled(true) // Disabled because user doesn't have permission
            .build_unchecked();

        // Test initial states
        assert!(dark_mode.is_off());
        assert!(notifications.is_on());
        assert!(auto_save.is_on() && auto_save.props().disabled);

        // User toggles dark mode
        let (_prev, _new) = dark_mode.toggle().unwrap();
        assert!(dark_mode.is_on());

        // User disables notifications
        notifications.update_state(SwitchState::Off).unwrap();
        assert!(notifications.is_off());

        // Validate all switches
        assert!(dark_mode.validate().is_ok());
        assert!(notifications.validate().is_ok());
        assert!(auto_save.validate().is_ok());
    }

    #[test]
    fn test_switch_state_transitions() {
        let mut switch = Switch::off().build_unchecked();
        let states = [SwitchState::Off, SwitchState::On, SwitchState::Off];

        for (i, expected_state) in states.iter().enumerate() {
            if i > 0 {
                let (_prev, _new) = switch.toggle().unwrap();
            }
            assert_eq!(switch.state(), *expected_state);
        }
    }

    #[test]
    fn test_switch_state_persistence() {
        let switch = Switch::on()
            .label("Test switch")
            .size(ComponentSize::Large)
            .build_unchecked();

        // Test that state can be serialized (for persistence)
        let state = switch.state();
        assert_eq!(state, SwitchState::On);

        // Test that we can recreate switch with same state
        let recreated = Switch::new(state)
            .label("Test switch")
            .size(ComponentSize::Large)
            .build_unchecked();

        assert_eq!(recreated.state(), switch.state());
        assert_eq!(recreated.props().label, switch.props().label);
        assert_eq!(recreated.props().size, switch.props().size);
    }
}
