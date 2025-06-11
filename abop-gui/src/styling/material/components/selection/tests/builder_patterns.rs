//! Builder pattern and fluent API tests
//!
//! This module tests the builder pattern implementation, fluent API design,
//! and advanced builder features for chip components.

use super::fixtures::{assertion_helpers::*, chip_factory::*, test_data::*};
use crate::styling::material::components::selection::builder::patterns::ComponentBuilder;
use crate::styling::material::components::selection::{
    ChipBuilder, ChipState, ChipVariant, ComponentSize,
};

#[cfg(test)]
mod builder_api_tests {
    use super::*;

    #[test]
    fn test_builder_method_chaining() {
        // Test complete method chain
        let chip = ChipBuilder::filter("Method Chain")
            .selected(true)
            .size(ComponentSize::Large)
            .disabled(false)
            .with_leading_icon("filter")
            .with_trailing_icon("times")
            .with_badge(5)
            .build()
            .unwrap();

        assert_eq!(chip.label(), "Method Chain");
        assert_eq!(chip.variant(), ChipVariant::Filter);
        assert_chip_selected(&chip);
        assert_chip_size(&chip, ComponentSize::Large);
        assert_chip_enabled(&chip);
        assert_chip_has_metadata(&chip, "leading_icon", "filter");
        assert_chip_has_metadata(&chip, "trailing_icon", "times");
        assert_chip_has_metadata(&chip, "badge_count", "5");
    }

    #[test]
    fn test_builder_convenience_constructors() {
        // Test all variant convenience constructors
        let filter = ChipBuilder::filter("Filter").build().unwrap();
        assert_eq!(filter.variant(), ChipVariant::Filter);

        let assist = ChipBuilder::assist("Assist").build().unwrap();
        assert_eq!(assist.variant(), ChipVariant::Assist);

        let input = ChipBuilder::input("Input").build().unwrap();
        assert_eq!(input.variant(), ChipVariant::Input);

        let suggestion = ChipBuilder::suggestion("Suggestion").build().unwrap();
        assert_eq!(suggestion.variant(), ChipVariant::Suggestion);
    }

    #[test]
    fn test_builder_immutability() {
        // Test that builder methods don't mutate the original builder
        let base_builder = ChipBuilder::filter("Base");

        let selected_builder = base_builder.clone().selected(true);
        let unselected_builder = base_builder.clone().selected(false);

        let selected_chip = selected_builder.build().unwrap();
        let unselected_chip = unselected_builder.build().unwrap();

        assert_chip_selected(&selected_chip);
        assert_chip_unselected(&unselected_chip);
    }

    #[test]
    fn test_builder_state_methods() {
        // Test with_state method
        for &state in ALL_CHIP_STATES {
            let chip = ChipBuilder::filter("State Test")
                .with_state(state)
                .build()
                .unwrap();
            assert_eq!(chip.state(), state);
        }

        // Test selected convenience method
        let selected = ChipBuilder::filter("Selected")
            .selected(true)
            .build()
            .unwrap();
        assert_chip_selected(&selected);

        let unselected = ChipBuilder::filter("Unselected")
            .selected(false)
            .build()
            .unwrap();
        assert_chip_unselected(&unselected);
    }

    #[test]
    fn test_builder_property_methods() {
        // Test size setting
        for &size in ALL_COMPONENT_SIZES {
            let chip = ChipBuilder::filter("Size Test").size(size).build().unwrap();
            assert_chip_size(&chip, size);
        }

        // Test disabled setting
        let disabled = ChipBuilder::filter("Disabled")
            .disabled(true)
            .build()
            .unwrap();
        assert_chip_disabled(&disabled);

        let enabled = ChipBuilder::filter("Enabled")
            .disabled(false)
            .build()
            .unwrap();
        assert_chip_enabled(&enabled);
    }

    #[test]
    fn test_builder_advanced_ui_methods() {
        // Test leading icon
        let with_leading = ChipBuilder::filter("Leading Icon")
            .with_leading_icon("star")
            .build()
            .unwrap();
        assert_chip_has_metadata(&with_leading, "leading_icon", "star");

        // Test trailing icon
        let with_trailing = ChipBuilder::filter("Trailing Icon")
            .with_trailing_icon("close")
            .build()
            .unwrap();
        assert_chip_has_metadata(&with_trailing, "trailing_icon", "close");

        // Test badge
        let with_badge = ChipBuilder::filter("Badge").with_badge(42).build().unwrap();
        assert_chip_has_metadata(&with_badge, "badge_count", "42");

        // Test deletable convenience method
        let deletable = ChipBuilder::input("Deletable").deletable().build().unwrap();
        assert_chip_has_metadata(&deletable, "trailing_icon", "times");
    }
}

#[cfg(test)]
mod builder_validation_tests {
    use super::*;

