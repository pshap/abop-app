//! Basic chip functionality tests
//!
//! This module tests the core functionality of chip components including
//! creation, state management, and basic property validation.

use super::chip_test_helpers::{
    ALL_CHIP_STATES, MAX_LABEL_LENGTH, VALID_LABELS, all_state_chips, all_variant_chips,
    assert_chip_basics, assert_chip_disabled, assert_chip_enabled, assert_chip_has_metadata,
    assert_chip_pressed, assert_chip_selected, assert_chip_size, assert_chip_unselected,
    assert_md3_height_compliance, assert_md3_touch_target_compliance, assert_within_time_limit,
    max_length_label, selected_filter_chip, test_chip,
};
use super::fixtures::{
    chip_factory::{
        assist_chip, deletable_input_chip, disabled_chip, filter_chip, input_chip, pressed_chip,
        selected_chip, sized_chip, suggestion_chip,
    },
    test_data::{ALL_CHIP_VARIANTS, ALL_COMPONENT_SIZES},
};
use crate::styling::material::components::selection::builder::ComponentBuilder;
use crate::styling::material::components::selection::common::{
    EnhancedSelectionWidget, SelectionWidget,
};
use crate::styling::material::components::selection::{
    ChipBuilder, ChipState, ChipVariant, ComponentSize,
};

#[cfg(test)]
mod chip_creation_tests {
    use super::*;
    #[test]
    fn test_basic_chip_creation() {
        for &variant in ALL_CHIP_VARIANTS {
            let chip = test_chip("Test Chip", variant);
            assert_chip_basics(&chip, "Test Chip", variant);
        }
    }

    #[test]
    fn test_chip_creation_with_label() {
        let chip = filter_chip("Filter Test");
        assert_eq!(chip.props().label, Some("Filter Test".to_string()));
        assert_eq!(chip.label(), "Filter Test");
    }

    #[test]
    fn test_variant_specific_creation() {
        let filter = filter_chip("Filter");
        assert_eq!(filter.variant(), ChipVariant::Filter);

        let assist = assist_chip("Assist");
        assert_eq!(assist.variant(), ChipVariant::Assist);

        let input = input_chip("Input");
        assert_eq!(input.variant(), ChipVariant::Input);

        let suggestion = suggestion_chip("Suggestion");
        assert_eq!(suggestion.variant(), ChipVariant::Suggestion);
    }

    #[test]
    fn test_chip_with_all_sizes() {
        for &variant in ALL_CHIP_VARIANTS {
            for &size in ALL_COMPONENT_SIZES {
                let chip = sized_chip("Test", variant, size);
                assert_chip_size(&chip, size);
                assert_md3_height_compliance(&chip);
                assert_md3_touch_target_compliance(&chip);
            }
        }
    }
    #[test]
    fn test_chip_creation_performance() {
        assert_within_time_limit(
            || {
                for i in 0..1000 {
                    let _chip = test_chip(&format!("Chip {i}"), ChipVariant::Filter);
                }
            },
            100, // 100ms for 1000 chips
            "Creating 1000 chips",
        );
    }
}

#[cfg(test)]
mod chip_state_tests {
    use super::*;
    #[test]
    fn test_default_state() {
        for &variant in ALL_CHIP_VARIANTS {
            let chip = test_chip("Default State", variant);
            assert_chip_unselected(&chip);
        }
    }

    #[test]
    fn test_selected_state() {
        for &variant in ALL_CHIP_VARIANTS {
            let chip = selected_chip("Selected State", variant);
            assert_chip_selected(&chip);
        }
    }

    #[test]
    fn test_pressed_state() {
        for &variant in ALL_CHIP_VARIANTS {
            let chip = pressed_chip("Pressed State", variant);
            assert_chip_pressed(&chip);
        }
    }

