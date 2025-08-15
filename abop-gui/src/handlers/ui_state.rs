//! UI state message handlers
//!
//! Handles messages that update UI state without requiring async operations

use std::path::PathBuf;

use iced::Task;

use crate::constants::{DEFAULT_SORT_COLUMN, VALID_SORT_COLUMNS};
use crate::messages::Message;
use crate::state::AppState;
use crate::theme::ThemeMode;
use crate::utils::path_utils::PathCompare;

/// Handles UI state changes that don't require async operations
#[must_use]
pub fn handle_ui_message(state: &mut AppState, message: Message) -> Option<Task<Message>> {
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

fn handle_show_settings(state: &mut AppState) -> Option<Task<Message>> {
    state.ui.open_settings();
    Some(Task::none())
}

fn handle_close_settings(state: &mut AppState) -> Option<Task<Message>> {
    state.ui.close_settings();
    Some(Task::none())
}

fn handle_show_recent_directories(state: &mut AppState) -> Option<Task<Message>> {
    state.ui.recent_directories_open = true;
    Some(Task::none())
}

fn handle_set_theme(state: &mut AppState, theme_mode: ThemeMode) -> Option<Task<Message>> {
    state.ui.theme_mode = theme_mode;
    Some(Task::none())
}

fn handle_select_recent_directory(state: &mut AppState, path: PathBuf) -> Option<Task<Message>> {
    log::info!("Selected recent directory: {}", path.display());
    state.library.library_path = path;
    Some(Task::none())
}

fn handle_play_pause(state: &mut AppState) -> Option<Task<Message>> {
    log::info!("Play/Pause button pressed");
    // Check current audio player state and respond accordingly
    let current_state = crate::audio::player::get_player_state();
    if current_state == abop_core::PlayerState::Playing {
        // Stop audio playback
        crate::audio::player::stop_audio();
        state.player.player_state = abop_core::PlayerState::Paused;
    } else {
        // Start playback if we have a file selected
        if let Some(current_file) = &state.player.current_playing_file {
            // Find the audiobook and start playing
            if let Some(audiobook) =
                state
                    .library
                    .audiobooks
                    .iter()
                    .find(|ab| match ab.path.eq_path(current_file) {
                        Ok(result) => result,
                        Err(e) => {
                            log::error!("Error comparing paths: {e}");
                            false
                        }
                    })
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
        } else if !state.library.selected_audiobooks.is_empty() {
            // Play first selected audiobook
            if let Some(first_selected_id) = state.library.selected_audiobooks.iter().next()
                && let Some(audiobook) = state
                    .library
                    .audiobooks
                    .iter()
                    .find(|ab| &ab.id == first_selected_id)
            {
                state.player.current_playing_file = Some(audiobook.path.clone());
                return Some(Task::perform(
                    crate::audio::player::play_selected_audio(
                        vec![audiobook.id.clone()],
                        vec![audiobook.clone()],
                    ),
                    Message::PlaybackStarted,
                ));
            }
        }
        state.player.player_state = abop_core::PlayerState::Playing;
    }
    Some(Task::none())
}

fn handle_stop(state: &mut AppState) -> Option<Task<Message>> {
    log::info!("Stop button pressed");
    crate::audio::player::stop_audio();
    state.player.player_state = abop_core::PlayerState::Stopped;
    Some(Task::none())
}

fn handle_previous(state: &mut AppState) -> Option<Task<Message>> {
    log::info!("Previous button pressed");
    // Find currently playing or first selected audiobook and move to previous
    if let Some(current_file) = &state.player.current_playing_file {
        if let Some(current_index) = state
            .library
            .audiobooks
            .iter()
            .position(|ab| ab.path == *current_file)
        {
            if current_index > 0 {
                let previous_audiobook = &state.library.audiobooks[current_index - 1];
                log::info!(
                    "Moving to previous track: {}",
                    previous_audiobook.title.as_deref().unwrap_or("Unknown")
                );
                // Start playing the previous audiobook
                state.player.current_playing_file = Some(previous_audiobook.path.clone());
                state.player.player_state = abop_core::PlayerState::Playing;
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
    } else if !state.library.selected_audiobooks.is_empty() {
        // If nothing is playing but audiobooks are selected, play the first selected one
        if let Some(first_selected_id) = state.library.selected_audiobooks.iter().next()
            && let Some(audiobook) = state
                .library
                .audiobooks
                .iter()
                .find(|ab| &ab.id == first_selected_id)
        {
            log::info!(
                "Starting playback from first selected: {}",
                audiobook.title.as_deref().unwrap_or("Unknown")
            );
            state.player.current_playing_file = Some(audiobook.path.clone());
            state.player.player_state = abop_core::PlayerState::Playing;
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

fn handle_next(state: &mut AppState) -> Option<Task<Message>> {
    log::info!("Next button pressed");
    // Find currently playing or first selected audiobook and move to next
    if let Some(current_file) = &state.player.current_playing_file {
        if let Some(current_index) = state
            .library
            .audiobooks
            .iter()
            .position(|ab| ab.path == *current_file)
        {
            if current_index < state.library.audiobooks.len() - 1 {
                let next_audiobook = &state.library.audiobooks[current_index + 1];
                log::info!(
                    "Moving to next track: {}",
                    next_audiobook.title.as_deref().unwrap_or("Unknown")
                );
                // Start playing the next audiobook
                state.player.current_playing_file = Some(next_audiobook.path.clone());
                state.player.player_state = abop_core::PlayerState::Playing;
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
    } else if !state.library.selected_audiobooks.is_empty() {
        // If nothing is playing but audiobooks are selected, play the first selected one
        if let Some(first_selected_id) = state.library.selected_audiobooks.iter().next()
            && let Some(audiobook) = state
                .library
                .audiobooks
                .iter()
                .find(|ab| &ab.id == first_selected_id)
        {
            log::info!(
                "Starting playback from first selected: {}",
                audiobook.title.as_deref().unwrap_or("Unknown")
            );
            state.player.current_playing_file = Some(audiobook.path.clone());
            state.player.player_state = abop_core::PlayerState::Playing;
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

fn handle_reset_redraw_flag(state: &mut AppState) -> Option<Task<Message>> {
    state.clear_redraw_flags();
    Some(Task::none())
}

fn handle_sort_by(state: &mut AppState, column_id: String) -> Option<Task<Message>> {
    log::info!("Sorting by column: {column_id}");
    // Validate the column ID against known valid columns
    let validated_column = if VALID_SORT_COLUMNS.contains(&column_id.as_str()) {
        column_id
    } else {
        log::warn!("Invalid sort column '{column_id}', defaulting to '{DEFAULT_SORT_COLUMN}'");
        DEFAULT_SORT_COLUMN.to_string()
    };

    // Update the sort state
    if state.library.table_state.sort_column == validated_column {
        // If clicking the same column, toggle sort direction
        state.library.table_state.sort_ascending = !state.library.table_state.sort_ascending;
    } else {
        // If clicking a different column, sort ascending by default
        state.library.table_state.sort_column = validated_column;
        state.library.table_state.sort_ascending = true;
    }

    // Apply the sort to the audiobooks
    crate::utils::sort_audiobooks(state);
    log::info!(
        "Sorted by {} ({})",
        state.library.table_state.sort_column,
        if state.library.table_state.sort_ascending {
            "ascending"
        } else {
            "descending"
        }
    );

    Some(Task::none())
}

fn handle_toggle_theme(state: &mut AppState) -> Option<Task<Message>> {
    state.ui.theme_mode = match state.ui.theme_mode {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
        // For other modes, default to Light
        _ => ThemeMode::Light,
    };
    log::info!("Theme toggled to: {:?}", state.ui.theme_mode);
    Some(Task::none())
}

fn handle_toggle_select_all(state: &mut AppState) -> Option<Task<Message>> {
    if state.library.selected_audiobooks.len() == state.library.audiobooks.len() {
        // All are selected, so deselect all
        state.library.selected_audiobooks.clear();
        log::info!("Deselected all audiobooks");
    } else {
        // Not all are selected, so select all
        state.library.selected_audiobooks = state
            .library
            .audiobooks
            .iter()
            .map(|ab| ab.id.clone())
            .collect();
        log::info!("Selected all {} audiobooks", state.library.audiobooks.len());
    }
    Some(Task::none())
}

fn handle_toggle_auto_save_library(state: &mut AppState) -> Option<Task<Message>> {
    state.library.auto_save_library = !state.library.auto_save_library;
    log::info!(
        "Auto-save library toggled to: {}",
        state.library.auto_save_library
    );
    Some(Task::none())
}

fn handle_toggle_scan_subdirectories(state: &mut AppState) -> Option<Task<Message>> {
    state.library.scan_subdirectories = !state.library.scan_subdirectories;
    log::info!(
        "Scan subdirectories toggled to: {}",
        state.library.scan_subdirectories
    );
    Some(Task::none())
}

fn handle_toggle_audiobook_selection(
    state: &mut AppState,
    audiobook_id: String,
) -> Option<Task<Message>> {
    if state.library.selected_audiobooks.contains(&audiobook_id) {
        state.library.selected_audiobooks.remove(&audiobook_id);
        log::info!("Deselected audiobook: {audiobook_id}");
    } else {
        state
            .library
            .selected_audiobooks
            .insert(audiobook_id.clone());
        log::info!("Selected audiobook: {audiobook_id}");
    }
    Some(Task::none())
}
