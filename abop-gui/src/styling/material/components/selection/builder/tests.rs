//! Comprehensive tests for the selection component builder system
//!
//! This module contains all tests for the modular builder architecture,
//! covering validation systems, builder patterns, individual builders,
//! components, and factory functions.

use super::*;

// ============================================================================
// Core Builder Tests
// ============================================================================

#[cfg(test)]
mod core_builder_tests {
    use super::*;

    #[test]
    fn test_checkbox_builder() {
        let checkbox = CheckboxBuilder::checked()
            .label("Test Checkbox")
            .size(ComponentSize::Large)
            .disabled(false)
            .error(true)
            .build()
            .expect("Should build valid checkbox");

        assert_eq!(checkbox.state, CheckboxState::Checked);
        assert_eq!(checkbox.props.label, Some("Test Checkbox".to_string()));
        assert_eq!(checkbox.props.size, ComponentSize::Large);
        assert!(!checkbox.props.disabled);
        assert!(checkbox.error_state);
    }

    #[test]
    fn test_radio_builder() {
        let radio = RadioBuilder::new("option_a")
            .label("Option A")
            .size(ComponentSize::Medium)
            .build()
            .expect("Should build valid radio");

        assert_eq!(radio.value, "option_a");
        assert_eq!(radio.props.label, Some("Option A".to_string()));
        assert_eq!(radio.props.size, ComponentSize::Medium);
    }

    #[test]
    fn test_switch_builder() {
        let switch = SwitchBuilder::on()
            .label("Enable feature")
            .disabled(false)
            .build()
            .expect("Should build valid switch");

        assert_eq!(switch.state, SwitchState::On);
        assert_eq!(switch.props.label, Some("Enable feature".to_string()));
        assert!(!switch.props.disabled);
    }

    #[test]
    fn test_chip_builder() {
        let chip = ChipBuilder::filter("Category")
            .selected(true)
            .size(ComponentSize::Small)
            .build()
            .expect("Should build valid chip");

        assert_eq!(chip.label, "Category");
        assert_eq!(chip.state, ChipState::Selected);
        assert_eq!(chip.variant, ChipVariant::Filter);
        assert_eq!(chip.props.size, ComponentSize::Small);
    }

    #[test]
    fn test_validation_failure() {
        let result = ChipBuilder::new("", ChipVariant::Filter).build();

        assert!(result.is_err());
        assert!(matches!(result, Err(SelectionError::EmptyLabel)));
    }

    #[test]
    fn test_conditional_builder() {
        let checkbox = CheckboxBuilder::unchecked()
            .when(true, |b| b.label("Conditional Label"))
            .when(false, |b| b.disabled(true))
            .build()
            .expect("Should build with conditional config");

        assert_eq!(checkbox.props.label, Some("Conditional Label".to_string()));
        assert!(!checkbox.props.disabled);
    }

    #[test]
    fn test_convenience_functions() {
        let cb = checkbox(CheckboxState::Checked)
            .label("Test")
            .build()
            .unwrap();
        let rb = radio("value").label("Test").build().unwrap();
        let sw = switch(SwitchState::On).label("Test").build().unwrap();
        let ch = chip("Test", ChipVariant::Filter).build().unwrap();

        assert_eq!(cb.state, CheckboxState::Checked);
        assert_eq!(rb.value, "value");
        assert_eq!(sw.state, SwitchState::On);
        assert_eq!(ch.label, "Test");
    }
}

