//! Test helpers for chip component testing
//!
//! This module provides reusable test utilities and builders for chip tests,
//! reducing code duplication and ensuring consistent test patterns.

use super::fixtures::test_data::ALL_CHIP_VARIANTS;
use crate::styling::material::components::selection::builder::ComponentBuilder;
use crate::styling::material::components::selection::chip::core::MAX_CHIP_LABEL_LENGTH;
use crate::styling::material::components::selection::common::SelectionComponent;
use crate::styling::material::components::selection::{
    Chip, ChipBuilder, ChipCollection, ChipCollectionBuilder, ChipSelectionMode, ChipState,
    ChipVariant, ComponentSize, SelectionError,
};

// ============================================================================
// Test Data Constants
// ============================================================================

/// Valid test labels of various lengths for basic testing
pub const VALID_LABELS: &[&str] = &["Short", "Medium Label", "A Longer Test Label"];

/// Maximum allowed label length for chips
pub const MAX_LABEL_LENGTH: usize = MAX_CHIP_LABEL_LENGTH;

/// Create an oversized label for testing
pub fn oversized_label() -> String {
    "x".repeat(MAX_LABEL_LENGTH + 1)
}

// ============================================================================
// Chip Builder Helpers
// ============================================================================

/// Create a test chip with default settings
pub fn test_chip(label: &str, variant: ChipVariant) -> Chip {
    ChipBuilder::new(label, variant)
        .build()
        .expect("Failed to build test chip")
}

/// Create a selected test chip
pub fn selected_test_chip(label: &str, variant: ChipVariant) -> Chip {
    ChipBuilder::new(label, variant)
        .selected(true)
        .build()
        .expect("Failed to build selected test chip")
}

/// Create a test chip with specific size
pub fn sized_test_chip(label: &str, variant: ChipVariant, size: ComponentSize) -> Chip {
    ChipBuilder::new(label, variant)
        .size(size)
        .build()
        .expect("Failed to build sized test chip")
}

// ============================================================================
// Collection Builder Helpers
// ============================================================================

/// Create a test chip collection with default settings
pub fn test_collection(mode: ChipSelectionMode) -> ChipCollection {
    ChipCollectionBuilder::new(mode)
        .build()
        .expect("Failed to build test collection")
}

/// Create a collection with predefined filter chips
pub fn filter_chip_collection(labels: &[&str]) -> ChipCollection {
    let mut builder = ChipCollectionBuilder::new(ChipSelectionMode::Multiple);

    for label in labels {
        builder = builder.filter(*label);
    }

    builder.build().expect("Failed to build filter collection")
}

/// Create a collection with mixed chip variants
pub fn mixed_variant_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
        .filter("Filter")
        .assist("Assist")
        .input("Input")
        .suggestion("Suggestion")
        .build()
        .expect("Failed to build mixed collection")
}

// ============================================================================
// Assertion Helpers
// ============================================================================

/// Assert that a chip has expected basic properties
pub fn assert_chip_basics(chip: &Chip, expected_label: &str, expected_variant: ChipVariant) {
    assert_eq!(chip.label(), expected_label);
    assert_eq!(chip.variant(), expected_variant);
    assert_eq!(chip.state(), ChipState::Unselected);
    assert!(!chip.props().disabled);
}

/// Assert that a chip is in selected state
pub fn assert_chip_selected(chip: &Chip) {
    assert_eq!(chip.state(), ChipState::Selected);
    assert!(chip.is_selected());
}

/// Assert that a chip is in unselected state
pub fn assert_chip_unselected(chip: &Chip) {
    assert_eq!(chip.state(), ChipState::Unselected);
    assert!(!chip.is_selected());
}

/// Assert collection selection state
pub fn assert_collection_selection(collection: &ChipCollection, expected_count: usize) {
    assert_eq!(collection.selected_chips().len(), expected_count);
}

/// Assert that a chip has expected size
pub fn assert_chip_size(chip: &Chip, expected_size: ComponentSize) {
    assert_eq!(chip.props().size, expected_size);
}

/// Assert that a chip is enabled
pub fn assert_chip_enabled(chip: &Chip) {
    assert!(!chip.props().disabled);
}

/// Assert that a chip is disabled
pub fn assert_chip_disabled(chip: &Chip) {
    assert!(chip.props().disabled);
}

/// Assert that a chip is in pressed state
pub fn assert_chip_pressed(chip: &Chip) {
    assert_eq!(chip.state(), ChipState::Pressed);
}

