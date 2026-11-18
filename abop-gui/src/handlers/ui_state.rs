//! UI state message handlers
//!
//! Handles messages that update UI state without requiring async operations

use std::path::PathBuf;

use iced::Task;

use crate::constants::{DEFAULT_SORT_COLUMN, VALID_SORT_COLUMNS};
use crate::messages::Message;
use crate::state::AppState;
use crate::theme::ThemeMode;

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
        Message::ResetRedrawFlag => handle_reset_redraw_flag(state),
        Message::SortBy(column_id) => handle_sort_by(state, column_id),
        Message::SearchQuery(query) => handle_search_query(state, query),
        Message::BeginRename(audiobook_id) => handle_begin_rename(state, audiobook_id),
        Message::RenameInputChanged(new_input) => handle_rename_input_changed(state, new_input),
        Message::CancelRename => handle_cancel_rename(state),
        Message::DismissNotice => {
            state.ui.notice = None;
            Some(Task::none())
        }
        _ => None, // Not a UI message
    }
}

fn handle_begin_rename(state: &mut AppState, audiobook_id: String) -> Option<Task<Message>> {
    log::info!("Beginning rename for audiobook: {audiobook_id}");

    // Pre-fill the input with the current name
    if let Some(audiobook) = state
        .library
        .audiobooks
        .iter()
        .find(|a| a.id == audiobook_id)
    {
        let current_name = audiobook
            .path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        state.ui.rename_input = current_name;
    }

    state.ui.renaming_id = Some(audiobook_id);
    Some(Task::none())
}

fn handle_rename_input_changed(state: &mut AppState, new_input: String) -> Option<Task<Message>> {
    state.ui.rename_input = new_input;
    Some(Task::none())
}

fn handle_cancel_rename(state: &mut AppState) -> Option<Task<Message>> {
    state.ui.renaming_id = None;
    state.ui.rename_input.clear();
    Some(Task::none())
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
    // Regenerates the design tokens too; assigning theme_mode alone left colors unchanged
    state.ui.set_theme_mode(theme_mode);
    Some(Task::none())
}

fn handle_select_recent_directory(state: &mut AppState, path: PathBuf) -> Option<Task<Message>> {
    log::info!("Selected recent directory: {}", path.display());
    state.library.library_path = path;
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
    let next = match state.ui.theme_mode {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
        // For other modes, default to Light
        _ => ThemeMode::Light,
    };
    state.ui.set_theme_mode(next);
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

fn handle_search_query(state: &mut AppState, query: String) -> Option<Task<Message>> {
    log::debug!("Handling search query: '{}'", query);
    state.library.update_search_query(query);
    Some(Task::none())
}
