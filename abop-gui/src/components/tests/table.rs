//! Tests for the AudiobookTable component
//!
//! Tests covering table rendering, selection states, and sorting functionality.

use super::*;
use crate::components::table_core::AudiobookTable;

#[test]
fn audiobook_table_renders_empty_state() {
    let tokens = MaterialTokens::default();
    let audiobooks = vec![];
    let selected = HashSet::new();
    let table_state = TableState::default();

    let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens);
    let _ = element;
}

#[test]
fn audiobook_table_renders_with_data() {
    let tokens = MaterialTokens::default();

    let audiobooks = vec![
        create_test_audiobook("1", "First Book"),
        create_test_audiobook("2", "Second Book"),
        create_custom_test_audiobook("3", "Third Book", "Custom Author", Some(7200)),
    ];

    let selected = HashSet::new();
    let table_state = TableState::default();

    let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens);
    let _ = element;
}

#[test]
fn audiobook_table_handles_selections() {
    let tokens = MaterialTokens::default();

    let audiobooks = vec![
        create_test_audiobook("book1", "Book One"),
        create_test_audiobook("book2", "Book Two"),
        create_test_audiobook("book3", "Book Three"),
    ];

    let selection_scenarios = [
        (HashSet::new(), "no selection"),
        (HashSet::from(["book1".to_string()]), "single selection"),
        (
            HashSet::from(["book1".to_string(), "book3".to_string()]),
            "multiple selection",
        ),
        (
            HashSet::from([
                "book1".to_string(),
                "book2".to_string(),
                "book3".to_string(),
            ]),
            "all selected",
        ),
        (
            HashSet::from(["nonexistent".to_string()]),
            "invalid selection",
        ),
    ];

    for (selected, description) in selection_scenarios {
        let table_state = TableState::default();
        let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens);
        let _ = element;
        println!("✓ Table rendered with {description}");
    }
}

#[test]
fn audiobook_table_handles_sorting_states() {
    let tokens = MaterialTokens::default();

    let audiobooks = vec![
        create_test_audiobook("1", "Zebra Book"),
        create_test_audiobook("2", "Alpha Book"),
        create_test_audiobook("3", "Beta Book"),
    ];

    let selected = HashSet::new();

    let sort_columns = ["title", "author", "duration", "size"];

    for column in sort_columns {
        for ascending in [true, false] {
            let mut table_state = TableState::default();
            table_state.sort_column = column.to_string();
            table_state.sort_ascending = ascending;

            let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens);
            let _ = element;

            let direction = if ascending { "ascending" } else { "descending" };
            println!("✓ Table rendered with {column} sort {direction}");
        }
    }
}

#[test]
fn audiobook_table_performance_large_dataset() {
    let tokens = MaterialTokens::default();

    // Test with larger dataset
    let large_audiobook_set: Vec<Audiobook> = (0..1000)
        .map(|i| create_test_audiobook(&format!("id_{i}"), &format!("Book {i:04}")))
        .collect();

    // Select every 100th book
    let large_selection: HashSet<String> =
        (0..1000).step_by(100).map(|i| format!("id_{i}")).collect();

    let table_state = TableState::default();
    let element = AudiobookTable::view(
        &large_audiobook_set,
        &large_selection,
        &table_state,
        &tokens,
    );
    let _ = element;
}

#[test]
fn audiobook_table_handles_malformed_data() {
    let tokens = MaterialTokens::default();

    // Create audiobooks with various data issues
    let problematic_audiobooks = vec![
        create_audiobook_with_empty_strings(),
        create_audiobook_with_special_characters(),
        create_audiobook_with_extreme_values(),
    ];

    let selected = HashSet::from(["empty".to_string()]);
    let table_state = TableState::default();

    let element = AudiobookTable::view(&problematic_audiobooks, &selected, &table_state, &tokens);
    let _ = element; // Should handle malformed data gracefully
}

// Helper functions for creating test audiobooks with specific characteristics

fn create_audiobook_with_empty_strings() -> Audiobook {
    create_custom_test_audiobook("empty", "", "", None)
}

fn create_audiobook_with_special_characters() -> Audiobook {
    create_custom_test_audiobook(
        "special",
        "Book with Special: !@#$%^&*()_+ Characters 测试 📚",
        "Author with émojí and ünïcödé",
        Some(3661), // Odd duration
    )
}

fn create_audiobook_with_extreme_values() -> Audiobook {
    let mut audiobook = create_custom_test_audiobook(
        "extreme",
        &"Very Long Title ".repeat(50),
        &"Extremely Long Author Name ".repeat(20),
        Some(u64::MAX / 2), // Very large duration
    );
    audiobook.size_bytes = Some(u64::MAX / 2);
    audiobook
}
