//! Tests for `audio_controls` component

use crate::components::audio_controls::AudioControls;
use crate::styling::material::MaterialTokens;
use crate::test_utils::{TestDataFactory, create_test_audiobook};
use abop_core::PlayerState;
use std::collections::HashSet;

#[test]
fn test_audio_controls_view() {
    let tokens = MaterialTokens::default();

    // Test with a single audiobook
    let audiobooks = vec![
        create_test_audiobook("1", "Test Book"),
        create_test_audiobook("2", "Another Book"),
    ];

    // Test cases with different selection states
    let test_cases = [
        (HashSet::new(), "no selection"),
        (
            {
                let mut set = HashSet::new();
                set.insert("1".to_string());
                set
            },
            "single selection",
        ),
        (
            {
                let mut set = HashSet::new();
                set.insert("1".to_string());
                set.insert("2".to_string());
                set
            },
            "multiple selection",
        ),
    ];

    // Test with different player states
    let player_states = [
        (PlayerState::Stopped, "stopped"),
        (PlayerState::Playing, "playing"),
        (PlayerState::Paused, "paused"),
    ];

    // Test all combinations of selection states and player states
    for (selected, selection_desc) in &test_cases {
        for (state, state_desc) in &player_states {
            println!("Testing with {selection_desc} and {state_desc} state");
            let _element = AudioControls::view(selected, &audiobooks, state.clone(), &tokens);
            // If we get here, the view function didn't panic
        }
    }

    // Test with empty audiobooks list
    let empty_audiobooks = Vec::new();
    let _element = AudioControls::view(
        &test_cases[0].0, // Use the empty selection set
        &empty_audiobooks,
        PlayerState::Stopped,
        &tokens,
    );
}

#[test]
fn test_audio_controls_with_selected_audiobooks() {
    let tokens = MaterialTokens::default();

    // Test with multiple audiobooks and various selection states
    let test_cases = [
        // Single selection
        (vec!["1"], "Single selection"),
        // Multiple selection
        (vec!["1", "2"], "Multiple selection"),
        // Non-existent selection
        (vec!["999"], "Non-existent selection"),
        // Empty selection
        (vec![], "Empty selection"),
    ];

    let audiobooks = vec![
        TestDataFactory::custom_audiobook("1", "Book One", "Author A", Some(3600), Some(1024000)),
        TestDataFactory::custom_audiobook("2", "Book Two", "Author B", Some(7200), Some(2048000)),
        TestDataFactory::custom_audiobook("3", "Book Three", "Author C", None, None), // Incomplete metadata
    ];

    for (selected, description) in test_cases {
        println!("Testing case: {description}");
        let selected_ids: HashSet<_> = selected.into_iter().map(String::from).collect();

        // Test with different player states
        let player_states = [
            PlayerState::Stopped,
            PlayerState::Playing,
            PlayerState::Paused,
        ];

        for state in &player_states {
            let _element = AudioControls::view(&selected_ids, &audiobooks, state.clone(), &tokens);
            // Element creation successful if we get here
        }
    }
}

#[test]
fn test_audio_controls_edge_cases() {
    let tokens = MaterialTokens::default();

    // Test with empty audiobooks and empty selection
    let empty_audiobooks = Vec::new();
    let empty_selection = HashSet::new();

    // Test with empty state
    let _element = AudioControls::view(
        &empty_selection,
        &empty_audiobooks,
        PlayerState::Stopped,
        &tokens,
    );

    // Test with very long metadata
    let long_title = "Audiobook with a very long title that should be properly handled in the UI without breaking the layout or causing any rendering issues";
    let long_author = "Author with a very long name that should also be properly handled in the UI";
    let long_metadata_audiobook = TestDataFactory::custom_audiobook(
        "1",
        long_title,
        long_author,
        Some(999999),
        Some(9999999999),
    );

    // Test with special characters in metadata
    let special_chars_audiobook = TestDataFactory::custom_audiobook(
        "2",
        "Book with special chars: !@#$%^&*()_+{}|:<>?",
        "Author with emoji 😊 and unicode 测试",
        Some(3600),
        Some(1024000),
    );

    // Test with missing metadata
    let missing_metadata_audiobook = TestDataFactory::custom_audiobook("3", "", "", None, None);

    // Test with extremely large numbers
    let large_numbers_audiobook = TestDataFactory::custom_audiobook(
        "4",
        "Book with large numbers",
        "Author",
        Some(u64::MAX),
        Some(u64::MAX),
    );

    let test_audiobooks = vec![
        long_metadata_audiobook,
        special_chars_audiobook,
        missing_metadata_audiobook,
        large_numbers_audiobook,
    ];

    // Test with different selection combinations
    let selection_sets = [
        (HashSet::new(), "no selection"),
        (
            {
                let mut set = HashSet::new();
                set.insert("1".to_string());
                set
            },
            "first item selected",
        ),
        (
            {
                let mut set = HashSet::new();
                set.insert("nonexistent".to_string());
                set
            },
            "non-existent selection",
        ),
    ];

    // Test all combinations of edge cases
    for (selected, selection_desc) in &selection_sets {
        println!("Testing edge cases with {selection_desc}");

        // Test with different player states
        let player_states = [
            PlayerState::Stopped,
            PlayerState::Playing,
            PlayerState::Paused,
        ];

        for state in &player_states {
            let _element = AudioControls::view(selected, &test_audiobooks, state.clone(), &tokens);
            // If we get here, the view function didn't panic
        }
    }
}
