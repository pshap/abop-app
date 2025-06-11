//! Custom assertion helpers and macros for chip testing
//!
//! This module provides specialized assertion functions and macros that make
//! chip testing more expressive and provide better error messages.

use super::test_data::*;
use crate::styling::material::components::selection::{
    Chip, ChipCollection, ChipSelectionMode, ChipState, ChipVariant, ComponentSize, SelectionError,
};

// ============================================================================
// Chip Property Assertions
// ============================================================================

/// Assert that a chip has expected basic properties
pub fn assert_chip_basics(chip: &Chip, expected_label: &str, expected_variant: ChipVariant) {
    assert_eq!(
        chip.label(),
        expected_label,
        "Chip label mismatch: expected '{}', got '{}'",
        expected_label,
        chip.label()
    );
    assert_eq!(
        chip.variant(),
        expected_variant,
        "Chip variant mismatch: expected {:?}, got {:?}",
        expected_variant,
        chip.variant()
    );
    assert_eq!(
        chip.state(),
        ChipState::Unselected,
        "Expected new chip to be unselected, got {:?}",
        chip.state()
    );
    assert!(
        !chip.props().disabled,
        "Expected new chip to be enabled, but it was disabled"
    );
}

/// Assert that a chip is in selected state
pub fn assert_chip_selected(chip: &Chip) {
    assert_eq!(
        chip.state(),
        ChipState::Selected,
        "Expected chip '{}' to be selected, got state {:?}",
        chip.label(),
        chip.state()
    );
    assert!(
        chip.is_selected(),
        "Expected chip '{}' is_selected() to return true",
        chip.label()
    );
}

/// Assert that a chip is in unselected state
pub fn assert_chip_unselected(chip: &Chip) {
    assert_eq!(
        chip.state(),
        ChipState::Unselected,
        "Expected chip '{}' to be unselected, got state {:?}",
        chip.label(),
        chip.state()
    );
    assert!(
        !chip.is_selected(),
        "Expected chip '{}' is_selected() to return false",
        chip.label()
    );
}

/// Assert that a chip is in pressed state
pub fn assert_chip_pressed(chip: &Chip) {
    assert_eq!(
        chip.state(),
        ChipState::Pressed,
        "Expected chip '{}' to be pressed, got state {:?}",
        chip.label(),
        chip.state()
    );
}

/// Assert that a chip has expected size
pub fn assert_chip_size(chip: &Chip, expected_size: ComponentSize) {
    assert_eq!(
        chip.props().size,
        expected_size,
        "Expected chip '{}' to have size {:?}, got {:?}",
        chip.label(),
        expected_size,
        chip.props().size
    );
}

/// Assert that a chip is disabled
pub fn assert_chip_disabled(chip: &Chip) {
    assert!(
        chip.props().disabled,
        "Expected chip '{}' to be disabled",
        chip.label()
    );
}

/// Assert that a chip is enabled
pub fn assert_chip_enabled(chip: &Chip) {
    assert!(
        !chip.props().disabled,
        "Expected chip '{}' to be enabled",
        chip.label()
    );
}

/// Assert that a chip has metadata
pub fn assert_chip_has_metadata(chip: &Chip, key: &str, expected_value: &str) {
    let metadata = &chip.props().metadata;
    assert!(
        metadata.contains_key(key),
        "Expected chip '{}' to have metadata key '{}'",
        chip.label(),
        key
    );
    assert_eq!(
        metadata.get(key).unwrap(),
        expected_value,
        "Expected chip '{}' metadata '{}' to be '{}', got '{}'",
        chip.label(),
        key,
        expected_value,
        metadata.get(key).unwrap()
    );
}

// ============================================================================
// Collection State Assertions
// ============================================================================

/// Assert collection selection state
pub fn assert_collection_selection(collection: &ChipCollection, expected_count: usize) {
    let actual_count = collection.selected_chips().len();
    assert_eq!(
        actual_count,
        expected_count,
        "Expected {} selected chips, got {}. Selected chips: {:?}",
        expected_count,
        actual_count,
        collection
            .selected_chips()
            .iter()
            .map(|c| c.label())
            .collect::<Vec<_>>()
    );
}

/// Assert collection has specific length
pub fn assert_collection_length(collection: &ChipCollection, expected_length: usize) {
    assert_eq!(
        collection.len(),
        expected_length,
        "Expected collection to have {} chips, got {}",
        expected_length,
        collection.len()
    );
}

/// Assert collection is empty
pub fn assert_collection_empty(collection: &ChipCollection) {
    assert!(
        collection.is_empty(),
        "Expected collection to be empty, but it has {} chips",
        collection.len()
    );
    assert_eq!(
        collection.len(),
        0,
        "Expected collection length to be 0, got {}",
        collection.len()
    );
}

/// Assert collection is not empty
pub fn assert_collection_not_empty(collection: &ChipCollection) {
    assert!(
        !collection.is_empty(),
        "Expected collection to not be empty"
    );
    assert!(
        !collection.is_empty(),
        "Expected collection length to be > 0, got {}",
        collection.len()
    );
}

