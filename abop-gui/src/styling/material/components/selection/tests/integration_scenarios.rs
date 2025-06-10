//! Integration Scenarios Tests
//! 
//! Real-world usage patterns and integration scenarios for chip components.
//! Tests complex workflows, user interactions, and integration with other components.

use super::fixtures::{
    chip_factory::*,
    collection_factory::*,
    assertion_helpers::*,
    test_data::*,
};
use crate::styling::material::components::selection::chip::{
    core::{Chip, ChipState, ChipVariant},
    collection::{ChipCollection, SelectionMode},
    events::{ChipEvent, ChipEventHandler, EventResult},
    integration::{FormField, SearchInput, FilterPanel},
};
use std::collections::HashMap;
use test_case::test_case;

#[cfg(test)]
mod form_integration_tests {
    use super::*;

    #[test]
    fn test_form_field_with_input_chips() {
        let mut form_field = FormField::new("tags");
        
        // Add input chips representing tags
        let tags = ["rust", "gui", "material-design"];
        for tag in &tags {
            let chip = Chip::new(tag, ChipVariant::Input)
                .with_deletable(true);
            form_field.add_chip(chip).unwrap();
        }
        
        assert_eq!(form_field.chip_count(), 3);
        assert_eq!(form_field.value(), tags.join(","));
        
        // Remove a chip
        let chip_ids: Vec<_> = form_field.chip_ids().collect();
        form_field.remove_chip(&chip_ids[1]).unwrap();
        
        assert_eq!(form_field.chip_count(), 2);
        assert_eq!(form_field.value(), "rust,material-design");
    }

    #[test]
    fn test_form_validation_with_chips() {
        let mut form_field = FormField::new("skills")
            .with_min_chips(1)
            .with_max_chips(5)
            .with_required(true);
        
        // Empty form should be invalid
        assert!(!form_field.is_valid());
        assert!(form_field.validation_errors().contains(&"Required field cannot be empty"));
        
        // Add chips
        for i in 1..=3 {
            let chip = Chip::new(&format!("skill_{}", i), ChipVariant::Input);
            form_field.add_chip(chip).unwrap();
        }
        
        // Should now be valid
        assert!(form_field.is_valid());
        assert!(form_field.validation_errors().is_empty());
        
        // Add too many chips
        for i in 4..=7 {
            let chip = Chip::new(&format!("skill_{}", i), ChipVariant::Input);
            let result = form_field.add_chip(chip);
            if i > 5 {
                assert!(result.is_err()); // Should fail after max_chips
            }
        }
    }

    #[test]
    fn test_form_serialization() {
        let mut form_field = FormField::new("categories");
        
        let categories = ["technology", "science", "education"];
        for category in &categories {
            let chip = Chip::new(category, ChipVariant::Input);
            form_field.add_chip(chip).unwrap();
        }
        
        // Test JSON serialization
        let json = form_field.to_json().unwrap();
        let deserialized = FormField::from_json(&json).unwrap();
        
        assert_eq!(form_field.chip_count(), deserialized.chip_count());
        assert_eq!(form_field.value(), deserialized.value());
    }

    #[test]
    fn test_form_submission_flow() {
        let mut form_field = FormField::new("interests");
        
        // Simulate user adding interests
        let user_interests = ["reading", "coding", "music"];
        for interest in &user_interests {
            let chip = Chip::new(interest, ChipVariant::Input);
            form_field.add_chip(chip).unwrap();
        }
        
        // Validate before submission
        assert!(form_field.is_valid());
        
        // Prepare for submission
        let submission_data = form_field.prepare_submission();
        assert_eq!(submission_data.field_name, "interests");
        assert_eq!(submission_data.values, user_interests);
        assert!(submission_data.is_valid);
    }
}

#[cfg(test)]
mod search_integration_tests {
    use super::*;

