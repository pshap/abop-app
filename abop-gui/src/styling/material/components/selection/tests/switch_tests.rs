//! Comprehensive tests for the switch component
//!
//! Tests cover switch state management, Material Design 3 compliance,
//! custom widget preparation, and builder patterns.

use super::super::common::*;
use super::super::switch::*;
use super::super::builder::{Switch, SwitchBuilder};

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
        let switch = Switch::off().label("Enable notifications").build_unchecked();
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
    fn test_switch_dimensions_thumb_position() {
        let dimensions = SwitchDimensions::for_size(ComponentSize::Medium);
        
        // Create a custom widget to test thumb positioning
        let widget = CustomSwitchWidget::new(SwitchState::Off, "Test".to_string())
            .size(ComponentSize::Medium);
            
        // Test that thumb position calculation works
        let pos = widget.thumb_position(1.0);
        assert!(pos >= 0.0);
        assert!(pos <= dimensions.track_width - dimensions.thumb_diameter);
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

    #[test]
    fn test_custom_switch_widget_colors() {
        // Create a mock MaterialColors for testing
        let colors = crate::styling::material::colors::MaterialColors::default();
        
        let widget_off = CustomSwitchWidget::new(SwitchState::Off, "Test".to_string());
        let widget_on = CustomSwitchWidget::new(SwitchState::On, "Test".to_string());

        // Create a mock MaterialColors for testing
        let colors = crate::styling::material::colors::MaterialColors::default();
        
        let track_color_off = widget_off.track_color(&colors);
        let track_color_on = widget_on.track_color(&colors);
        let thumb_color_off = widget_off.thumb_color(&colors);
        let thumb_color_on = widget_on.thumb_color(&colors);

        // Colors should be different for different states
        assert_ne!(track_color_off, track_color_on);
        assert_ne!(thumb_color_off, thumb_color_on);
    }

    #[test]
    fn test_custom_switch_widget_thumb_position() {
        let dimensions = SwitchDimensions::for_size(ComponentSize::Medium);

        let widget_off = CustomSwitchWidget::new(SwitchState::Off, "Test".to_string())
            .size(ComponentSize::Medium);
        let widget_on = CustomSwitchWidget::new(SwitchState::On, "Test".to_string())
            .size(ComponentSize::Medium);

        // Test with animation progress at 0% and 100%
        let pos_off = widget_off.thumb_position(0.0);
        let pos_on = widget_on.thumb_position(1.0);

        // On position should be to the right of off position
        assert!(pos_on > pos_off);
    }

    #[test]
    fn test_custom_switch_widget_disabled_colors() {
        // Create a mock MaterialColors for testing
        let colors = crate::styling::material::colors::MaterialColors::default();
        
        let enabled_widget = CustomSwitchWidget::new(SwitchState::On, "Test".to_string());
        let disabled_widget = CustomSwitchWidget::new(SwitchState::On, "Test".to_string())
            .disabled(true);

        // Disabled widget should have different (typically muted) colors
        let enabled_track = enabled_widget.track_color(&colors);
        let disabled_track = disabled_widget.track_color(&colors);

        // Colors should be different for disabled state
        assert_ne!(enabled_track, disabled_track);
    }

    #[test]
    fn test_custom_switch_widget_animation_config() {
        let widget = CustomSwitchWidget::new(SwitchState::Off, "Test".to_string())
            .animation(AnimationConfig {
                duration: std::time::Duration::from_millis(300),
                enabled: true,
                respect_reduced_motion: true,
                easing: EasingCurve::Standard,
            });

        // Test that animation configuration is accessible
        assert_eq!(widget.animation_config.duration.as_millis(), 300);
        assert!(widget.animation_config.enabled);
        assert!(widget.animation_config.respect_reduced_motion);
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

        let mut auto_save = Switch::on()
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
        // Create a custom widget with specific dimensions
        let widget = CustomSwitchWidget::new(SwitchState::On, "Test".to_string())
            .size(ComponentSize::Large);
            
        // Get the dimensions for the large size
        let dimensions = SwitchDimensions::for_size(ComponentSize::Large);
        
        // Verify dimensions match the expected values
        assert_eq!(widget.dimensions.track_width, dimensions.track_width);
        assert_eq!(widget.dimensions.track_height, dimensions.track_height);
        assert_eq!(widget.dimensions.thumb_diameter, dimensions.thumb_diameter);
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