    #[test]
    fn test_state_transitions() {
        let mut chip = test_chip("State Test", ChipVariant::Filter);
        assert_chip_unselected(&chip);

        // Test selection
        chip.set_state(ChipState::Selected);
        assert_chip_selected(&chip);

        // Test deselection
        chip.set_state(ChipState::Unselected);
        assert_chip_unselected(&chip);

        // Test pressed state
        chip.set_state(ChipState::Pressed);
        assert_chip_pressed(&chip);
    }

    #[test]
    fn test_all_state_combinations() {
        for &variant in ALL_CHIP_VARIANTS {
            let chips = all_state_chips("State Test", variant);

            assert_eq!(chips.len(), ALL_CHIP_STATES.len());

            for (chip, &expected_state) in chips.iter().zip(ALL_CHIP_STATES.iter()) {
                assert_eq!(chip.state(), expected_state);
            }
        }
    }

    #[test]
    fn test_is_selected_consistency() {
        for &variant in ALL_CHIP_VARIANTS {
            // Test unselected
            let unselected = test_chip("Unselected", variant);
            assert!(!unselected.is_selected());
            assert_eq!(unselected.state(), ChipState::Unselected);

            // Test selected
            let selected = selected_chip("Selected", variant);
            assert!(selected.is_selected());
            assert_eq!(selected.state(), ChipState::Selected);

            // Test pressed (should not be considered selected)
            let pressed = pressed_chip("Pressed", variant);
            assert!(!pressed.is_selected());
            assert_eq!(pressed.state(), ChipState::Pressed);
        }
    }
}

#[cfg(test)]
mod chip_builder_tests {
    use super::*;

    #[test]
    fn test_builder_pattern_basic() {
        let chip = ChipBuilder::filter("Builder Test")
            .selected(true)
            .disabled(false)
            .size(ComponentSize::Large)
            .build()
            .unwrap();

        assert_chip_selected(&chip);
        assert_chip_enabled(&chip);
        assert_chip_size(&chip, ComponentSize::Large);
        assert_eq!(chip.label(), "Builder Test");
        assert_eq!(chip.variant(), ChipVariant::Filter);
    }

    #[test]
    fn test_builder_fluent_api() {
        let chip = ChipBuilder::input("Fluent API")
            .selected(true)
            .size(ComponentSize::Small)
            .disabled(false)
            .build()
            .unwrap();

        assert_eq!(chip.label(), "Fluent API");
        assert_eq!(chip.variant(), ChipVariant::Input);
        assert_chip_selected(&chip);
        assert_chip_size(&chip, ComponentSize::Small);
        assert_chip_enabled(&chip);
    }

    #[test]
    fn test_builder_state_methods() {
        // Test with_state method
        let chip1 = ChipBuilder::assist("State Method")
            .with_state(ChipState::Pressed)
            .build()
            .unwrap();
        assert_chip_pressed(&chip1);

        // Test selected method
        let chip2 = ChipBuilder::assist("Selected Method")
            .selected(true)
            .build()
            .unwrap();
        assert_chip_selected(&chip2);

        // Test unselected by default
        let chip3 = ChipBuilder::assist("Default State").build().unwrap();
        assert_chip_unselected(&chip3);
    }

    #[test]
    fn test_convenience_constructors() {
        let filter = ChipBuilder::filter("Filter Chip").build().unwrap();
        assert_eq!(filter.variant(), ChipVariant::Filter);

        let assist = ChipBuilder::assist("Assist Chip").build().unwrap();
        assert_eq!(assist.variant(), ChipVariant::Assist);

        let input = ChipBuilder::input("Input Chip").build().unwrap();
        assert_eq!(input.variant(), ChipVariant::Input);

        let suggestion = ChipBuilder::suggestion("Suggestion Chip").build().unwrap();
        assert_eq!(suggestion.variant(), ChipVariant::Suggestion);
    }