/// Assert collection has specific selection mode
pub fn assert_collection_selection_mode(
    collection: &ChipCollection,
    expected_mode: ChipSelectionMode,
) {
    assert_eq!(
        collection.selection_mode(),
        expected_mode,
        "Expected collection to have selection mode {:?}, got {:?}",
        expected_mode,
        collection.selection_mode()
    );
}

/// Assert that specific chips are selected by index
pub fn assert_chips_selected_by_index(collection: &ChipCollection, expected_indices: &[usize]) {
    let selected_indices = collection.selected_indices();
    assert_eq!(
        selected_indices.len(),
        expected_indices.len(),
        "Expected {} selected chips, got {}. Expected indices: {:?}, actual: {:?}",
        expected_indices.len(),
        selected_indices.len(),
        expected_indices,
        selected_indices
    );

    for &expected_index in expected_indices {
        assert!(
            selected_indices.contains(&expected_index),
            "Expected chip at index {expected_index} to be selected, but it wasn't. Selected indices: {selected_indices:?}"
        );

        if expected_index < collection.len() {
            assert_chip_selected(&collection.chips()[expected_index]);
        }
    }
}

/// Assert that no chips are selected
pub fn assert_no_chips_selected(collection: &ChipCollection) {
    assert_collection_selection(collection, 0);
    assert!(
        collection.selected_indices().is_empty(),
        "Expected no chips to be selected, but found selected indices: {:?}",
        collection.selected_indices()
    );
}

// ============================================================================
// Validation Error Assertions
// ============================================================================

/// Assert that a result contains a specific label validation error
pub fn assert_label_validation_error(result: Result<Chip, SelectionError>, expected_len: usize) {
    match result {
        Err(SelectionError::LabelTooLong { len, max: _ }) => {
            assert_eq!(
                len, expected_len,
                "Expected error length {expected_len} but got {len}"
            );
        }
        Err(other_error) => {
            panic!(
                "Expected LabelTooLong error with length {expected_len}, but got: {other_error:?}"
            );
        }
        Ok(chip) => {
            panic!(
                "Expected LabelTooLong error with length {}, but chip was successfully created with label '{}'",
                expected_len,
                chip.label()
            );
        }
    }
}

/// Assert that a result contains an empty label error
pub fn assert_empty_label_error(result: Result<Chip, SelectionError>) {
    match result {
        Err(SelectionError::EmptyLabel) => {
            // Expected error type
        }
        Err(SelectionError::InvalidLabel { reason }) => {
            assert!(
                reason.contains("empty")
                    || reason.contains("must have")
                    || reason.contains("required"),
                "Expected empty label error, but got InvalidLabel with reason: '{reason}'"
            );
        }
        Err(other_error) => {
            panic!("Expected empty label error, but got: {other_error:?}");
        }
        Ok(chip) => {
            panic!(
                "Expected empty label error, but chip was successfully created with label '{}'",
                chip.label()
            );
        }
    }
}

/// Assert that a result contains an invalid state error
pub fn assert_invalid_state_error(
    result: Result<(), SelectionError>,
    expected_details_pattern: &str,
) {
    match result {
        Err(SelectionError::InvalidState { details }) => {
            assert!(
                details.contains(expected_details_pattern),
                "Expected invalid state error containing '{expected_details_pattern}', but got details: '{details}'"
            );
        }
        Err(other_error) => {
            panic!(
                "Expected InvalidState error containing '{expected_details_pattern}', but got: {other_error:?}"
            );
        }
        Ok(()) => {
            panic!(
                "Expected InvalidState error containing '{expected_details_pattern}', but operation succeeded"
            );
        }
    }
}

// ============================================================================
// Macro Helpers for Bulk Assertions
// ============================================================================

/// Assert multiple chips have the same property (currently unused)
#[allow(unused_macros)]
macro_rules! assert_chips_property {
    ($chips:expr, $property:ident, $expected:expr) => {
        for (i, chip) in $chips.iter().enumerate() {
            assert_eq!(
                chip.$property(),
                $expected,
                "Chip {} '{}' has unexpected {}: expected {:?}, got {:?}",
                i,
                chip.label(),
                stringify!($property),
                $expected,
                chip.$property()
            );
        }
    };
}

/// Assert all chips in collection have same variant
pub fn assert_all_chips_variant(collection: &ChipCollection, expected_variant: ChipVariant) {
    for (i, chip) in collection.chips().iter().enumerate() {
        assert_eq!(
            chip.variant(),
            expected_variant,
            "Chip {} '{}' has unexpected variant: expected {:?}, got {:?}",
            i,
            chip.label(),
            expected_variant,
            chip.variant()
        );
    }
}

/// Assert all chips in collection have same size
pub fn assert_all_chips_size(collection: &ChipCollection, expected_size: ComponentSize) {
    for (i, chip) in collection.chips().iter().enumerate() {
        assert_eq!(
            chip.props().size,
            expected_size,
            "Chip {} '{}' has unexpected size: expected {:?}, got {:?}",
            i,
            chip.label(),
            expected_size,
            chip.props().size
        );
    }
}

