//! Factory functions for creating test chip collections
//!
//! This module provides comprehensive factory functions for creating chip
//! collections in various configurations for testing purposes.

use crate::styling::material::components::selection::{
    ChipCollection, ChipCollectionBuilder, ChipSelectionMode, SelectionError,
};
use super::{chip_factory::*, test_data::*};

// ============================================================================
// Basic Collection Factories
// ============================================================================

/// Create an empty test collection with specified selection mode
pub fn test_collection(mode: ChipSelectionMode) -> ChipCollection {
    ChipCollectionBuilder::new(mode)
        .build()
        .expect("Failed to build test collection")
}

/// Create a collection with predefined filter chips
pub fn filter_collection(labels: &[&str]) -> ChipCollection {
    let mut builder = ChipCollectionBuilder::new(ChipSelectionMode::Multiple);
    
    for label in labels {
        builder = builder.filter(*label);
    }
    
    builder.build().expect("Failed to build filter collection")
}

/// Create a single-select collection
pub fn single_select_collection(labels: &[&str]) -> ChipCollection {
    let mut builder = ChipCollectionBuilder::new(ChipSelectionMode::Single);
    
    for label in labels {
        builder = builder.filter(*label);
    }
    
    builder.build().expect("Failed to build single-select collection")
}

/// Create a multiple-select collection
pub fn multiple_select_collection(labels: &[&str]) -> ChipCollection {
    let mut builder = ChipCollectionBuilder::new(ChipSelectionMode::Multiple);
    
    for label in labels {
        builder = builder.filter(*label);
    }
    
    builder.build().expect("Failed to build multiple-select collection")
}

/// Create a non-selectable collection
pub fn display_only_collection(labels: &[&str]) -> ChipCollection {
    let mut builder = ChipCollectionBuilder::new(ChipSelectionMode::None);
    
    for label in labels {
        builder = builder.assist(*label); // Assist chips for display
    }
    
    builder.build().expect("Failed to build display-only collection")
}

// ============================================================================
// Variant-Specific Collection Factories
// ============================================================================

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

/// Create a collection of filter chips only
pub fn filter_only_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
        .filters(FILTER_CHIP_LABELS.iter().map(|s| s.to_string()))
        .build()
        .expect("Failed to build filter-only collection")
}

/// Create a collection of input chips only
pub fn input_only_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::None)
        .inputs(INPUT_CHIP_LABELS.iter().map(|s| s.to_string()))
        .build()
        .expect("Failed to build input-only collection")
}

/// Create a collection of assist chips only
pub fn assist_only_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::None)
        .assists(ASSIST_CHIP_LABELS.iter().map(|s| s.to_string()))
        .build()
        .expect("Failed to build assist-only collection")
}

/// Create a collection of suggestion chips only
pub fn suggestion_only_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::Single)
        .suggestions(SUGGESTION_CHIP_LABELS.iter().map(|s| s.to_string()))
        .build()
        .expect("Failed to build suggestion-only collection")
}

// ============================================================================
// Size and Configuration Factories
// ============================================================================

/// Create collection with all chips having the same size
pub fn uniform_size_collection(mode: ChipSelectionMode, size: crate::styling::material::components::selection::ComponentSize) -> ChipCollection {
    ChipCollectionBuilder::new(mode)
        .size(size)
        .filters(VALID_LABELS.iter().map(|s| s.to_string()))
        .build()
        .expect("Failed to build uniform size collection")
}

/// Create collection with disabled chips
pub fn disabled_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
        .disabled(true)
        .filters(VALID_LABELS.iter().take(5).map(|s| s.to_string()))
        .build()
        .expect("Failed to build disabled collection")
}

/// Create collection with compact spacing
pub fn compact_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
        .compact_spacing()
        .filters(VALID_LABELS.iter().map(|s| s.to_string()))
        .build()
        .expect("Failed to build compact collection")
}

/// Create collection with comfortable spacing
pub fn comfortable_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
        .comfortable_spacing()
        .filters(VALID_LABELS.iter().map(|s| s.to_string()))
        .build()
        .expect("Failed to build comfortable collection")
}

// ============================================================================
// Pre-Selected Collection Factories
// ============================================================================

/// Create collection with some chips pre-selected
pub fn pre_selected_collection(
    mode: ChipSelectionMode,
    labels: &[&str],
    selected_indices: &[usize]
) -> ChipCollection {
    let mut collection = filter_collection(labels);
    
    // Apply selection mode after creation
    let mut new_collection = ChipCollection::new(mode);
    for chip in collection.chips() {
        new_collection.add_chip(chip.clone());
    }
    
    // Select specified chips
    for &index in selected_indices {
        if index < new_collection.len() {
            let _ = new_collection.select_chip(index);
        }
    }
    
    new_collection
}

/// Create collection with maximum selections
pub fn max_selected_collection(mode: ChipSelectionMode) -> ChipCollection {
    let labels = VALID_LABELS;
    match mode {
        ChipSelectionMode::None => display_only_collection(labels),
        ChipSelectionMode::Single => {
            pre_selected_collection(mode, labels, &[0])
        }
        ChipSelectionMode::Multiple => {
            pre_selected_collection(mode, labels, &(0..labels.len()).collect::<Vec<_>>())
        }
    }
}

// ============================================================================
// Performance Testing Factories
// ============================================================================

