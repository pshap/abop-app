//! Comprehensive tests for the radio button component
//!
//! Tests cover radio button state management, radio group functionality,
//! type-safe value handling, validation, and builder patterns.

use super::super::common::*;
use super::super::radio::*;

#[cfg(test)]
mod radio_button_tests {
    use super::*;

    #[test]
    fn test_radio_button_creation() {
        let radio = RadioButton::new("option1", "Option 1");
        assert_eq!(radio.value(), &"option1");
        assert_eq!(radio.props().label, Some("Option 1".to_string()));
        assert!(!radio.is_selected());
        assert!(!radio.props().disabled);
    }

    #[test]
    fn test_radio_button_builder() {
        let radio = RadioButton::new("test", "Test")
            .selected(true)
            .disabled(true)
            .size(ComponentSize::Large);

        assert!(radio.is_selected());
        assert!(radio.props().disabled);
        assert_eq!(radio.props().size, ComponentSize::Large);
    }

    #[test]
    fn test_radio_button_state_changes() {
        let mut radio = RadioButton::new("test", "Test");
        assert!(!radio.is_selected());

        radio.set_selected(true);
        assert!(radio.is_selected());

        radio.set_selected(false);
        assert!(!radio.is_selected());
    }

    #[test]
    fn test_radio_button_validation() {
        let radio = RadioButton::new("valid", "Valid Label");
        assert!(radio.validate().is_ok());

        let long_label = "x".repeat(201);
        let invalid_radio = RadioButton::new("invalid", &long_label);
        assert!(invalid_radio.validate().is_err());
    }

    #[test]
    fn test_radio_button_trait_implementations() {
        let radio = RadioButton::new("test", "Test");

        // Test SelectionWidget trait
        assert!(!radio.is_selected());
        assert!(radio.validate().is_ok());

        // Test Clone
        let cloned = radio.clone();
        assert_eq!(radio.value(), cloned.value());
        assert_eq!(radio.is_selected(), cloned.is_selected());
    }
}

#[cfg(test)]
mod radio_group_tests {
    use super::*;

    #[test]
    fn test_radio_group_creation() {
        let group: RadioGroup<&str> = RadioGroup::new();
        assert!(group.selected_value().is_none());
        assert_eq!(group.options().len(), 0);
    }

    #[test]
    fn test_radio_group_add_options() {
        let mut group = RadioGroup::new();
        group.add_option("option1", "Option 1");
        group.add_option("option2", "Option 2");

        assert_eq!(group.options().len(), 2);
        assert!(group.has_option(&"option1"));
        assert!(group.has_option(&"option2"));
        assert!(!group.has_option(&"option3"));
    }

    #[test]
    fn test_radio_group_selection() {
        let mut group = RadioGroup::new();
        group.add_option("a", "Option A");
        group.add_option("b", "Option B");
        group.add_option("c", "Option C");

        // Test selection
        assert!(group.select("a").is_ok());
        assert_eq!(group.selected_value(), Some(&"a"));

        // Test changing selection
        assert!(group.select("b").is_ok());
        assert_eq!(group.selected_value(), Some(&"b"));

        // Test invalid selection
        assert!(group.select("invalid").is_err());
        assert_eq!(group.selected_value(), Some(&"b")); // Should remain unchanged
    }

    #[test]
    fn test_radio_group_clear_selection() {
        let mut group = RadioGroup::new();
        group.add_option("test", "Test");
        group.select("test").unwrap();

        assert!(group.selected_value().is_some());
        group.clear_selection();
        assert!(group.selected_value().is_none());
    }

    #[test]
    fn test_radio_group_builder() {
        let group = RadioGroup::builder()
            .option("a", "Option A")
            .option("b", "Option B")
            .selected("a")
            .build();

        assert_eq!(group.options().len(), 2);
        assert_eq!(group.selected_value(), Some(&"a"));
    }

    #[test]
    fn test_radio_group_validation() {
        let group = RadioGroup::<&str>::new();
        assert!(group.validate().is_ok());

        let mut invalid_group = RadioGroup::new();
        let long_label = "x".repeat(201);
        invalid_group.add_option_with_props("test", ComponentProps::new().with_label(long_label));
        assert!(invalid_group.validate().is_err());
    }

    #[test]
    fn test_radio_group_find_option() {
        let mut group = RadioGroup::new();
        group.add_option("test", "Test Label");

        let option = group.find_option(&"test");
        assert!(option.is_some());
        assert_eq!(
            option.unwrap().props().label,
            Some("Test Label".to_string())
        );

        assert!(group.find_option(&"nonexistent").is_none());
    }

    #[test]
    fn test_radio_group_iteration() {
        let mut group = RadioGroup::new();
        group.add_option("a", "A");
        group.add_option("b", "B");
        group.add_option("c", "C");

        let values: Vec<_> = group.iter().map(|radio| radio.value()).collect();
        assert_eq!(values, vec![&"a", &"b", &"c"]);
    }