    #[test]
    fn test_builder_property_setters() {
        let chip = ChipBuilder::filter("Property Test")
            .disabled(true)
            .size(ComponentSize::Large)
            .build()
            .unwrap();

        assert_chip_disabled(&chip);
        assert_chip_size(&chip, ComponentSize::Large);
    }

    #[test]
    fn test_builder_method_chaining() {
        // Test that all methods return Self for chaining
        let _builder = ChipBuilder::suggestion("Chain Test")
            .selected(true)
            .disabled(false)
            .size(ComponentSize::Medium)
            .with_state(ChipState::Selected);

        // If this compiles, method chaining works
    }
}

#[cfg(test)]
mod chip_variant_tests {
    use super::*;

    #[test]
    fn test_all_variants_have_unique_behavior() {
        // Each variant should be distinct
        let variants = all_variant_chips("Variant Test");

        assert_eq!(variants.len(), ALL_CHIP_VARIANTS.len());

        for (chip, &expected_variant) in variants.iter().zip(ALL_CHIP_VARIANTS.iter()) {
            assert_eq!(chip.variant(), expected_variant);
            assert_eq!(chip.label(), "Variant Test");
        }
    }

    #[test]
    fn test_variant_default_behaviors() {
        for &variant in ALL_CHIP_VARIANTS {
            let chip = test_chip("Behavior Test", variant);

            // All variants start unselected by default
            assert_chip_unselected(&chip);

            // All variants start enabled by default
            assert_chip_enabled(&chip);

            // All variants have medium size by default
            assert_chip_size(&chip, ComponentSize::Medium);
        }
    }

    #[test]
    fn test_filter_chip_behavior() {
        let chip = filter_chip("Filter Test");
        assert_eq!(chip.variant(), ChipVariant::Filter);

        // Filter chips should be selectable
        let selected_filter = selected_filter_chip("Selected Filter");
        assert_chip_selected(&selected_filter);
    }

    #[test]
    fn test_input_chip_behavior() {
        let chip = input_chip("Input Test");
        assert_eq!(chip.variant(), ChipVariant::Input);

        // Test deletable input chip
        let deletable = deletable_input_chip("Deletable");
        assert_chip_has_metadata(&deletable, "trailing_icon", "times");
    }

    #[test]
    fn test_assist_chip_behavior() {
        let chip = assist_chip("Assist Test");
        assert_eq!(chip.variant(), ChipVariant::Assist);

        // Assist chips are typically not selectable in most implementations
        assert_chip_unselected(&chip);
    }

    #[test]
    fn test_suggestion_chip_behavior() {
        let chip = suggestion_chip("Suggestion Test");
        assert_eq!(chip.variant(), ChipVariant::Suggestion);

        // Suggestion chips are typically used for single actions
        assert_chip_unselected(&chip);
    }
}

#[cfg(test)]
mod chip_properties_tests {
    use super::*;

    #[test]
    fn test_size_properties() {
        for &size in ALL_COMPONENT_SIZES {
            for &variant in ALL_CHIP_VARIANTS {
                let chip = sized_chip("Size Test", variant, size);

                // Test size property
                assert_chip_size(&chip, size);

                // Test Material Design compliance
                assert_md3_height_compliance(&chip);
                assert_md3_touch_target_compliance(&chip);
            }
        }
    }

    #[test]
    fn test_disabled_property() {
        let enabled_chip = test_chip("Enabled", ChipVariant::Filter);
        assert_chip_enabled(&enabled_chip);

        let disabled_chip = disabled_chip("Disabled", ChipVariant::Filter);
        assert_chip_disabled(&disabled_chip);
    }

    #[test]
    fn test_label_property() {
        for label in VALID_LABELS {
            let chip = test_chip(label, ChipVariant::Filter);
            assert_eq!(chip.label(), *label);
            assert_eq!(chip.props().label, Some(label.to_string()));
        }
    }