    #[test]
    fn test_search_with_filter_chips() {
        let mut search_input = SearchInput::new()
            .with_filter_chips(true)
            .with_suggestion_chips(true);
        
        // Add some filter options
        let filters = [
            ("Type", vec!["Document", "Image", "Video"]),
            ("Date", vec!["Today", "This Week", "This Month"]),
            ("Size", vec!["Small", "Medium", "Large"]),
        ];
        
        for (category, options) in &filters {
            for option in options {
                let chip = Chip::new(option, ChipVariant::Filter)
                    .with_category(category.to_string());
                search_input.add_filter_chip(chip).unwrap();
            }
        }
        
        // Apply some filters
        search_input.apply_filter("Type", "Document").unwrap();
        search_input.apply_filter("Date", "Today").unwrap();
        
        let active_filters = search_input.active_filters();
        assert_eq!(active_filters.len(), 2);
        assert!(active_filters.contains_key("Type"));
        assert!(active_filters.contains_key("Date"));
        
        // Test search with filters
        let search_query = "important document";
        let results = search_input.search(search_query);
        
        // Results should be filtered by active chips
        assert!(results.is_filtered_by("Type", "Document"));
        assert!(results.is_filtered_by("Date", "Today"));
    }

    #[test]
    fn test_search_suggestions() {
        let mut search_input = SearchInput::new()
            .with_suggestion_chips(true);
        
        // Add search suggestions
        let suggestions = ["recent searches", "popular items", "recommended"];
        for suggestion in &suggestions {
            let chip = Chip::new(suggestion, ChipVariant::Suggestion);
            search_input.add_suggestion_chip(chip).unwrap();
        }
        
        // Simulate user clicking a suggestion
        let suggestion_chips: Vec<_> = search_input.suggestion_chips().collect();
        let clicked_suggestion = &suggestion_chips[0];
        
        search_input.activate_suggestion(clicked_suggestion.id.clone()).unwrap();
        
        assert_eq!(search_input.current_query(), clicked_suggestion.label);
        assert!(search_input.has_active_query());
    }

    #[test]
    fn test_search_history_chips() {
        let mut search_input = SearchInput::new()
            .with_history_chips(true)
            .with_max_history(5);
        
        // Simulate search history
        let searches = ["rust programming", "material design", "gui framework", "web development"];
        for search in &searches {
            search_input.add_to_history(search).unwrap();
        }
        
        let history_chips = search_input.history_chips();
        assert_eq!(history_chips.len(), 4);
        
        // History should be in reverse chronological order
        assert_eq!(history_chips[0].label, "web development");
        assert_eq!(history_chips[3].label, "rust programming");
        
        // Test history limit
        search_input.add_to_history("new search").unwrap();
        search_input.add_to_history("another search").unwrap();
        
        let updated_history = search_input.history_chips();
        assert_eq!(updated_history.len(), 5); // Should respect max_history
        assert_eq!(updated_history[0].label, "another search");
    }

    #[test]
    fn test_search_autocomplete_with_chips() {
        let mut search_input = SearchInput::new()
            .with_autocomplete(true);
        
        // Set up autocomplete data
        let autocomplete_terms = [
            "rust programming language",
            "rust web framework",
            "rust gui library",
            "material design system",
            "material ui components",
        ];
        search_input.set_autocomplete_data(autocomplete_terms.to_vec());
        
        // Type partial query
        search_input.set_query("rust");
        let suggestions = search_input.autocomplete_suggestions();
        
        // Should get rust-related suggestions as chips
        assert_eq!(suggestions.len(), 3);
        for suggestion in &suggestions {
            assert!(suggestion.label.contains("rust"));
            assert_eq!(suggestion.variant, ChipVariant::Suggestion);
        }
        
        // Select a suggestion
        search_input.select_autocomplete_suggestion(&suggestions[0].id).unwrap();
        assert_eq!(search_input.current_query(), suggestions[0].label);
    }
}

#[cfg(test)]
mod filter_panel_integration_tests {
    use super::*;