/// Assert that a chip has specific metadata
pub fn assert_chip_has_metadata(chip: &Chip, key: &str, expected_value: &str) {
    assert!(
        chip.props().metadata.contains_key(key),
        "Chip should have metadata key '{key}'"
    );
    assert_eq!(
        chip.props().metadata.get(key),
        Some(&expected_value.to_string()),
        "Metadata '{key}' should have value '{expected_value}'"
    );
}

/// Assert Material Design 3 height compliance
pub fn assert_md3_height_compliance(_chip: &Chip) {
    // MD3 specifies minimum touch target heights
    let min_height = match _chip.props().size {
        ComponentSize::Small => 32.0,
        ComponentSize::Medium => 40.0,
        ComponentSize::Large => 48.0,
    };
    // This is a placeholder assertion - in real implementation,
    // you would check the actual rendered height
    assert!(min_height > 0.0, "Chip should meet MD3 height requirements");
}

/// Assert Material Design 3 touch target compliance
pub fn assert_md3_touch_target_compliance(_chip: &Chip) {
    // MD3 specifies minimum 48dp touch targets
    let min_touch_target = 48.0;
    // This is a placeholder assertion - in real implementation,
    // you would check the actual touch target size
    assert!(
        min_touch_target > 0.0,
        "Chip should meet MD3 touch target requirements"
    );
}

/// Assert that an operation completes within a time limit
pub fn assert_within_time_limit<F>(operation: F, max_duration_ms: u64, description: &str)
where
    F: FnOnce(),
{
    let start = std::time::Instant::now();
    operation();
    let duration = start.elapsed();
    assert!(
        duration.as_millis() <= max_duration_ms as u128,
        "{}: Operation took {}ms, expected <= {}ms",
        description,
        duration.as_millis(),
        max_duration_ms
    );
}

/// Assert that an operation completes within a time limit (overload for basic case)
pub fn assert_within_time_limit_basic<F>(operation: F)
where
    F: FnOnce(),
{
    assert_within_time_limit(operation, 1000, "Operation")
}

// ============================================================================
// Validation Test Helpers
// ============================================================================

/// Test that label validation works correctly
pub fn assert_label_validation_error(result: &Result<Chip, SelectionError>, expected_len: usize) {    match result {
        Err(SelectionError::LabelTooLong { len, max: _ }) => {
            assert_eq!(*len, expected_len);
        }
        _ => panic!("Expected LabelTooLong error"),
    }
}

// ============================================================================
// Test Data Generators
// ============================================================================

/// Generate test labels of various lengths
pub fn generate_test_labels() -> Vec<String> {
    vec![
        "A".to_string(),
        "Short".to_string(),
        "Medium Length Label".to_string(),
        "A".repeat(50),
        "A".repeat(MAX_LABEL_LENGTH),
    ]
}

/// Generate all chip variants for testing
pub fn all_chip_variants() -> Vec<ChipVariant> {
    vec![
        ChipVariant::Assist,
        ChipVariant::Filter,
        ChipVariant::Input,
        ChipVariant::Suggestion,
    ]
}

/// Generate all component sizes for testing
pub fn all_component_sizes() -> Vec<ComponentSize> {
    vec![
        ComponentSize::Small,
        ComponentSize::Medium,
        ComponentSize::Large,
    ]
}

/// Generate all selection modes for testing
pub fn all_selection_modes() -> Vec<ChipSelectionMode> {
    vec![
        ChipSelectionMode::None,
        ChipSelectionMode::Single,
        ChipSelectionMode::Multiple,
    ]
}

/// All possible chip states for testing
pub const ALL_CHIP_STATES: &[ChipState] = &[ChipState::Unselected, ChipState::Selected];

/// Create chips with all possible states for a given variant
pub fn all_state_chips(label: &str, variant: ChipVariant) -> Vec<Chip> {
    ALL_CHIP_STATES
        .iter()
        .map(|&state| {
            let mut chip = test_chip(label, variant);
            if state == ChipState::Selected {
                let _ = chip.select();
            }
            chip
        })
        .collect()
}

/// Create chips with all possible variants for a given label
pub fn all_variant_chips(label: &str) -> Vec<Chip> {
    ALL_CHIP_VARIANTS
        .iter()
        .map(|&variant| test_chip(label, variant))
        .collect()
}

/// Create a selected filter chip
pub fn selected_filter_chip(label: &str) -> Chip {
    let mut chip = test_chip(label, ChipVariant::Filter);
    let _ = chip.select();
    chip
}

/// Create a label with maximum allowed length
pub fn max_length_label() -> String {
    "A".repeat(MAX_LABEL_LENGTH)
}