    #[test]
    fn test_radio_group_type_safety() {
        // Test with different types
        let mut string_group: RadioGroup<String> = RadioGroup::new();
        string_group.add_option("test".to_string(), "Test");

        let mut int_group: RadioGroup<i32> = RadioGroup::new();
        int_group.add_option(42, "Answer");

        assert!(string_group.select("test".to_string()).is_ok());
        assert!(int_group.select(42).is_ok());
    }

    #[test]
    fn test_radio_group_disabled_options() {
        let mut group = RadioGroup::new();
        group.add_option_with_props(
            "disabled",
            ComponentProps::new().with_label("Disabled").disabled(true),
        );
        group.add_option("enabled", "Enabled");

        // Should be able to select even disabled options (UI prevents interaction)
        assert!(group.select("disabled").is_ok());
        assert!(group.select("enabled").is_ok());
    }
}

#[cfg(test)]
mod radio_group_builder_tests {
    use super::*;

    #[test]
    fn test_builder_empty() {
        let group: RadioGroup<&str> = RadioGroupBuilder::new().build();
        assert_eq!(group.options().len(), 0);
        assert!(group.selected_value().is_none());
    }

    #[test]
    fn test_builder_with_options() {
        let group = RadioGroupBuilder::new()
            .option("first", "First Option")
            .option("second", "Second Option")
            .build();

        assert_eq!(group.options().len(), 2);
        assert!(group.has_option(&"first"));
        assert!(group.has_option(&"second"));
    }

    #[test]
    fn test_builder_with_selection() {
        let group = RadioGroupBuilder::new()
            .option("a", "A")
            .option("b", "B")
            .selected("b")
            .build();

        assert_eq!(group.selected_value(), Some(&"b"));
    }

    #[test]
    fn test_builder_with_props() {
        let props = ComponentProps::new()
            .with_label("Custom Label")
            .size(ComponentSize::Large);

        let group = RadioGroupBuilder::new()
            .option_with_props("test", props)
            .build();

        let option = group.find_option(&"test").unwrap();
        assert_eq!(option.props().label, Some("Custom Label".to_string()));
        assert_eq!(option.props().size, ComponentSize::Large);
    }

    #[test]
    fn test_builder_invalid_selection() {
        let group = RadioGroupBuilder::new()
            .option("valid", "Valid")
            .selected("invalid") // This option doesn't exist
            .build();

        // Builder should ignore invalid selection
        assert!(group.selected_value().is_none());
    }

    #[test]
    fn test_builder_fluent_api() {
        let group = RadioGroupBuilder::new()
            .option("small", "Small")
            .option("medium", "Medium")
            .option("large", "Large")
            .selected("medium")
            .build();

        assert_eq!(group.options().len(), 3);
        assert_eq!(group.selected_value(), Some(&"medium"));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_radio_component_integration() {
        // Test creating a complete radio group for a form
        let mut preferences_group = RadioGroup::new();

        preferences_group.add_option("light", "Light Theme");
        preferences_group.add_option("dark", "Dark Theme");
        preferences_group.add_option("auto", "Auto (System)");

        // Initially no selection
        assert!(preferences_group.selected_value().is_none());

        // User selects dark theme
        preferences_group.select("dark").unwrap();
        assert_eq!(preferences_group.selected_value(), Some(&"dark"));

        // User changes to light theme
        preferences_group.select("light").unwrap();
        assert_eq!(preferences_group.selected_value(), Some(&"light"));

        // Validate the entire group
        assert!(preferences_group.validate().is_ok());
    }

    #[test]
    fn test_radio_with_different_sizes() {
        let small_radio = RadioButton::new("s", "Small").size(ComponentSize::Small);
        let medium_radio = RadioButton::new("m", "Medium").size(ComponentSize::Medium);
        let large_radio = RadioButton::new("l", "Large").size(ComponentSize::Large);

        assert_eq!(small_radio.props().size, ComponentSize::Small);
        assert_eq!(medium_radio.props().size, ComponentSize::Medium);
        assert_eq!(large_radio.props().size, ComponentSize::Large);
    }

    #[test]
    fn test_radio_error_handling() {
        let mut group = RadioGroup::new();

        // Try to select from empty group
        assert!(group.select("anything").is_err());

        // Add option and select it
        group.add_option("valid", "Valid");
        assert!(group.select("valid").is_ok());

        // Try to select non-existent option
        assert!(group.select("invalid").is_err());
    }

    #[test]
    fn test_radio_state_consistency() {
        let mut group = RadioGroup::new();
        group.add_option("a", "A");
        group.add_option("b", "B");

        // Select first option
        group.select("a").unwrap();
        let option_a = group.find_option(&"a").unwrap();
        let option_b = group.find_option(&"b").unwrap();

        assert!(option_a.is_selected());
        assert!(!option_b.is_selected());

        // Select second option
        group.select("b").unwrap();
        let option_a = group.find_option(&"a").unwrap();
        let option_b = group.find_option(&"b").unwrap();

        assert!(!option_a.is_selected());
        assert!(option_b.is_selected());
    }
}