    #[test]
    fn test_builder_validation_on_build() {
        // Valid chip should build successfully
        let valid_result = ChipBuilder::filter("Valid").build();
        assert!(valid_result.is_ok());

        // Invalid chip should fail validation
        let invalid_result = empty_label_chip(ChipVariant::Filter);
        assert_empty_label_error(invalid_result);

        let oversized_result = oversized_label_chip(ChipVariant::Filter);
        assert_label_validation_error(oversized_result, oversized_label().len());
    }

    #[test]
    fn test_builder_validation_methods() {
        // Test validate method
        let valid_builder = ChipBuilder::filter("Valid");
        assert!(valid_builder.validate().is_ok());

        let invalid_builder = ChipBuilder::filter("");
        assert!(invalid_builder.validate().is_err());
    }

    #[test]
    fn test_builder_label_validation() {
        // Test label_validated method (if available)
        let builder = ChipBuilder::filter("Initial");

        // Try to set valid label
        if let Ok(updated) = builder.clone().label_validated("Updated Label") {
            let chip = updated.build().unwrap();
            assert_eq!(chip.label(), "Updated Label");
        }

        // Try to set invalid label
        let invalid_result = builder.label_validated("");
        assert!(invalid_result.is_err());
    }

    #[test]
    fn test_builder_state_validation() {
        // All states should be valid for all variants
        for &variant in ALL_CHIP_VARIANTS {
            for &state in ALL_CHIP_STATES {
                let result = ChipBuilder::new("State Valid", variant)
                    .with_state(state)
                    .build();
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn test_builder_edge_case_validation() {
        // Test maximum length label
        let max_result = max_length_label_chip(ChipVariant::Filter);
        assert!(max_result.is_ok());

        // Test oversized label
        let oversized_result = oversized_label_chip(ChipVariant::Filter);
        assert!(oversized_result.is_err());

        // Test empty label
        let empty_result = empty_label_chip(ChipVariant::Filter);
        assert!(empty_result.is_err());
    }
}

#[cfg(test)]
mod builder_performance_tests {
    use super::*;

    #[test]
    fn test_builder_creation_performance() {
        assert_within_time_limit(
            || {
                for i in 0..100 {
                    let _builder = ChipBuilder::filter(&format!("Chip {}", i));
                }
            },
            50, // 50ms for 100 builders
            "Creating 100 chip builders",
        );
    }

    #[test]
    fn test_builder_chaining_performance() {
        assert_within_time_limit(
            || {
                for i in 0..100 {
                    let _chip = ChipBuilder::filter(&format!("Chip {}", i))
                        .selected(i % 2 == 0)
                        .size(ALL_COMPONENT_SIZES[i % ALL_COMPONENT_SIZES.len()])
                        .disabled(i % 10 == 0)
                        .build()
                        .unwrap();
                }
            },
            100, // 100ms for 100 complex builds
            "Building 100 chips with method chaining",
        );
    }
}

#[cfg(test)]
mod builder_advanced_features_tests {
    use super::*;

    #[test]
    fn test_builder_metadata_support() {
        let chip = ChipBuilder::filter("Metadata Test")
            .with_metadata("custom_key", "custom_value")
            .with_metadata("another_key", "another_value")
            .build()
            .unwrap();

        assert_chip_has_metadata(&chip, "custom_key", "custom_value");
        assert_chip_has_metadata(&chip, "another_key", "another_value");
    }

    #[test]
    fn test_builder_icon_support() {
        // Test various icon combinations
        let icons = ["star", "heart", "home", "user", "settings", "search"];

        for (i, &icon) in icons.iter().enumerate() {
            let chip = ChipBuilder::filter(&format!("Icon Test {}", i))
                .with_leading_icon(icon)
                .build()
                .unwrap();
            assert_chip_has_metadata(&chip, "leading_icon", icon);

            let chip2 = ChipBuilder::filter(&format!("Trailing Test {}", i))
                .with_trailing_icon(icon)
                .build()
                .unwrap();
            assert_chip_has_metadata(&chip2, "trailing_icon", icon);
        }
    }

    #[test]
    fn test_builder_badge_support() {
        // Test various badge values
        let badge_values = [0, 1, 5, 10, 99, 999, 9999];

        for &count in &badge_values {
            let chip = ChipBuilder::filter(&format!("Badge {}", count))
                .with_badge(count)
                .build()
                .unwrap();
            assert_chip_has_metadata(&chip, "badge_count", &count.to_string());
        }
    }

    #[test]
    fn test_builder_complex_combinations() {
        // Test complex UI combinations
        let complex_chip = ChipBuilder::filter("Complex")
            .selected(true)
            .size(ComponentSize::Large)
            .with_leading_icon("filter")
            .with_trailing_icon("close")
            .with_badge(42)
            .disabled(false)
            .build()
            .unwrap();

        assert_chip_selected(&complex_chip);
        assert_chip_size(&complex_chip, ComponentSize::Large);
        assert_chip_enabled(&complex_chip);
        assert_chip_has_metadata(&complex_chip, "leading_icon", "filter");
        assert_chip_has_metadata(&complex_chip, "trailing_icon", "close");
        assert_chip_has_metadata(&complex_chip, "badge_count", "42");
    }

    #[test]
    fn test_builder_conditional_building() {
        // Test building with conditions
        for condition in [true, false] {
            let mut builder = ChipBuilder::filter("Conditional");

            if condition {
                builder = builder.with_leading_icon("star");
            }

            let chip = builder.build().unwrap();

            if condition {
                assert_chip_has_metadata(&chip, "leading_icon", "star");
            } else {
                // Should not have the metadata
                assert!(!chip.props().metadata.contains_key("leading_icon"));
            }
        }
    }
}

#[cfg(test)]
mod builder_error_handling_tests {
    use super::*;

    #[test]
    fn test_builder_error_propagation() {
        // Test that validation errors are properly propagated
        let result = ChipBuilder::filter("")
            .selected(true)
            .size(ComponentSize::Large)
            .build();

        assert!(result.is_err());
        // Error should be about empty label, not about other properties
    }

    #[test]
    fn test_builder_partial_construction() {
        // Test that builder can be partially constructed without errors
        let partial_builder = ChipBuilder::filter("Partial")
            .selected(true)
            .size(ComponentSize::Large);

        // Should be able to continue building
        let chip = partial_builder.disabled(false).build().unwrap();

        assert_eq!(chip.label(), "Partial");
        assert_chip_selected(&chip);
        assert_chip_size(&chip, ComponentSize::Large);
        assert_chip_enabled(&chip);
    }

    #[test]
    fn test_builder_method_order_independence() {
        // Test that method order doesn't affect the result
        let chip1 = ChipBuilder::filter("Order Test")
            .selected(true)
            .size(ComponentSize::Large)
            .disabled(false)
            .build()
            .unwrap();

        let chip2 = ChipBuilder::filter("Order Test")
            .disabled(false)
            .size(ComponentSize::Large)
            .selected(true)
            .build()
            .unwrap();

        // Should be equivalent
        assert_eq!(chip1.label(), chip2.label());
        assert_eq!(chip1.state(), chip2.state());
        assert_eq!(chip1.props().size, chip2.props().size);
        assert_eq!(chip1.props().disabled, chip2.props().disabled);
    }

    #[test]
    fn test_builder_validation_detailed() {
        // Test detailed validation if available
        let valid_builder = ChipBuilder::filter("Valid Chip");

        if let Ok(detailed_result) = valid_builder.build_with_detailed_validation() {
            // Should succeed for valid builder
            assert_eq!(detailed_result.label(), "Valid Chip");
        }

        let invalid_builder = ChipBuilder::filter("");
        let invalid_result = invalid_builder.build_with_detailed_validation();
        assert!(invalid_result.is_err());
    }
}

#[cfg(test)]
mod builder_consistency_tests {
    use super::*;

    #[test]
    fn test_builder_getter_consistency() {
        let builder = ChipBuilder::filter("Getter Test")
            .selected(true)
            .size(ComponentSize::Large);

        // Test that getters return the set values
        assert_eq!(builder.label(), "Getter Test");
        assert_eq!(builder.variant(), ChipVariant::Filter);
        assert_eq!(builder.state(), ChipState::Selected);
        assert_eq!(builder.props().size, ComponentSize::Large);
    }

    #[test]
    fn test_builder_build_consistency() {
        // Multiple builds from same builder should produce equivalent chips
        let builder = ChipBuilder::filter("Consistency")
            .selected(true)
            .size(ComponentSize::Large);

        let chip1 = builder.clone().build().unwrap();
        let chip2 = builder.clone().build().unwrap();

        assert_eq!(chip1.label(), chip2.label());
        assert_eq!(chip1.state(), chip2.state());
        assert_eq!(chip1.variant(), chip2.variant());
        assert_eq!(chip1.props().size, chip2.props().size);
        assert_eq!(chip1.props().disabled, chip2.props().disabled);
    }

    #[test]
    fn test_builder_modification_isolation() {
        let base_builder = ChipBuilder::filter("Base");

        let modified1 = base_builder.clone().selected(true);
        let modified2 = base_builder.clone().size(ComponentSize::Large);

        let chip1 = modified1.build().unwrap();
        let chip2 = modified2.build().unwrap();

        // Modifications should be isolated
        assert_chip_selected(&chip1);
        assert_chip_unselected(&chip2); // Should not be affected by modified1

        assert_chip_size(&chip1, ComponentSize::Medium); // Default size
        assert_chip_size(&chip2, ComponentSize::Large); // Should not be affected by modified1
    }

    #[test]
    fn test_builder_defaults_consistency() {
        // Test that default values are consistent across all variants
        for &variant in ALL_CHIP_VARIANTS {
            let chip = ChipBuilder::new("Default Test", variant).build().unwrap();

            // All chips should have consistent defaults
            assert_chip_unselected(&chip);
            assert_chip_enabled(&chip);
            assert_chip_size(&chip, ComponentSize::Medium);
        }
    }
}
