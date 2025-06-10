//! Phase 3+ Selection Components API Demonstration
//!
//! This file demonstrates the new modular, state-based selection component API
//! with modern builder patterns, comprehensive validation, and future-ready design.

#[cfg(test)]
mod selection_api_demo {
    use crate::styling::material::components::selection::*;

    #[test]
    fn demonstrate_modern_checkbox_api() {
        // Create checkboxes with the new state-based API
        let agreement_checkbox = Checkbox::new("agree")
            .with_label("I agree to the terms and conditions")
            .checked()
            .size(ComponentSize::Large);

        let newsletter_checkbox = Checkbox::new("newsletter")
            .with_label("Subscribe to newsletter")
            .unchecked()
            .size(ComponentSize::Medium);

        // Demonstrate state queries
        assert!(agreement_checkbox.is_checked());
        assert!(!newsletter_checkbox.is_checked());
        assert_eq!(agreement_checkbox.state(), CheckboxState::Checked);
        assert_eq!(newsletter_checkbox.state(), CheckboxState::Unchecked);

        // Demonstrate validation
        assert!(agreement_checkbox.validate().is_ok());
        assert!(newsletter_checkbox.validate().is_ok());

        println!("✅ Modern Checkbox API working correctly");
    }

    #[test]
    fn demonstrate_radio_group_api() {
        // Create a theme selection radio group
        let theme_group = RadioGroup::builder()
            .option("light", "Light Theme")
            .option("dark", "Dark Theme")
            .option("auto", "Auto (System)")
            .selected("auto")
            .build();

        assert_eq!(theme_group.selected_value(), Some(&"auto"));
        assert_eq!(theme_group.options().len(), 3);

        // Demonstrate type safety with different value types
        let priority_group: RadioGroup<i32> = RadioGroup::builder()
            .option(1, "Low Priority")
            .option(2, "Medium Priority")
            .option(3, "High Priority")
            .selected(2)
            .build();

        assert_eq!(priority_group.selected_value(), Some(&2));

        println!("✅ Type-safe Radio Group API working correctly");
    }

    #[test]
    fn demonstrate_switch_api() {
        // Create switches with Material Design 3 compliance
        let notifications_switch = Switch::new()
            .with_label("Enable notifications")
            .on()
            .size(ComponentSize::Medium);

        let dark_mode_switch = Switch::new()
            .with_label("Dark mode")
            .off()
            .size(ComponentSize::Large);

        // Demonstrate state management
        assert!(notifications_switch.is_on());
        assert!(!notifications_switch.is_off());
        assert!(dark_mode_switch.is_off());
        assert!(!dark_mode_switch.is_on());

        // Test Material Design dimensions
        let dimensions = SwitchDimensions::for_size(ComponentSize::Medium);
        assert_eq!(dimensions.track_width, 52.0);
        assert_eq!(dimensions.track_height, 32.0);

        println!("✅ Material Design 3 Switch API working correctly");
    }

    #[test]
    fn demonstrate_chip_collection_api() {
        // Create a filter chip collection for search filters
        let search_filters = ChipCollection::builder(SelectionMode::Multiple)
            .chip(filter_chip("category", "Category"))
            .chip(filter_chip("price", "Price Range"))
            .chip(filter_chip("rating", "Rating"))
            .chip(filter_chip("availability", "In Stock"))
            .selected(&["category", "rating"])
            .build();

        assert_eq!(search_filters.selected_chips().len(), 2);
        assert!(search_filters.is_chip_selected("category"));
        assert!(search_filters.is_chip_selected("rating"));

        // Create an input chip collection for tags
        let tag_collection = ChipCollection::builder(SelectionMode::None)
            .chip(input_chip("rust", "Rust"))
            .chip(input_chip("gui", "GUI"))
            .chip(input_chip("material", "Material Design"))
            .build();

        assert_eq!(tag_collection.chips().len(), 3);
        assert_eq!(tag_collection.selection_mode(), &SelectionMode::None);

        println!("✅ Chip Collection API with multiple variants working correctly");
    }

    #[test]
    fn demonstrate_builder_convenience_functions() {
        // Use the convenient builder functions
        let simple_checkbox = builders::labeled_checkbox("simple", "Simple checkbox");
        let simple_switch = builders::labeled_switch("Enable feature");
        let filter_chips = builders::filter_chips()
            .chip(builders::filter_chip("test", "Test Filter"))
            .build();

        assert_eq!(
            simple_checkbox.props().label,
            Some("Simple checkbox".to_string())
        );
        assert_eq!(
            simple_switch.props().label,
            Some("Enable feature".to_string())
        );
        assert_eq!(filter_chips.chips().len(), 1);

        println!("✅ Convenience builder functions working correctly");
    }

    #[test]
    fn demonstrate_validation_framework() {
        // Test validation utilities
        let checkboxes = vec![
            Checkbox::new("valid1").with_label("Valid checkbox"),
            Checkbox::new("valid2").with_label("Another valid checkbox"),
        ];

        assert!(validation::validate_collection(&checkboxes).is_ok());
        assert!(validation::all_valid(&checkboxes));
        assert!(validation::collect_validation_errors(&checkboxes).is_empty());

        // Test invalid component
        let long_label = "x".repeat(201);
        let invalid_checkbox = Checkbox::new("invalid").with_label(&long_label);
        assert!(invalid_checkbox.validate().is_err());

        println!("✅ Validation framework working correctly");
    }

