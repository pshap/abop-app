//! Collection Behavior Tests
//! 
//! Tests for chip collection management, selection modes, and group behaviors.
//! Validates multi-chip interactions, selection patterns, and state synchronization.

use super::fixtures::{
    chip_factory::*,
    collection_factory::*,
    assertion_helpers::*,
    test_data::*,
};
use crate::styling::material::components::selection::chip::{
    core::{Chip, ChipState, ChipVariant, SelectionMode},
    collection::{ChipCollection, CollectionConfig, SelectionConfig},
};
use std::collections::HashSet;
use test_case::test_case;

#[cfg(test)]
mod collection_creation_tests {
    use super::*;

    #[test]
    fn test_empty_collection_creation() {
        let collection = ChipCollection::new();
        
        assert_eq!(collection.len(), 0);
        assert!(collection.is_empty());
        assert_eq!(collection.selected_count(), 0);
        assert!(collection.selected_ids().is_empty());
    }

    #[test]
    fn test_collection_with_initial_chips() {
        let chips = create_test_chip_set(3);
        let collection = ChipCollection::from_chips(chips.clone());
        
        assert_eq!(collection.len(), 3);
        assert!(!collection.is_empty());
        assert_eq!(collection.selected_count(), 0);
        
        for chip in &chips {
            assert!(collection.contains(&chip.id));
        }
    }

    #[test]
    fn test_collection_with_config() {
        let config = CollectionConfig {
            max_chips: Some(5),
            allow_duplicates: false,
            auto_sort: true,
        };
        
        let collection = ChipCollection::with_config(config.clone());
        
        assert_eq!(collection.config(), &config);
        assert_eq!(collection.len(), 0);
    }

    #[test_case(1; "single chip")]
    #[test_case(5; "multiple chips")]
    #[test_case(50; "large collection")]
    fn test_collection_capacity(chip_count: usize) {
        let chips = create_test_chip_set(chip_count);
        let collection = ChipCollection::from_chips(chips);
        
        assert_eq!(collection.len(), chip_count);
        assert_eq!(collection.capacity() >= chip_count, true);
    }
}

#[cfg(test)]
mod selection_mode_tests {
    use super::*;

    #[test]
    fn test_single_selection_mode() {
        let mut collection = create_single_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Select first chip
        collection.select(&chip_ids[0]).unwrap();
        assert_eq!(collection.selected_count(), 1);
        assert!(collection.is_selected(&chip_ids[0]));
        
        // Select second chip - should deselect first
        collection.select(&chip_ids[1]).unwrap();
        assert_eq!(collection.selected_count(), 1);
        assert!(!collection.is_selected(&chip_ids[0]));
        assert!(collection.is_selected(&chip_ids[1]));
    }

    #[test]
    fn test_multiple_selection_mode() {
        let mut collection = create_multiple_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Select multiple chips
        for &id in &chip_ids[0..3] {
            collection.select(&id).unwrap();
        }
        
        assert_eq!(collection.selected_count(), 3);
        for &id in &chip_ids[0..3] {
            assert!(collection.is_selected(&id));
        }
    }

    #[test]
    fn test_no_selection_mode() {
        let mut collection = create_no_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Attempt to select should fail
        let result = collection.select(&chip_ids[0]);
        assert!(result.is_err());
        assert_eq!(collection.selected_count(), 0);
    }

    #[test]
    fn test_selection_mode_change() {
        let mut collection = create_multiple_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Select multiple chips
        collection.select(&chip_ids[0]).unwrap();
        collection.select(&chip_ids[1]).unwrap();
        assert_eq!(collection.selected_count(), 2);
        
        // Change to single selection mode
        collection.set_selection_mode(SelectionMode::Single);
        assert_eq!(collection.selected_count(), 1); // Should keep only one
    }
}

#[cfg(test)]
mod chip_manipulation_tests {
    use super::*;

    #[test]
    fn test_add_chip_to_collection() {
        let mut collection = ChipCollection::new();
        let chip = create_test_chip("test", ChipVariant::Filter);
        let chip_id = chip.id.clone();
        
        collection.add(chip).unwrap();
        
        assert_eq!(collection.len(), 1);
        assert!(collection.contains(&chip_id));
    }

    #[test]
    fn test_remove_chip_from_collection() {
        let mut collection = create_test_collection(5);
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        let remove_id = chip_ids[0].clone();
        
        let removed_chip = collection.remove(&remove_id).unwrap();
        
        assert_eq!(collection.len(), 4);
        assert!(!collection.contains(&remove_id));
        assert_eq!(removed_chip.id, remove_id);
    }

    #[test]
    fn test_update_chip_in_collection() {
        let mut collection = create_test_collection(3);
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        let update_id = chip_ids[0].clone();
        
        let updated_chip = Chip::new("updated_label", ChipVariant::Filter)
            .with_id(update_id.clone());
        
        collection.update(updated_chip).unwrap();
        
        let chip = collection.get(&update_id).unwrap();
        assert_eq!(chip.label, "updated_label");
    }

