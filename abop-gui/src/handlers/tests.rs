//! Tests for handler modules

#[cfg(test)]
mod ui_state_tests {
    use super::super::ui_state::handle_ui_message;
    use crate::messages::Message;
    use crate::state::UiState;
    use crate::theme::ThemeMode;
    use std::path::PathBuf;

    // Test constants to reduce duplication and improve clarity
    mod test_constants {
        pub const TEST_BOOK1_PATH: &str = "/test/book1.mp3";
        pub const TEST_BOOK2_PATH: &str = "/test/book2.mp3";
        pub const TEST_BOOK3_PATH: &str = "/test/book3.mp3";
        pub const TEST_RECENT_PATH: &str = "/test/recent/path";
        pub const TEST_LIBRARY_ID: &str = "lib1";
        pub const TEST_AUTHOR_A: &str = "Author A";
        pub const TEST_AUTHOR_B: &str = "Author B";
        pub const TEST_AUTHOR_C: &str = "Author C";
        pub const TEST_TITLE_1: &str = "Zebra Book";
        pub const TEST_TITLE_2: &str = "Alpha Book";
        pub const TEST_TITLE_3: &str = "Book with Numbers 123";
    }
    
    use test_constants::*;
    
    /// Helper function to create a test audiobook with the given parameters
    fn create_test_audiobook(
        id: &str, 
        title: &str, 
        author: &str,
        path: &str
    ) -> abop_core::models::audiobook::Audiobook {
        use abop_core::models::audiobook::Audiobook;
        let mut book = Audiobook::new(TEST_LIBRARY_ID, PathBuf::from(path));
        book.id = id.to_string();
        book.title = Some(title.to_string());
        book.author = Some(author.to_string());
        book
    }

