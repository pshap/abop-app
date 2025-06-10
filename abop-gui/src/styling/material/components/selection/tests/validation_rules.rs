//! Validation Rules Tests
//! 
//! Comprehensive validation testing for chip properties, constraints, and business rules.
//! Tests input validation, data constraints, and Material Design 3 compliance.

use super::fixtures::{
    chip_factory::*,
    collection_factory::*,
    assertion_helpers::*,
    test_data::*,
};
use crate::styling::material::components::selection::chip::{
    core::{Chip, ChipState, ChipVariant, MAX_CHIP_LABEL_LENGTH},
    validation::{ValidationError, ValidationResult, ChipValidator},
    theming::MD3_CONSTRAINTS,
};
use test_case::test_case;
use quickcheck::{quickcheck, TestResult};
use proptest::prelude::*;

#[cfg(test)]
mod label_validation_tests {
    use super::*;

    #[test]
    fn test_valid_label_lengths() {
        let valid_labels = [
            "A",                                    // Single character
            "Valid",                               // Normal length
            "A".repeat(MAX_CHIP_LABEL_LENGTH),     // Maximum length
        ];
        
        for label in &valid_labels {
            let chip = Chip::new(label, ChipVariant::Filter);
            assert_eq!(chip.label, *label);
            assert!(ChipValidator::validate_label(label).is_ok());
        }
    }

    #[test]
    fn test_invalid_label_lengths() {
        let invalid_labels = [
            "",                                         // Empty
            "A".repeat(MAX_CHIP_LABEL_LENGTH + 1),      // Too long
            "A".repeat(MAX_CHIP_LABEL_LENGTH + 100),    // Way too long
        ];
        
        for label in &invalid_labels {
            let result = ChipValidator::validate_label(label);
            assert!(result.is_err());
            
            match result.unwrap_err() {
                ValidationError::InvalidLabelLength { .. } => {},
                _ => panic!("Expected InvalidLabelLength error"),
            }
        }
    }

    #[test_case("Valid Label"; "normal text")]
    #[test_case("Label with 123"; "with numbers")]
    #[test_case("Label-with_symbols"; "with allowed symbols")]
    #[test_case("Lábel with Ünicöde"; "with unicode")]
    fn test_valid_label_characters(label: &str) {
        if label.len() <= MAX_CHIP_LABEL_LENGTH {
            let result = ChipValidator::validate_label(label);
            assert!(result.is_ok(), "Label '{}' should be valid", label);
        }
    }

    #[test_case("\t"; "tab character")]
    #[test_case("\n"; "newline")]
    #[test_case("\r"; "carriage return")]
    #[test_case("   "; "only whitespace")]
    fn test_invalid_label_characters(label: &str) {
        let result = ChipValidator::validate_label(label);
        assert!(result.is_err(), "Label '{}' should be invalid", label);
    }

    #[test]
    fn test_label_trimming() {
        let labels_with_whitespace = [
            " Leading space",
            "Trailing space ",
            " Both sides ",
            "\tTab prefix",
            "Tab suffix\t",
        ];
        
        for original in &labels_with_whitespace {
            let trimmed = original.trim();
            if !trimmed.is_empty() && trimmed.len() <= MAX_CHIP_LABEL_LENGTH {
                let chip = Chip::new(original, ChipVariant::Filter);
                assert_eq!(chip.label, trimmed);
            }
        }
    }

    #[quickcheck]
    fn prop_label_validation_consistency(label: String) -> TestResult {
        if label.is_empty() || label.len() > MAX_CHIP_LABEL_LENGTH {
            return TestResult::discard();
        }
        
        let validation_result = ChipValidator::validate_label(&label);
        let chip_creation_result = std::panic::catch_unwind(|| {
            Chip::new(&label, ChipVariant::Filter)
        });
        
        TestResult::from_bool(
            validation_result.is_ok() == chip_creation_result.is_ok()
        )
    }
}

#[cfg(test)]
mod id_validation_tests {
    use super::*;

    #[test]
    fn test_unique_id_generation() {
        let mut ids = std::collections::HashSet::new();
        
        for _ in 0..1000 {
            let chip = create_test_chip("test", ChipVariant::Filter);
            assert!(ids.insert(chip.id.clone()), "Duplicate ID generated: {}", chip.id);
        }
    }

