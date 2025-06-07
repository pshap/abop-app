//! UI state message handlers
//!
//! Handles messages that update UI state without requiring async operations

use iced::Task;

use crate::messages::Message;
use crate::state::UiState;

/// Handles UI state changes that don't require async operations
#[must_use]
pub fn handle_ui_message(state: &mut UiState, message: Message) -> Option<Task<Message>> {
    match message {
        Message::ShowSettings => {
            state.settings_open = true;
            Some(Task::none())
        }
        Message::CloseSettings => {
            state.settings_open = false;
            Some(Task::none())
        }
        Message::ShowRecentDirectories => {
            state.recent_directories_open = true;
            Some(Task::none())
        }
        Message::SetTheme(theme_mode) => {
            state.theme_mode = theme_mode;
            Some(Task::none())
        }
        Message::SelectRecentDirectory(path) => {
            log::info!("Selected recent directory: {}", path.display());
            state.library_path = path;
            Some(Task::none())
        }
        Message::PlayPause => {
            log::info!("Play/Pause button pressed");
            // Check current audio player state and respond accordingly
            let current_state = crate::audio::player::get_player_state();
            if current_state == abop_core::PlayerState::Playing {
                // Stop audio playback
                crate::audio::player::stop_audio();
                state.player_state = abop_core::PlayerState::Paused;
            } else {
                // Start playback if we have a file selected
                if let Some(current_file) = &state.current_playing_file {
                    // Find the audiobook and start playing
                    if let Some(audiobook) =
                        state.audiobooks.iter().find(|ab| ab.path == *current_file)
                    {
                        // Return a command to play the audio
                        return Some(Task::perform(
                            crate::audio::player::play_selected_audio(
                                vec![audiobook.id.clone()],
                                vec![audiobook.clone()],
                            ),
                            Message::PlaybackStarted,
                        ));
                    }
                } else if !state.selected_audiobooks.is_empty() {
                    // Play first selected audiobook
                    if let Some(first_selected_id) = state.selected_audiobooks.iter().next()
                        && let Some(audiobook) = state
                            .audiobooks
                            .iter()
                            .find(|ab| &ab.id == first_selected_id)
                    {
                        state.current_playing_file = Some(audiobook.path.clone());
                        return Some(Task::perform(
                            crate::audio::player::play_selected_audio(
                                vec![audiobook.id.clone()],
                                vec![audiobook.clone()],
                            ),
                            Message::PlaybackStarted,
                        ));
                    }
                }
                state.player_state = abop_core::PlayerState::Playing;
            }
            Some(Task::none())
        }
        Message::Stop => {
            log::info!("Stop button pressed");
            crate::audio::player::stop_audio();
            state.player_state = abop_core::PlayerState::Stopped;
            Some(Task::none())
        }
        Message::Previous => {
            log::info!("Previous button pressed");
            // Find currently playing or first selected audiobook and move to previous
            if let Some(current_file) = &state.current_playing_file {
                if let Some(current_index) = state
                    .audiobooks
                    .iter()
                    .position(|ab| ab.path == *current_file)
                {
                    if current_index > 0 {
                        let previous_audiobook = &state.audiobooks[current_index - 1];
                        log::info!(
                            "Moving to previous track: {}",
                            previous_audiobook.title.as_deref().unwrap_or("Unknown")
                        );
                        // Start playing the previous audiobook
                        state.current_playing_file = Some(previous_audiobook.path.clone());
                        state.player_state = abop_core::PlayerState::Playing;
                        return Some(Task::perform(
                            crate::audio::player::play_selected_audio(
                                vec![previous_audiobook.id.clone()],
                                vec![previous_audiobook.clone()],
                            ),
                            Message::PlaybackStarted,
                        ));
                    }
                    log::info!("Already at first track");
                } else {
                    log::warn!("Current playing file not found in audiobooks list");
                }
            } else if !state.selected_audiobooks.is_empty() {
                // If nothing is playing but audiobooks are selected, play the first selected one
                if let Some(first_selected_id) = state.selected_audiobooks.iter().next()
                    && let Some(audiobook) = state
                        .audiobooks
                        .iter()
                        .find(|ab| &ab.id == first_selected_id)
                {
                    log::info!(
                        "Starting playback from first selected: {}",
                        audiobook.title.as_deref().unwrap_or("Unknown")
                    );
                    state.current_playing_file = Some(audiobook.path.clone());
                    state.player_state = abop_core::PlayerState::Playing;
                    return Some(Task::perform(
                        crate::audio::player::play_selected_audio(
                            vec![audiobook.id.clone()],
                            vec![audiobook.clone()],
                        ),
                        Message::PlaybackStarted,
                    ));
                }
            }
            Some(Task::none())
        }
        Message::Next => {
            log::info!("Next button pressed");
            // Find currently playing or first selected audiobook and move to next
            if let Some(current_file) = &state.current_playing_file {
                if let Some(current_index) = state
                    .audiobooks
                    .iter()
                    .position(|ab| ab.path == *current_file)
                {
                    if current_index < state.audiobooks.len() - 1 {
                        let next_audiobook = &state.audiobooks[current_index + 1];
                        log::info!(
                            "Moving to next track: {}",
                            next_audiobook.title.as_deref().unwrap_or("Unknown")
                        );
                        // Start playing the next audiobook
                        state.current_playing_file = Some(next_audiobook.path.clone());
                        state.player_state = abop_core::PlayerState::Playing;
                        return Some(Task::perform(
                            crate::audio::player::play_selected_audio(
                                vec![next_audiobook.id.clone()],
                                vec![next_audiobook.clone()],
                            ),
                            Message::PlaybackStarted,
                        ));
                    }
                    log::info!("Already at last track");
                } else {
                    log::warn!("Current playing file not found in audiobooks list");
                }
            } else if !state.selected_audiobooks.is_empty() {
                // If nothing is playing but audiobooks are selected, play the first selected one
                if let Some(first_selected_id) = state.selected_audiobooks.iter().next()
                    && let Some(audiobook) = state
                        .audiobooks
                        .iter()
                        .find(|ab| &ab.id == first_selected_id)
                {
                    log::info!(
                        "Starting playback from first selected: {}",
                        audiobook.title.as_deref().unwrap_or("Unknown")
                    );
                    state.current_playing_file = Some(audiobook.path.clone());
                    state.player_state = abop_core::PlayerState::Playing;
                    return Some(Task::perform(
                        crate::audio::player::play_selected_audio(
                            vec![audiobook.id.clone()],
                            vec![audiobook.clone()],
                        ),
                        Message::PlaybackStarted,
                    ));
                }
            }
            Some(Task::none())
        }
        Message::ResetRedrawFlag => {
            state.needs_redraw = false;
            Some(Task::none())
        }
        Message::SortBy(column_id) => {
            log::info!("Sorting by column: {column_id}");

            // Update the sort state
            if state.table_state.sort_column == column_id {
                // If clicking the same column, toggle sort direction
                state.table_state.sort_ascending = !state.table_state.sort_ascending;
            } else {
                // If clicking a different column, sort ascending by default
                state.table_state.sort_column = column_id;
                state.table_state.sort_ascending = true;
            }

            // Apply the sort to the audiobooks
            crate::utils::sort_audiobooks(state);

            log::info!(
                "Sorted by {} ({})",
                state.table_state.sort_column,
                if state.table_state.sort_ascending {
                    "ascending"
                } else {
                    "descending"
                }
            );

            Some(Task::none())
        }
        _ => None, // Not a UI message
    }
}
