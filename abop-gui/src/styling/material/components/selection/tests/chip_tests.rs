//! Comprehensive tests for chip components
//!
//! Tests cover all chip variants (Assist, Filter, Input, Suggestion),
//! chip collections, selection modes, state management, and builder patterns.

use super::super::chip::*;
use super::super::common::*;

#[cfg(test)]
mod chip_tests {
    use super::*;

    #[test]
    fn test_chip_creation() {
        let chip = Chip::new("test-chip", ChipVariant::Assist);
        assert_eq!(chip.id(), "test-chip");
        assert_eq!(chip.variant(), &ChipVariant::Assist);
        assert_eq!(chip.state(), ChipState::Unselected);
        assert!(!chip.props().disabled);
    }

    #[test]
    fn test_chip_with_label() {
        let chip = Chip::new("chip", ChipVariant::Filter).with_label("Filter Chip");
        assert_eq!(chip.props().label, Some("Filter Chip".to_string()));
    }

    #[test]
    fn test_chip_state_changes() {
        let mut chip = Chip::new("test", ChipVariant::Filter);
        assert_eq!(chip.state(), ChipState::Unselected);

        chip.set_state(ChipState::Selected);
        assert_eq!(chip.state(), ChipState::Selected);
        assert!(chip.is_selected());

        chip.set_state(ChipState::Pressed);
        assert_eq!(chip.state(), ChipState::Pressed);

        chip.set_state(ChipState::Unselected);
        assert!(!chip.is_selected());
    }

    #[test]
    fn test_chip_builder_pattern() {
        let chip = Chip::new("builder-test", ChipVariant::Input)
            .with_label("Input Chip")
            .selected()
            .disabled(true)
            .size(ComponentSize::Large);

        assert_eq!(chip.state(), ChipState::Selected);
        assert!(chip.props().disabled);
        assert_eq!(chip.props().size, ComponentSize::Large);
    }

    #[test]
    fn test_chip_variants() {
        let assist = Chip::new("assist", ChipVariant::Assist);
        let filter = Chip::new("filter", ChipVariant::Filter);
        let input = Chip::new("input", ChipVariant::Input);
        let suggestion = Chip::new("suggestion", ChipVariant::Suggestion);

        assert!(matches!(assist.variant(), ChipVariant::Assist));
        assert!(matches!(filter.variant(), ChipVariant::Filter));
        assert!(matches!(input.variant(), ChipVariant::Input));
        assert!(matches!(suggestion.variant(), ChipVariant::Suggestion));
    }

    #[test]
    fn test_chip_validation() {
        let valid_chip = Chip::new("valid", ChipVariant::Assist).with_label("Valid Label");
        assert!(valid_chip.validate().is_ok());

        let long_label = "x".repeat(201);
        let invalid_chip = Chip::new("invalid", ChipVariant::Filter).with_label(&long_label);
        assert!(invalid_chip.validate().is_err());

        let empty_id_chip = Chip::new("", ChipVariant::Input);
        assert!(empty_id_chip.validate().is_err());
    }

    #[test]
    fn test_chip_trait_implementations() {
        let chip = Chip::new("test", ChipVariant::Filter).selected();

        // Test SelectionWidget trait
        assert!(chip.is_selected());
        assert!(chip.validate().is_ok());

        // Test StatefulWidget trait
        assert_eq!(chip.current_state(), "Selected");

        // Test Clone
        let cloned = chip.clone();
        assert_eq!(chip.id(), cloned.id());
        assert_eq!(chip.state(), cloned.state());
    }

    #[test]
    fn test_chip_convenience_methods() {
        let chip = Chip::new("test", ChipVariant::Filter);

        let selected_chip = chip.clone().selected();
        assert!(selected_chip.is_selected());

        let unselected_chip = selected_chip.clone().unselected();
        assert!(!unselected_chip.is_selected());
    }

    #[test]
    fn test_chip_sizes() {
        let small = Chip::new("s", ChipVariant::Assist).size(ComponentSize::Small);
        let medium = Chip::new("m", ChipVariant::Filter).size(ComponentSize::Medium);
        let large = Chip::new("l", ChipVariant::Input).size(ComponentSize::Large);

        assert_eq!(small.props().size, ComponentSize::Small);
        assert_eq!(medium.props().size, ComponentSize::Medium);
        assert_eq!(large.props().size, ComponentSize::Large);
    }
}

#[cfg(test)]
mod chip_collection_tests {
    use super::*;

    #[test]
    fn test_chip_collection_creation() {
        let collection = ChipCollection::new(SelectionMode::Single);
        assert_eq!(collection.selection_mode(), &SelectionMode::Single);
        assert_eq!(collection.chips().len(), 0);
        assert!(collection.selected_chips().is_empty());
    }

