//! Utility modules for the GUI application
//!
//! This module contains various utility functions and safe conversion
//! utilities used throughout the GUI components.

use crate::state::UiState;
use std::path::Path;

pub mod path_utils;
pub mod platform;
pub mod safe_conversions;

// Re-export commonly used utilities
pub use safe_conversions::ui_conversions;

/// Detects the file format from a file path extension
///
/// This helper function extracts the file extension from a path and returns it as an uppercase string.
/// If the extension is missing or cannot be determined, it returns "Unknown".
///
/// # Arguments
/// * `path` - The file path to analyze
///
/// # Returns
/// Uppercase file extension or "Unknown"
///
/// # Examples
/// ```
/// use abop_gui::utils::get_file_format_simple;
/// use std::path::Path;
///
/// let ext = get_file_format_simple(Path::new("foo.mp3"));
/// assert_eq!(ext, "MP3");
/// ```
#[must_use]
pub fn get_file_format_simple(path: &Path) -> String {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map_or_else(|| "Unknown".to_string(), str::to_uppercase)
}

/// Sorts the audiobooks in the application state based on the current table configuration
///
/// This function sorts the `audiobooks` vector in `UiState` according to the selected column
/// and sort order (ascending/descending). Supported columns: title, author, duration, format.
///
/// # Arguments
/// * `state` - Mutable reference to the GUI application state
///
/// # Examples
/// ```
/// use abop_gui::utils::sort_audiobooks;
/// use abop_gui::state::UiState;
/// use abop_gui::state::TableState;
/// use std::collections::HashMap;
///
/// let mut state = UiState::default();
/// sort_audiobooks(&mut state);
/// ```
pub fn sort_audiobooks(state: &mut UiState) {
    let column = &state.table_state.sort_column;
    let ascending = state.table_state.sort_ascending;
    
    state.audiobooks.sort_by(|a, b| {
        let ordering = match column.as_str() {
            "title" => a
                .title
                .as_deref()
                .unwrap_or(&a.id)
                .cmp(b.title.as_deref().unwrap_or(&b.id)),
            "author" => a
                .author
                .as_deref()
                .unwrap_or("Unknown")
                .cmp(b.author.as_deref().unwrap_or("Unknown")),
            "duration" => a.duration_seconds.cmp(&b.duration_seconds),
            "size" => a.size_bytes.unwrap_or(0).cmp(&b.size_bytes.unwrap_or(0)),
            "format" => get_file_format_simple(&a.path).cmp(&get_file_format_simple(&b.path)),
            "path" => a.path.to_string_lossy().cmp(&b.path.to_string_lossy()),
            "library_id" => a.library_id.cmp(&b.library_id),
            // For unknown columns, fall back to sorting by title to provide consistent behavior
            _ => {
                log::warn!("Attempted to sort by unknown column '{}', falling back to title", column);
                a.title
                    .as_deref()
                    .unwrap_or(&a.id)
                    .cmp(b.title.as_deref().unwrap_or(&b.id))
            }
        };

        if ascending {
            ordering
        } else {
            ordering.reverse()
        }
    });
}
