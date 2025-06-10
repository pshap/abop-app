//! Comprehensive tests for the switch component
//!
//! Tests cover switch state management, Material Design 3 compliance,
//! custom widget preparation, and builder patterns.

use super::super::builder::{ComponentBuilder, Switch};
use super::super::common::*;
use super::super::switch::*;

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

        switch.toggle().unwrap();
        assert_eq!(switch.state(), SwitchState::Off);

        switch.toggle().unwrap();
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
        switch.toggle().unwrap();
        assert_eq!(switch.state(), SwitchState::On);
    }

    #[test]
    fn test_switch_sizes() {
        let small = Switch::off().size(ComponentSize::Small).build_unchecked();
        let medium = Switch::off().size(ComponentSize::Medium).build_unchecked();
        let large = Switch::off().size(ComponentSize::Large).build_unchecked();

        assert_eq!(small.props().size, ComponentSize::Small);
        assert_eq!(medium.props().size, ComponentSize::Medium);
        assert_eq!(large.props().size, ComponentSize::Large);
    }
}

#[cfg(test)]
mod switch_dimensions_tests {
    use super::*;

    #[test]
    fn test_switch_dimensions_creation() {
        let dimensions = SwitchDimensions::for_size(ComponentSize::Medium);

        // Test Material Design 3 default dimensions
        assert_eq!(dimensions.track_width, 52.0);
        assert_eq!(dimensions.track_height, 32.0);
        assert_eq!(dimensions.thumb_diameter, 24.0);
        assert_eq!(dimensions.track_radius, 16.0);
    }

    #[test]
    fn test_switch_dimensions_for_size() {
        let small = SwitchDimensions::for_size(ComponentSize::Small);
        let medium = SwitchDimensions::for_size(ComponentSize::Medium);
        let large = SwitchDimensions::for_size(ComponentSize::Large);

        // Small should be smaller than medium
        assert!(small.track_width < medium.track_width);
        assert!(small.track_height < medium.track_height);
        assert!(small.thumb_diameter < medium.thumb_diameter);

        // Large should be larger than medium
        assert!(large.track_width > medium.track_width);
        assert!(large.track_height > medium.track_height);
        assert!(large.thumb_diameter > medium.thumb_diameter);
    }

    #[test]
    fn test_switch_dimensions_proportions() {
        let dimensions = SwitchDimensions::for_size(ComponentSize::Medium);

        // Track radius should be half the height for rounded ends
        assert_eq!(dimensions.track_radius, dimensions.track_height / 2.0);

        // Thumb should fit within track height
        assert!(dimensions.thumb_diameter <= dimensions.track_height);

        // Track width should be sufficient for thumb travel
        assert!(dimensions.track_width > dimensions.thumb_diameter);
    }

    #[test]
    fn test_switch_dimensions_material_compliance() {
        for size in [
            ComponentSize::Small,
            ComponentSize::Medium,
            ComponentSize::Large,
        ] {
            let dimensions = SwitchDimensions::for_size(size);

            // Material Design 3 requires track to be fully rounded
            assert_eq!(dimensions.track_radius, dimensions.track_height / 2.0);

            // Track should be able to contain thumb
            assert!(dimensions.track_width >= dimensions.thumb_diameter);
            assert!(dimensions.track_height >= dimensions.thumb_diameter);

            // Thumb should have proper elevation
            assert!(dimensions.thumb_elevation > 0.0);
        }
    }
}

// NOTE: CustomSwitchWidget tests are commented out because CustomSwitchWidget
// is a private implementation detail for Phase 4 and not part of the public API
/*
#[cfg(test)]
mod custom_switch_widget_tests {
    use super::*;

    #[test]
    fn test_custom_switch_widget_creation() {
        let custom_widget = CustomSwitchWidget::new(SwitchState::On, "Test".to_string());

        // Test that custom widget preserves switch state
        assert_eq!(custom_widget.state, SwitchState::On);
        assert_eq!(custom_widget.label, "Test");
    }

    // ... other CustomSwitchWidget tests commented out
}
*/

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
        dark_mode.toggle().unwrap();
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
    fn test_switch_with_custom_dimensions() {
        // Test that we can create switches with different sizes
        // and that dimensions are calculated correctly
        let large_dimensions = SwitchDimensions::for_size(ComponentSize::Large);
        let medium_dimensions = SwitchDimensions::for_size(ComponentSize::Medium);

        // Verify dimensions are different for different sizes
        assert!(large_dimensions.track_width > medium_dimensions.track_width);
        assert!(large_dimensions.track_height > medium_dimensions.track_height);
        assert!(large_dimensions.thumb_diameter > medium_dimensions.thumb_diameter);
    }

    #[test]
    fn test_switch_state_transitions() {
        let mut switch = Switch::off().build_unchecked();
        let states = vec![SwitchState::Off, SwitchState::On, SwitchState::Off];

        for (i, expected_state) in states.iter().enumerate() {
            if i > 0 {
                switch.toggle().unwrap();
            }
            assert_eq!(switch.state(), *expected_state);
        }
    }

    #[test]
    fn test_switch_material_design_compliance() {
        for size in [
            ComponentSize::Small,
            ComponentSize::Medium,
            ComponentSize::Large,
        ] {
            let switch = Switch::off().size(size).build_unchecked();
            let dimensions = SwitchDimensions::for_size(size);

            // Test that dimensions match Material Design 3 specifications
            assert_eq!(dimensions.track_radius, dimensions.track_height / 2.0);
            assert!(dimensions.thumb_diameter <= dimensions.track_height);

            // Test that switch validates correctly
            assert!(switch.validate().is_ok());
        }
    }

    #[test]
    fn test_switch_error_conditions() {
        // Test validation errors
        let long_label = "x".repeat(201);
        let invalid_switch = Switch::off().label(&long_label).build();

        match invalid_switch {
            Err(SelectionError::LabelTooLong { len, max }) => {
                assert_eq!(len, 201);
                assert_eq!(max, 200);
            }
            _ => panic!("Expected LabelTooLong validation error"),
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