    #[test]
    fn test_chip_collection_add_chips() {
        let mut collection = ChipCollection::new(SelectionMode::Multiple);

        collection.add_chip(Chip::new("chip1", ChipVariant::Filter).with_label("Chip 1"));
        collection.add_chip(Chip::new("chip2", ChipVariant::Filter).with_label("Chip 2"));

        assert_eq!(collection.chips().len(), 2);
        assert!(collection.has_chip("chip1"));
        assert!(collection.has_chip("chip2"));
        assert!(!collection.has_chip("chip3"));
    }

    #[test]
    fn test_chip_collection_single_selection() {
        let mut collection = ChipCollection::new(SelectionMode::Single);

        collection.add_chip(Chip::new("a", ChipVariant::Filter).with_label("A"));
        collection.add_chip(Chip::new("b", ChipVariant::Filter).with_label("B"));
        collection.add_chip(Chip::new("c", ChipVariant::Filter).with_label("C"));

        // Select first chip
        assert!(collection.select_chip("a").is_ok());
        assert_eq!(collection.selected_chips().len(), 1);
        assert!(collection.is_chip_selected("a"));

        // Select second chip - should deselect first
        assert!(collection.select_chip("b").is_ok());
        assert_eq!(collection.selected_chips().len(), 1);
        assert!(!collection.is_chip_selected("a"));
        assert!(collection.is_chip_selected("b"));
    }

    #[test]
    fn test_chip_collection_multiple_selection() {
        let mut collection = ChipCollection::new(SelectionMode::Multiple);

        collection.add_chip(Chip::new("1", ChipVariant::Filter).with_label("One"));
        collection.add_chip(Chip::new("2", ChipVariant::Filter).with_label("Two"));
        collection.add_chip(Chip::new("3", ChipVariant::Filter).with_label("Three"));

        // Select multiple chips
        assert!(collection.select_chip("1").is_ok());
        assert!(collection.select_chip("3").is_ok());

        assert_eq!(collection.selected_chips().len(), 2);
        assert!(collection.is_chip_selected("1"));
        assert!(!collection.is_chip_selected("2"));
        assert!(collection.is_chip_selected("3"));
    }

    #[test]
    fn test_chip_collection_none_selection() {
        let mut collection = ChipCollection::new(SelectionMode::None);

        collection.add_chip(Chip::new("test", ChipVariant::Assist));

        // Should not allow selection
        assert!(collection.select_chip("test").is_err());
        assert!(collection.selected_chips().is_empty());
    }

    #[test]
    fn test_chip_collection_deselection() {
        let mut collection = ChipCollection::new(SelectionMode::Multiple);

        collection.add_chip(Chip::new("chip", ChipVariant::Filter));
        collection.select_chip("chip").unwrap();

        assert!(collection.is_chip_selected("chip"));

        collection.deselect_chip("chip").unwrap();
        assert!(!collection.is_chip_selected("chip"));
    }

    #[test]
    fn test_chip_collection_clear_selection() {
        let mut collection = ChipCollection::new(SelectionMode::Multiple);

        collection.add_chip(Chip::new("a", ChipVariant::Filter));
        collection.add_chip(Chip::new("b", ChipVariant::Filter));

        collection.select_chip("a").unwrap();
        collection.select_chip("b").unwrap();

        assert_eq!(collection.selected_chips().len(), 2);

        collection.clear_selection();
        assert!(collection.selected_chips().is_empty());
    }

    #[test]
    fn test_chip_collection_validation() {
        let mut collection = ChipCollection::new(SelectionMode::Single);
        assert!(collection.validate().is_ok());

        // Add chip with invalid label
        let long_label = "x".repeat(201);
        collection.add_chip(Chip::new("invalid", ChipVariant::Filter).with_label(&long_label));
        assert!(collection.validate().is_err());
    }

    #[test]
    fn test_chip_collection_find_chip() {
        let mut collection = ChipCollection::new(SelectionMode::Single);
        collection.add_chip(Chip::new("test", ChipVariant::Filter).with_label("Test Chip"));

        let chip = collection.find_chip("test");
        assert!(chip.is_some());
        assert_eq!(chip.unwrap().props().label, Some("Test Chip".to_string()));

        assert!(collection.find_chip("nonexistent").is_none());
    }

    #[test]
    fn test_chip_collection_remove_chip() {
        let mut collection = ChipCollection::new(SelectionMode::Multiple);

        collection.add_chip(Chip::new("remove-me", ChipVariant::Filter));
        collection.select_chip("remove-me").unwrap();

        assert!(collection.has_chip("remove-me"));
        assert!(collection.is_chip_selected("remove-me"));

        assert!(collection.remove_chip("remove-me").is_ok());
        assert!(!collection.has_chip("remove-me"));
        assert!(!collection.is_chip_selected("remove-me"));
    }