    #[test]
    fn test_custom_id_validation() {
        let valid_ids = [
            "valid_id",
            "id123",
            "component-id",
            "ID_WITH_CAPS",
            "123-456-789",
        ];
        
        for id in &valid_ids {
            let result = ChipValidator::validate_id(id);
            assert!(result.is_ok(), "ID '{}' should be valid", id);
        }
    }

    #[test]
    fn test_invalid_custom_ids() {
        let invalid_ids = [
            "",                     // Empty
            " ",                    // Whitespace only
            "id with spaces",       // Contains spaces
            "id\twith\ttabs",      // Contains tabs
            "id\nwith\nnewlines",  // Contains newlines
            "id@with!symbols",     // Invalid symbols
        ];
        
        for id in &invalid_ids {
            let result = ChipValidator::validate_id(id);
            assert!(result.is_err(), "ID '{}' should be invalid", id);
        }
    }

    #[test]
    fn test_id_length_constraints() {
        let too_long_id = "a".repeat(256); // Assuming max length of 255
        let result = ChipValidator::validate_id(&too_long_id);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ValidationError::InvalidIdLength { .. } => {},
            _ => panic!("Expected InvalidIdLength error"),
        }
    }

    #[quickcheck]
    fn prop_id_uniqueness(count: u8) -> bool {
        let count = count.min(100) as usize; // Limit for performance
        let mut ids = std::collections::HashSet::new();
        
        for _ in 0..count {
            let chip = create_test_chip("test", ChipVariant::Filter);
            ids.insert(chip.id);
        }
        
        ids.len() == count
    }
}

#[cfg(test)]
mod variant_validation_tests {
    use super::*;

    #[test]
    fn test_all_variants_are_valid() {
        let variants = [
            ChipVariant::Action,
            ChipVariant::Filter,
            ChipVariant::Input,
            ChipVariant::Suggestion,
        ];
        
        for variant in &variants {
            let chip = Chip::new("test", variant.clone());
            assert_eq!(chip.variant, *variant);
            assert!(ChipValidator::validate_variant(variant).is_ok());
        }
    }

    #[test]
    fn test_variant_specific_constraints() {
        // Action chips should support callbacks
        let action_chip = Chip::new("action", ChipVariant::Action);
        assert!(action_chip.can_have_callback());
        
        // Filter chips should support selection
        let filter_chip = Chip::new("filter", ChipVariant::Filter);
        assert!(filter_chip.supports_selection());
        
        // Input chips should support deletion
        let input_chip = Chip::new("input", ChipVariant::Input);
        assert!(input_chip.supports_deletion());
        
        // Suggestion chips should support activation
        let suggestion_chip = Chip::new("suggestion", ChipVariant::Suggestion);
        assert!(suggestion_chip.supports_activation());
    }

    #[test]
    fn test_variant_behavioral_constraints() {
        // Test that variants enforce their specific behaviors
        let mut filter_chip = Chip::new("filter", ChipVariant::Filter);
        assert!(filter_chip.set_selectable(true).is_ok());
        
        let mut action_chip = Chip::new("action", ChipVariant::Action);
        // Action chips typically don't support persistent selection
        if !action_chip.variant.supports_persistent_selection() {
            assert!(action_chip.set_selectable(false).is_ok());
        }
    }
}

#[cfg(test)]
mod state_validation_tests {
    use super::*;

    #[test]
    fn test_valid_state_transitions() {
        let valid_transitions = [
            (ChipState::Enabled, ChipState::Selected),
            (ChipState::Selected, ChipState::Enabled),
            (ChipState::Enabled, ChipState::Disabled),
            (ChipState::Disabled, ChipState::Enabled),
            (ChipState::Enabled, ChipState::Hover),
            (ChipState::Hover, ChipState::Enabled),
            (ChipState::Selected, ChipState::Hover),
            (ChipState::Hover, ChipState::Selected),
        ];
        
        for (from_state, to_state) in &valid_transitions {
            let mut chip = Chip::new("test", ChipVariant::Filter);
            chip.state = from_state.clone();
            
            let result = chip.transition_to(to_state.clone());
            assert!(result.is_ok(), "Transition from {:?} to {:?} should be valid", from_state, to_state);
        }
    }

