//! Tests for UI state management and workflows
//!
//! This module contains comprehensive tests for:
//! - UI state initialization and default values
//! - State transitions between different view modes  
//! - Library scanning workflow and progress tracking
//! - Audiobook selection and player state management
//! - Settings persistence and theme switching
//! - Complete workflow validations with side effects

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
        assert_eq!(
            state.processing_status,
            Some("Converting audio...".to_string())
        );
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
        use abop_core::models::audiobook::Audiobook;
        use std::path::PathBuf;

        // Test with default state
        let mut state = AppState::default();
        let summary = state.get_summary();

        // Verify summary format and required fields
        assert!(
            summary.starts_with("AppState {"),
            "Summary should start with AppState"
        );
        assert!(
            summary.contains("view=Library"),
            "Summary should include current view"
        );
        assert!(
            summary.contains("audiobooks=0"),
            "Summary should include audiobooks count"
        );
        assert!(
            summary.contains("libraries=0"),
            "Summary should include libraries count"
        );
        assert!(
            summary.ends_with('}'),
            "Summary should end with closing brace"
        );

        // Test with an audiobook selected
        let book_id = "test-book-123".to_string();
        state.select_audiobook(Some(book_id.clone()));
        let summary_with_selection = state.get_summary();
        assert!(
            summary_with_selection.contains(&format!("selected={}", book_id)),
            "Summary should include selected audiobook ID"
        );

        // Test with multiple audiobooks and libraries
        state
            .app_data
            .audiobooks
            .push(Audiobook::new("lib1", PathBuf::from("/test/book1.mp3")));
        state
            .app_data
            .audiobooks
            .push(Audiobook::new("lib1", PathBuf::from("/test/book2.mp3")));
        state
            .app_data
            .libraries
            .push(abop_core::models::library::Library::new(
                "Test Library",
                "/test/path",
            ));

        let summary_with_content = state.get_summary();
        assert!(
            summary_with_content.contains("audiobooks=2"),
            "Summary should reflect updated audiobooks count"
        );
        assert!(
            summary_with_content.contains("libraries=1"),
            "Summary should reflect updated libraries count"
        );

        // Verify summary remains concise
        assert!(
            summary_with_content.len() < 250,
            "Summary should remain concise with more content"
        );
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
        use abop_core::models::audiobook::Audiobook;
        use std::path::PathBuf;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::time::Duration;
        use tokio::runtime::Runtime;

        // Create a new runtime for testing async operations
        let rt = Runtime::new().expect("Failed to create runtime");
        rt.block_on(async {
            let mut state = UiState::default();
            let scan_complete = Arc::new(AtomicBool::new(false));
            let scan_progress = Arc::new(std::sync::Mutex::new(0.0));

            // Simulate scan start
            state.core_state.set_current_view(ViewType::Library);
            state.scanning = true;
            state.scan_progress = Some(0.0);

            assert!(state.scanning, "Scan should be in progress");
            assert_eq!(
                state.scan_progress,
                Some(0.0),
                "Initial progress should be 0%"
            );

            // Simulate scan progress updates
            let progress_updates = vec![0.1, 0.25, 0.5, 0.75, 0.9, 1.0];
            for progress in progress_updates {
                state.scan_progress = Some(progress);
                *scan_progress.lock().unwrap() = progress;

                // Verify progress updates are reflected in the UI state
                assert_eq!(
                    state.scan_progress,
                    Some(progress),
                    "Progress should be updated to {}",
                    progress
                );

                // Simulate UI updates based on progress
                if progress >= 0.5 && progress < 1.0 {
                    assert!(
                        state.scanning,
                        "Scan should still be in progress at {}%",
                        progress * 100.0
                    );
                }

                // Small delay to simulate work
                tokio::time::sleep(Duration::from_millis(10)).await;
            }

            // Simulate scan completion
            state.scanning = false;
            state.scan_progress = Some(1.0);
            scan_complete.store(true, Ordering::SeqCst);

            // Load test audiobooks with various valid paths
            let test_audiobooks = vec![
                Audiobook::new("test-lib", PathBuf::from("/valid/path/book1.mp3")),
                Audiobook::new(
                    "test-lib",
                    PathBuf::from(r"C:\Users\test\audiobooks\book2.mp3"),
                ),
                Audiobook::new("test-lib", PathBuf::from("relative/path/book3.mp3")),
            ];

            // Verify path validation in the test
            for book in &test_audiobooks {
                assert!(
                    !book.path.as_os_str().is_empty(),
                    "Audiobook path should not be empty"
                );

                // Check for common invalid path patterns
                let path_str = book.path.to_string_lossy();
                assert!(
                    !path_str.contains("\\?"),
                    "Path should not contain invalid characters: {}",
                    path_str
                );
            }

            // Update state with scanned audiobooks
            state.audiobooks = test_audiobooks;

            // Verify final state
            assert!(!state.scanning, "Scan should be complete");
            assert_eq!(state.scan_progress, Some(1.0), "Progress should be 100%");
            assert_eq!(state.audiobooks.len(), 3, "All audiobooks should be loaded");
            assert!(
                state.selected_audiobooks.is_empty(),
                "No audiobooks should be selected after scan"
            );

            // Verify all audiobooks have unique IDs
            let mut ids = std::collections::HashSet::new();
            for book in &state.audiobooks {
                assert!(!book.id.is_empty(), "Audiobook ID should not be empty");
                assert!(
                    ids.insert(&book.id),
                    "Duplicate audiobook ID found: {}",
                    book.id
                );
            }

            // Verify the scan completion flag was set
            assert!(
                scan_complete.load(Ordering::SeqCst),
                "Scan completion flag should be set"
            );

            // Verify the final progress value
            assert_eq!(
                *scan_progress.lock().unwrap(),
                1.0,
                "Final progress should be 100%"
            );
        });
    }

    #[test]
    fn test_audiobook_playback_workflow() {
        use abop_core::models::audiobook::Audiobook;
        use std::path::PathBuf;

        let mut state = UiState::default();

        // Test data with realistic audiobook paths
        let test_audiobooks = [
            ("Test Audiobook 1", "/valid/path/audiobook.mp3"),
            ("Test Audiobook 2", r"C:\Users\test\audiobooks\book.mp3"),
            ("Test Audiobook 3", "relative/path/audiobook.mp3"),
            ("Test Audiobook 4", "with spaces/path to/book.mp3"),
        ];

        for (i, (_title, test_path)) in test_audiobooks.iter().enumerate() {
            let _book_id = format!("playbook_test_{}", i);
            let library_id = "test-library";
            
            // Setup: Create and add an audiobook to the state
            let audiobook = Audiobook::new(library_id, PathBuf::from(test_path));
            let audiobook_id = audiobook.id.clone();
            state.audiobooks.push(audiobook);

            // Select the audiobook
            state.selected_audiobooks.clear();
            state.selected_audiobooks.insert(audiobook_id.clone());

            // Get the path for playback (must match an existing audiobook)
            let playing_path = state.audiobooks
                .iter()
                .find(|book| book.id == audiobook_id)
                .unwrap()
                .path
                .clone();

            // Step 1: Start playback
            state.player_state = PlayerState::Playing;
            state.current_playing_file = Some(playing_path.clone());

            // Verify playback started correctly
            assert_eq!(
                state.player_state,
                PlayerState::Playing,
                "Playback should be in progress"
            );
            assert!(
                state.current_playing_file.is_some(),
                "Current playing file should be set"
            );

            // Verify the playing file matches an existing audiobook
            assert!(
                state.audiobooks.iter().any(|book| 
                    &book.path == state.current_playing_file.as_ref().unwrap()
                ),
                "Playing file must correspond to an existing audiobook"
            );

            // Step 2: Pause playback
            state.player_state = PlayerState::Paused;
            assert_eq!(
                state.player_state,
                PlayerState::Paused,
                "Playback should be paused"
            );

            // Step 3: Resume playback
            state.player_state = PlayerState::Playing;
            assert_eq!(
                state.player_state,
                PlayerState::Playing,
                "Playback should have resumed"
            );

            // Step 4: Stop playback
            state.player_state = PlayerState::Stopped;
            state.current_playing_file = None;

            // Verify final state
            assert_eq!(
                state.player_state,
                PlayerState::Stopped,
                "Playback should be stopped"
            );
            assert!(
                state.current_playing_file.is_none(),
                "No file should be playing"
            );

            // Verify the audiobook is still selected after playback
            assert!(
                state.selected_audiobooks.contains(&audiobook_id),
                "Audiobook should remain selected after playback"
            );

            // Clear audiobooks for next iteration
            state.audiobooks.clear();
        }

        // Test edge case: Try to play a file that doesn't exist in audiobooks
        state.current_playing_file = Some(PathBuf::from("/nonexistent/file.mp3"));
        assert!(
            !state.audiobooks.iter().any(|book| 
                &book.path == state.current_playing_file.as_ref().unwrap()
            ),
            "Playing a non-existent file should not match any audiobook"
        );
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