    #[test]
    fn test_duplicate_prevention() {
        let config = CollectionConfig {
            max_chips: None,
            allow_duplicates: false,
            auto_sort: false,
        };
        let mut collection = ChipCollection::with_config(config);
        
        let chip1 = create_test_chip("test", ChipVariant::Filter);
        let chip2 = create_test_chip("test", ChipVariant::Filter);
        
        collection.add(chip1).unwrap();
        let result = collection.add(chip2);
        
        assert!(result.is_err());
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn test_collection_capacity_limit() {
        let config = CollectionConfig {
            max_chips: Some(2),
            allow_duplicates: true,
            auto_sort: false,
        };
        let mut collection = ChipCollection::with_config(config);
        
        // Add chips up to limit
        collection.add(create_test_chip("chip1", ChipVariant::Filter)).unwrap();
        collection.add(create_test_chip("chip2", ChipVariant::Filter)).unwrap();
        
        // Adding beyond limit should fail
        let result = collection.add(create_test_chip("chip3", ChipVariant::Filter));
        assert!(result.is_err());
        assert_eq!(collection.len(), 2);
    }
}

#[cfg(test)]
mod selection_operations_tests {
    use super::*;

    #[test]
    fn test_select_all() {
        let mut collection = create_multiple_selection_collection();
        let initial_count = collection.len();
        
        collection.select_all().unwrap();
        
        assert_eq!(collection.selected_count(), initial_count);
        for id in collection.chip_ids() {
            assert!(collection.is_selected(&id));
        }
    }

    #[test]
    fn test_deselect_all() {
        let mut collection = create_multiple_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Select some chips first
        for &id in &chip_ids[0..3] {
            collection.select(&id).unwrap();
        }
        
        collection.deselect_all();
        
        assert_eq!(collection.selected_count(), 0);
        for id in collection.chip_ids() {
            assert!(!collection.is_selected(&id));
        }
    }

    #[test]
    fn test_toggle_selection() {
        let mut collection = create_multiple_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        let target_id = &chip_ids[0];
        
        // Initially not selected
        assert!(!collection.is_selected(target_id));
        
        // Toggle to selected
        collection.toggle_selection(target_id).unwrap();
        assert!(collection.is_selected(target_id));
        
        // Toggle back to not selected
        collection.toggle_selection(target_id).unwrap();
        assert!(!collection.is_selected(target_id));
    }

    #[test]
    fn test_select_range() {
        let mut collection = create_multiple_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        if chip_ids.len() >= 5 {
            collection.select_range(&chip_ids[1], &chip_ids[3]).unwrap();
            
            // Check that chips 1, 2, 3 are selected
            for i in 1..=3 {
                assert!(collection.is_selected(&chip_ids[i]));
            }
            
            // Check that chips 0 and 4+ are not selected
            assert!(!collection.is_selected(&chip_ids[0]));
            if chip_ids.len() > 4 {
                assert!(!collection.is_selected(&chip_ids[4]));
            }
        }
    }

    #[test]
    fn test_invert_selection() {
        let mut collection = create_multiple_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Select first half
        let mid_point = chip_ids.len() / 2;
        for &id in &chip_ids[0..mid_point] {
            collection.select(&id).unwrap();
        }
        
        let initially_selected: HashSet<_> = collection.selected_ids().collect();
        
        collection.invert_selection().unwrap();
        
        // Check that previously selected are now deselected
        for id in &initially_selected {
            assert!(!collection.is_selected(id));
        }
        
        // Check that previously deselected are now selected
        for &id in &chip_ids[mid_point..] {
            assert!(collection.is_selected(&id));
        }
    }
}

#[cfg(test)]
mod state_synchronization_tests {
    use super::*;

    #[test]
    fn test_chip_state_sync_on_selection() {
        let mut collection = create_single_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        let target_id = &chip_ids[0];
        
        collection.select(target_id).unwrap();
        
        let chip = collection.get(target_id).unwrap();
        assert_eq!(chip.state, ChipState::Selected);
    }

    #[test]
    fn test_chip_state_sync_on_deselection() {
        let mut collection = create_single_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        let target_id = &chip_ids[0];
        
        collection.select(target_id).unwrap();
        collection.deselect(target_id).unwrap();
        
        let chip = collection.get(target_id).unwrap();
        assert_eq!(chip.state, ChipState::Enabled);
    }

    #[test]
    fn test_disabled_chip_cannot_be_selected() {
        let mut collection = create_test_collection(3);
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        let target_id = &chip_ids[0];
        
        // Disable the chip
        let mut chip = collection.get(target_id).unwrap().clone();
        chip.state = ChipState::Disabled;
        collection.update(chip).unwrap();
        
        // Attempt to select disabled chip should fail
        let result = collection.select(target_id);
        assert!(result.is_err());
        assert!(!collection.is_selected(target_id));
    }