    #[test]
    fn test_chip_collection_max_selections() {
        let mut collection = ChipCollection::with_max_selections(SelectionMode::Multiple, 2);

        collection.add_chip(Chip::new("1", ChipVariant::Filter));
        collection.add_chip(Chip::new("2", ChipVariant::Filter));
        collection.add_chip(Chip::new("3", ChipVariant::Filter));

        // Should allow up to max selections
        assert!(collection.select_chip("1").is_ok());
        assert!(collection.select_chip("2").is_ok());

        // Should reject additional selection
        assert!(collection.select_chip("3").is_err());
        assert_eq!(collection.selected_chips().len(), 2);
    }
}

#[cfg(test)]
mod chip_collection_builder_tests {
    use super::*;

    #[test]
    fn test_builder_empty() {
        let collection = ChipCollectionBuilder::new(SelectionMode::Single).build();
        assert_eq!(collection.chips().len(), 0);
        assert_eq!(collection.selection_mode(), &SelectionMode::Single);
    }

    #[test]
    fn test_builder_with_chips() {
        let collection = ChipCollectionBuilder::new(SelectionMode::Multiple)
            .chip(Chip::new("1", ChipVariant::Filter).with_label("One"))
            .chip(Chip::new("2", ChipVariant::Filter).with_label("Two"))
            .build();

        assert_eq!(collection.chips().len(), 2);
        assert!(collection.has_chip("1"));
        assert!(collection.has_chip("2"));
    }

    #[test]
    fn test_builder_with_selections() {
        let collection = ChipCollectionBuilder::new(SelectionMode::Multiple)
            .chip(Chip::new("a", ChipVariant::Filter))
            .chip(Chip::new("b", ChipVariant::Filter))
            .chip(Chip::new("c", ChipVariant::Filter))
            .selected(&["a", "c"])
            .build();

        assert!(collection.is_chip_selected("a"));
        assert!(!collection.is_chip_selected("b"));
        assert!(collection.is_chip_selected("c"));
    }

    #[test]
    fn test_builder_with_max_selections() {
        let collection = ChipCollectionBuilder::new(SelectionMode::Multiple)
            .max_selections(3)
            .chip(Chip::new("1", ChipVariant::Filter))
            .chip(Chip::new("2", ChipVariant::Filter))
            .build();

        assert_eq!(collection.max_selections(), Some(3));
    }

    #[test]
    fn test_builder_fluent_api() {
        let collection = ChipCollectionBuilder::new(SelectionMode::Single)
            .chip(Chip::new("small", ChipVariant::Filter).size(ComponentSize::Small))
            .chip(Chip::new("medium", ChipVariant::Filter).size(ComponentSize::Medium))
            .chip(Chip::new("large", ChipVariant::Filter).size(ComponentSize::Large))
            .selected(&["medium"])
            .build();

        assert_eq!(collection.chips().len(), 3);
        assert!(collection.is_chip_selected("medium"));
    }

    #[test]
    fn test_builder_invalid_selections() {
        let collection = ChipCollectionBuilder::new(SelectionMode::Single)
            .chip(Chip::new("valid", ChipVariant::Filter))
            .selected(&["valid", "invalid"]) // "invalid" doesn't exist
            .build();

        // Builder should only select valid chips
        assert!(collection.is_chip_selected("valid"));
        assert_eq!(collection.selected_chips().len(), 1);
    }
}

#[cfg(test)]
mod chip_variant_tests {
    use super::*;

    #[test]
    fn test_assist_chip() {
        let chip = Chip::new("assist", ChipVariant::Assist).with_label("Get help");

        assert!(matches!(chip.variant(), ChipVariant::Assist));
        // Assist chips are typically not selectable in collections
    }

    #[test]
    fn test_filter_chip() {
        let chip = Chip::new("filter", ChipVariant::Filter).with_label("Category");

        assert!(matches!(chip.variant(), ChipVariant::Filter));
        // Filter chips are commonly used in collections for filtering
    }

    #[test]
    fn test_input_chip() {
        let chip = Chip::new("input", ChipVariant::Input).with_label("Tag");

        assert!(matches!(chip.variant(), ChipVariant::Input));
        // Input chips represent user input/tags
    }

    #[test]
    fn test_suggestion_chip() {
        let chip = Chip::new("suggestion", ChipVariant::Suggestion).with_label("Quick action");

        assert!(matches!(chip.variant(), ChipVariant::Suggestion));
        // Suggestion chips provide quick actions
    }