// ============================================================================
// Advanced Validation System Tests
// ============================================================================

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_validation_result_aggregation() {
        let context = ValidationContext::new("TestBuilder".to_string(), "test".to_string());
        let mut result = ValidationResult::new(context);

        result.add_error(SelectionError::EmptyLabel);
        result.add_warning("This is a warning".to_string());

        assert!(!result.is_valid());
        assert!(result.has_warnings());
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_detailed_validation() {
        let checkbox = CheckboxBuilder::checked().label("Valid checkbox");

        let result = checkbox.validate_detailed();
        assert!(result.is_valid());
        assert_eq!(result.errors.len(), 0);

        // Test with invalid long label
        let checkbox_invalid = CheckboxBuilder::checked().label("x".repeat(150)); // Very long label to trigger warning

        let result_invalid = checkbox_invalid.validate_detailed();
        assert!(result_invalid.is_valid()); // Still valid, just has warnings
        assert!(result_invalid.has_warnings());
    }

    #[test]
    fn test_batch_validation() {
        let items = vec![
            CheckboxBuilder::checked().label("Valid 1"),
            CheckboxBuilder::unchecked().label("Valid 2"),
            CheckboxBuilder::indeterminate().label("x".repeat(250)), // Invalid - too long
        ];

        let result = validate_batch(&items, "CheckboxBuilder", |builder| builder.validate());

        assert!(!result.is_valid());
        assert!(result.errors.len() > 0);
    }

    #[test]
    fn test_validation_composer() {
        let composer = DefaultValidationComposer;
        let checkbox = CheckboxBuilder::checked().label("Test");

        let validators: Vec<Box<dyn Fn(&CheckboxBuilder) -> ValidationResult>> =
            vec![Box::new(|builder| builder.validate_detailed())];

        let result = composer.compose_validations(&checkbox, validators);
        assert!(result.is_valid());
    }

    #[test]
    fn test_validated_methods() {
        // Test label validation
        let result = CheckboxBuilder::unchecked().label_validated("Valid label");
        assert!(result.is_ok());

        let result_invalid = CheckboxBuilder::unchecked().label_validated("x".repeat(250)); // Too long
        assert!(result_invalid.is_err());

        // Test state validation
        let result_state = CheckboxBuilder::unchecked().state_validated(CheckboxState::Checked);
        assert!(result_state.is_ok());
    }

    #[test]
    fn test_detailed_validation_build() {
        let checkbox = CheckboxBuilder::checked().label("Detailed validation test");

        let result = checkbox.build_with_detailed_validation();
        assert!(result.is_ok());

        // For ChipBuilder, empty labels are specifically checked in validate_detailed
        let invalid_chip = ChipBuilder::new("", ChipVariant::Filter);
        let result_invalid = invalid_chip.build_with_detailed_validation();
        assert!(result_invalid.is_err());
    }

    #[test]
    fn test_size_validation() {
        let result = CheckboxBuilder::unchecked().size_validated(
            ComponentSize::Medium,
            Some(ComponentSize::Small),
            Some(ComponentSize::Large),
        );
        assert!(result.is_ok());

        let result_invalid = CheckboxBuilder::unchecked().size_validated(
            ComponentSize::Small,
            Some(ComponentSize::Medium), // Min larger than actual
            Some(ComponentSize::Large),
        );
        assert!(result_invalid.is_err());
    }
}

// ============================================================================
// Advanced Builder Pattern Tests
// ============================================================================

#[cfg(test)]
mod builder_patterns_tests {
    use super::*;

    #[test]
    fn test_advanced_conditional_building() {
        let checkbox = CheckboxBuilder::unchecked()
            .when_validated(
                |builder| {
                    if builder.state() == CheckboxState::Unchecked {
                        Ok(())
                    } else {
                        Err(SelectionError::ValidationError(
                            "Expected unchecked".to_string(),
                        ))
                    }
                },
                |builder| builder.label("Validated label"),
            )
            .unwrap();

        assert_eq!(checkbox.props().label, Some("Validated label".to_string()));
    }

    #[test]
    fn test_stateful_builder_validation() {
        let checkbox = CheckboxBuilder::unchecked();

        // Test valid state transition
        let result = checkbox.validate_state_transition(CheckboxState::Checked);
        assert!(result.is_ok());

        // Apply validated state
        let updated_checkbox = checkbox
            .apply_state_validated(CheckboxState::Indeterminate)
            .unwrap();
        assert_eq!(updated_checkbox.state(), CheckboxState::Indeterminate);
    }

    #[test]
    fn test_builder_composition() {
        let composer = BuilderComposer::new();
        let checkbox = CheckboxBuilder::unchecked();

        let configurations = [
            Box::new(|b: CheckboxBuilder| Ok(b.label("Composed")))
                as Box<dyn Fn(CheckboxBuilder) -> Result<CheckboxBuilder, SelectionError>>,
            Box::new(|b: CheckboxBuilder| Ok(b.size(ComponentSize::Large))),
            Box::new(|b: CheckboxBuilder| Ok(b.disabled(false))),
        ];

        let result = composer.compose(checkbox, &configurations).unwrap();

        assert_eq!(result.props().label, Some("Composed".to_string()));
        assert_eq!(result.props().size, ComponentSize::Large);
        assert!(!result.props().disabled);
    }

