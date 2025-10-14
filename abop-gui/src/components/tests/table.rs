//! Tests for `table_core` component

use crate::components::table_core::AudiobookTable;
use crate::state::TableState;
use crate::styling::material::MaterialTokens;
use crate::test_utils::create_test_audiobook;
use crate::utils::image_cache::ImageCache;
use std::collections::HashSet;
use std::sync::Arc;

#[test]
fn test_audiobook_table_empty() {
    let tokens = MaterialTokens::default();
    let audiobooks = vec![];
    let selected = HashSet::new();
    let table_state = TableState::default();

    let cache = Arc::new(ImageCache::new());
    let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens, &cache);
    let _ = element; // Just verify it compiles and runs
}

#[test]
fn test_audiobook_table_with_data() {
    let tokens = MaterialTokens::default();
    let audiobooks = vec![
        create_test_audiobook("1", "Book One"),
        create_test_audiobook("2", "Book Two"),
    ];
    let selected = HashSet::new();
    let table_state = TableState::default();

    let cache = Arc::new(ImageCache::new());
    let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens, &cache);
    let _ = element; // Just verify it compiles and runs
}

#[test]
fn test_audiobook_table_with_selection() {
    let tokens = MaterialTokens::default();
    let audiobooks = vec![
        create_test_audiobook("1", "Book One"),
        create_test_audiobook("2", "Book Two"),
    ];
    let mut selected = HashSet::new();
    selected.insert("1".to_string());
    let table_state = TableState::default();

    let cache = Arc::new(ImageCache::new());
    let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens, &cache);
    let _ = element; // Just verify it compiles and runs
}
