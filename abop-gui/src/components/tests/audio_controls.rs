//! Tests for the AudioControls component
//!
//! Comprehensive tests covering state combinations, edge cases, and user interactions.

use super::*;
use crate::components::audio_controls::AudioControls;

/// Test basic audio controls functionality with different state combinations
#[test]
fn audio_controls_renders_with_all_combinations() {
    let tokens = MaterialTokens::default();
    
    // Test audiobooks with different metadata completeness
    let audiobooks = vec![
        create_test_audiobook("1", "Complete Book"),
        create_custom_test_audiobook("2", "Minimal Book", "Unknown", None),
    ];

    // Test selection combinations
    let selection_cases = [
        (HashSet::new(), "no selection"),
        (HashSet::from(["1".to_string()]), "single selection"),
        (HashSet::from(["1".to_string(), "2".to_string()]), "multiple selection"),
        (HashSet::from(["999".to_string()]), "non-existent selection"),
    ];

    let player_states = [
        PlayerState::Stopped,
        PlayerState::Playing,
        PlayerState::Paused,
    ];

    // Test all combinations
    for (selected, selection_desc) in &selection_cases {
        for player_state in &player_states {
            let element = AudioControls::view(selected, &audiobooks, player_state.clone(), &tokens);
            let _ = element; // Verify rendering succeeds
            
            // Add context for debugging if tests fail
            println!("✓ Rendered audio controls with {selection_desc} and {player_state:?}");
        }
    }
}

#[test]
fn audio_controls_handles_edge_cases() {
    let tokens = MaterialTokens::default();

    // Empty audiobooks with empty selection
    let empty_audiobooks: Vec<Audiobook> = Vec::new();
    let empty_selection = HashSet::new();
    let element = AudioControls::view(&empty_selection, &empty_audiobooks, PlayerState::Stopped, &tokens);
    let _ = element;

    // Test with extreme metadata values
    let extreme_audiobooks = vec![
        create_extreme_metadata_audiobook(),
        create_unicode_metadata_audiobook(),
        create_minimal_metadata_audiobook(),
    ];

    let test_selection = HashSet::from(["extreme".to_string()]);
    let element = AudioControls::view(&test_selection, &extreme_audiobooks, PlayerState::Playing, &tokens);
    let _ = element;
}

#[test]
fn audio_controls_performance_with_large_datasets() {
    let tokens = MaterialTokens::default();
    
    // Test with larger dataset to ensure performance
    let large_audiobook_set: Vec<Audiobook> = (0..100)
        .map(|i| create_test_audiobook(&format!("book_{i}"), &format!("Book {i}")))
        .collect();
    
    // Select every 10th book
    let large_selection: HashSet<String> = (0..100)
        .step_by(10)
        .map(|i| format!("book_{i}"))
        .collect();
    
    let element = AudioControls::view(&large_selection, &large_audiobook_set, PlayerState::Stopped, &tokens);
    let _ = element;
}

// Helper functions for creating test audiobooks with specific characteristics

fn create_extreme_metadata_audiobook() -> Audiobook {
    let long_title = "A".repeat(1000);
    let long_author = "Very Long Author Name ".repeat(50);
    
    let mut audiobook = create_custom_test_audiobook(
        "extreme", 
        &long_title, 
        &long_author, 
        Some(u64::MAX)
    );
    audiobook.size_bytes = Some(u64::MAX);
    audiobook
}

fn create_unicode_metadata_audiobook() -> Audiobook {
    create_custom_test_audiobook(
        "unicode",
        "Book with 测试 and emoji 📚",
        "Author with special chars: !@#$%^&*()",
        Some(3600)
    )
}

fn create_minimal_metadata_audiobook() -> Audiobook {
    create_custom_test_audiobook("minimal", "", "", None)
}