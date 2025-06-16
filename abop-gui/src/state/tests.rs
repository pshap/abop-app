//! Tests for AppState management
//! 
//! This module contains tests for application state initialization,
//! transitions, and persistence.

use super::*;
use abop_core::models::ui::{AppState, ViewType};

#[cfg(test)]
mod ui_state_tests {
    use super::*;

    #[test]
    fn test_ui_state_default_initialization() {
        let state = UiState::default();
        
        assert_eq!(state.theme_mode, ThemeMode::Dark);
        assert!(!state.settings_open);
        assert!(!state.recent_directories_open);
        assert!(state.auto_save_library);
        assert!(state.scan_subdirectories);
        assert!(state.selected_audiobooks.is_empty());
        assert!(!state.scanning);
        assert!(!state.processing_audio);
        assert_eq!(state.player_state, PlayerState::Stopped);
    }

    #[test]
    fn test_ui_state_from_core_state() {
        let core_state = AppState::default();
        let ui_state = UiState::from_core_state(core_state);
        
        assert_eq!(ui_state.core_state.current_view, ViewType::Library);
        assert!(ui_state.core_state.selected_audiobook_id.is_none());
        assert!(ui_state.audiobooks.is_empty());
    }

    #[test]
    fn test_theme_mode_switching() {
        let mut state = UiState::default();
        
        // Test switching to light theme
        state.set_theme_mode(ThemeMode::Light);
        assert_eq!(state.theme_mode, ThemeMode::Light);
        
        // Test switching to material dark
        state.set_theme_mode(ThemeMode::MaterialDark);
        assert_eq!(state.theme_mode, ThemeMode::MaterialDark);
    }

    #[test]
    fn test_library_selection() {
        let mut state = UiState::default();
        let test_book_id = "test_book_123".to_string();
        
        // Test selecting a book
        state.selected_audiobooks.insert(test_book_id.clone());
        assert!(state.selected_audiobooks.contains(&test_book_id));
        assert_eq!(state.selected_audiobooks.len(), 1);
        
        // Test clearing selection
        state.selected_audiobooks.clear();
        assert!(state.selected_audiobooks.is_empty());
    }

    #[test]
    fn test_scan_state_management() {
        let mut state = UiState::default();
        
        // Test setting scan state
        state.scanning = true;
        state.scan_progress = Some(0.5);
        assert!(state.scanning);
        assert_eq!(state.scan_progress, Some(0.5));
        
        // Test clearing scan state
        state.scanning = false;
        state.scan_progress = None;
        assert!(!state.scanning);
        assert!(state.scan_progress.is_none());
    }

    #[test]
    fn test_audio_player_state() {
        let mut state = UiState::default();
        
        // Test play state
        state.player_state = PlayerState::Playing;
        assert_eq!(state.player_state, PlayerState::Playing);
        
        // Test pause state
        state.player_state = PlayerState::Paused;
        assert_eq!(state.player_state, PlayerState::Paused);
        
        // Test stop state
        state.player_state = PlayerState::Stopped;
        assert_eq!(state.player_state, PlayerState::Stopped);
    }

    #[test]
    fn test_processing_state() {
        let mut state = UiState::default();
        
        // Test audio processing state
        state.processing_audio = true;
        state.processing_progress = Some(0.75);
        state.processing_status = Some("Converting audio...".to_string());
        
        assert!(state.processing_audio);
        assert_eq!(state.processing_progress, Some(0.75));
        assert_eq!(state.processing_status, Some("Converting audio...".to_string()));
    }

    #[test]
    fn test_settings_dialogs() {
        let mut state = UiState::default();
        
        // Test settings dialog
        state.settings_open = true;
        assert!(state.settings_open);
        
        state.settings_open = false;
        assert!(!state.settings_open);
        
        // Test recent directories dialog
        state.recent_directories_open = true;
        assert!(state.recent_directories_open);
        
        state.recent_directories_open = false;
        assert!(!state.recent_directories_open);
    }
}

#[cfg(test)]
mod core_state_tests {
    use super::*;

    #[test]
    fn test_core_state_default_initialization() {
        let state = AppState::default();
        
        assert_eq!(state.current_view, ViewType::Library);
        assert!(state.selected_audiobook_id.is_none());
        assert!(state.app_data.audiobooks.is_empty());
        assert!(state.app_data.libraries.is_empty());
        assert!(state.app_data.progress.is_empty());
    }

    #[test]
    fn test_view_transitions() {
        let mut state = AppState::default();
        
        // Test transition to settings view
        state.set_current_view(ViewType::Settings);
        assert_eq!(state.current_view, ViewType::Settings);
        
        // Test transition to audio processing view
        state.set_current_view(ViewType::AudioProcessing);
        assert_eq!(state.current_view, ViewType::AudioProcessing);
        
        // Test transition back to library
        state.set_current_view(ViewType::Library);
        assert_eq!(state.current_view, ViewType::Library);
    }