    #[test]
    fn test_configuration_summary() {
        let checkbox = CheckboxBuilder::checked()
            .label("Test checkbox")
            .size(ComponentSize::Large)
            .disabled(true)
            .error(true);

        let summary = checkbox.configuration_summary();

        assert!(summary.disabled);
        assert_eq!(summary.size, ComponentSize::Large);
        assert!(summary.has_label);
        assert_eq!(summary.label_length, 13); // "Test checkbox" - corrected length
        assert!(summary.has_error);
        assert!(summary.animation_enabled); // Default
    }

    #[test]
    fn test_configuration_chain() {
        let configurations = [
            Box::new(|b: CheckboxBuilder| Ok(b.label("Chain test")))
                as Box<dyn Fn(CheckboxBuilder) -> Result<CheckboxBuilder, SelectionError>>,
            Box::new(|b: CheckboxBuilder| Ok(b.size(ComponentSize::Large))),
        ];

        let result = CheckboxBuilder::unchecked().configure_chain(&configurations);

        assert!(result.is_ok());
        let checkbox = result.unwrap();
        assert_eq!(checkbox.props().label, Some("Chain test".to_string()));
        assert_eq!(checkbox.props().size, ComponentSize::Large);
    }
}

// ============================================================================
// Performance and System Integration Tests
// ============================================================================

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_performance_optimized_methods() {
        let checkbox = CheckboxBuilder::checked()
            .label("Fast build test")
            .build_fast(); // No validation

        assert_eq!(checkbox.state, CheckboxState::Checked);
        assert_eq!(checkbox.props.label, Some("Fast build test".to_string()));
    }

    #[test]
    fn test_system_preferences_integration() {
        let checkbox = CheckboxBuilder::checked().with_system_preferences();

        // Animation config should be adjusted based on system preferences
        // In test environment, we can't reliably test system_has_reduced_motion()
        // but we can verify the method doesn't crash
        assert_eq!(checkbox.state(), CheckboxState::Checked);
    }

    #[test]
    fn test_clone_with_modifications() {
        let original = CheckboxBuilder::unchecked().label("Original");

        let modified = original.clone_with_state(CheckboxState::Checked);

        assert_eq!(original.state(), CheckboxState::Unchecked);
        assert_eq!(modified.state(), CheckboxState::Checked);
        assert_eq!(modified.props().label, Some("Original".to_string()));
    }

    #[test]
    fn test_reset_to_defaults() {
        let checkbox = CheckboxBuilder::checked()
            .label("Will be reset")
            .size(ComponentSize::Large)
            .disabled(true)
            .error(true)
            .reset_to_defaults();

        assert_eq!(checkbox.props().label, None);
        assert_eq!(checkbox.props().size, ComponentSize::Medium); // Default
        assert!(!checkbox.props().disabled);
        assert!(!checkbox.has_error());
    }
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_error_context_enhancement() {
        let mut reporter = ErrorReporter::new();

        let context = ErrorContext {
            operation: "build".to_string(),
            component_type: "CheckboxBuilder".to_string(),
            field: Some("label".to_string()),
            suggestion: Some("Provide a non-empty label".to_string()),
            severity: ErrorSeverity::Error,
        };

        reporter.push_context(context);

        let original_error = SelectionError::EmptyLabel;
        let enhanced_error = reporter.report_error(original_error);

        // Verify error was enhanced (exact format may vary)
        assert!(matches!(enhanced_error, SelectionError::EmptyLabel));
    }

    #[test]
    fn test_conditional_methods() {
        let checkbox = CheckboxBuilder::unchecked()
            .disabled_when(true)
            .error_when(false);

        assert!(checkbox.props().disabled);
        assert!(!checkbox.has_error());
    }
}

// ============================================================================
// Component-Specific Advanced Tests
// ============================================================================

#[cfg(test)]
mod component_specific_tests {
    use super::*;

    #[test]
    fn test_radio_advanced_features() {
        let radio = RadioBuilder::new("option_1")
            .label_validated("Option 1")
            .unwrap()
            .with_system_preferences();

        assert_eq!(radio.value(), &"option_1");
        assert_eq!(radio.props().label, Some("Option 1".to_string()));

        let cloned = radio.clone_with_value("option_2");
        assert_eq!(cloned.value(), &"option_2");
        assert_eq!(cloned.props().label, Some("Option 1".to_string()));
    }

    #[test]
    fn test_switch_advanced_features() {
        let switch = SwitchBuilder::off()
            .toggled()
            .label_validated("Toggle feature")
            .unwrap();

        assert_eq!(switch.state(), SwitchState::On); // Should be toggled

        let cloned = switch.clone_with_state(SwitchState::Off);
        assert_eq!(cloned.state(), SwitchState::Off);
    }