    #[test]
    fn test_invalid_state_transitions() {
        let invalid_transitions = [
            (ChipState::Disabled, ChipState::Selected), // Can't select disabled
            (ChipState::Disabled, ChipState::Hover),    // Can't hover disabled
        ];
        
        for (from_state, to_state) in &invalid_transitions {
            let mut chip = Chip::new("test", ChipVariant::Filter);
            chip.state = from_state.clone();
            
            let result = chip.transition_to(to_state.clone());
            assert!(result.is_err(), "Transition from {:?} to {:?} should be invalid", from_state, to_state);
        }
    }

    #[test]
    fn test_state_consistency_validation() {
        let mut chip = Chip::new("test", ChipVariant::Filter);
        
        // Set chip to selected state
        chip.state = ChipState::Selected;
        
        // Validate that the chip's internal state is consistent
        let validation_result = ChipValidator::validate_state_consistency(&chip);
        assert!(validation_result.is_ok());
        
        // If chip is selected, it should also be marked as selected in properties
        if chip.state == ChipState::Selected {
            assert!(chip.is_selected());
        }
    }

    #[quickcheck]
    fn prop_state_transition_symmetry(initial_state: ChipState, target_state: ChipState) -> TestResult {
        let mut chip = Chip::new("test", ChipVariant::Filter);
        chip.state = initial_state.clone();
        
        let forward_result = chip.transition_to(target_state.clone());
        
        if forward_result.is_ok() {
            let reverse_result = chip.transition_to(initial_state.clone());
            TestResult::from_bool(reverse_result.is_ok())
        } else {
            TestResult::discard()
        }
    }
}

#[cfg(test)]
mod material_design_validation_tests {
    use super::*;

    #[test]
    fn test_md3_elevation_constraints() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        // MD3 specifies specific elevation values for chips
        assert!(chip.elevation() >= MD3_CONSTRAINTS.min_elevation);
        assert!(chip.elevation() <= MD3_CONSTRAINTS.max_elevation);
        assert!(MD3_CONSTRAINTS.allowed_elevations.contains(&chip.elevation()));
    }

    #[test]
    fn test_md3_corner_radius_constraints() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        let corner_radius = chip.corner_radius();
        assert!(corner_radius >= MD3_CONSTRAINTS.min_corner_radius);
        assert!(corner_radius <= MD3_CONSTRAINTS.max_corner_radius);
        
        // Corner radius should be divisible by 4 (MD3 constraint)
        assert_eq!(corner_radius % 4.0, 0.0);
    }

    #[test]
    fn test_md3_padding_constraints() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        let padding = chip.padding();
        assert!(padding.horizontal >= MD3_CONSTRAINTS.min_horizontal_padding);
        assert!(padding.horizontal <= MD3_CONSTRAINTS.max_horizontal_padding);
        assert!(padding.vertical >= MD3_CONSTRAINTS.min_vertical_padding);
        assert!(padding.vertical <= MD3_CONSTRAINTS.max_vertical_padding);
    }

    #[test]
    fn test_md3_minimum_touch_target() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        let touch_target = chip.touch_target_size();
        assert!(touch_target.width >= MD3_CONSTRAINTS.min_touch_target_size);
        assert!(touch_target.height >= MD3_CONSTRAINTS.min_touch_target_size);
    }

    #[test]
    fn test_md3_color_contrast_requirements() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        let foreground = chip.foreground_color();
        let background = chip.background_color();
        let contrast_ratio = calculate_contrast_ratio(foreground, background);
        
        // WCAG AA compliance requires 4.5:1 for normal text
        assert!(contrast_ratio >= MD3_CONSTRAINTS.min_contrast_ratio);
    }

    #[test]
    fn test_md3_typography_constraints() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        let typography = chip.typography();
        assert_eq!(typography.font_family, MD3_CONSTRAINTS.required_font_family);
        assert!(MD3_CONSTRAINTS.allowed_font_sizes.contains(&typography.font_size));
        assert!(typography.font_weight >= MD3_CONSTRAINTS.min_font_weight);
        assert!(typography.font_weight <= MD3_CONSTRAINTS.max_font_weight);
    }

    #[test]
    fn test_md3_animation_timing() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        let animation_config = chip.animation_config();
        assert!(animation_config.duration >= MD3_CONSTRAINTS.min_animation_duration);
        assert!(animation_config.duration <= MD3_CONSTRAINTS.max_animation_duration);
        assert!(MD3_CONSTRAINTS.allowed_easing_functions.contains(&animation_config.easing));
    }
}

