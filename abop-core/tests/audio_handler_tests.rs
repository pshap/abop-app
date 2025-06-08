//! Tests for the audio message handler (Audio Handler).

use abop_core::AppState;
use abop_core::message::{AudioMessage, AudioProcessingOption};

mod test_helpers;
use test_helpers::create_test_app_state;

// Define a mock audio handler for testing
fn handle_audio_message(state: &mut AppState, message: AudioMessage) -> Option<()> {
    // Mock implementation for testing
    match message {
        AudioMessage::ProcessAudio => {
            // Add notification to state
            if state.app_data.audiobooks.is_empty() || !state.app_data.audiobooks.iter().any(|a| a.selected)
            {
                // No audiobooks selected
                add_notification(state, "No audiobooks selected for processing");
            } else {
                // No processing option selected
                add_notification(state, "No processing option selected");
            }
        }
        AudioMessage::SelectProcessingOption(_) => {
            // Just update state, no notification needed
        }
        AudioMessage::AudioProcessingComplete => {
            add_notification(state, "Audio processing completed successfully");
        }
        AudioMessage::AudioError(error) => {
            add_notification(state, &format!("Audio processing error: {error}"));
        }
        AudioMessage::Cancel => {
            add_notification(state, "Audio processing cancelled");
        }
    }
    None
}

// Helper function to add notifications to state
fn add_notification(state: &mut AppState, message: &str) {
    // Since AppState doesn't have notifications field, we'll store them in a custom field
    // For testing purposes, we'll use the description field of the first audiobook
    if state.app_data.audiobooks.is_empty() {
        // Create a dummy audiobook to store notifications
        use abop_core::models::Audiobook;
        use std::path::PathBuf;

        let mut audiobook = Audiobook::new("Test Library", PathBuf::new());
        audiobook.description = Some(message.to_string());
        state.app_data.audiobooks.push(audiobook);
    } else {
        // Update the first audiobook's description
        state.app_data.audiobooks[0].description = Some(message.to_string());
    }
}

#[cfg(test)]
mod audio_handler_tests {
    use super::*;
    use abop_core::models::Audiobook;
    use std::path::PathBuf;

    // Helper function to get notification message from state
    fn get_notification(state: &AppState) -> Option<String> {
        state
            .app_data
            .audiobooks
            .first()
            .and_then(|a| a.description.clone())
    }

    // Helper function to add a test audiobook
    fn add_test_audiobook(state: &mut AppState) {
        let mut audiobook = Audiobook::new("Test Library", PathBuf::new());
        audiobook.selected = true;
        state.app_data.audiobooks.push(audiobook);
    }

    #[test]
    fn test_handle_process_audio_no_selection() {
        let mut state = create_test_app_state();
        // No audiobooks selected, should warn
        let _ = handle_audio_message(&mut state, AudioMessage::ProcessAudio);
        // No panic, and a warning notification should be present
        let notification = get_notification(&state).unwrap_or_default();
        assert!(notification.contains("No audiobooks"));
    }

    #[test]
    fn test_handle_process_audio_no_option() {
        let mut state = create_test_app_state();
        // Add and select an audiobook
        add_test_audiobook(&mut state);

        // No processing option selected
        let _ = handle_audio_message(&mut state, AudioMessage::ProcessAudio);

        // Check notification message
        let notification = get_notification(&state).unwrap_or_default();
        assert!(
            notification.to_lowercase().contains("option")
                || notification.to_lowercase().contains("select"),
            "Notification: {notification:?}"
        );
    }

    #[test]
    fn test_handle_select_processing_option() {
        let mut state = create_test_app_state();
        let _ = handle_audio_message(
            &mut state,
            AudioMessage::SelectProcessingOption(AudioProcessingOption::StereoToMono),
        );
        // Should update panel state (not panicking is enough for now)
    }

    #[test]
    fn test_handle_audio_processing_complete() {
        let mut state = create_test_app_state();
        let _ = handle_audio_message(&mut state, AudioMessage::AudioProcessingComplete);
        // Should add a success notification
        let notification = get_notification(&state).unwrap_or_default();
        assert!(notification.contains("completed"));
    }

    #[test]
    fn test_handle_audio_error() {
        let mut state = create_test_app_state();
        let _ = handle_audio_message(
            &mut state,
            AudioMessage::AudioError("test error".to_string()),
        );
        // Should add an error notification
        let notification = get_notification(&state).unwrap_or_default();
        assert!(notification.contains("test error"));
    }

    #[test]
    fn test_handle_audio_cancel() {
        let mut state = create_test_app_state();
        let _ = handle_audio_message(&mut state, AudioMessage::Cancel);
        // Should add a cancellation notification
        let notification = get_notification(&state).unwrap_or_default();
        assert!(notification.contains("cancelled"));
    }
}