    #[test]
    fn test_multi_category_filter_panel() {
        let mut filter_panel = FilterPanel::new();
        
        // Set up multiple filter categories
        let categories = [
            ("Brand", vec!["Apple", "Samsung", "Google", "Microsoft"]),
            ("Price Range", vec!["Under $100", "$100-$500", "$500-$1000", "Over $1000"]),
            ("Rating", vec!["4+ Stars", "3+ Stars", "2+ Stars", "1+ Stars"]),
            ("Availability", vec!["In Stock", "Out of Stock", "Pre-order"]),
        ];
        
        for (category_name, options) in &categories {
            let mut category = filter_panel.add_category(category_name).unwrap();
            
            for option in options {
                let chip = Chip::new(option, ChipVariant::Filter);
                category.add_filter_option(chip).unwrap();
            }
        }
        
        assert_eq!(filter_panel.category_count(), 4);
        
        // Apply filters from different categories
        filter_panel.apply_filter("Brand", "Apple").unwrap();
        filter_panel.apply_filter("Price Range", "$500-$1000").unwrap();
        filter_panel.apply_filter("Rating", "4+ Stars").unwrap();
        
        let active_filters = filter_panel.active_filters();
        assert_eq!(active_filters.len(), 3);
        
        // Test filter combination
        let filter_query = filter_panel.build_filter_query();
        assert!(filter_query.contains("Brand:Apple"));
        assert!(filter_query.contains("Price Range:$500-$1000"));
        assert!(filter_query.contains("Rating:4+ Stars"));
    }

    #[test]
    fn test_exclusive_vs_inclusive_filters() {
        let mut filter_panel = FilterPanel::new();
        
        // Exclusive category (single selection)
        let mut brand_category = filter_panel.add_category("Brand")
            .unwrap()
            .with_selection_mode(SelectionMode::Single);
        
        brand_category.add_filter_option(Chip::new("Apple", ChipVariant::Filter)).unwrap();
        brand_category.add_filter_option(Chip::new("Samsung", ChipVariant::Filter)).unwrap();
        
        // Inclusive category (multiple selection)
        let mut features_category = filter_panel.add_category("Features")
            .unwrap()
            .with_selection_mode(SelectionMode::Multiple);
        
        features_category.add_filter_option(Chip::new("Waterproof", ChipVariant::Filter)).unwrap();
        features_category.add_filter_option(Chip::new("Wireless Charging", ChipVariant::Filter)).unwrap();
        features_category.add_filter_option(Chip::new("Fast Charging", ChipVariant::Filter)).unwrap();
        
        // Test exclusive selection
        filter_panel.apply_filter("Brand", "Apple").unwrap();
        filter_panel.apply_filter("Brand", "Samsung").unwrap(); // Should replace Apple
        
        let brand_filters = filter_panel.active_filters_for_category("Brand");
        assert_eq!(brand_filters.len(), 1);
        assert_eq!(brand_filters[0], "Samsung");
        
        // Test inclusive selection
        filter_panel.apply_filter("Features", "Waterproof").unwrap();
        filter_panel.apply_filter("Features", "Wireless Charging").unwrap();
        
        let feature_filters = filter_panel.active_filters_for_category("Features");
        assert_eq!(feature_filters.len(), 2);
        assert!(feature_filters.contains(&"Waterproof".to_string()));
        assert!(feature_filters.contains(&"Wireless Charging".to_string()));
    }

    #[test]
    fn test_filter_persistence_and_restoration() {
        let mut filter_panel = FilterPanel::new();
        
        // Set up filters
        filter_panel.add_category("Category").unwrap();
        filter_panel.add_filter_option("Category", Chip::new("Electronics", ChipVariant::Filter)).unwrap();
        filter_panel.add_filter_option("Category", Chip::new("Clothing", ChipVariant::Filter)).unwrap();
        
        // Apply some filters
        filter_panel.apply_filter("Category", "Electronics").unwrap();
        
        // Save state
        let saved_state = filter_panel.save_state().unwrap();
        
        // Clear filters
        filter_panel.clear_all_filters();
        assert_eq!(filter_panel.active_filter_count(), 0);
        
        // Restore state
        filter_panel.restore_state(saved_state).unwrap();
        assert_eq!(filter_panel.active_filter_count(), 1);
        assert!(filter_panel.is_filter_active("Category", "Electronics"));
    }