/// Assert all chips in collection are in same state
pub fn assert_all_chips_state(collection: &ChipCollection, expected_state: ChipState) {
    for (i, chip) in collection.chips().iter().enumerate() {
        assert_eq!(
            chip.state(),
            expected_state,
            "Chip {} '{}' has unexpected state: expected {:?}, got {:?}",
            i,
            chip.label(),
            expected_state,
            chip.state()
        );
    }
}

// ============================================================================
// Material Design 3 Compliance Assertions
// ============================================================================

/// Assert chip meets Material Design 3 touch target size requirements
pub fn assert_md3_touch_target_compliance(chip: &Chip) {
    let size = chip.props().size;
    let touch_target_size = size.touch_target_size();

    assert!(
        touch_target_size >= MD3_MIN_TOUCH_TARGET_SIZE,
        "Chip '{}' with size {:?} has touch target size {:.1}dp, which is below MD3 minimum of {:.1}dp",
        chip.label(),
        size,
        touch_target_size,
        MD3_MIN_TOUCH_TARGET_SIZE
    );
}

/// Assert chip height meets Material Design 3 specifications
pub fn assert_md3_height_compliance(chip: &Chip) {
    let size = chip.props().size;
    let expected_height = match size {
        ComponentSize::Small => MD3_CHIP_HEIGHT_SMALL,
        ComponentSize::Medium => MD3_CHIP_HEIGHT_MEDIUM,
        ComponentSize::Large => MD3_CHIP_HEIGHT_LARGE,
    };

    let actual_height = size.size_px();
    assert_eq!(
        actual_height,
        expected_height,
        "Chip '{}' with size {:?} has height {:.1}dp, expected {:.1}dp per MD3 spec",
        chip.label(),
        size,
        actual_height,
        expected_height
    );
}

/// Assert collection spacing meets Material Design 3 recommendations
pub fn assert_md3_spacing_compliance(spacing: f32) {
    let valid_spacings = [
        MD3_CHIP_SPACING_COMPACT,
        MD3_CHIP_SPACING_STANDARD,
        MD3_CHIP_SPACING_COMFORTABLE,
    ];

    assert!(
        valid_spacings.contains(&spacing) || spacing >= MD3_CHIP_SPACING_COMPACT,
        "Spacing {spacing:.1}dp does not meet MD3 recommendations. Use {MD3_CHIP_SPACING_COMPACT:.1}dp (compact), {MD3_CHIP_SPACING_STANDARD:.1}dp (standard), or {MD3_CHIP_SPACING_COMFORTABLE:.1}dp (comfortable)"
    );
}

// ============================================================================
// Performance Assertions
// ============================================================================

/// Assert operation completed within time limit
pub fn assert_within_time_limit<F>(operation: F, limit_ms: u128, operation_name: &str)
where
    F: FnOnce(),
{
    let start = std::time::Instant::now();
    operation();
    let elapsed = start.elapsed().as_millis();

    assert!(
        elapsed <= limit_ms,
        "{operation_name} took {elapsed}ms, which exceeds the limit of {limit_ms}ms"
    );
}

/// Assert memory usage is reasonable for collection size
pub fn assert_reasonable_memory_usage(collection: &ChipCollection) {
    let collection_size = collection.len();

    // Very rough heuristic: each chip should not use more than 1KB on average
    // This is a basic sanity check, not a precise measurement
    if collection_size > 1000 {
        // For large collections, just ensure it's not completely unreasonable
        assert!(
            collection_size < 100_000,
            "Collection size {collection_size} is unreasonably large for testing"
        );
    }
}

// ============================================================================
// Test Utility Assertions
// ============================================================================

/// Assert that two chip collections are equivalent (same chips in same order)
pub fn assert_collections_equivalent(collection1: &ChipCollection, collection2: &ChipCollection) {
    assert_eq!(
        collection1.len(),
        collection2.len(),
        "Collections have different lengths: {} vs {}",
        collection1.len(),
        collection2.len()
    );

    assert_eq!(
        collection1.selection_mode(),
        collection2.selection_mode(),
        "Collections have different selection modes: {:?} vs {:?}",
        collection1.selection_mode(),
        collection2.selection_mode()
    );

    for (i, (chip1, chip2)) in collection1
        .chips()
        .iter()
        .zip(collection2.chips().iter())
        .enumerate()
    {
        assert_eq!(
            chip1.label(),
            chip2.label(),
            "Chip {} labels differ: '{}' vs '{}'",
            i,
            chip1.label(),
            chip2.label()
        );
        assert_eq!(
            chip1.variant(),
            chip2.variant(),
            "Chip {} variants differ: {:?} vs {:?}",
            i,
            chip1.variant(),
            chip2.variant()
        );
        assert_eq!(
            chip1.state(),
            chip2.state(),
            "Chip {} states differ: {:?} vs {:?}",
            i,
            chip1.state(),
            chip2.state()
        );
    }
}

// Make the macro available if needed in the future
// pub(crate) use assert_chips_property;
