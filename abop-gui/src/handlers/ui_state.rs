//! UI state message handlers
//!
//! Handles messages that update UI state without requiring async operations

use std::path::PathBuf;

use iced::Task;
use iced::futures::FutureExt;

use crate::messages::Message;
use crate::state::UiState;
use crate::theme::ThemeMode;
use crate::utils::path_utils::PathCompare;

/// Handles UI state changes that don't require async operations
#[must_use]
pub fn handle_ui_message(state: &mut UiState, message: Message) -> Option<Task<Message>> {
    match message {
        Message::ShowSettings => handle_show_settings(state),
        Message::CloseSettings => handle_close_settings(state),
        Message::ShowRecentDirectories => handle_show_recent_directories(state),
        Message::SetTheme(theme_mode) => handle_set_theme(state, theme_mode),
        Message::ToggleTheme => handle_toggle_theme(state),
        Message::ToggleSelectAll => handle_toggle_select_all(state),
        Message::ToggleAutoSaveLibrary => handle_toggle_auto_save_library(state),
        Message::ToggleScanSubdirectories => handle_toggle_scan_subdirectories(state),
        Message::ToggleAudiobookSelection(audiobook_id) => {
            handle_toggle_audiobook_selection(state, audiobook_id)
        }
        Message::SelectRecentDirectory(path) => handle_select_recent_directory(state, path),
        Message::PlayPause => handle_play_pause(state),
        Message::Stop => handle_stop(state),
        Message::Previous => handle_previous(state),
        Message::Next => handle_next(state),
        Message::ResetRedrawFlag => handle_reset_redraw_flag(state),
        Message::SortBy(column_id) => handle_sort_by(state, column_id),
        _ => None, // Not a UI message
    }
}

fn handle_show_settings(state: &mut UiState) -> Option<Task<Message>> {
    state.settings_open = true;
    Some(Task::none())
}

fn handle_close_settings(state: &mut UiState) -> Option<Task<Message>> {
    state.settings_open = false;
    Some(Task::none())
}

fn handle_show_recent_directories(state: &mut UiState) -> Option<Task<Message>> {
    state.recent_directories_open = true;
    Some(Task::none())
}

fn handle_set_theme(state: &mut UiState, theme_mode: ThemeMode) -> Option<Task<Message>> {
    state.theme_mode = theme_mode;
    Some(Task::none())
}

fn handle_select_recent_directory(state: &mut UiState, path: PathBuf) -> Option<Task<Message>> {
    log::info!("Selected recent directory: {}", path.display());
    state.library_path = path;
    Some(Task::none())
}

fn handle_play_pause(state: &mut UiState) -> Option<Task<Message>> {
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
            if let Some(audiobook) = state
                .audiobooks
                .iter()
                .find(|ab| ab.path.eq_path(current_file).unwrap_or(false))
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

fn handle_stop(state: &mut UiState) -> Option<Task<Message>> {
    log::info!("Stop button pressed");
    crate::audio::player::stop_audio();
    state.player_state = abop_core::PlayerState::Stopped;
    Some(Task::none())
}

fn handle_previous(state: &mut UiState) -> Option<Task<Message>> {
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

fn handle_next(state: &mut UiState) -> Option<Task<Message>> {
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

fn handle_reset_redraw_flag(state: &mut UiState) -> Option<Task<Message>> {
    state.needs_redraw = false;
    Some(Task::none())
}

fn handle_sort_by(state: &mut UiState, column_id: String) -> Option<Task<Message>> {
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

fn handle_toggle_theme(state: &mut UiState) -> Option<Task<Message>> {
    state.theme_mode = match state.theme_mode {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
        // For other modes, default to Light
        _ => ThemeMode::Light,
    };
    log::info!("Theme toggled to: {:?}", state.theme_mode);
    Some(Task::none())
}

fn handle_toggle_select_all(state: &mut UiState) -> Option<Task<Message>> {
    if state.selected_audiobooks.len() == state.audiobooks.len() {
        // All are selected, so deselect all
        state.selected_audiobooks.clear();
        log::info!("Deselected all audiobooks");
    } else {
        // Not all are selected, so select all
        state.selected_audiobooks = state.audiobooks.iter().map(|ab| ab.id.clone()).collect();
        log::info!("Selected all {} audiobooks", state.audiobooks.len());
    }
    Some(Task::none())
}

fn handle_toggle_auto_save_library(state: &mut UiState) -> Option<Task<Message>> {
    state.auto_save_library = !state.auto_save_library;
    log::info!("Auto-save library toggled to: {}", state.auto_save_library);
    Some(Task::none())
}

fn handle_toggle_scan_subdirectories(state: &mut UiState) -> Option<Task<Message>> {
    state.scan_subdirectories = !state.scan_subdirectories;
    log::info!(
        "Scan subdirectories toggled to: {}",
        state.scan_subdirectories
    );
    Some(Task::none())
}

fn handle_toggle_audiobook_selection(
    state: &mut UiState,
    audiobook_id: String,
) -> Option<Task<Message>> {
    if state.selected_audiobooks.contains(&audiobook_id) {
        state.selected_audiobooks.remove(&audiobook_id);
        log::info!("Deselected audiobook: {audiobook_id}");
    } else {
        state.selected_audiobooks.insert(audiobook_id.clone());
        log::info!("Selected audiobook: {audiobook_id}");
    }
    Some(Task::none())
}
