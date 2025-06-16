//! Tests for view modules

#[cfg(test)]
mod about_tests {
    use super::super::about::about_view;
    use crate::state::UiState;
    use crate::theme::ThemeMode;

    #[test]
    fn test_about_view_creation() {
        let state = UiState::default();
        let element = about_view(&state);

        // The function should return an Element without panicking
        // We can't easily test the content without complex UI testing framework,
        // but we can ensure the view function doesn't crash
        let _ = element; // Just verify it compiles and runs without panicking
    }
    #[test]
    fn test_about_view_with_different_themes() {
        // Test with light theme
        {
            let mut state = UiState::default();
            state.theme_mode = ThemeMode::Light;
            let light_element = about_view(&state);
            let _ = light_element; // Just verify it compiles and runs
        }

        // Test with dark theme
        {
            let mut state = UiState::default();
            state.theme_mode = ThemeMode::Dark;
            let dark_element = about_view(&state);
            let _ = dark_element; // Just verify it compiles and runs
        }
    }
}

#[cfg(test)]
mod library_tests {
    use super::super::library::library_view;
    use crate::state::UiState;
    use abop_core::models::audiobook::Audiobook;
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
    fn test_library_view_empty_state() {
        let state = UiState::default();
        let element = library_view(&state);

        // Should create view without crashing
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_library_view_with_audiobooks() {
        let mut state = UiState::default();

        // Add some test audiobooks
        state.audiobooks = vec![
            create_test_audiobook("1", "Book One"),
            create_test_audiobook("2", "Book Two"),
        ];

        let element = library_view(&state);
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_library_view_with_selected_audiobooks() {
        let mut state = UiState::default();

        // Add audiobooks and select some
        state.audiobooks = vec![
            create_test_audiobook("1", "Book One"),
            create_test_audiobook("2", "Book Two"),
        ];

        state.selected_audiobooks.insert("1".to_string());

        let element = library_view(&state);
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_library_view_scanning_state() {
        let mut state = UiState::default();
        state.scanning = true;

        let element = library_view(&state);
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_library_view_processing_state() {
        let mut state = UiState::default();
        state.processing_audio = true;
        state.processing_progress = Some(0.5);

        let element = library_view(&state);
        let _ = element; // Just verify it compiles and runs
    }
}

#[cfg(test)]
mod settings_tests {
    use super::super::settings::settings_view;
    use crate::state::UiState;
    use crate::theme::ThemeMode;

    #[test]
    fn test_settings_view_creation() {
        let state = UiState::default();
        let element = settings_view(&state);

        let _ = element; // Just verify it compiles and runs
    }
    #[test]
    fn test_settings_view_with_different_themes() {
        // Test light theme
        {
            let mut state = UiState::default();
            state.theme_mode = ThemeMode::Light;
            let light_element = settings_view(&state);
            let _ = light_element; // Just verify it compiles and runs
        }

        // Test dark theme
        {
            let mut state = UiState::default();
            state.theme_mode = ThemeMode::Dark;
            let dark_element = settings_view(&state);
            let _ = dark_element; // Just verify it compiles and runs
        }
    }

    #[test]
    fn test_settings_view_with_auto_save_enabled() {
        let mut state = UiState::default();
        state.auto_save_library = true;

        let element = settings_view(&state);
        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_settings_view_with_scan_subdirectories_enabled() {
        let mut state = UiState::default();
        state.scan_subdirectories = true;

        let element = settings_view(&state);
        let _ = element; // Just verify it compiles and runs
    }
}

#[cfg(test)]
mod audio_processing_tests {
    use super::super::audio_processing::audio_processing_view;
    use crate::state::UiState;
    use abop_core::PlayerState;
    use std::path::PathBuf;

    #[test]
    fn test_audio_processing_view_creation() {
        let state = UiState::default();
        let element = audio_processing_view(&state);

        let _ = element; // Just verify it compiles and runs
    }

    #[test]
    fn test_audio_processing_view_with_player_states() {
        let mut state = UiState::default();

        // Test with different player states
        let player_states = vec![
            PlayerState::Stopped,
            PlayerState::Playing,
            PlayerState::Paused,
        ];

        for player_state in player_states {
            state.player_state = player_state;
            let element = audio_processing_view(&state);
            let _ = element; // Just verify it compiles and runs
        }
    }

    #[test]
    fn test_audio_processing_view_with_current_file() {
        let mut state = UiState::default();
        state.current_playing_file = Some(PathBuf::from("/test/path/audiobook.mp3"));

        let element = audio_processing_view(&state);
        let _ = element; // Just verify it compiles and runs
    }
}
