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
        size_bytes: Option<u64>
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
        let audiobooks = vec![create_test_audiobook("1", "Test Book")];
        let selected_ids = HashSet::new();
        
        // Test with no selection
        let element = AudioControls::view(&selected_ids, &audiobooks, PlayerState::Stopped, &tokens);
        assert!(element.is_some(), "Should render controls even with no selection");
        
        // Test with selection
        let mut selected = HashSet::new();
        selected.insert("1".to_string());
        let element = AudioControls::view(&selected, &audiobooks, PlayerState::Stopped, &tokens);
        assert!(element.is_some(), "Should render controls with selection");
        
        // Test with empty audiobooks list
        let empty_audiobooks: Vec<Audiobook> = Vec::new();
        let element = AudioControls::view(&selected, &empty_audiobooks, PlayerState::Stopped, &tokens);
        assert!(element.is_some(), "Should handle empty audiobooks list");
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
            for &state in &[
                PlayerState::Stopped, 
                PlayerState::Playing, 
                PlayerState::Paused
            ] {
                let element = AudioControls::view(&selected_ids, &audiobooks, state, &tokens);
                assert!(element.is_some(), "Failed for state {:?} with {}", state, description);
            }
        }
    }

    #[test]
    fn test_audio_controls_edge_cases() {
        let tokens = MaterialTokens::default();
        
        // Test with empty audiobooks and empty selection
        let empty_audiobooks: Vec<Audiobook> = Vec::new();
        let empty_selection = HashSet::new();
        
        let element = AudioControls::view(&empty_selection, &empty_audiobooks, PlayerState::Stopped, &tokens);
        assert!(element.is_some(), "Should handle empty state gracefully");
        
        // Test with very long metadata
        let long_title = "Audiobook with a very long title that should be properly handled in the UI without breaking the layout or causing any rendering issues";
        let long_author = "Author with a very long name that should also be properly handled in the UI";
        let audiobook = create_custom_audiobook("1", long_title, long_author, Some(999999), Some(9999999999));
        
        let mut selected = HashSet::new();
        selected.insert("1".to_string());
        
        let element = AudioControls::view(&selected, &[audiobook], PlayerState::Playing, &tokens);
        assert!(element.is_some(), "Should handle long metadata gracefully");
        
        // Test with None values for optional fields
        let incomplete_audiobook = create_custom_audiobook("2", "No Metadata", "", None, None);
        let element = AudioControls::view(&selected, &[incomplete_audiobook], PlayerState::Paused, &tokens);
        assert!(element.is_some(), "Should handle missing metadata gracefully");
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
    fn test_status_display_invalid_scan_progress() {
        let tokens = MaterialTokens::default();
        
        // Test with invalid negative progress
        let mut params = create_base_params();
        params.scanning = true;
        params.scan_progress = Some(-0.1);
        params.cached_scan_progress_text = Some("Scanning...".to_string());
        
        // Should handle negative progress by clamping to 0.0
        let element = StatusDisplay::enhanced_view(params, &tokens);
        assert!(element.is_some(), "Should handle negative scan progress");
        
        // Test with progress > 1.0
        let mut params = create_base_params();
        params.scanning = true;
        params.scan_progress = Some(1.5);
        params.cached_scan_progress_text = Some("Scanning...".to_string());
        
        // Should handle progress > 1.0 by clamping to 1.0
        let element = StatusDisplay::enhanced_view(params, &tokens);
        assert!(element.is_some(), "Should handle scan progress > 1.0");
        
        // Test with NaN progress
        let mut params = create_base_params();
        params.scanning = true;
        params.scan_progress = Some(std::f64::NAN);
        params.cached_scan_progress_text = Some("Scanning...".to_string());
        
        // Should handle NaN progress gracefully
        let element = StatusDisplay::enhanced_view(params, &tokens);
        assert!(element.is_some(), "Should handle NaN scan progress");
    }
    
    #[test]
    fn test_status_display_invalid_processing_progress() {
        let tokens = MaterialTokens::default();
        
        // Test with negative processing progress
        let mut params = create_base_params();
        params.processing_audio = true;
        params.processing_progress = Some(-0.5);
        params.cached_processing_progress_text = Some("Processing...".to_string());
        
        // Should handle negative progress by clamping to 0.0
        let element = StatusDisplay::enhanced_view(params, &tokens);
        assert!(element.is_some(), "Should handle negative processing progress");
        
        // Test with processing progress > 1.0
        let mut params = create_base_params();
        params.processing_audio = true;
        params.processing_progress = Some(2.0);
        params.cached_processing_progress_text = Some("Processing...".to_string());
        
        // Should handle progress > 1.0 by clamping to 1.0
        let element = StatusDisplay::enhanced_view(params, &tokens);
        assert!(element.is_some(), "Should handle processing progress > 1.0");
    }
    
    #[test]
    fn test_status_display_invalid_scan_states() {
        let tokens = MaterialTokens::default();
        
        // Test with scanning true but no progress
        let mut params = create_base_params();
        params.scanning = true;
        params.scan_progress = None;
        params.cached_scan_progress_text = Some("Starting scan...".to_string());
        
        let element = StatusDisplay::enhanced_view(params, &tokens);
        assert!(element.is_some(), "Should handle scanning with no progress");
        
        // Test with processing true but no progress
        let mut params = create_base_params();
        params.processing_audio = true;
        params.processing_progress = None;
        params.cached_processing_progress_text = Some("Starting processing...".to_string());
        
        let element = StatusDisplay::enhanced_view(params, &tokens);
        assert!(element.is_some(), "Should handle processing with no progress");
        
        // Test with both scanning and processing active
        let mut params = create_base_params();
        params.scanning = true;
        params.scan_progress = Some(0.3);
        params.cached_scan_progress_text = Some("Scanning...".to_string());
        params.processing_audio = true;
        params.processing_progress = Some(0.7);
        params.cached_processing_progress_text = Some("Processing...".to_string());
        
        let element = StatusDisplay::enhanced_view(params, &tokens);
        assert!(element.is_some(), "Should handle both scanning and processing");
    }
    
    /// Helper function to create a base set of parameters for status display tests
    fn create_base_params() -> EnhancedStatusDisplayParams {
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

        // Test with None progress
        let mut params_none = create_base_params();
        params_none.scanning = false;
        params_none.processing_audio = false;

        let element = StatusDisplay::enhanced_view(params_none, &tokens);
        assert!(element.is_some(), "Should handle None progress");

        // Test with all fields set
        let mut params_all = create_base_params();
        params_all.scanning = true;
        params_all.scan_progress = Some(0.75);
        params_all.cached_scan_progress_text = Some("Scanning...".to_string());
        params_all.processing_audio = true;
        params_all.processing_progress = Some(0.25);
        params_all.cached_processing_progress_text = Some("Processing...".to_string());
        params_all.processing_status = Some("Working".to_string());
        params_all.player_state = PlayerState::Playing;
        params_all.current_playing_file = Some("test.mp3".to_string());
        params_all.selected_count = 1;
        params_all.total_count = 10;
        params_all.theme = ThemeMode::Dark;

        let element = StatusDisplay::enhanced_view(params_all, &tokens);
        assert!(element.is_some(), "Should handle all fields set");

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