    #[test]
    fn demonstrate_state_utilities() {
        // Test state conversion utilities
        assert!(state_utils::checkbox_to_bool(CheckboxState::Checked));
        assert!(!state_utils::checkbox_to_bool(CheckboxState::Unchecked));
        assert!(!state_utils::checkbox_to_bool(CheckboxState::Indeterminate));

        assert!(state_utils::switch_to_bool(SwitchState::On));
        assert!(!state_utils::switch_to_bool(SwitchState::Off));

        assert!(state_utils::chip_is_selected(ChipState::Selected));
        assert!(!state_utils::chip_is_selected(ChipState::Unselected));

        println!("✅ State utilities working correctly");
    }

    #[test]
    fn demonstrate_material_design_compliance() {
        // Test Material Design 3 compliance
        for size in [
            ComponentSize::Small,
            ComponentSize::Medium,
            ComponentSize::Large,
        ] {
            let dimensions = SwitchDimensions::for_size(size);
            let touch_target = dimensions.touch_target_size();

            // Ensure minimum touch target size (48dp)
            assert!(touch_target.width >= constants::MIN_TOUCH_TARGET_SIZE);
            assert!(touch_target.height >= constants::MIN_TOUCH_TARGET_SIZE);
        }

        println!("✅ Material Design 3 compliance verified");
    }

    #[test]
    fn demonstrate_phase_4_preparation() {
        // Demonstrate Phase 4 preparation - custom switch widget
        let switch = Switch::new().on().size(ComponentSize::Large);
        let custom_widget = CustomSwitchWidget::new(switch);

        // Test that dimensions are accessible
        assert!(custom_widget.dimensions().track_width > 0.0);

        // Test that colors are calculated
        let track_color = custom_widget.track_color();
        let thumb_color = custom_widget.thumb_color();
        assert_ne!(track_color.r, 0.0); // Basic color sanity check
        assert_ne!(thumb_color.r, 0.0);

        println!("✅ Phase 4 preparation (custom switch widget) ready");
    }

    #[test]
    fn demonstrate_animation_configuration() {
        // Demonstrate animation configuration for Phase 6
        let checkbox = Checkbox::new("animated").checked();
        let animation_config = checkbox.animation_config();

        assert!(animation_config.is_some());
        let config = animation_config.unwrap();

        if config.reduced_motion {
            assert_eq!(config.duration_ms, constants::REDUCED_MOTION_DURATION_MS);
        } else {
            assert_eq!(config.duration_ms, constants::DEFAULT_ANIMATION_DURATION_MS);
        }

        println!("✅ Animation configuration system ready for Phase 6");
    }

    // Helper functions using the convenience builders
    fn filter_chip(id: &str, label: &str) -> Chip {
        builders::filter_chip(id, label)
    }

    fn input_chip(id: &str, label: &str) -> Chip {
        builders::input_chip(id, label)
    }
}

#[cfg(test)]
mod integration_examples {
    use super::*;
    use crate::styling::material::components::selection::*;

    #[test]
    fn example_settings_form() {
        // Example: Settings form with multiple selection components

        // Theme selection (radio group)
        let theme_selection = RadioGroup::builder()
            .option("light", "Light")
            .option("dark", "Dark")
            .option("system", "System")
            .selected("system")
            .build();

        // Feature toggles (switches)
        let auto_save = Switch::new().with_label("Auto-save changes").on();

        let spell_check = Switch::new().with_label("Enable spell check").on();

        // Preferences (checkboxes)
        let show_line_numbers = Checkbox::new("line_numbers")
            .with_label("Show line numbers")
            .checked();

        let wrap_text = Checkbox::new("wrap_text")
            .with_label("Wrap long lines")
            .unchecked();

        // Validate all components
        assert!(theme_selection.validate().is_ok());
        assert!(auto_save.validate().is_ok());
        assert!(spell_check.validate().is_ok());
        assert!(show_line_numbers.validate().is_ok());
        assert!(wrap_text.validate().is_ok());

        // Test state queries
        assert_eq!(theme_selection.selected_value(), Some(&"system"));
        assert!(auto_save.is_on());
        assert!(show_line_numbers.is_checked());

        println!("✅ Settings form example working correctly");
    }

    #[test]
    fn example_search_interface() {
        // Example: Search interface with filter chips

        let search_filters = ChipCollection::builder(SelectionMode::Multiple)
            .chip(builders::filter_chip("category", "Category"))
            .chip(builders::filter_chip("price_range", "Price: $0-50"))
            .chip(builders::filter_chip("brand", "Brand"))
            .chip(builders::filter_chip("rating", "4+ Stars"))
            .chip(builders::filter_chip("availability", "In Stock"))
            .max_selections(3)
            .selected(&["price_range", "rating"])
            .build();

        // Test filter state
        assert_eq!(search_filters.selected_chips().len(), 2);
        assert!(search_filters.is_chip_selected("price_range"));
        assert!(search_filters.is_chip_selected("rating"));
        assert_eq!(search_filters.max_selections(), Some(3));

        println!("✅ Search interface example working correctly");
    }

    #[test]
    fn example_tag_editor() {
        // Example: Tag editor with input chips

        let tag_editor = ChipCollection::builder(SelectionMode::None)
            .chip(builders::input_chip("rust", "Rust"))
            .chip(builders::input_chip("programming", "Programming"))
            .chip(builders::input_chip("gui", "GUI"))
            .chip(builders::input_chip("desktop", "Desktop"))
            .build();

        // Input chips don't have selections, they represent existing tags
        assert_eq!(tag_editor.selected_chips().len(), 0);
        assert_eq!(tag_editor.chips().len(), 4);
        assert_eq!(tag_editor.selection_mode(), &SelectionMode::None);

        println!("✅ Tag editor example working correctly");
    }
}