#[cfg(test)]
mod accessibility_validation_tests {
    use super::*;

    #[test]
    fn test_accessibility_labels() {
        let chip = Chip::new("Filter Option", ChipVariant::Filter);
        
        let a11y_label = chip.accessibility_label();
        assert!(!a11y_label.is_empty());
        assert!(a11y_label.contains("Filter Option"));
        
        if chip.variant == ChipVariant::Filter {
            assert!(a11y_label.contains("filter") || a11y_label.contains("Filter"));
        }
    }

    #[test]
    fn test_screen_reader_announcements() {
        let mut chip = Chip::new("Test Chip", ChipVariant::Filter);
        
        // Test state change announcements
        chip.state = ChipState::Selected;
        let announcement = chip.screen_reader_announcement();
        assert!(announcement.contains("selected") || announcement.contains("Selected"));
        
        chip.state = ChipState::Disabled;
        let announcement = chip.screen_reader_announcement();
        assert!(announcement.contains("disabled") || announcement.contains("Disabled"));
    }

    #[test]
    fn test_keyboard_navigation_support() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        assert!(chip.supports_keyboard_navigation());
        assert!(chip.tab_index().is_some());
        
        let key_handlers = chip.keyboard_handlers();
        assert!(key_handlers.contains_key("Enter"));
        assert!(key_handlers.contains_key("Space"));
        
        if chip.supports_deletion() {
            assert!(key_handlers.contains_key("Delete") || key_handlers.contains_key("Backspace"));
        }
    }

    #[test]
    fn test_aria_attributes() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        let aria_attrs = chip.aria_attributes();
        assert!(aria_attrs.contains_key("role"));
        assert!(aria_attrs.contains_key("aria-label"));
        
        if chip.supports_selection() {
            assert!(aria_attrs.contains_key("aria-selected"));
        }
        
        if chip.state == ChipState::Disabled {
            assert!(aria_attrs.contains_key("aria-disabled"));
            assert_eq!(aria_attrs["aria-disabled"], "true");
        }
    }

    #[test]
    fn test_focus_management() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        // Chip should be focusable when enabled
        if chip.state != ChipState::Disabled {
            assert!(chip.is_focusable());
            assert!(chip.focus_ring_visible_on_focus());
        }
        
        // Disabled chips should not be focusable
        let mut disabled_chip = chip.clone();
        disabled_chip.state = ChipState::Disabled;
        assert!(!disabled_chip.is_focusable());
    }
}

#[cfg(test)]
mod business_rule_validation_tests {
    use super::*;

    #[test]
    fn test_chip_lifecycle_rules() {
        let mut chip = Chip::new("test", ChipVariant::Input);
        
        // Input chips should start as enabled
        assert_eq!(chip.state, ChipState::Enabled);
        
        // Should be able to transition to selected if selectable
        if chip.supports_selection() {
            assert!(chip.transition_to(ChipState::Selected).is_ok());
        }
        
        // Should be able to be disabled at any time
        assert!(chip.transition_to(ChipState::Disabled).is_ok());
        
        // Once disabled, should not be able to be selected directly
        assert!(chip.transition_to(ChipState::Selected).is_err());
    }

    #[test]
    fn test_variant_specific_business_rules() {
        // Action chips should trigger callbacks
        let action_chip = Chip::new("action", ChipVariant::Action);
        assert!(action_chip.requires_callback());
        
        // Filter chips should maintain selection state
        let filter_chip = Chip::new("filter", ChipVariant::Filter);
        assert!(filter_chip.maintains_selection_state());
        
        // Input chips should support removal
        let input_chip = Chip::new("input", ChipVariant::Input);
        assert!(input_chip.supports_removal());
        
        // Suggestion chips should support activation
        let suggestion_chip = Chip::new("suggestion", ChipVariant::Suggestion);
        assert!(suggestion_chip.supports_activation());
    }

    #[test]
    fn test_collection_business_rules() {
        let mut collection = create_single_selection_collection();
        
        // Single selection collections should only allow one selected chip
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        collection.select(&chip_ids[0]).unwrap();
        assert_eq!(collection.selected_count(), 1);
        
        collection.select(&chip_ids[1]).unwrap();
        assert_eq!(collection.selected_count(), 1); // Still only one
        assert!(!collection.is_selected(&chip_ids[0])); // First deselected
        assert!(collection.is_selected(&chip_ids[1])); // Second selected
    }

