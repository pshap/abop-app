//! Comprehensive tests for the switch component
//!
//! Tests cover switch state management, Material Design 3 compliance,
//! custom widget preparation, and builder patterns.

use super::super::common::*;
use super::super::switch::*;

#[cfg(test)]
mod switch_tests {
    use super::*;

    #[test]
    fn test_switch_creation() {
        let switch = Switch::new();
        assert_eq!(switch.state(), SwitchState::Off);
        assert!(!switch.props().disabled);
        assert_eq!(switch.props().size, ComponentSize::Medium);
    }

    #[test]
    fn test_switch_with_label() {
        let switch = Switch::new().with_label("Enable notifications");
        assert_eq!(
            switch.props().label,
            Some("Enable notifications".to_string())
        );
    }

    #[test]
    fn test_switch_state_changes() {
        let mut switch = Switch::new();
        assert_eq!(switch.state(), SwitchState::Off);

        switch.set_state(SwitchState::On);
        assert_eq!(switch.state(), SwitchState::On);

        switch.toggle();
        assert_eq!(switch.state(), SwitchState::Off);

        switch.toggle();
        assert_eq!(switch.state(), SwitchState::On);
    }

    #[test]
    fn test_switch_builder_pattern() {
        let switch = Switch::new()
            .with_label("Dark mode")
            .on()
            .disabled(true)
            .size(ComponentSize::Large);

        assert_eq!(switch.state(), SwitchState::On);
        assert!(switch.props().disabled);
        assert_eq!(switch.props().size, ComponentSize::Large);
        assert_eq!(switch.props().label, Some("Dark mode".to_string()));
    }

    #[test]
    fn test_switch_convenience_methods() {
        let switch_on = Switch::new().on();
        assert_eq!(switch_on.state(), SwitchState::On);
        assert!(switch_on.is_on());
        assert!(!switch_on.is_off());

        let switch_off = Switch::new().off();
        assert_eq!(switch_off.state(), SwitchState::Off);
        assert!(switch_off.is_off());
        assert!(!switch_off.is_on());
    }

    #[test]
    fn test_switch_validation() {
        let valid_switch = Switch::new().with_label("Valid");
        assert!(valid_switch.validate().is_ok());

        let long_label = "x".repeat(201);
        let invalid_switch = Switch::new().with_label(&long_label);
        assert!(invalid_switch.validate().is_err());
    }

    #[test]
    fn test_switch_trait_implementations() {
        let switch = Switch::new().on();

        // Test SelectionWidget trait
        assert!(switch.is_selected());
        assert!(switch.validate().is_ok());

        // Test StatefulWidget trait
        assert_eq!(switch.current_state(), "On");

        // Test Clone
        let cloned = switch.clone();
        assert_eq!(switch.state(), cloned.state());
    }

    #[test]
    fn test_switch_disabled_behavior() {
        let disabled_switch = Switch::new().disabled(true);
        assert!(disabled_switch.props().disabled);

        // Disabled state should still allow programmatic changes
        let mut switch = disabled_switch.clone();
        switch.toggle();
        assert_eq!(switch.state(), SwitchState::On);
    }

    #[test]
    fn test_switch_sizes() {
        let small = Switch::new().size(ComponentSize::Small);
        let medium = Switch::new().size(ComponentSize::Medium);
        let large = Switch::new().size(ComponentSize::Large);

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
        let dimensions = SwitchDimensions::default();

        // Test Material Design 3 default dimensions
        assert_eq!(dimensions.track_width, 52.0);
        assert_eq!(dimensions.track_height, 32.0);
        assert_eq!(dimensions.thumb_size, 24.0);
        assert_eq!(dimensions.padding, 8.0);
    }

    #[test]
    fn test_switch_dimensions_for_size() {
        let small = SwitchDimensions::for_size(ComponentSize::Small);
        let medium = SwitchDimensions::for_size(ComponentSize::Medium);
        let large = SwitchDimensions::for_size(ComponentSize::Large);

        // Small should be smaller than medium
        assert!(small.track_width < medium.track_width);
        assert!(small.track_height < medium.track_height);
        assert!(small.thumb_size < medium.thumb_size);

        // Large should be larger than medium
        assert!(large.track_width > medium.track_width);
        assert!(large.track_height > medium.track_height);
        assert!(large.thumb_size > medium.thumb_size);
    }