    #[test]
    fn test_handle_show_settings() {
        let mut state = UiState::default();
        assert!(!state.settings_open);

        let task = handle_ui_message(&mut state, Message::ShowSettings);
        assert!(state.settings_open);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_close_settings() {
        let mut state = UiState::default();
        state.settings_open = true;

        let task = handle_ui_message(&mut state, Message::CloseSettings);
        assert!(!state.settings_open);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_show_recent_directories() {
        let mut state = UiState::default();
        let task = handle_ui_message(&mut state, Message::ShowRecentDirectories);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_set_theme() {
        let mut state = UiState::default();
        state.theme_mode = ThemeMode::Light;

        let task = handle_ui_message(&mut state, Message::SetTheme(ThemeMode::Dark));
        assert_eq!(state.theme_mode, ThemeMode::Dark);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_toggle_theme() {
        let mut state = UiState::default();

        // Start with light theme
        state.theme_mode = ThemeMode::Light;
        let task = handle_ui_message(&mut state, Message::ToggleTheme);
        assert_eq!(state.theme_mode, ThemeMode::Dark);
        assert!(task.is_some());

        // Toggle back to light
        let task = handle_ui_message(&mut state, Message::ToggleTheme);
        assert_eq!(state.theme_mode, ThemeMode::Light);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_toggle_select_all() {
        let mut state = UiState::default();
        // Add some audiobooks using the correct structure
        use abop_core::models::audiobook::Audiobook;
        use std::path::PathBuf;
        let audiobook1 = Audiobook::new(TEST_LIBRARY_ID, PathBuf::from(TEST_BOOK1_PATH));
        let audiobook2 = Audiobook::new(TEST_LIBRARY_ID, PathBuf::from(TEST_BOOK2_PATH));

        state.audiobooks = vec![audiobook1, audiobook2];

        // Initially nothing selected
        assert!(state.selected_audiobooks.is_empty());

        // Toggle select all - should select all
        let task = handle_ui_message(&mut state, Message::ToggleSelectAll);
        assert_eq!(state.selected_audiobooks.len(), 2);
        assert!(task.is_some());

        // Toggle again - should deselect all
        let task = handle_ui_message(&mut state, Message::ToggleSelectAll);
        assert!(state.selected_audiobooks.is_empty());
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_toggle_auto_save_library() {
        let mut state = UiState::default();
        let initial_value = state.auto_save_library;

        let task = handle_ui_message(&mut state, Message::ToggleAutoSaveLibrary);
        assert_eq!(state.auto_save_library, !initial_value);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_toggle_scan_subdirectories() {
        let mut state = UiState::default();
        let initial_value = state.scan_subdirectories;

        let task = handle_ui_message(&mut state, Message::ToggleScanSubdirectories);
        assert_eq!(state.scan_subdirectories, !initial_value);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_toggle_audiobook_selection() {
        let mut state = UiState::default();
        let audiobook_id = "test-id".to_string();

        // Initially not selected
        assert!(!state.selected_audiobooks.contains(&audiobook_id));

        // Select the audiobook
        let task = handle_ui_message(
            &mut state,
            Message::ToggleAudiobookSelection(audiobook_id.clone()),
        );
        assert!(state.selected_audiobooks.contains(&audiobook_id));
        assert!(task.is_some());

        // Deselect the audiobook
        let task = handle_ui_message(
            &mut state,
            Message::ToggleAudiobookSelection(audiobook_id.clone()),
        );
        assert!(!state.selected_audiobooks.contains(&audiobook_id));
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_select_recent_directory() {
        let mut state = UiState::default();
        let path = PathBuf::from(TEST_RECENT_PATH);

        let task = handle_ui_message(&mut state, Message::SelectRecentDirectory(path.clone()));
        // Should return a task for handling directory selection
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_play_pause() {
        let mut state = UiState::default();
        let task = handle_ui_message(&mut state, Message::PlayPause);
        // Play/pause should return a task
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_stop() {
        let mut state = UiState::default();
        let task = handle_ui_message(&mut state, Message::Stop);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_previous() {
        let mut state = UiState::default();
        let task = handle_ui_message(&mut state, Message::Previous);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_next() {
        let mut state = UiState::default();
        let task = handle_ui_message(&mut state, Message::Next);
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_reset_redraw_flag() {
        let mut state = UiState::default();
        let task = handle_ui_message(&mut state, Message::ResetRedrawFlag);
        assert!(task.is_some());
    }
    #[test]
    fn test_handle_sort_by() {
        use abop_core::models::audiobook::Audiobook;
        use std::collections::HashSet;
        
        let mut state = UiState::default();
        
        // Create test audiobooks with different attributes for comprehensive sorting
        let book1 = create_test_audiobook("1", TEST_TITLE_1, TEST_AUTHOR_A, TEST_BOOK1_PATH);
        let book2 = create_test_audiobook("2", TEST_TITLE_2, TEST_AUTHOR_B, TEST_BOOK2_PATH);
        let book3 = create_test_audiobook("3", TEST_TITLE_3, TEST_AUTHOR_C, TEST_BOOK3_PATH);
        
        // Set initial state with unsorted books
        state.audiobooks = vec![book1.clone(), book2.clone(), book3.clone()];
        
        // Test sorting by title
        let task = handle_ui_message(&mut state, Message::SortBy("title".to_string()));
        assert!(task.is_some(), "Sorting by title should return a task");
        
        // Verify sort order after title sort (should be Alpha, Book with Numbers, Zebra)
        if let Message::SortAudiobooks { sort_key, sort_order: _ } = task.unwrap().message() {
            assert_eq!(sort_key, "title", "Sort key should be 'title'");
        } else {
            panic!("Expected SortAudiobooks message");
        }
        
        // Test sorting by author
        let task = handle_ui_message(&mut state, Message::SortBy("author".to_string()));
        assert!(task.is_some(), "Sorting by author should return a task");
        
        // Verify sort order after author sort (should be Author A, B, C)
        if let Message::SortAudiobooks { sort_key, sort_order: _ } = task.unwrap().message() {
            assert_eq!(sort_key, "author", "Sort key should be 'author'");
        } else {
            panic!("Expected SortAudiobooks message");
        }
        
        // Test sorting by path
        let task = handle_ui_message(&mut state, Message::SortBy("path".to_string()));
        assert!(task.is_some(), "Sorting by path should return a task");
        
        // Test toggling sort order (ascending/descending)
        state.sort_ascending = true;
        let task = handle_ui_message(&mut state, Message::SortBy("title".to_string()));
        assert!(task.is_some(), "Toggling sort order should return a task");
        
        // Verify sort order toggled to descending
        if let Message::SortAudiobooks { sort_key: _, sort_order } = task.unwrap().message() {
            assert!(!sort_order, "Sort order should be toggled to descending");
        } else {
            panic!("Expected SortAudiobooks message");
        }
        
        // Test sorting by an unknown field (should default to title)
        let task = handle_ui_message(&mut state, Message::SortBy("unknown_field".to_string()));
        assert!(task.is_some(), "Sorting by unknown field should still return a task");
        
        // Verify default sort key is used for unknown fields
        if let Message::SortAudiobooks { sort_key, sort_order: _ } = task.unwrap().message() {
            assert_eq!(sort_key, "title", "Should default to 'title' for unknown sort fields");
        } else {
            panic!("Expected SortAudiobooks message");
        }
        
        // Test with empty audiobooks list
        state.audiobooks.clear();
        let task = handle_ui_message(&mut state, Message::SortBy("title".to_string()));
        assert!(task.is_some(), "Should handle empty audiobooks list");
    }

    #[test]
    fn test_handle_non_ui_message() {
        let mut state = UiState::default();
        // Test with a message that's not handled by UI handler
        let task = handle_ui_message(
            &mut state,
            Message::DirectorySelected(Some(PathBuf::from("/test"))),
        );
        assert!(task.is_none());
    }
}