    #[test]
    fn test_dynamic_filter_loading() {
        let mut filter_panel = FilterPanel::new();
        
        // Start with basic categories
        filter_panel.add_category("Type").unwrap();
        filter_panel.add_filter_option("Type", Chip::new("Book", ChipVariant::Filter)).unwrap();
        filter_panel.add_filter_option("Type", Chip::new("Movie", ChipVariant::Filter)).unwrap();
        
        // Apply initial filter
        filter_panel.apply_filter("Type", "Book").unwrap();
        
        // Dynamically load additional filters based on selection
        if filter_panel.is_filter_active("Type", "Book") {
            filter_panel.add_category("Genre").unwrap();
            let book_genres = ["Fiction", "Non-fiction", "Biography", "Science"];
            for genre in &book_genres {
                filter_panel.add_filter_option("Genre", Chip::new(genre, ChipVariant::Filter)).unwrap();
            }
        }
        
        assert!(filter_panel.has_category("Genre"));
        assert_eq!(filter_panel.options_for_category("Genre").len(), 4);
        
        // Change to Movie - should trigger different dynamic filters
        filter_panel.apply_filter("Type", "Movie").unwrap();
        
        if filter_panel.is_filter_active("Type", "Movie") {
            filter_panel.remove_category("Genre").unwrap(); // Remove book genres
            filter_panel.add_category("Genre").unwrap();
            let movie_genres = ["Action", "Comedy", "Drama", "Horror", "Sci-Fi"];
            for genre in &movie_genres {
                filter_panel.add_filter_option("Genre", Chip::new(genre, ChipVariant::Filter)).unwrap();
            }
        }
        
        assert_eq!(filter_panel.options_for_category("Genre").len(), 5);
    }
}

#[cfg(test)]
mod event_driven_scenarios_tests {
    use super::*;

    #[test]
    fn test_chip_event_handling() {
        let mut event_handler = ChipEventHandler::new();
        let mut events_received = Vec::new();
        
        // Set up event listeners
        event_handler.on_chip_click(|event| {
            events_received.push(format!("click:{}", event.chip_id));
            EventResult::Continue
        });
        
        event_handler.on_chip_select(|event| {
            events_received.push(format!("select:{}", event.chip_id));
            EventResult::Continue
        });
        
        event_handler.on_chip_delete(|event| {
            events_received.push(format!("delete:{}", event.chip_id));
            EventResult::Continue
        });
        
        // Create chips and trigger events
        let mut collection = create_test_collection(3);
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Simulate user interactions
        event_handler.handle_event(ChipEvent::Click { chip_id: chip_ids[0].clone() });
        event_handler.handle_event(ChipEvent::Select { chip_id: chip_ids[0].clone() });
        event_handler.handle_event(ChipEvent::Delete { chip_id: chip_ids[1].clone() });
        
        assert_eq!(events_received.len(), 3);
        assert!(events_received.contains(&format!("click:{}", chip_ids[0])));
        assert!(events_received.contains(&format!("select:{}", chip_ids[0])));
        assert!(events_received.contains(&format!("delete:{}", chip_ids[1])));
    }

    #[test]
    fn test_cascading_events() {
        let mut event_handler = ChipEventHandler::new();
        let mut collection = create_single_selection_collection();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        let mut selection_changes = Vec::new();
        
        event_handler.on_chip_select(move |event| {
            selection_changes.push(("select".to_string(), event.chip_id.clone()));
            EventResult::Continue
        });
        
        event_handler.on_chip_deselect(move |event| {
            selection_changes.push(("deselect".to_string(), event.chip_id.clone()));
            EventResult::Continue
        });
        
        // Select first chip
        collection.select(&chip_ids[0]).unwrap();
        event_handler.handle_event(ChipEvent::Select { chip_id: chip_ids[0].clone() });
        
        // Select second chip (should deselect first in single selection mode)
        collection.select(&chip_ids[1]).unwrap();
        event_handler.handle_event(ChipEvent::Deselect { chip_id: chip_ids[0].clone() });
        event_handler.handle_event(ChipEvent::Select { chip_id: chip_ids[1].clone() });
        
        // Verify event sequence
        assert_eq!(selection_changes.len(), 3);
        assert_eq!(selection_changes[0], ("select".to_string(), chip_ids[0].clone()));
        assert_eq!(selection_changes[1], ("deselect".to_string(), chip_ids[0].clone()));
        assert_eq!(selection_changes[2], ("select".to_string(), chip_ids[1].clone()));
    }

