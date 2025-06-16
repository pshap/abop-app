//! Tests for component modules

#[cfg(test)]
mod about_tests {
    use super::super::about::AboutView;
    use crate::theme::ThemeMode;

    #[test]
    fn test_about_view_component_creation() {
        let element = AboutView::view(ThemeMode::Light);

        // Should create element without panicking
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_about_view_with_different_themes() {
        // Test light theme
        let light_element = AboutView::view(ThemeMode::Light);
        let _ = light_element; // Just verify it compiles and runs

        // Test dark theme
        let dark_element = AboutView::view(ThemeMode::Dark);
        let _ = dark_element; // Just verify it compiles and runs
    }
}

#[cfg(test)]
mod audio_controls_tests {
    use super::super::audio_controls::AudioControls;
    use crate::styling::material::MaterialTokens;
    use abop_core::PlayerState;
    use abop_core::models::audiobook::Audiobook;
    use std::collections::HashSet;
    use std::path::PathBuf;

    fn create_test_audiobook(id: &str, title: &str) -> Audiobook {
        let path = PathBuf::from(format!("/test/path/{}.mp3", title));
        let mut audiobook = Audiobook::new("test-library-id", &path);
        audiobook.id = id.to_string();
        audiobook.title = Some(title.to_string());
        audiobook.author = Some("Test Author".to_string());
        audiobook.duration_seconds = Some(3600);
        audiobook.size_bytes = Some(1024000);
        audiobook
    }
    #[test]
    fn test_audio_controls_view() {
        let tokens = MaterialTokens::default();
        let audiobooks = vec![create_test_audiobook("1", "Test Book")];
        let selected_ids = HashSet::new();

        let element =
            AudioControls::view(&selected_ids, &audiobooks, PlayerState::Stopped, &tokens);
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_audio_controls_with_selected_audiobooks() {
        let tokens = MaterialTokens::default();
        let audiobooks = vec![
            create_test_audiobook("1", "Book One"),
            create_test_audiobook("2", "Book Two"),
        ];
        let mut selected_ids = HashSet::new();
        selected_ids.insert("1".to_string());

        let element =
            AudioControls::view(&selected_ids, &audiobooks, PlayerState::Stopped, &tokens);
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_audio_controls_with_different_player_states() {
        let tokens = MaterialTokens::default();
        let audiobooks = vec![create_test_audiobook("1", "Test Book")];
        let mut selected_ids = HashSet::new();
        selected_ids.insert("1".to_string());

        // Test different player states
        let states = [
            PlayerState::Stopped,
            PlayerState::Playing,
            PlayerState::Paused,
        ];
        for state in states {
            let element = AudioControls::view(&selected_ids, &audiobooks, state, &tokens);
            let _ = element; // Just verify it compiles and runs
        }
    }
}

#[cfg(test)]
mod audio_toolbar_tests {
    use super::super::audio_toolbar::AudioToolbar;
    use crate::styling::material::MaterialTokens;

    #[test]
    fn test_audio_toolbar_creation() {
        let tokens = MaterialTokens::default();
        let toolbar = AudioToolbar::new();
        let element = toolbar.view(&tokens);

        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_audio_toolbar_with_playing_state() {
        let tokens = MaterialTokens::default();
        let mut toolbar = AudioToolbar::new();

        toolbar.set_playing(true);
        let playing_element = toolbar.view(&tokens);
        let _ = playing_element; // Just verify it compiles and runs

        toolbar.set_playing(false);
        let stopped_element = toolbar.view(&tokens);
        let _ = stopped_element; // Just verify it compiles and runs
    }
}

#[cfg(test)]
mod status_tests {
    use super::super::status::{EnhancedStatusDisplayParams, StatusDisplay};
    use crate::styling::material::MaterialTokens;
    use crate::theme::ThemeMode;
    use abop_core::PlayerState;
    use abop_core::scanner::ScanProgress;

    #[test]
    fn test_status_display_enhanced() {
        let tokens = MaterialTokens::default();
        let params = EnhancedStatusDisplayParams {
            scanning: false,
            scan_progress: None,
            cached_scan_progress_text: Some("Scanning progress"),
            processing_audio: false,
            processing_progress: Some(0.5),
            cached_processing_progress_text: Some("Processing progress"),
            processing_status: Some("Processing status"),
            player_state: PlayerState::Stopped,
            current_playing_file: None,
            selected_count: 2,
            total_count: 10,
            theme: ThemeMode::Light,
        };

        let element = StatusDisplay::enhanced_view(params, &tokens);
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_status_display_edge_cases() {
        let tokens = MaterialTokens::default();

        // Test with None progress
        let params_none = EnhancedStatusDisplayParams {
            scanning: false,
            scan_progress: None,
            cached_scan_progress_text: None,
            processing_audio: false,
            processing_progress: None,
            cached_processing_progress_text: None,
            processing_status: None,
            player_state: PlayerState::Stopped,
            current_playing_file: None,
            selected_count: 0,
            total_count: 0,
            theme: ThemeMode::Light,
        };
        let element = StatusDisplay::enhanced_view(params_none, &tokens);
        let _ = element; // Verify None values work
        // Test with starting scan progress
        let params_zero = EnhancedStatusDisplayParams {
            scanning: true,
            scan_progress: Some(ScanProgress::Started { total_files: 100 }),
            cached_scan_progress_text: Some("Starting scan..."),
            processing_audio: true,
            processing_progress: Some(0.0),
            cached_processing_progress_text: Some("Starting processing..."),
            processing_status: Some("Initializing"),
            player_state: PlayerState::Playing,
            current_playing_file: None,
            selected_count: 0,
            total_count: 100,
            theme: ThemeMode::Dark,
        };
        let element = StatusDisplay::enhanced_view(params_zero, &tokens);
        let _ = element; // Verify started scan progress works
        // Test with file processing progress
        let test_audio_path = std::path::PathBuf::from("/test/audio.mp3");
        let params_complete = EnhancedStatusDisplayParams {
            scanning: false,
            scan_progress: Some(ScanProgress::Complete {
                processed: 100,
                errors: 0,
                duration: std::time::Duration::from_secs(10),
            }),
            cached_scan_progress_text: Some("Scan complete"),
            processing_audio: false,
            processing_progress: Some(1.0),
            cached_processing_progress_text: Some("Processing complete"),
            processing_status: Some("Finished"),
            player_state: PlayerState::Paused,
            current_playing_file: Some(&test_audio_path),
            selected_count: 1,
            total_count: 1,
            theme: ThemeMode::MaterialDark,
        };
        let element = StatusDisplay::enhanced_view(params_complete, &tokens);
        let _ = element; // Verify complete scan progress works

        // Test out-of-bounds progress (should be handled gracefully)
        let params_oob = EnhancedStatusDisplayParams {
            scanning: true,
            scan_progress: Some(ScanProgress::FileProcessed {
                current: 150,
                total: 100, // Out of bounds - more current than total
                file_name: "test.mp3".to_string(),
                progress_percentage: 1.5, // Out of bounds percentage
            }),
            cached_scan_progress_text: Some("Invalid progress"),
            processing_audio: true,
            processing_progress: Some(-0.1), // Out of bounds
            cached_processing_progress_text: Some("Invalid processing"),
            processing_status: Some("Error state"),
            player_state: PlayerState::Stopped,
            current_playing_file: None,
            selected_count: 999,
            total_count: 1,
            theme: ThemeMode::Light,
        };
        let element = StatusDisplay::enhanced_view(params_oob, &tokens);
        let _ = element; // Verify out-of-bounds handled gracefully
    }

    #[test]
    fn test_status_display_app_footer() {
        let element = StatusDisplay::app_footer(15, ThemeMode::Light);
        let _ = element; // Just verify it compiles and runs
    }
}

#[cfg(test)]
mod table_tests {
    use super::super::table_core::AudiobookTable;
    use crate::state::TableState;
    use crate::styling::material::MaterialTokens;
    use crate::test_utils::create_test_audiobook;
    use std::collections::HashSet;
    #[test]
    fn test_audiobook_table_empty() {
        let tokens = MaterialTokens::default();
        let audiobooks = vec![];
        let selected = HashSet::new();
        let table_state = TableState::default();

        let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens);
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

        let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens);
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

        let element = AudiobookTable::view(&audiobooks, &selected, &table_state, &tokens);
        let _ = element; // Just verify it compiles and runs
    }
}