    #[test]
    fn test_props_consistency() {
        let chip = ChipBuilder::filter("Props Test")
            .size(ComponentSize::Large)
            .disabled(true)
            .build()
            .unwrap();

        let props = chip.props();
        assert_eq!(props.size, ComponentSize::Large);
        assert!(props.disabled);
        assert_eq!(props.label, Some("Props Test".to_string()));
    }

    #[test]
    fn test_component_metadata() {
        let chip = ChipBuilder::filter("Metadata Test")
            .with_leading_icon("filter")
            .build()
            .unwrap();

        assert_chip_has_metadata(&chip, "leading_icon", "filter");
    }
}

#[cfg(test)]
mod chip_trait_implementations_tests {
    use super::*;

    #[test]
    fn test_clone_implementation() {
        let original = selected_chip("Original", ChipVariant::Filter);
        let cloned = original.clone();

        assert_eq!(original.label(), cloned.label());
        assert_eq!(original.state(), cloned.state());
        assert_eq!(original.variant(), cloned.variant());
        assert_eq!(original.props().size, cloned.props().size);
        assert_eq!(original.props().disabled, cloned.props().disabled);
    }

    #[test]
    fn test_debug_implementation() {
        let chip = test_chip("Debug Test", ChipVariant::Filter);
        let debug_string = format!("{chip:?}");

        // Debug output should contain key information
        assert!(debug_string.contains("Debug Test"));
        // Note: Exact format depends on implementation
    }

    #[test]
    fn test_selection_widget_trait() {
        for &variant in ALL_CHIP_VARIANTS {
            let unselected = test_chip("Trait Test", variant);
            assert!(!unselected.is_selected());

            let selected = selected_chip("Trait Test", variant);
            assert!(selected.is_selected());
        }
    }

    #[test]
    fn test_validation_trait() {
        for &variant in ALL_CHIP_VARIANTS {
            let valid_chip = test_chip("Valid", variant);
            assert!(valid_chip.validate().is_ok());
        }
    }
}

#[cfg(test)]
mod chip_edge_cases_tests {
    use super::*;

    #[test]
    fn test_minimum_length_labels() {
        // Test single character labels
        let chip = test_chip("A", ChipVariant::Filter);
        assert_eq!(chip.label(), "A");
    }

    #[test]
    fn test_maximum_length_labels() {
        let max_label = max_length_label();
        let result = ChipBuilder::filter(&max_label).build();
        assert!(result.is_ok());

        let chip = result.unwrap();
        assert_eq!(chip.label(), max_label);
        assert_eq!(chip.label().len(), MAX_LABEL_LENGTH);
    }

    #[test]
    fn test_unicode_labels() {
        let unicode_labels = ["🚀 Rocket", "测试标签", "Пример", "テスト", "مثال"];

        for label in &unicode_labels {
            if label.len() <= MAX_LABEL_LENGTH {
                let chip = test_chip(label, ChipVariant::Filter);
                assert_eq!(chip.label(), *label);
            }
        }
    }

    #[test]
    fn test_special_character_labels() {
        let special_labels = [
            "Test-Label",
            "Test_Label",
            "Test.Label",
            "Test Label",
            "Test/Label",
            "Test\\Label",
            "Test@Label",
            "Test#Label",
        ];

        for label in &special_labels {
            if !label.trim().is_empty() && label.len() <= MAX_LABEL_LENGTH {
                let chip = test_chip(label, ChipVariant::Filter);
                assert_eq!(chip.label(), *label);
            }
        }
    }

    #[test]
    fn test_state_edge_cases() {
        // Test rapid state changes
        let mut chip = test_chip("State Changes", ChipVariant::Filter);

        for _ in 0..100 {
            chip.set_state(ChipState::Selected);
            assert_chip_selected(&chip);

            chip.set_state(ChipState::Unselected);
            assert_chip_unselected(&chip);

            chip.set_state(ChipState::Pressed);
            assert_chip_pressed(&chip);
        }
    }
}
