//! Comprehensive tests for the modern Checkbox component

use crate::styling::material::components::selection::common::{AnimatedWidget, ErrorState};
use crate::styling::material::components::selection::{
    builder::{Checkbox, CheckboxBuilder, ComponentBuilder, ConditionalBuilder},
    checkbox::{checkbox_from_bool, checked_checkbox, indeterminate_checkbox, unchecked_checkbox},
    common::{CheckboxState, ComponentSize, SelectionError, SelectionWidget, StatefulWidget},
};

#[test]
fn test_checkbox_state_enum() {
    // Test default
    assert_eq!(CheckboxState::default(), CheckboxState::Unchecked);

    // Test boolean conversion
    assert_eq!(CheckboxState::from_bool(true), CheckboxState::Checked);
    assert_eq!(CheckboxState::from_bool(false), CheckboxState::Unchecked);
    assert!(CheckboxState::Checked.to_bool());
    assert!(!CheckboxState::Unchecked.to_bool());
    assert!(!CheckboxState::Indeterminate.to_bool());

    // Test selection status
    assert!(CheckboxState::Checked.is_selected());
    assert!(CheckboxState::Indeterminate.is_selected());
    assert!(!CheckboxState::Unchecked.is_selected());

    // Test toggle behavior
    assert_eq!(CheckboxState::Unchecked.toggle(), CheckboxState::Checked);
    assert_eq!(CheckboxState::Checked.toggle(), CheckboxState::Unchecked);
    assert_eq!(
        CheckboxState::Indeterminate.toggle(),
        CheckboxState::Checked
    );
}

#[test]
fn test_checkbox_builder_pattern() {
    let checkbox = CheckboxBuilder::checked()
        .label("Test Checkbox")
        .size(ComponentSize::Large)
        .disabled(false)
        .error(true)
        .build()
        .expect("Should build valid checkbox");

    assert_eq!(checkbox.state(), CheckboxState::Checked);
    assert_eq!(checkbox.props().label, Some("Test Checkbox".to_string()));
    assert_eq!(checkbox.props().size, ComponentSize::Large);
    assert!(!checkbox.props().disabled);
    assert!(checkbox.has_error());
}

#[test]
fn test_checkbox_builder_convenience_methods() {
    let checked = CheckboxBuilder::checked().build().unwrap();
    let unchecked = CheckboxBuilder::unchecked().build().unwrap();
    let indeterminate = CheckboxBuilder::indeterminate().build().unwrap();
    let from_bool = CheckboxBuilder::from_bool(true).build().unwrap();

    assert_eq!(checked.state(), CheckboxState::Checked);
    assert_eq!(unchecked.state(), CheckboxState::Unchecked);
    assert_eq!(indeterminate.state(), CheckboxState::Indeterminate);
    assert_eq!(from_bool.state(), CheckboxState::Checked);
}

#[test]
fn test_checkbox_validation() {
    // Valid checkbox
    let valid = CheckboxBuilder::checked().label("Valid Label").build();
    assert!(valid.is_ok());

    // Invalid - label too long
    let long_label = "x".repeat(201);
    let invalid = CheckboxBuilder::unchecked().label(&long_label).build();
    assert!(invalid.is_err());
    assert!(matches!(
        invalid.unwrap_err(),
        SelectionError::LabelTooLong { len: 201, max: 200 }
    ));
}

#[test]
fn test_checkbox_state_management() {
    let mut checkbox = CheckboxBuilder::unchecked()
        .build()
        .expect("Should create valid checkbox");

    // Initial state
    assert_eq!(checkbox.state(), CheckboxState::Unchecked);
    assert!(!checkbox.is_selected());
    assert!(!checkbox.to_bool());

    // Toggle to checked
    let (_prev_state, new_state) = checkbox.toggle().expect("Should toggle successfully");
    assert_eq!(new_state, CheckboxState::Checked);
    assert_eq!(checkbox.state(), CheckboxState::Checked);
    assert!(checkbox.is_selected());
    assert!(checkbox.to_bool());

    // Update state directly
    checkbox
        .update_state(CheckboxState::Indeterminate)
        .expect("Should update state");
    assert_eq!(checkbox.state(), CheckboxState::Indeterminate);
    assert!(checkbox.is_selected());
    assert!(!checkbox.to_bool()); // Indeterminate is false in boolean context
}

