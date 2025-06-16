//! Tests for handler modules

#[cfg(test)]
mod ui_state_tests {
    use super::super::ui_state::handle_ui_message;
    use crate::messages::Message;
    use crate::state::UiState;
    use crate::theme::ThemeMode;
    use std::path::PathBuf;

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
        
        let audiobook1 = Audiobook::new("lib1", PathBuf::from("/test/book1.mp3"));
        let audiobook2 = Audiobook::new("lib1", PathBuf::from("/test/book2.mp3"));
        
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
        let task = handle_ui_message(&mut state, Message::ToggleAudiobookSelection(audiobook_id.clone()));
        assert!(state.selected_audiobooks.contains(&audiobook_id));
        assert!(task.is_some());
        
        // Deselect the audiobook
        let task = handle_ui_message(&mut state, Message::ToggleAudiobookSelection(audiobook_id.clone()));
        assert!(!state.selected_audiobooks.contains(&audiobook_id));
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_select_recent_directory() {
        let mut state = UiState::default();
        let path = PathBuf::from("/test/recent/path");
        
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
        let mut state = UiState::default();
        let task = handle_ui_message(&mut state, Message::SortBy("title".to_string()));
        assert!(task.is_some());
    }

    #[test]
    fn test_handle_non_ui_message() {
        let mut state = UiState::default();
        // Test with a message that's not handled by UI handler
        let task = handle_ui_message(&mut state, Message::DirectorySelected(Some(PathBuf::from("/test"))));
        assert!(task.is_none());
    }
}