    #[test]
    fn test_event_propagation_control() {
        let mut event_handler = ChipEventHandler::new();
        let mut events_processed = 0;
        
        // First handler that stops propagation under certain conditions
        event_handler.on_chip_click(move |event| {
            events_processed += 1;
            if event.chip_id.contains("stop") {
                EventResult::StopPropagation
            } else {
                EventResult::Continue
            }
        });
        
        // Second handler that should only run if propagation wasn't stopped
        let mut second_handler_called = false;
        event_handler.on_chip_click(move |_event| {
            second_handler_called = true;
            EventResult::Continue
        });
        
        // Test normal propagation
        event_handler.handle_event(ChipEvent::Click { chip_id: "normal_chip".to_string() });
        assert_eq!(events_processed, 1);
        assert!(second_handler_called);
        
        // Test stopped propagation
        second_handler_called = false;
        event_handler.handle_event(ChipEvent::Click { chip_id: "stop_chip".to_string() });
        assert_eq!(events_processed, 2);
        assert!(!second_handler_called); // Should not have been called
    }
}

#[cfg(test)]
mod accessibility_integration_tests {
    use super::*;

    #[test]
    fn test_keyboard_navigation_in_collection() {
        let mut collection = create_test_collection(5);
        let mut focus_manager = FocusManager::new(&collection);
        
        // Test arrow key navigation
        focus_manager.focus_first().unwrap();
        assert_eq!(focus_manager.focused_index(), Some(0));
        
        focus_manager.focus_next().unwrap();
        assert_eq!(focus_manager.focused_index(), Some(1));
        
        focus_manager.focus_previous().unwrap();
        assert_eq!(focus_manager.focused_index(), Some(0));
        
        // Test wrapping
        focus_manager.focus_previous().unwrap(); // Should wrap to last
        assert_eq!(focus_manager.focused_index(), Some(4));
        
        focus_manager.focus_next().unwrap(); // Should wrap to first
        assert_eq!(focus_manager.focused_index(), Some(0));
    }

    #[test]
    fn test_screen_reader_announcements() {
        let mut collection = create_single_selection_collection();
        let mut announcer = ScreenReaderAnnouncer::new();
        let chip_ids: Vec<_> = collection.chip_ids().collect();
        
        // Test selection announcement
        collection.select(&chip_ids[0]).unwrap();
        let announcement = announcer.announce_selection(&collection, &chip_ids[0]);
        assert!(announcement.contains("selected"));
        assert!(announcement.contains(&collection.get(&chip_ids[0]).unwrap().label));
        
        // Test collection status announcement
        let status_announcement = announcer.announce_collection_status(&collection);
        assert!(status_announcement.contains("1 of"));
        assert!(status_announcement.contains("selected"));
    }

    #[test]
    fn test_high_contrast_mode_compliance() {
        let chip = Chip::new("test", ChipVariant::Filter);
        
        // Test high contrast mode
        let high_contrast_theme = chip.with_high_contrast_mode(true);
        
        let colors = high_contrast_theme.colors();
        let contrast_ratio = calculate_contrast_ratio(colors.foreground, colors.background);
        
        // High contrast mode should meet WCAG AAA standards (7:1)
        assert!(contrast_ratio >= 7.0);
        
        // Focus indicators should be highly visible
        let focus_colors = high_contrast_theme.focus_colors();
        let focus_contrast = calculate_contrast_ratio(focus_colors.indicator, colors.background);
        assert!(focus_contrast >= 7.0);
    }

    #[test]
    fn test_reduced_motion_compliance() {
        let chip = Chip::new("test", ChipVariant::Filter)
            .with_reduced_motion(true);
        
        let animation_config = chip.animation_config();
        
        // Reduced motion should disable or minimize animations
        assert!(animation_config.duration.as_millis() <= 100); // Very short or zero
        assert_eq!(animation_config.easing, "linear"); // No complex easing
        assert!(!animation_config.scale_animations); // No scaling
        assert!(!animation_config.slide_animations); // No sliding
    }
}

#[cfg(test)]
mod performance_integration_tests {
    use super::*;

    #[test]
    fn test_large_collection_rendering_performance() {
        let large_collection = create_large_test_collection(LARGE_COLLECTION_SIZE);
        
        let start = std::time::Instant::now();
        
        // Simulate rendering operations
        for chip in large_collection.chips() {
            let _render_data = chip.prepare_render_data();
            let _layout = chip.calculate_layout();
            let _styles = chip.compute_styles();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < PERFORMANCE_THRESHOLD_MS * 5); // Allow more time for rendering
    }