    #[test]
    fn test_switch_dimensions_touch_target() {
        let dimensions = SwitchDimensions::default();
        let touch_target = dimensions.touch_target_size();

        // Touch target should meet Material Design minimum (48dp)
        assert!(touch_target.width >= 48.0);
        assert!(touch_target.height >= 48.0);
    }

    #[test]
    fn test_switch_dimensions_thumb_positions() {
        let dimensions = SwitchDimensions::default();

        let off_position = dimensions.thumb_position_off();
        let on_position = dimensions.thumb_position_on();

        // On position should be to the right of off position
        assert!(on_position > off_position);

        // Positions should be within track bounds
        assert!(off_position >= 0.0);
        assert!(on_position <= dimensions.track_width - dimensions.thumb_size);
    }

    #[test]
    fn test_switch_dimensions_validation() {
        let valid_dimensions = SwitchDimensions::default();
        assert!(valid_dimensions.validate().is_ok());

        let invalid_dimensions = SwitchDimensions {
            track_width: 10.0, // Too small for thumb
            track_height: 20.0,
            thumb_size: 15.0, // Larger than track width
            padding: 4.0,
        };
        assert!(invalid_dimensions.validate().is_err());
    }

    #[test]
    fn test_switch_dimensions_material_compliance() {
        for size in [
            ComponentSize::Small,
            ComponentSize::Medium,
            ComponentSize::Large,
        ] {
            let dimensions = SwitchDimensions::for_size(size);

            // All sizes should meet minimum accessibility requirements
            let touch_target = dimensions.touch_target_size();
            assert!(
                touch_target.width >= 48.0,
                "Touch target width too small for {:?}",
                size
            );
            assert!(
                touch_target.height >= 48.0,
                "Touch target height too small for {:?}",
                size
            );

            // Track should be able to contain thumb
            assert!(dimensions.track_width >= dimensions.thumb_size);
            assert!(dimensions.track_height >= dimensions.thumb_size);
        }
    }
}

#[cfg(test)]
mod custom_switch_widget_tests {
    use super::*;

    #[test]
    fn test_custom_switch_widget_creation() {
        let switch = Switch::new().on();
        let custom_widget = CustomSwitchWidget::new(switch);

        // Test that custom widget preserves switch state
        assert_eq!(custom_widget.switch().state(), SwitchState::On);
    }

    #[test]
    fn test_custom_switch_widget_colors() {
        let switch_off = Switch::new().off();
        let switch_on = Switch::new().on();

        let widget_off = CustomSwitchWidget::new(switch_off);
        let widget_on = CustomSwitchWidget::new(switch_on);

        let track_color_off = widget_off.track_color();
        let track_color_on = widget_on.track_color();
        let thumb_color_off = widget_off.thumb_color();
        let thumb_color_on = widget_on.thumb_color();

        // Colors should be different for different states
        assert_ne!(track_color_off, track_color_on);
        assert_ne!(thumb_color_off, thumb_color_on);
    }

    #[test]
    fn test_custom_switch_widget_thumb_position() {
        let dimensions = SwitchDimensions::default();

        let switch_off = Switch::new().off();
        let switch_on = Switch::new().on();

        let widget_off = CustomSwitchWidget::with_dimensions(switch_off, dimensions.clone());
        let widget_on = CustomSwitchWidget::with_dimensions(switch_on, dimensions);

        let pos_off = widget_off.calculate_thumb_position();
        let pos_on = widget_on.calculate_thumb_position();

        // On position should be to the right of off position
        assert!(pos_on > pos_off);
    }