/// Create small collection for performance testing
pub fn small_performance_collection() -> ChipCollection {
    let labels: Vec<String> = (0..SMALL_COLLECTION_SIZE)
        .map(|i| format!("Chip {}", i))
        .collect();
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
    filter_collection(&label_refs)
}

/// Create medium collection for performance testing
pub fn medium_performance_collection() -> ChipCollection {
    let labels: Vec<String> = (0..MEDIUM_COLLECTION_SIZE)
        .map(|i| format!("Chip {}", i))
        .collect();
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
    filter_collection(&label_refs)
}

/// Create large collection for performance testing
pub fn large_performance_collection() -> ChipCollection {
    let labels: Vec<String> = (0..LARGE_COLLECTION_SIZE)
        .map(|i| format!("Chip {}", i))
        .collect();
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
    filter_collection(&label_refs)
}

/// Create stress test collection
pub fn stress_test_collection() -> ChipCollection {
    let labels: Vec<String> = (0..STRESS_TEST_COLLECTION_SIZE)
        .map(|i| format!("Chip {}", i))
        .collect();
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
    filter_collection(&label_refs)
}

// ============================================================================
// Real-World Scenario Factories
// ============================================================================

/// Create collection for search interface testing
pub fn search_interface_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
        .filter("Category")
        .filter("Price Range")
        .filter("Rating")
        .filter("In Stock")
        .filter("Brand")
        .filter("Location")
        .filter("Date Added")
        .build()
        .expect("Failed to build search interface collection")
}

/// Create collection for tag input interface testing
pub fn tag_interface_collection() -> ChipCollection {
    let mut collection = ChipCollection::new(ChipSelectionMode::None);
    
    for &label in INPUT_CHIP_LABELS.iter().take(5) {
        collection.add_chip(deletable_input_chip(label));
    }
    
    collection
}

/// Create collection for help interface testing
pub fn help_interface_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::None)
        .assists(ASSIST_CHIP_LABELS.iter().map(|s| s.to_string()))
        .build()
        .expect("Failed to build help interface collection")
}

/// Create collection for action selection interface testing
pub fn action_selection_collection() -> ChipCollection {
    ChipCollectionBuilder::new(ChipSelectionMode::Single)
        .suggestions(SUGGESTION_CHIP_LABELS.iter().map(|s| s.to_string()))
        .build()
        .expect("Failed to build action selection collection")
}

// ============================================================================
// Error Case Factories
// ============================================================================

/// Attempt to create collection with invalid configuration (should fail)
pub fn invalid_collection() -> Result<ChipCollection, SelectionError> {
    // Try to create collection with chip that has oversized label
    let mut builder = ChipCollectionBuilder::new(ChipSelectionMode::Multiple);
    
    // Add valid chips first
    builder = builder.filter("Valid");
    
    // Now try to add an invalid chip through the collection (will be caught by individual chip validation)
    // Since ChipCollectionBuilder validates chips when building, we'll test through direct collection manipulation
    let mut collection = builder.build().expect("Valid collection should build");
    
    // Try to add invalid chip directly
    match oversized_label_chip(crate::styling::material::components::selection::ChipVariant::Filter) {
        Ok(chip) => {
            collection.add_chip(chip);
            collection.validate() // This should fail
        }
        Err(e) => Err(e), // Return the chip creation error
    }
}

// ============================================================================
// Property-Based Testing Factories
// ============================================================================

/// Create collection with random configuration
pub fn random_collection() -> ChipCollection {
    let mode = random_selection_mode();
    let size = (1..=20).into_iter().nth(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize % 20
    ).unwrap_or(5);
    
    let labels: Vec<String> = (0..size)
        .map(|i| format!("{} {}", random_valid_label(), i))
        .collect();
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
    
    let mut builder = ChipCollectionBuilder::new(mode);
    for label in label_refs {
        builder = builder.filter(label);
    }
    
    builder.build().expect("Failed to build random collection")
}

/// Create multiple random collections
pub fn random_collections(count: usize) -> Vec<ChipCollection> {
    (0..count).map(|_| random_collection()).collect()
}

// ============================================================================
// Boundary Testing Factories
// ============================================================================

/// Create empty collection for boundary testing
pub fn empty_collections() -> Vec<ChipCollection> {
    ALL_SELECTION_MODES
        .iter()
        .map(|&mode| test_collection(mode))
        .collect()
}

/// Create single-item collections for boundary testing
pub fn single_item_collections() -> Vec<ChipCollection> {
    ALL_SELECTION_MODES
        .iter()
        .map(|&mode| {
            let mut builder = ChipCollectionBuilder::new(mode);
            builder = builder.filter("Single Item");
            builder.build().expect("Failed to build single-item collection")
        })
        .collect()
}

/// Create collections with edge case labels
pub fn edge_case_collections() -> Vec<ChipCollection> {
    let mut collections = Vec::new();
    
    for &mode in ALL_SELECTION_MODES {
        let mut builder = ChipCollectionBuilder::new(mode);
        
        // Add chips with edge case labels (only valid ones will be added)
        for &label in EDGE_CASE_LABELS {
            if !label.trim().is_empty() && label.len() <= MAX_LABEL_LENGTH {
                builder = builder.filter(label);
            }
        }
        
        if let Ok(collection) = builder.build() {
            collections.push(collection);
        }
    }
    
    collections
}