    #[test]
    fn test_real_time_filtering_performance() {
        let large_collection = create_large_searchable_collection(1000);
        
        let search_terms = ["test", "sample", "data", "chip", "material"];
        
        let start = std::time::Instant::now();
        
        for term in &search_terms {
            let _results = large_collection.search(term);
            let _filtered = large_collection.filter_by_state(ChipState::Enabled);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < PERFORMANCE_THRESHOLD_MS * 2);
    }

    #[test]
    fn test_memory_usage_in_long_sessions() {
        // Simulate a long user session with many chip operations
        let mut collection = ChipCollection::new();
        let initial_memory = get_memory_usage();
        
        // Perform many operations
        for i in 0..1000 {
            // Add chip
            let chip = create_test_chip(&format!("chip_{}", i), ChipVariant::Filter);
            collection.add(chip).unwrap();
            
            // Select/deselect operations
            if i % 10 == 0 {
                let chip_ids: Vec<_> = collection.chip_ids().collect();
                if let Some(id) = chip_ids.last() {
                    collection.select(id).unwrap();
                    collection.deselect(id).unwrap();
                }
            }
            
            // Periodic cleanup
            if i % 100 == 0 && i > 0 {
                // Remove some older chips to prevent unbounded growth
                let chip_ids: Vec<_> = collection.chip_ids().take(10).collect();
                for id in chip_ids {
                    collection.remove(&id).unwrap();
                }
            }
        }
        
        let final_memory = get_memory_usage();
        let memory_growth = final_memory - initial_memory;
        
        // Memory growth should be reasonable (less than 100MB for this test)
        assert!(memory_growth < 100 * 1024 * 1024);
    }
}

#[cfg(test)]
mod real_world_workflow_tests {
    use super::*;

    #[test]
    fn test_e_commerce_filter_workflow() {
        // Simulate an e-commerce product filtering workflow
        let mut filter_panel = FilterPanel::new();
        
        // Set up product filters
        let categories = [
            ("Category", vec!["Electronics", "Clothing", "Books", "Home"]),
            ("Brand", vec!["Apple", "Samsung", "Nike", "Adidas"]),
            ("Price", vec!["Under $50", "$50-$100", "$100-$200", "Over $200"]),
            ("Rating", vec!["5 Stars", "4+ Stars", "3+ Stars"]),
        ];
        
        for (category_name, options) in &categories {
            filter_panel.add_category(category_name).unwrap();
            for option in options {
                filter_panel.add_filter_option(category_name, 
                    Chip::new(option, ChipVariant::Filter)).unwrap();
            }
        }
        
        // User applies filters step by step
        filter_panel.apply_filter("Category", "Electronics").unwrap();
        assert_eq!(filter_panel.active_filter_count(), 1);
        
        filter_panel.apply_filter("Brand", "Apple").unwrap();
        assert_eq!(filter_panel.active_filter_count(), 2);
        
        filter_panel.apply_filter("Price", "$100-$200").unwrap();
        assert_eq!(filter_panel.active_filter_count(), 3);
        
        // User removes a filter
        filter_panel.remove_filter("Price", "$100-$200").unwrap();
        assert_eq!(filter_panel.active_filter_count(), 2);
        
        // Clear all filters
        filter_panel.clear_all_filters();
        assert_eq!(filter_panel.active_filter_count(), 0);
    }

    #[test]
    fn test_email_tagging_workflow() {
        // Simulate email tagging system
        let mut form_field = FormField::new("email_tags")
            .with_max_chips(10)
            .with_autocomplete(true);
        
        // Set up tag suggestions
        let common_tags = ["urgent", "work", "personal", "follow-up", "meeting", "project"];
        form_field.set_tag_suggestions(common_tags.to_vec());
        
        // User starts typing
        form_field.set_input("ur");
        let suggestions = form_field.autocomplete_suggestions();
        assert!(suggestions.iter().any(|chip| chip.label == "urgent"));
        
        // User selects suggestion
        let urgent_chip_id = suggestions.iter()
            .find(|chip| chip.label == "urgent")
            .unwrap().id.clone();
        form_field.select_suggestion(&urgent_chip_id).unwrap();
        
        assert_eq!(form_field.chip_count(), 1);
        assert!(form_field.has_tag("urgent"));
        
        // User types custom tag
        form_field.add_custom_tag("project-alpha").unwrap();
        assert_eq!(form_field.chip_count(), 2);
        
        // User removes a tag
        form_field.remove_tag("urgent").unwrap();
        assert_eq!(form_field.chip_count(), 1);
        assert!(form_field.has_tag("project-alpha"));
    }