    #[test]
    fn test_variant_specific_behavior() {
        // Different variants might have different default behaviors
        let assist = Chip::new("assist", ChipVariant::Assist);
        let filter = Chip::new("filter", ChipVariant::Filter);
        let input = Chip::new("input", ChipVariant::Input);
        let suggestion = Chip::new("suggestion", ChipVariant::Suggestion);

        // All should start unselected
        assert!(!assist.is_selected());
        assert!(!filter.is_selected());
        assert!(!input.is_selected());
        assert!(!suggestion.is_selected());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_chip_filter_collection() {
        // Test a filter chip collection like in a search interface
        let mut filters = ChipCollection::new(SelectionMode::Multiple);

        filters.add_chip(Chip::new("category", ChipVariant::Filter).with_label("Category"));
        filters.add_chip(Chip::new("price", ChipVariant::Filter).with_label("Price Range"));
        filters.add_chip(Chip::new("rating", ChipVariant::Filter).with_label("Rating"));
        filters.add_chip(Chip::new("availability", ChipVariant::Filter).with_label("In Stock"));

        // User selects some filters
        filters.select_chip("category").unwrap();
        filters.select_chip("rating").unwrap();

        assert_eq!(filters.selected_chips().len(), 2);

        // User removes a filter
        filters.deselect_chip("category").unwrap();
        assert_eq!(filters.selected_chips().len(), 1);
        assert!(filters.is_chip_selected("rating"));
    }

    #[test]
    fn test_chip_input_collection() {
        // Test input chips like tags in a form
        let mut tags = ChipCollection::new(SelectionMode::None); // Input chips aren't typically "selected"

        tags.add_chip(Chip::new("rust", ChipVariant::Input).with_label("Rust"));
        tags.add_chip(Chip::new("gui", ChipVariant::Input).with_label("GUI"));
        tags.add_chip(Chip::new("material", ChipVariant::Input).with_label("Material Design"));

        assert_eq!(tags.chips().len(), 3);

        // User removes a tag
        tags.remove_chip("gui").unwrap();
        assert_eq!(tags.chips().len(), 2);
        assert!(!tags.has_chip("gui"));
    }

    #[test]
    fn test_chip_suggestion_collection() {
        // Test suggestion chips for quick actions
        let suggestions = ChipCollectionBuilder::new(SelectionMode::Single)
            .chip(Chip::new("save", ChipVariant::Suggestion).with_label("Save"))
            .chip(Chip::new("share", ChipVariant::Suggestion).with_label("Share"))
            .chip(Chip::new("export", ChipVariant::Suggestion).with_label("Export"))
            .build();

        assert_eq!(suggestions.chips().len(), 3);
        // Suggestions typically allow only one action at a time
    }

    #[test]
    fn test_chip_mixed_variants() {
        // Test collection with mixed chip variants
        let mut mixed = ChipCollection::new(SelectionMode::Multiple);

        mixed.add_chip(Chip::new("filter1", ChipVariant::Filter).with_label("Filter"));
        mixed.add_chip(Chip::new("input1", ChipVariant::Input).with_label("Tag"));
        mixed.add_chip(Chip::new("assist1", ChipVariant::Assist).with_label("Help"));

        // All variants should work together in a collection
        mixed.select_chip("filter1").unwrap();
        mixed.select_chip("input1").unwrap();

        assert_eq!(mixed.selected_chips().len(), 2);
    }

    #[test]
    fn test_chip_state_persistence() {
        // Test that chip states can be preserved and restored
        let original_collection = ChipCollectionBuilder::new(SelectionMode::Multiple)
            .chip(Chip::new("a", ChipVariant::Filter).with_label("A"))
            .chip(Chip::new("b", ChipVariant::Filter).with_label("B"))
            .selected(&["a"])
            .build();

        let selected_ids: Vec<_> = original_collection
            .selected_chips()
            .iter()
            .map(|chip| chip.id())
            .collect();

        // Recreate collection with same state
        let mut restored_collection = ChipCollection::new(SelectionMode::Multiple);
        restored_collection.add_chip(Chip::new("a", ChipVariant::Filter).with_label("A"));
        restored_collection.add_chip(Chip::new("b", ChipVariant::Filter).with_label("B"));

        for id in selected_ids {
            restored_collection.select_chip(id).unwrap();
        }

        assert_eq!(
            original_collection.selected_chips().len(),
            restored_collection.selected_chips().len()
        );
        assert!(restored_collection.is_chip_selected("a"));
    }

    #[test]
    fn test_chip_validation_in_collection() {
        let mut collection = ChipCollection::new(SelectionMode::Single);

        // Add valid chip
        collection.add_chip(Chip::new("valid", ChipVariant::Filter).with_label("Valid"));
        assert!(collection.validate().is_ok());

        // Add invalid chip
        let long_label = "x".repeat(201);
        collection.add_chip(Chip::new("invalid", ChipVariant::Filter).with_label(&long_label));

        match collection.validate() {
            Err(SelectionError::ValidationError(_)) => {
                // Expected validation error
            }
            _ => panic!("Expected validation error"),
        }
    }
}