#[test]
fn test_checkbox_error_state() {
    let mut checkbox = CheckboxBuilder::checked()
        .error(true)
        .build()
        .expect("Should create checkbox with error");

    assert!(checkbox.has_error());

    // Change error state
    checkbox.set_error(false);
    assert!(!checkbox.has_error());
}

#[test]
fn test_checkbox_trait_implementations() {
    let checkbox = CheckboxBuilder::checked()
        .build()
        .expect("Should create valid checkbox");

    // Test SelectionWidget trait
    assert_eq!(checkbox.state(), CheckboxState::Checked);
    assert!(checkbox.validate().is_ok());

    // Test component properties
    let props = checkbox.props();
    assert_eq!(props.size, ComponentSize::Medium); // default
    assert!(!props.disabled); // default

    // Test animation support
    let animation_config = checkbox.animation_config();
    assert!(animation_config.enabled);
    assert_eq!(animation_config.duration.as_millis(), 200);
}

#[test]
fn test_checkbox_default() {
    let checkbox = Checkbox::default();
    assert_eq!(checkbox.state(), CheckboxState::Unchecked);
    assert!(!checkbox.props().disabled);
    assert_eq!(checkbox.props().size, ComponentSize::Medium);
    assert!(!checkbox.has_error());
}

#[test]
fn test_checkbox_convenience_functions() {
    let cb1 = checked_checkbox().build().unwrap();
    let cb2 = unchecked_checkbox().build().unwrap();
    let cb3 = indeterminate_checkbox().build().unwrap();
    let cb4 = checkbox_from_bool(true).build().unwrap();

    assert_eq!(cb1.state(), CheckboxState::Checked);
    assert_eq!(cb2.state(), CheckboxState::Unchecked);
    assert_eq!(cb3.state(), CheckboxState::Indeterminate);
    assert_eq!(cb4.state(), CheckboxState::Checked);
}

#[test]
fn test_checkbox_builder_validation() {
    let builder = CheckboxBuilder::checked().label("Test");

    // Test validation passes
    assert!(builder.validate().is_ok());

    // Test build with validation
    let checkbox = builder.build();
    assert!(checkbox.is_ok());
}

#[test]
fn test_checkbox_conditional_builder() {
    let checkbox = CheckboxBuilder::unchecked()
        .when(true, |b| b.label("Conditional Label"))
        .when(false, |b| b.disabled(true))
        .when_some(Some(ComponentSize::Large), |b, size| b.size(size))
        .when_some(None::<bool>, |b, error| b.error(error))
        .build()
        .expect("Should build with conditional config");

    assert_eq!(
        checkbox.props().label,
        Some("Conditional Label".to_string())
    );
    assert!(!checkbox.props().disabled); // false condition not applied
    assert_eq!(checkbox.props().size, ComponentSize::Large); // Some value applied
    assert!(!checkbox.has_error()); // None value not applied
}

#[test]
fn test_checkbox_size_properties() {
    let small = CheckboxBuilder::checked()
        .size(ComponentSize::Small)
        .build()
        .unwrap();
    let medium = CheckboxBuilder::checked()
        .size(ComponentSize::Medium)
        .build()
        .unwrap();
    let large = CheckboxBuilder::checked()
        .size(ComponentSize::Large)
        .build()
        .unwrap();

    assert_eq!(small.props().size, ComponentSize::Small);
    assert_eq!(medium.props().size, ComponentSize::Medium);
    assert_eq!(large.props().size, ComponentSize::Large);
}