    #[test]
    fn test_custom_switch_widget_disabled_colors() {
        let enabled_switch = Switch::new().on();
        let disabled_switch = Switch::new().on().disabled(true);

        let enabled_widget = CustomSwitchWidget::new(enabled_switch);
        let disabled_widget = CustomSwitchWidget::new(disabled_switch);

        // Disabled widget should have different (typically muted) colors
        let enabled_track = enabled_widget.track_color();
        let disabled_track = disabled_widget.track_color();

        // This test assumes disabled colors are different
        // In actual implementation, disabled colors would be more muted
        assert_ne!(enabled_track.alpha, disabled_track.alpha);
    }

    #[test]
    fn test_custom_switch_widget_animation_preparation() {
        let switch = Switch::new();
        let custom_widget = CustomSwitchWidget::new(switch);

        // Test that animation configuration is accessible
        let anim_config = custom_widget.switch().animation_config();
        assert!(anim_config.is_some());

        // Test that reduced motion is respected
        let config = anim_config.unwrap();
        if config.reduced_motion {
            assert_eq!(config.duration_ms, 0);
        } else {
            assert!(config.duration_ms > 0);
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_switch_in_settings_form() {
        // Simulate a settings form with multiple switches
        let mut dark_mode = Switch::new().with_label("Dark mode").off();

        let mut notifications = Switch::new().with_label("Enable notifications").on();

        let mut auto_save = Switch::new()
            .with_label("Auto-save changes")
            .on()
            .disabled(true); // Disabled because user doesn't have permission

        // Test initial states
        assert!(dark_mode.is_off());
        assert!(notifications.is_on());
        assert!(auto_save.is_on() && auto_save.props().disabled);

        // User toggles dark mode
        dark_mode.toggle();
        assert!(dark_mode.is_on());

        // User disables notifications
        notifications.set_state(SwitchState::Off);
        assert!(notifications.is_off());

        // Validate all switches
        assert!(dark_mode.validate().is_ok());
        assert!(notifications.validate().is_ok());
        assert!(auto_save.validate().is_ok());
    }

    #[test]
    fn test_switch_with_custom_dimensions() {
        let custom_dimensions = SwitchDimensions {
            track_width: 60.0,
            track_height: 36.0,
            thumb_size: 28.0,
            padding: 10.0,
        };

        let switch = Switch::new().on();
        let custom_widget = CustomSwitchWidget::with_dimensions(switch, custom_dimensions.clone());

        assert_eq!(custom_widget.dimensions().track_width, 60.0);
        assert_eq!(custom_widget.dimensions().track_height, 36.0);
        assert_eq!(custom_widget.dimensions().thumb_size, 28.0);
    }

    #[test]
    fn test_switch_state_transitions() {
        let mut switch = Switch::new();
        let states = vec![SwitchState::Off, SwitchState::On, SwitchState::Off];

        for (i, expected_state) in states.iter().enumerate() {
            if i > 0 {
                switch.toggle();
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
            let switch = Switch::new().size(size);
            let dimensions = SwitchDimensions::for_size(size);

            // Test touch target compliance
            let touch_target = dimensions.touch_target_size();
            assert!(touch_target.width >= 48.0);
            assert!(touch_target.height >= 48.0);

            // Test that switch validates correctly
            assert!(switch.validate().is_ok());
        }
    }

    #[test]
    fn test_switch_error_conditions() {
        // Test validation errors
        let long_label = "x".repeat(201);
        let invalid_switch = Switch::new().with_label(&long_label);

        match invalid_switch.validate() {
            Err(SelectionError::ValidationError(msg)) => {
                assert!(msg.contains("label"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_switch_serialization() {
        let switch = Switch::new()
            .with_label("Test switch")
            .on()
            .size(ComponentSize::Large);

        // Test that state can be serialized (for persistence)
        let state = switch.state();
        assert_eq!(state, SwitchState::On);

        // Test that we can recreate switch with same state
        let recreated = Switch::new()
            .with_label("Test switch")
            .size(ComponentSize::Large)
            .set_state(state);

        assert_eq!(recreated.state(), switch.state());
        assert_eq!(recreated.props().label, switch.props().label);
        assert_eq!(recreated.props().size, switch.props().size);
    }
}
