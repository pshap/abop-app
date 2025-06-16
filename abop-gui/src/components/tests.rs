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
    }    #[test]
    fn test_audio_controls_view() {
        let tokens = MaterialTokens::default();
        let audiobooks = vec![create_test_audiobook("1", "Test Book")];
        let selected_ids = HashSet::new();
        
        let element = AudioControls::view(&selected_ids, &audiobooks, PlayerState::Stopped, &tokens);
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
        
        let element = AudioControls::view(&selected_ids, &audiobooks, PlayerState::Stopped, &tokens);
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_audio_controls_with_different_player_states() {
        let tokens = MaterialTokens::default();
        let audiobooks = vec![create_test_audiobook("1", "Test Book")];
        let mut selected_ids = HashSet::new();
        selected_ids.insert("1".to_string());
        
        // Test different player states
        let states = [PlayerState::Stopped, PlayerState::Playing, PlayerState::Paused];
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
    use super::super::status::{StatusDisplay, EnhancedStatusDisplayParams};
    use crate::styling::material::MaterialTokens;
    use crate::theme::ThemeMode;
    use abop_core::PlayerState;

    #[test]
    fn test_status_display_enhanced() {
        let tokens = MaterialTokens::default();
        let params = EnhancedStatusDisplayParams {
            scanning: false,
            scan_progress: None, // Simplify - don't use scan_progress
            cached_scan_progress_text: Some("Scanning progress"),
            processing_audio: false,
            processing_progress: Some(0.5), // Use Option<f32>
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
    fn test_status_display_app_footer() {
        let element = StatusDisplay::app_footer(15, ThemeMode::Light);
        let _ = element; // Just verify it compiles and runs
    }
}

#[cfg(test)]
mod table_tests {
    use super::super::table_core::AudiobookTable;
    use crate::styling::material::MaterialTokens;
    use crate::state::TableState;
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
