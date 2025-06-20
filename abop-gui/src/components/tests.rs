//! Tests for component modules
//!
//! This module contains comprehensive tests for all UI components, ensuring they
//! handle various states, edge cases, and user interactions correctly.

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

    /// Helper function to create a test audiobook with the given ID and title
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the audiobook
    /// * `title` - Title of the audiobook
    ///
    /// # Returns
    /// A new `Audiobook` instance with test data
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

    /// Helper function to create a test audiobook with custom metadata
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the audiobook
    /// * `title` - Title of the audiobook
    /// * `author` - Author of the audiobook
    /// * `duration_seconds` - Duration in seconds
    /// * `size_bytes` - File size in bytes
    ///
    /// # Returns
    /// A new `Audiobook` instance with the specified metadata
    fn create_custom_audiobook(
        id: &str,
        title: &str,
        author: &str,
        duration_seconds: Option<u64>,
        size_bytes: Option<u64>,
    ) -> Audiobook {
        let path = PathBuf::from(format!("/test/path/{}.mp3", title));
        let mut audiobook = Audiobook::new("test-library-id", &path);
        audiobook.id = id.to_string();
        audiobook.title = Some(title.to_string());
        audiobook.author = Some(author.to_string());
        audiobook.duration_seconds = duration_seconds;
        audiobook.size_bytes = size_bytes;
        audiobook
    }
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
                println!("Testing with {} and {} state", selection_desc, state_desc);
                let _element = AudioControls::view(selected, &audiobooks, state.clone(), &tokens);
                // If we get here, the view function didn't panic
            }
        }

        // Test with empty audiobooks list
        let empty_audiobooks: Vec<Audiobook> = Vec::new();
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
            create_custom_audiobook("1", "Book One", "Author A", Some(3600), Some(1024000)),
            create_custom_audiobook("2", "Book Two", "Author B", Some(7200), Some(2048000)),
            create_custom_audiobook("3", "Book Three", "Author C", None, None), // Incomplete metadata
        ];

        for (selected, description) in test_cases {
            println!("Testing case: {}", description);
            let selected_ids: HashSet<_> = selected.into_iter().map(String::from).collect();

            // Test with different player states
            let player_states = [
                PlayerState::Stopped,
                PlayerState::Playing,
                PlayerState::Paused,
            ];

            for state in &player_states {
                let _element =
                    AudioControls::view(&selected_ids, &audiobooks, state.clone(), &tokens);
                // Element creation successful if we get here
            }
        }
    }

    #[test]
    fn test_audio_controls_edge_cases() {
        let tokens = MaterialTokens::default();

        // Test with empty audiobooks and empty selection
        let empty_audiobooks: Vec<Audiobook> = Vec::new();
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
        let long_author =
            "Author with a very long name that should also be properly handled in the UI";
        let long_metadata_audiobook =
            create_custom_audiobook("1", long_title, long_author, Some(999999), Some(9999999999));

        // Test with special characters in metadata
        let special_chars_audiobook = create_custom_audiobook(
            "2",
            "Book with special chars: !@#$%^&*()_+{}|:<>?",
            "Author with emoji 😊 and unicode 测试",
            Some(3600),
            Some(1024000),
        );

        // Test with missing metadata
        let missing_metadata_audiobook = create_custom_audiobook("3", "", "", None, None);

        // Test with extremely large numbers
        let large_numbers_audiobook = create_custom_audiobook(
            "4",
            "Book with large numbers",
            "Author",
            Some(std::u64::MAX),
            Some(std::u64::MAX),
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
            println!("Testing edge cases with {}", selection_desc);

            // Test with different player states
            let player_states = [
                PlayerState::Stopped,
                PlayerState::Playing,
                PlayerState::Paused,
            ];

            for state in &player_states {
                let _element =
                    AudioControls::view(selected, &test_audiobooks, state.clone(), &tokens);
                // If we get here, the view function didn't panic
            }
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
    use std::path::PathBuf;

    #[test]
    fn test_status_display_enhanced() {
        let tokens = MaterialTokens::default();
        let test_audio_path = PathBuf::from("/test/audio.mp3");

        let params = EnhancedStatusDisplayParams {
            scanning: true,
            scan_progress: Some(ScanProgress::FileProcessed {
                current: 5,
                total: 10,
                file_name: "test.mp3".to_string(),
                progress_percentage: 0.5,
            }),
            cached_scan_progress_text: Some("Scanning progress"),
            processing_audio: true,
            processing_progress: Some(0.5),
            cached_processing_progress_text: Some("Processing progress"),
            processing_status: Some("Processing status"),
            player_state: PlayerState::Playing,
            current_playing_file: Some(&test_audio_path),
            selected_count: 2,
            total_count: 10,
            theme: ThemeMode::Light,
        };

        // Just verify it compiles and runs without panicking
        let _element = StatusDisplay::enhanced_view(params, &tokens);
    }

    #[test]
    fn test_status_display_invalid_scan_progress() {
        let tokens = MaterialTokens::default();

        // Test with file processed progress
        let mut params = create_base_params();
        params.scanning = true;
        params.scan_progress = Some(ScanProgress::FileProcessed {
            current: 5,
            total: 10,
            file_name: "test.mp3".to_string(),
            progress_percentage: 0.5,
        });
        params.cached_scan_progress_text = Some("Scanning...");

        // Should handle file processed progress
        let _element = StatusDisplay::enhanced_view(params, &tokens);

        // Test with batch committed progress
        let mut params = create_base_params();
        params.scanning = true;
        params.scan_progress = Some(ScanProgress::BatchCommitted {
            count: 5,
            total_processed: 10,
        });
        params.cached_scan_progress_text = Some("Processing batch...");

        // Should handle batch committed progress
        let _element = StatusDisplay::enhanced_view(params, &tokens);

        // Test with scan started
        let mut params = create_base_params();
        params.scanning = true;
        params.scan_progress = Some(ScanProgress::Started { total_files: 10 });
        params.cached_scan_progress_text = Some("Starting scan...");

        // Should handle scan started
        let _element = StatusDisplay::enhanced_view(params, &tokens);
    }

    #[test]
    fn test_status_display_invalid_processing_progress() {
        let tokens = MaterialTokens::default();

        // Test with valid processing progress
        let mut params = create_base_params();
        params.processing_audio = true;
        params.processing_progress = Some(0.25);
        params.cached_processing_progress_text = Some("Processing 25%...");

        // Should handle valid progress
        let _element = StatusDisplay::enhanced_view(params, &tokens);

        // Test with completed processing
        let mut params = create_base_params();
        params.processing_audio = false;
        params.processing_progress = Some(1.0);
        params.cached_processing_progress_text = Some("Processing complete");

        // Should handle completed processing
        let _element = StatusDisplay::enhanced_view(params, &tokens);

        // Test with no processing
        let mut params = create_base_params();
        params.processing_audio = false;
        params.processing_progress = None;
        params.cached_processing_progress_text = None;

        // Should handle no processing state
        let _element = StatusDisplay::enhanced_view(params, &tokens);

        // Test with processing true but no progress
        let mut params = create_base_params();
        params.processing_audio = true;
        params.processing_progress = None;
        params.cached_processing_progress_text = Some("Processing...");

        // Should handle missing progress value
        let _element = StatusDisplay::enhanced_view(params, &tokens);

        // Test with negative processing progress
        let mut params = create_base_params();
        params.processing_audio = true;
        params.processing_progress = Some(-0.5);
        params.cached_processing_progress_text = Some("Processing...");

        // Should handle negative progress by clamping to 0.0
        let _element = StatusDisplay::enhanced_view(params, &tokens);

        // Test with processing progress > 1.0
        let mut params = create_base_params();
        params.processing_audio = true;
        params.processing_progress = Some(2.0);
        params.cached_processing_progress_text = Some("Processing...");

        // Should handle progress > 1.0 by clamping to 1.0
        let _element = StatusDisplay::enhanced_view(params, &tokens);
    }

    /// Helper function to create a base set of parameters for status display tests
    fn create_base_params<'a>() -> EnhancedStatusDisplayParams<'a> {
        EnhancedStatusDisplayParams {
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
        }
    }

    #[test]
    fn test_status_display_edge_cases() {
        let tokens = MaterialTokens::default();
        let test_audio_path = PathBuf::from("/test/audio with spaces/audio.mp3");
        let long_path = PathBuf::from(
            "/a/very/long/path/that/should/be/truncated/properly/this/is/a/very/long/path/that/should/be/truncated/properly/audio.mp3",
        );

        // Test minimal params (all defaults)
        let params_minimal = create_base_params();
        let _element = StatusDisplay::enhanced_view(params_minimal, &tokens);

        // Test scan starting with zero total (division by zero protection)
        let mut scan_starting = create_base_params();
        scan_starting.scanning = true;
        scan_starting.scan_progress = Some(ScanProgress::FileProcessed {
            current: 0,
            total: 0,
            file_name: "test.mp3".to_string(),
            progress_percentage: 0.0,
        });
        scan_starting.cached_scan_progress_text = Some("Starting scan...");
        let _element = StatusDisplay::enhanced_view(scan_starting, &tokens);

        // Test processing starting state
        let mut processing_starting = create_base_params();
        processing_starting.processing_audio = true;
        processing_starting.processing_progress = Some(0.0);
        processing_starting.cached_processing_progress_text = Some("Starting processing...");
        let _element = StatusDisplay::enhanced_view(processing_starting, &tokens);

        // Test player playing state
        let mut player_playing = create_base_params();
        player_playing.player_state = PlayerState::Playing;
        player_playing.current_playing_file = Some(&test_audio_path);
        let _element = StatusDisplay::enhanced_view(player_playing, &tokens);

        // Test player stopped state
        let mut player_stopped = create_base_params();
        player_stopped.player_state = PlayerState::Stopped;
        let _element = StatusDisplay::enhanced_view(player_stopped, &tokens);

        // Test long file path
        let mut long_path_test = create_base_params();
        long_path_test.player_state = PlayerState::Playing;
        long_path_test.current_playing_file = Some(&long_path);
        let _element = StatusDisplay::enhanced_view(long_path_test, &tokens);

        // Test selection state
        let mut selection_test = create_base_params();
        selection_test.selected_count = 5;
        selection_test.total_count = 100;
        let _element = StatusDisplay::enhanced_view(selection_test, &tokens);

        // Test all fields set
        let mut all_fields = create_base_params();

        // Scan state
        all_fields.scanning = true;
        all_fields.scan_progress = Some(ScanProgress::FileProcessed {
            current: 7,
            total: 10,
            file_name: "test with spaces and special_chars_!@#.mp3".to_string(),
            progress_percentage: 0.7,
        });
        all_fields.cached_scan_progress_text = Some("Scanning files...");

        // Processing state
        all_fields.processing_audio = true;
        all_fields.processing_progress = Some(0.25);
        all_fields.cached_processing_progress_text = Some("Processing audio files...");
        all_fields.processing_status = Some("Working on batch 3/5");

        // Player state
        all_fields.player_state = PlayerState::Playing;
        all_fields.current_playing_file = Some(&test_audio_path);

        // UI state
        all_fields.selected_count = 1;
        all_fields.total_count = 10;
        all_fields.theme = ThemeMode::Dark;

        let _element = StatusDisplay::enhanced_view(all_fields, &tokens);

        // Test scan complete state
        let mut scan_complete = create_base_params();
        scan_complete.scan_progress = Some(ScanProgress::Complete {
            processed: 100,
            errors: 0,
            duration: std::time::Duration::from_secs(10),
        });
        scan_complete.cached_scan_progress_text = Some("Scan complete");
        let _element = StatusDisplay::enhanced_view(scan_complete, &tokens);

        // Test scan with errors
        let mut scan_errors = create_base_params();
        scan_errors.scanning = true;
        scan_errors.scan_progress = Some(ScanProgress::Complete {
            processed: 95,
            errors: 5,
            duration: std::time::Duration::from_secs(15),
        });
        scan_errors.cached_scan_progress_text = Some("Scan completed with errors");
        let _element = StatusDisplay::enhanced_view(scan_errors, &tokens);

        // Test batch processing state
        let mut batch_processing = create_base_params();
        batch_processing.scanning = true;
        batch_processing.scan_progress = Some(ScanProgress::BatchCommitted {
            count: 10,
            total_processed: 50,
        });
        batch_processing.cached_scan_progress_text = Some("Processing batch...");
        let _element = StatusDisplay::enhanced_view(batch_processing, &tokens);

        // Test scan started state
        let mut scan_started = create_base_params();
        scan_started.scanning = true;
        scan_started.scan_progress = Some(ScanProgress::Started { total_files: 100 });
        scan_started.cached_scan_progress_text = Some("Starting scan...");
        let _element = StatusDisplay::enhanced_view(scan_started, &tokens);

        // Test processing complete state
        let mut processing_complete = create_base_params();
        processing_complete.processing_audio = false;
        processing_complete.processing_progress = Some(1.0);
        processing_complete.cached_processing_progress_text = Some("Processing complete");
        let _element = StatusDisplay::enhanced_view(processing_complete, &tokens);

        // Test error state with processing
        let mut error_processing = create_base_params();
        error_processing.processing_audio = true;
        error_processing.processing_progress = Some(0.5);
        error_processing.processing_status = Some("Error occurred");
        error_processing.cached_processing_progress_text = Some("Error processing files");
        let _element = StatusDisplay::enhanced_view(error_processing, &tokens);

        // Test minimal state with just processing complete
        let mut processing_complete_min = create_base_params();
        processing_complete_min.processing_audio = false;
        processing_complete_min.processing_progress = Some(1.0);
        processing_complete_min.cached_processing_progress_text = Some("Processing complete");
        let _element = StatusDisplay::enhanced_view(processing_complete_min, &tokens);

        // Test state with all fields at minimum values
        let mut min_values = create_base_params();
        min_values.scanning = false;
        min_values.scan_progress = None;
        min_values.cached_scan_progress_text = None;
        min_values.processing_audio = false;
        min_values.processing_progress = None;
        min_values.cached_processing_progress_text = None;
        min_values.processing_status = None;
        min_values.player_state = PlayerState::Stopped;
        min_values.current_playing_file = None;
        min_values.selected_count = 0;
        min_values.total_count = 0;
        min_values.theme = ThemeMode::Light;
        let _element = StatusDisplay::enhanced_view(min_values, &tokens);

        // Test state with all fields at maximum values
        let mut max_values = create_base_params();
        max_values.scanning = true;
        max_values.scan_progress = Some(ScanProgress::FileProcessed {
            current: usize::MAX,
            total: usize::MAX,
            file_name: "a".repeat(1000), // Very long filename
            progress_percentage: 1.0,
        });
        max_values.cached_scan_progress_text = Some(
            "Scanning with very long text that should be properly truncated or wrapped in the UI",
        );
        max_values.processing_audio = true;
        max_values.processing_progress = Some(1.0);
        max_values.cached_processing_progress_text = Some(
            "Processing with very long text that should be properly truncated or wrapped in the UI",
        );
        max_values.processing_status = Some(
            "Status with very long text that should be properly truncated or wrapped in the UI",
        );
        max_values.player_state = PlayerState::Playing;
        max_values.current_playing_file = Some(&long_path);
        max_values.selected_count = std::usize::MAX;
        max_values.total_count = std::usize::MAX;
        max_values.theme = ThemeMode::Dark;
        let _element = StatusDisplay::enhanced_view(max_values, &tokens);

        // Test state with paused player and current file
        let mut paused_state = create_base_params();
        paused_state.processing_status = Some("Finished");
        paused_state.player_state = PlayerState::Paused;
        paused_state.current_playing_file = Some(&test_audio_path);
        let _element = StatusDisplay::enhanced_view(paused_state, &tokens);

        // Test state with playing player and current file
        let mut playing_state = create_base_params();
        playing_state.player_state = PlayerState::Playing;
        playing_state.current_playing_file = Some(&test_audio_path);
        let _element = StatusDisplay::enhanced_view(playing_state, &tokens);

        // Test state with stopped player and current file
        let mut stopped_state = create_base_params();
        stopped_state.player_state = PlayerState::Stopped;
        stopped_state.current_playing_file = Some(&test_audio_path);
        let _element = StatusDisplay::enhanced_view(stopped_state, &tokens);

        // Test state with paused player and current file
        let mut paused_state = create_base_params();
        paused_state.player_state = PlayerState::Paused;
        paused_state.current_playing_file = Some(&test_audio_path);
        let _element = StatusDisplay::enhanced_view(paused_state, &tokens);

        // Test state with selection counts
        let mut selection_state = create_base_params();
        selection_state.selected_count = 5;
        selection_state.total_count = 100;
        let _element = StatusDisplay::enhanced_view(selection_state, &tokens);

        // Test state with theme changes
        let mut dark_theme_state = create_base_params();
        dark_theme_state.theme = ThemeMode::Dark;
        let _element = StatusDisplay::enhanced_view(dark_theme_state, &tokens);

        let mut light_theme_state = create_base_params();
        light_theme_state.theme = ThemeMode::Light;
        let _element = StatusDisplay::enhanced_view(light_theme_state, &tokens);

        // Test with MaterialDark theme
        let params_complete = EnhancedStatusDisplayParams {
            scanning: false,
            scan_progress: None,
            cached_scan_progress_text: None,
            processing_audio: false,
            processing_progress: None,
            cached_processing_progress_text: None,
            processing_status: None,
            player_state: PlayerState::Stopped,
            current_playing_file: None,
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