    #[test]
    fn test_collection_state_consistency() {
        let mut collection = create_multiple_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Perform various operations
        collection.select(&chip_ids[0]).unwrap();
        collection.select(&chip_ids[1]).unwrap();
        collection.deselect(&chip_ids[0]).unwrap();
        collection.toggle_selection(&chip_ids[2]).unwrap();
        
        // Verify state consistency
        assert_chip_collection_state_consistency(&collection);
    }
}

#[cfg(test)]
mod filtering_and_search_tests {
    use super::*;

    #[test]
    fn test_filter_by_variant() {
        let collection = create_mixed_variant_collection();
        
        let filter_chips = collection.filter_by_variant(ChipVariant::Filter);
        let action_chips = collection.filter_by_variant(ChipVariant::Action);
        
        assert!(filter_chips.len() > 0);
        assert!(action_chips.len() > 0);
        
        for chip in &filter_chips {
            assert_eq!(chip.variant, ChipVariant::Filter);
        }
        
        for chip in &action_chips {
            assert_eq!(chip.variant, ChipVariant::Action);
        }
    }

    #[test]
    fn test_filter_by_state() {
        let mut collection = create_test_collection(5);
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Select some chips
        collection.select(&chip_ids[0]).unwrap();
        collection.select(&chip_ids[1]).unwrap();
        
        let selected_chips = collection.filter_by_state(ChipState::Selected);
        let enabled_chips = collection.filter_by_state(ChipState::Enabled);
        
        assert_eq!(selected_chips.len(), 2);
        assert_eq!(enabled_chips.len(), 3);
    }

    #[test]
    fn test_search_by_label() {
        let collection = create_searchable_collection();
        
        let results = collection.search("test");
        assert!(results.len() > 0);
        
        for chip in &results {
            assert!(chip.label.to_lowercase().contains("test"));
        }
    }

    #[test]
    fn test_search_case_insensitive() {
        let collection = create_searchable_collection();
        
        let lower_results = collection.search("test");
        let upper_results = collection.search("TEST");
        let mixed_results = collection.search("TeSt");
        
        assert_eq!(lower_results.len(), upper_results.len());
        assert_eq!(lower_results.len(), mixed_results.len());
    }

    #[test]
    fn test_search_empty_query() {
        let collection = create_test_collection(5);
        
        let results = collection.search("");
        assert_eq!(results.len(), collection.len());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_collection_operations() {
        let large_collection = create_large_test_collection(LARGE_COLLECTION_SIZE);
        
        // Test that basic operations complete in reasonable time
        let start = std::time::Instant::now();
        
        let count = large_collection.len();
        let _selected = large_collection.selected_count();
        let _first = large_collection.chip_ids().next();
        
        let duration = start.elapsed();
        
        assert_eq!(count, LARGE_COLLECTION_SIZE);
        assert!(duration.as_millis() < PERFORMANCE_THRESHOLD_MS);
    }

    #[test]
    fn test_selection_performance() {
        let mut large_collection = create_large_multiple_selection_collection(1000);
        let chip_ids: Vec<_> = large_collection.chip_ids().collect();
        
        let start = std::time::Instant::now();
        
        // Select every 10th chip
        for (i, id) in chip_ids.iter().enumerate() {
            if i % 10 == 0 {
                large_collection.select(id).unwrap();
            }
        }
        
        let duration = start.elapsed();
        
        assert_eq!(large_collection.selected_count(), 100);
        assert!(duration.as_millis() < PERFORMANCE_THRESHOLD_MS * 2);
    }

    #[test]
    fn test_search_performance() {
        let large_collection = create_large_searchable_collection(1000);
        
        let start = std::time::Instant::now();
        let _results = large_collection.search("test");
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < PERFORMANCE_THRESHOLD_MS);
    }
}

#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_operations_on_empty_collection() {
        let mut collection = ChipCollection::new();
        
        // All operations should handle empty collection gracefully
        assert!(collection.select_all().is_ok());
        collection.deselect_all();
        assert!(collection.invert_selection().is_ok());
        
        let results = collection.search("anything");
        assert!(results.is_empty());
        
        let filtered = collection.filter_by_variant(ChipVariant::Filter);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_operations_with_invalid_ids() {
        let mut collection = create_test_collection(3);
        let invalid_id = "nonexistent_id".to_string();
        
        assert!(collection.select(&invalid_id).is_err());
        assert!(collection.deselect(&invalid_id).is_err());
        assert!(collection.toggle_selection(&invalid_id).is_err());
        assert!(collection.remove(&invalid_id).is_err());
        assert!(collection.get(&invalid_id).is_none());
    }

    #[test]
    fn test_concurrent_modifications() {
        let mut collection = create_test_collection(5);
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Simulate concurrent-like operations
        collection.select(&chip_ids[0]).unwrap();
        collection.remove(&chip_ids[1]).unwrap();
        collection.select(&chip_ids[2]).unwrap();
        
        // Collection should maintain consistency
        assert_chip_collection_state_consistency(&collection);
        assert_eq!(collection.len(), 4);
        assert_eq!(collection.selected_count(), 2);
    }
}