    #[test]
    fn test_chip_advanced_features() {
        let chip = ChipBuilder::filter("Category")
            .toggled()
            .with_metadata("custom_key", "custom_value")
            .build_fast();

        assert_eq!(chip.state, ChipState::Selected); // Should be toggled
        assert_eq!(chip.variant, ChipVariant::Filter);

        let cloned_chip = ChipBuilder::filter("Category").clone_with_variant(ChipVariant::Input);
        assert_eq!(cloned_chip.variant(), ChipVariant::Input);
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_workflow_checkbox() {
        // Test complete checkbox workflow from builder to component
        let checkbox = CheckboxBuilder::unchecked()
            .label("Integration Test")
            .size(ComponentSize::Large)
            .with_system_preferences()
            .validate_and_build()
            .expect("Should build with validation");

        assert_eq!(checkbox.state(), CheckboxState::Unchecked);
        assert_eq!(checkbox.props().label, Some("Integration Test".to_string()));
        assert_eq!(checkbox.props().size, ComponentSize::Large);
    }

    #[test]
    fn test_full_workflow_radio() {
        // Test complete radio workflow
        let radio = RadioBuilder::new("test_value")
            .label("Radio Test")
            .size(ComponentSize::Medium)
            .build()
            .expect("Should build radio");

        assert_eq!(radio.value(), &"test_value");
        assert_eq!(radio.props().label, Some("Radio Test".to_string()));
    }

    #[test]
    fn test_full_workflow_switch() {
        // Test complete switch workflow
        let switch = SwitchBuilder::off()
            .label("Switch Test")
            .toggled() // Should become on
            .build()
            .expect("Should build switch");

        assert_eq!(switch.state(), SwitchState::On);
        assert_eq!(switch.props().label, Some("Switch Test".to_string()));
    }

    #[test]
    fn test_full_workflow_chip() {
        // Test complete chip workflow
        let chip = ChipBuilder::filter("Filter Test")
            .selected(true)
            .with_leading_icon("filter")
            .size(ComponentSize::Small)
            .build()
            .expect("Should build chip");

        assert_eq!(chip.label(), "Filter Test");
        assert_eq!(chip.state(), ChipState::Selected);
        assert_eq!(chip.variant(), ChipVariant::Filter);
        assert_eq!(chip.props().size, ComponentSize::Small);
    }

    #[test]
    fn test_factory_integration() {
        // Test factory functions integration
        let checked_cb = checked_checkbox().label("Factory Test").build().unwrap();

        let filter_chip = filter_chip("Factory Filter")
            .selected(true)
            .build()
            .unwrap();

        let on_switch = switch_on().label("Factory Switch").build().unwrap();

        assert_eq!(checked_cb.state(), CheckboxState::Checked);
        assert_eq!(filter_chip.variant(), ChipVariant::Filter);
        assert_eq!(on_switch.state(), SwitchState::On);
    }
}

// ============================================================================
// Module Structure Tests
// ============================================================================

#[cfg(test)]
mod module_structure_tests {
    use super::*;

    #[test]
    fn test_module_exports_accessible() {
        // Test that all builders are accessible through module re-exports
        let _checkbox_builder = CheckboxBuilder::unchecked();
        let _radio_builder = RadioBuilder::new("test");
        let _switch_builder = SwitchBuilder::off();
        let _chip_builder = ChipBuilder::filter("test");
    }

    #[test]
    fn test_validation_system_accessible() {
        // Test validation system accessibility
        let context = ValidationContext::new("test".to_string(), "test".to_string());
        let _result = ValidationResult::new(context);
        let _composer = DefaultValidationComposer;
    }

    #[test]
    fn test_component_structs_accessible() {
        // Test component struct accessibility
        let _checkbox = Checkbox::new(CheckboxState::Unchecked);
        let _radio = Radio::new("test");
        let _switch = Switch::new(SwitchState::Off);
        let _chip = Chip::new("test", ChipVariant::Filter);
    }

    #[test]
    fn test_factory_functions_accessible() {
        // Test factory function accessibility
        let _cb = checkbox(CheckboxState::Checked);
        let _rb = radio("test");
        let _sw = switch(SwitchState::On);
        let _ch = chip("test", ChipVariant::Filter);

        // Test convenience functions
        let _checked = checked_checkbox();
        let _unchecked = unchecked_checkbox();
        let _filter = filter_chip("test");
        let _assist = assist_chip("test");
    }
}