    #[test]
    fn test_validation_error_handling() {
        // Test that validation errors are properly categorized
        let errors = [
            ValidationError::InvalidLabelLength { length: 0, max: MAX_CHIP_LABEL_LENGTH },
            ValidationError::InvalidIdFormat { id: "invalid id".to_string() },
            ValidationError::InvalidStateTransition { from: ChipState::Disabled, to: ChipState::Selected },
            ValidationError::BusinessRuleViolation { rule: "single_selection".to_string() },
        ];
        
        for error in &errors {
            assert!(error.is_validation_error());
            assert!(!error.error_message().is_empty());
            assert!(error.error_code() != 0);
        }
    }
}

#[cfg(test)]
mod property_based_validation_tests {
    use super::*;

    proptest! {
        #[test]
        fn prop_label_length_validation(label in ".*") {
            let result = ChipValidator::validate_label(&label);
            
            if label.trim().is_empty() {
                prop_assert!(result.is_err());
            } else if label.trim().len() > MAX_CHIP_LABEL_LENGTH {
                prop_assert!(result.is_err());
            } else if label.chars().any(|c| c.is_control()) {
                prop_assert!(result.is_err());
            } else {
                prop_assert!(result.is_ok());
            }
        }

        #[test]
        fn prop_chip_creation_validation(
            label in "[a-zA-Z0-9 ]{1,50}",
            variant in prop::sample::select(vec![
                ChipVariant::Action,
                ChipVariant::Filter,
                ChipVariant::Input,
                ChipVariant::Suggestion,
            ])
        ) {
            let chip_result = std::panic::catch_unwind(|| {
                Chip::new(&label, variant.clone())
            });
            
            let validation_result = ChipValidator::validate_label(&label);
            
            prop_assert_eq!(chip_result.is_ok(), validation_result.is_ok());
        }

        #[test]
        fn prop_state_transition_validation(
            initial_state in prop::sample::select(vec![
                ChipState::Enabled,
                ChipState::Selected,
                ChipState::Disabled,
                ChipState::Hover,
            ]),
            target_state in prop::sample::select(vec![
                ChipState::Enabled,
                ChipState::Selected,
                ChipState::Disabled,
                ChipState::Hover,
            ])
        ) {
            let mut chip = Chip::new("test", ChipVariant::Filter);
            chip.state = initial_state.clone();
            
            let transition_result = chip.transition_to(target_state.clone());
            let is_valid_transition = ChipValidator::is_valid_state_transition(&initial_state, &target_state);
            
            prop_assert_eq!(transition_result.is_ok(), is_valid_transition);
        }
    }
}

#[cfg(test)]
mod validation_performance_tests {
    use super::*;

    #[test]
    fn test_validation_performance() {
        let test_labels: Vec<String> = (0..1000)
            .map(|i| format!("test_label_{}", i))
            .collect();
        
        let start = std::time::Instant::now();
        
        for label in &test_labels {
            let _result = ChipValidator::validate_label(label);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < PERFORMANCE_THRESHOLD_MS);
    }

    #[test]
    fn test_bulk_validation_performance() {
        let chips: Vec<_> = (0..1000)
            .map(|i| create_test_chip(&format!("chip_{}", i), ChipVariant::Filter))
            .collect();
        
        let start = std::time::Instant::now();
        
        for chip in &chips {
            let _result = ChipValidator::validate_chip(chip);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < PERFORMANCE_THRESHOLD_MS * 2);
    }
}

// Helper function for color contrast calculation
fn calculate_contrast_ratio(foreground: Color, background: Color) -> f64 {
    let l1 = relative_luminance(foreground);
    let l2 = relative_luminance(background);
    
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    
    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(color: Color) -> f64 {
    let r = gamma_correct(color.r as f64 / 255.0);
    let g = gamma_correct(color.g as f64 / 255.0);
    let b = gamma_correct(color.b as f64 / 255.0);
    
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn gamma_correct(value: f64) -> f64 {
    if value <= 0.03928 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

// Placeholder types for compilation (would be defined in actual codebase)
#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}
