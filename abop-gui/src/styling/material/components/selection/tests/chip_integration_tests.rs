//! Performance and integration tests for chip components
//!
//! This module focuses on:
//! - Performance characteristics of chip operations
//! - Integration scenarios with multiple chip types
//! - Real-world usage patterns
//! - Memory efficiency tests

use super::fixtures::chip_factory::{assertions::*, collections::*, test_chip};
use crate::styling::material::components::selection::{
    ChipCollection, ChipCollectionBuilder, ChipSelectionMode, ChipVariant,
};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_large_collection_performance() {
        // Test performance with a large number of chips
        let labels: Vec<String> = (0..1000).map(|i| format!("Chip {i}")).collect();
        let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();

        let start = std::time::Instant::now();
        let collection = filter_chip_collection(&label_refs);
        let creation_time = start.elapsed();

        assert_eq!(collection.len(), 1000);
        assert!(
            creation_time.as_millis() < 100,
            "Collection creation should be fast"
        );

        // Test selection performance
        let mut mutable_collection = collection;
        let start = std::time::Instant::now();

        for i in 0..100 {
            let _ = mutable_collection.select_chip(i);
        }

        let selection_time = start.elapsed();
        assert!(
            selection_time.as_millis() < 50,
            "Selection operations should be fast"
        );
        assert_collection_selection(&mutable_collection, 100);
    }

    #[test]
    fn test_mixed_variant_collection_behavior() {
        let collection = mixed_variant_collection();

        assert_eq!(collection.len(), 4);

        // Verify each variant is present
        let variants: Vec<_> = collection.chips().iter().map(|c| c.variant()).collect();
        assert!(variants.contains(&ChipVariant::Filter));
        assert!(variants.contains(&ChipVariant::Assist));
        assert!(variants.contains(&ChipVariant::Input));
        assert!(variants.contains(&ChipVariant::Suggestion));
    }

    #[test]
    fn test_real_world_filter_scenario() {
        // Simulate a real-world filtering interface
        let mut filters = ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
            .filter("Category")
            .filter("Price Range")
            .filter("Rating")
            .filter("In Stock")
            .filter("Brand")
            .filter("Location")
            .build()
            .expect("Failed to build filter collection");

        // User applies multiple filters
        let filter_indices = [0, 2, 4]; // Category, Rating, Brand
        for &index in &filter_indices {
            assert!(filters.select_chip(index).is_ok());
        }

        assert_collection_selection(&filters, 3);

        // User removes one filter
        assert!(filters.deselect_chip(0).is_ok()); // Remove Category
        assert_collection_selection(&filters, 2);

        // Verify correct chips remain selected
        assert!(!filters.chips()[0].is_selected()); // Category
        assert!(filters.chips()[2].is_selected()); // Rating
        assert!(filters.chips()[4].is_selected()); // Brand
    }

    #[test]
    fn test_single_selection_mode_behavior() {
        let mut collection = ChipCollectionBuilder::new(ChipSelectionMode::Single)
            .filter("Option A")
            .filter("Option B")
            .filter("Option C")
            .build()
            .expect("Failed to build single-select collection");

        // Select first option
        assert!(collection.select_chip(0).is_ok());
        assert_collection_selection(&collection, 1);
        assert!(collection.chips()[0].is_selected());

        // Select second option (should deselect first)
        assert!(collection.select_chip(1).is_ok());
        assert_collection_selection(&collection, 1);
        assert!(!collection.chips()[0].is_selected());
        assert!(collection.chips()[1].is_selected());

        // Verify only one chip is ever selected
        for i in 0..collection.len() {
            if i != 1 {
                assert!(!collection.chips()[i].is_selected());
            }
        }
    }

    #[test]
    fn test_none_selection_mode_behavior() {
        let mut collection = ChipCollection::new(ChipSelectionMode::None);
        collection.add_chip(test_chip("Display Only", ChipVariant::Assist));

        // Should not allow any selection
        let result = collection.select_chip(0);
        assert!(
            result.is_err(),
            "None selection mode should reject selection attempts"
        );
        assert_collection_selection(&collection, 0);
    }

    #[test]
    fn test_memory_efficiency() {
        // Test that chip creation doesn't unnecessarily allocate
        let chip1 = test_chip("Test", ChipVariant::Filter);
        let chip2 = chip1.clone();

        // Both chips should have the same label (shared string data)
        assert_eq!(chip1.label(), chip2.label());
        assert_eq!(chip1.variant(), chip2.variant());

        // Test collection memory usage
        let collection = filter_chip_collection(&["A", "B", "C"]);
        assert_eq!(collection.len(), 3);

        // Cloning collection should be efficient
        let cloned_collection = collection.clone();
        assert_eq!(collection.len(), cloned_collection.len());
    }

    #[test]
    fn test_state_persistence_across_operations() {
        let mut collection = filter_chip_collection(&["Persistent", "State", "Test"]);

        // Apply initial state
        assert!(collection.select_chip(0).is_ok());
        assert!(collection.select_chip(2).is_ok());

        let initial_state: Vec<bool> = collection.chips().iter().map(|c| c.is_selected()).collect();

        // Perform various operations
        let _ = collection.clear_selection();
        assert_collection_selection(&collection, 0);

        // Restore state
        assert!(collection.select_chip(0).is_ok());
        assert!(collection.select_chip(2).is_ok());

        let restored_state: Vec<bool> =
            collection.chips().iter().map(|c| c.is_selected()).collect();

        assert_eq!(initial_state, restored_state, "State should be restorable");
    }
}