    #[test]
    fn test_audiobook_selection() {
        let mut state = AppState::default();
        let test_book_id = "audiobook_456".to_string();
        
        // Test selecting an audiobook
        state.select_audiobook(Some(test_book_id.clone()));
        assert_eq!(state.selected_audiobook_id, Some(test_book_id));
        
        // Test clearing selection
        state.select_audiobook(None);
        assert!(state.selected_audiobook_id.is_none());
    }

    #[test]
    fn test_state_summary() {
        let state = AppState::default();
        let summary = state.get_summary();
        
        assert!(summary.contains("AppState"));
        assert!(summary.contains("view="));
        assert!(summary.contains("audiobooks=0"));
        assert!(summary.contains("libraries=0"));
    }

    #[test]
    fn test_state_reset() {
        let mut state = AppState::default();
        
        // Modify state
        state.set_current_view(ViewType::Settings);
        state.select_audiobook(Some("test".to_string()));
        
        // Reset state
        state.reset();
        
        // Verify reset
        assert_eq!(state.current_view, ViewType::Library);
        assert!(state.selected_audiobook_id.is_none());
    }
}

#[cfg(test)]
mod state_workflow_tests {
    use super::*;

    #[test]
    fn test_library_scanning_workflow() {
        let mut state = UiState::default();
        
        // Step 1: User starts a scan
        state.scanning = true;
        state.scan_progress = Some(0.0);
        
        // Step 2: Scan progresses
        state.scan_progress = Some(0.5);
        
        // Step 3: Scan completes
        state.scanning = false;
        state.scan_progress = Some(1.0);
        
        // Step 4: Results are loaded
        // (In real app, audiobooks would be populated)
        
        // Verify workflow state
        assert!(!state.scanning);
        assert_eq!(state.scan_progress, Some(1.0));
    }

    #[test]
    fn test_audiobook_playback_workflow() {
        let mut state = UiState::default();
        use std::path::PathBuf;
        
        // Setup: Audiobook is selected
        let book_id = "playback_test".to_string();
        state.selected_audiobooks.insert(book_id.clone());
        
        // Step 1: User starts playback
        state.player_state = PlayerState::Playing;
        state.current_playing_file = Some(PathBuf::from("/test/audiobook.mp3"));
        
        // Step 2: User pauses
        state.player_state = PlayerState::Paused;
        
        // Step 3: User resumes
        state.player_state = PlayerState::Playing;
        
        // Step 4: Playback stops
        state.player_state = PlayerState::Stopped;
        state.current_playing_file = None;
        
        // Verify final state
        assert_eq!(state.player_state, PlayerState::Stopped);
        assert!(state.current_playing_file.is_none());
        assert!(state.selected_audiobooks.contains(&book_id));
    }

    #[test]
    fn test_audio_processing_workflow() {
        let mut state = UiState::default();
        
        // Setup: Audiobooks are selected
        state.selected_audiobooks.insert("book1".to_string());
        state.selected_audiobooks.insert("book2".to_string());
        
        // Step 1: User starts processing
        state.processing_audio = true;
        state.processing_progress = Some(0.0);
        state.processing_status = Some("Starting audio processing...".to_string());
        
        // Step 2: Processing progresses
        state.processing_progress = Some(0.25);
        state.processing_status = Some("Converting book1...".to_string());
        
        // Step 3: Processing continues
        state.processing_progress = Some(0.75);
        state.processing_status = Some("Converting book2...".to_string());
        
        // Step 4: Processing completes
        state.processing_audio = false;
        state.processing_progress = Some(1.0);
        state.processing_status = Some("Processing complete!".to_string());
        
        // Verify completion
        assert!(!state.processing_audio);
        assert_eq!(state.processing_progress, Some(1.0));
        assert_eq!(state.selected_audiobooks.len(), 2);
    }

    #[test]
    fn test_view_navigation_workflow() {
        let mut state = UiState::default();
        
        // Start in library view
        assert_eq!(state.core_state.current_view, ViewType::Library);
        
        // Navigate to settings
        state.core_state.set_current_view(ViewType::Settings);
        assert_eq!(state.core_state.current_view, ViewType::Settings);
        
        // Navigate to audio processing
        state.core_state.set_current_view(ViewType::AudioProcessing);
        assert_eq!(state.core_state.current_view, ViewType::AudioProcessing);
        
        // Navigate to about
        state.core_state.set_current_view(ViewType::About);
        assert_eq!(state.core_state.current_view, ViewType::About);
        
        // Return to library
        state.core_state.set_current_view(ViewType::Library);
        assert_eq!(state.core_state.current_view, ViewType::Library);
    }
}
