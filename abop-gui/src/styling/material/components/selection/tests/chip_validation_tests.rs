//! Focused tests for chip validation and error handling
//!
//! This module tests the validation logic for chips including:
//! - Label length validation
//! - State transition validation  
//! - Builder pattern validation
//! - Collection validation

use super::chip_test_helpers::*;
use super::fixtures::assertion_helpers::assert_empty_label_error;
use crate::styling::material::components::selection::ChipBuilder;
use crate::styling::material::components::selection::builder::ComponentBuilder;

#[cfg(test)]
mod validation_tests {
    use super::*;
    #[test]
    fn test_label_length_validation() {
        // Test valid labels
        for &label in VALID_LABELS {
            let result = ChipBuilder::filter(label).build();
            assert!(
                result.is_ok(),
                "Valid label '{label}' should pass validation"
            );
        }

        // Test maximum valid length
        let max_valid = "a".repeat(MAX_LABEL_LENGTH);
        let result = ChipBuilder::filter(&max_valid).build();
        assert!(result.is_ok(), "Maximum length label should be valid"); // Test oversized label
        let oversized = oversized_label();
        let result = ChipBuilder::filter(&oversized).build();
        assert_label_validation_error(&result, MAX_LABEL_LENGTH + 1);
    }

    #[test]
    fn test_empty_label_validation() {
        let result = ChipBuilder::filter("").build();
        assert_empty_label_error(result);
    }

    #[test]
    fn test_variant_specific_validation() {
        // Test that all variants accept the same label constraints
        for variant in all_chip_variants() {
            let valid_result = ChipBuilder::new("Valid", variant).build();
            assert!(
                valid_result.is_ok(),
                "Variant {variant:?} should accept valid labels"
            );

            let invalid_result = ChipBuilder::new(oversized_label(), variant).build();
            assert!(
                invalid_result.is_err(),
                "Variant {variant:?} should reject oversized labels"
            );
        }
    }

    #[test]
    fn test_builder_validation_chain() {
        // Test that validation works throughout the builder chain
        let result = ChipBuilder::filter("Valid")
            .selected(true)
            .disabled(false)
            .build();

        assert!(result.is_ok(), "Valid builder chain should succeed"); // Test that invalid label fails even with other valid settings
        let result = ChipBuilder::filter(oversized_label())
            .selected(true)
            .disabled(false)
            .build();

        assert!(
            result.is_err(),
            "Invalid label should fail regardless of other settings"
        );
    }

    #[test]
    fn test_collection_validation() {
        // Test that collections validate their contained chips
        let collection = filter_chip_collection(VALID_LABELS);
        assert!(
            collection.validate().is_ok(),
            "Collection with valid chips should be valid"
        );

        // Test that individual chip validation is independent
        let long_label = oversized_label();
        let invalid_chip_result = ChipBuilder::filter(&long_label).build();
        assert!(
            invalid_chip_result.is_err(),
            "Individual chip validation should still work"
        );
    }

    #[test]
    fn test_state_validation() {
        use crate::styling::material::components::selection::ChipState;

        // Test all valid state transitions
        for state in [
            ChipState::Unselected,
            ChipState::Selected,
            ChipState::Pressed,
        ] {
            let result = ChipBuilder::filter("Test").with_state(state).build();

            assert!(result.is_ok(), "State {state:?} should be valid");
        }
    }
}