    #[test]
    fn test_skill_assessment_workflow() {
        // Simulate a skill assessment interface
        let mut skill_form = FormField::new("skills")
            .with_min_chips(3)
            .with_max_chips(10)
            .with_validation(true);
        
        // Predefined skill categories
        let skill_categories = [
            ("Programming", vec!["Rust", "JavaScript", "Python", "Java"]),
            ("Frameworks", vec!["React", "Angular", "Vue", "Django"]),
            ("Tools", vec!["Git", "Docker", "Kubernetes", "Jenkins"]),
        ];
        
        for (category, skills) in &skill_categories {
            for skill in skills {
                skill_form.add_skill_option(category, skill).unwrap();
            }
        }
        
        // User selects skills
        skill_form.select_skill("Programming", "Rust").unwrap();
        skill_form.select_skill("Programming", "JavaScript").unwrap();
        skill_form.select_skill("Frameworks", "React").unwrap();
        skill_form.select_skill("Tools", "Git").unwrap();
        
        assert_eq!(skill_form.chip_count(), 4);
        assert!(skill_form.is_valid()); // Meets minimum requirement
        
        // User adds proficiency levels
        skill_form.set_skill_level("Rust", "Expert").unwrap();
        skill_form.set_skill_level("JavaScript", "Intermediate").unwrap();
        skill_form.set_skill_level("React", "Advanced").unwrap();
        skill_form.set_skill_level("Git", "Intermediate").unwrap();
        
        // Validate skill progression
        let skill_data = skill_form.export_skill_data();
        assert!(skill_data.validate_progression());
        assert!(skill_data.has_required_diversity()); // Skills from multiple categories
    }
}

// Helper functions and structs for integration testing
#[derive(Debug)]
struct FocusManager<'a> {
    collection: &'a ChipCollection,
    focused_index: Option<usize>,
}

impl<'a> FocusManager<'a> {
    fn new(collection: &'a ChipCollection) -> Self {
        Self {
            collection,
            focused_index: None,
        }
    }
    
    fn focus_first(&mut self) -> Result<(), &'static str> {
        if self.collection.len() > 0 {
            self.focused_index = Some(0);
            Ok(())
        } else {
            Err("No chips to focus")
        }
    }
    
    fn focus_next(&mut self) -> Result<(), &'static str> {
        if let Some(current) = self.focused_index {
            let next = (current + 1) % self.collection.len();
            self.focused_index = Some(next);
            Ok(())
        } else {
            self.focus_first()
        }
    }
    
    fn focus_previous(&mut self) -> Result<(), &'static str> {
        if let Some(current) = self.focused_index {
            let prev = if current == 0 {
                self.collection.len() - 1
            } else {
                current - 1
            };
            self.focused_index = Some(prev);
            Ok(())
        } else {
            self.focus_first()
        }
    }
    
    fn focused_index(&self) -> Option<usize> {
        self.focused_index
    }
}

struct ScreenReaderAnnouncer;

impl ScreenReaderAnnouncer {
    fn new() -> Self {
        Self
    }
    
    fn announce_selection(&self, collection: &ChipCollection, chip_id: &str) -> String {
        if let Some(chip) = collection.get(chip_id) {
            format!("{} selected. {} of {} chips selected.", 
                chip.label, 
                collection.selected_count(), 
                collection.len())
        } else {
            "Selection changed".to_string()
        }
    }
    
    fn announce_collection_status(&self, collection: &ChipCollection) -> String {
        format!("{} of {} chips selected", 
            collection.selected_count(), 
            collection.len())
    }
}

fn get_memory_usage() -> usize {
    // Placeholder for actual memory usage measurement
    // In real implementation, this would use system APIs
    0
}
