//! Test helpers for chip component testing
//!
//! This module provides reusable test utilities and builders for chip tests,
//! reducing code duplication and ensuring consistent test patterns.

use crate::styling::material::components::selection::{
    Chip, ChipBuilder, ChipCollection, ChipCollectionBuilder, ChipSelectionMode, ChipState,
    ChipVariant, ComponentSize, SelectionError,
};
use crate::styling::material::components::selection::builder::ComponentBuilder;
use crate::styling::material::components::selection::chip::core::MAX_CHIP_LABEL_LENGTH;

// ============================================================================
// Test Data Constants  
// ============================================================================

pub const VALID_LABELS: &[&str] = &["Short", "Medium Label", "A Longer Test Label"];
pub const MAX_LABEL_LENGTH: usize = MAX_CHIP_LABEL_LENGTH;

/// Create an oversized label for testing validation
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

// ============================================================================
// Validation Test Helpers
// ============================================================================

/// Test that label validation works correctly
pub fn assert_label_validation_error(result: Result<Chip, SelectionError>, expected_len: usize) {
    match result {
        Err(SelectionError::LabelTooLong { len, max: _ }) => {
            assert_eq!(len, expected_len);
        }
        _ => panic!("Expected LabelTooLong error"),
    }
}

/// Test that empty label validation works
pub fn assert_empty_label_error(result: Result<Chip, SelectionError>) {
    match result {
        Err(SelectionError::InvalidLabel { reason }) => {
            assert!(reason.contains("empty") || reason.contains("must have"));
        }
        _ => panic!("Expected InvalidLabel error for empty label"),
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
