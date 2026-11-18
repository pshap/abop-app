//! Utility modules for the GUI application
//!
//! This module contains various utility functions and safe conversion
//! utilities used throughout the GUI components.

use crate::state::AppState;
use abop_core::models::Audiobook;
use std::path::Path;

pub mod image_cache;
pub mod path_utils;
pub mod platform;

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
/// This function sorts the `audiobooks` vector in `AppState.library` according to the selected column
/// and sort order (ascending/descending). Supported columns: title, author, duration, format.
///
/// # Arguments
/// * `state` - Mutable reference to the GUI application state
///
/// # Examples
/// ```
/// use abop_gui::utils::sort_audiobooks;
/// use abop_gui::state::AppState;
/// use abop_gui::state::TableState;
/// use std::collections::HashMap;
///
/// let mut state = AppState::default();
/// sort_audiobooks(&mut state);
/// ```
pub fn sort_audiobooks(state: &mut AppState) {
    let column = state.library.table_state.sort_column.clone();
    let ascending = state.library.table_state.sort_ascending;
    let progress = state.library.progress.clone();
    let compare = |a: &Audiobook, b: &Audiobook| {
        let ordering = compare_audiobooks(a, b, &column, &progress);
        if ascending {
            ordering
        } else {
            ordering.reverse()
        }
    };

    state.library.audiobooks.sort_by(compare);
    // Keep the search results (shown while a search is active) in the same order
    state.library.filtered_audiobooks.sort_by(compare);
}

/// Case-insensitive comparison of text fields used for sorting
fn compare_text(a: &str, b: &str) -> std::cmp::Ordering {
    a.to_lowercase()
        .cmp(&b.to_lowercase())
        .then_with(|| a.cmp(b))
}

/// Compares two audiobooks by the given table column
fn compare_audiobooks(
    a: &Audiobook,
    b: &Audiobook,
    column: &str,
    progress: &std::collections::HashMap<String, crate::state::ListeningProgress>,
) -> std::cmp::Ordering {
    let listened = |book: &Audiobook| {
        progress
            .get(&book.id)
            .and_then(|p| p.fraction(book.duration_seconds))
            .unwrap_or(0.0)
    };
    let title = |book: &Audiobook| book.title.clone().unwrap_or_else(|| book.id.clone());
    match column {
        "title" => compare_text(&title(a), &title(b)),
        "author" => compare_text(
            a.author.as_deref().unwrap_or("Unknown"),
            b.author.as_deref().unwrap_or("Unknown"),
        ),
        "duration" => a.duration_seconds.cmp(&b.duration_seconds),
        "size" => a.size_bytes.unwrap_or(0).cmp(&b.size_bytes.unwrap_or(0)),
        "progress" => listened(a).total_cmp(&listened(b)),
        "format" => get_file_format_simple(&a.path).cmp(&get_file_format_simple(&b.path)),
        "path" => a.path.to_string_lossy().cmp(&b.path.to_string_lossy()),
        "library_id" => a.library_id.cmp(&b.library_id),
        // For unknown columns, fall back to sorting by title to provide consistent behavior
        _ => {
            log::warn!("Attempted to sort by unknown column '{column}', falling back to title");
            compare_text(&title(a), &title(b))
        }
    }
}

#[cfg(test)]
mod sort_tests {
    use super::*;

    fn book(title: &str) -> Audiobook {
        let mut book = Audiobook::new("lib", format!("/books/{title}.mp3"));
        book.title = Some(title.to_string());
        book
    }

    fn titles(books: &[Audiobook]) -> Vec<&str> {
        books.iter().filter_map(|b| b.title.as_deref()).collect()
    }

    #[test]
    fn sorts_titles_case_insensitively_in_both_directions() {
        let mut state = AppState::default();
        state.library.audiobooks = vec![book("zebra"), book("Apple"), book("mango")];
        state.library.filtered_audiobooks = vec![book("mango"), book("Apple")];

        sort_audiobooks(&mut state);
        assert_eq!(
            titles(&state.library.audiobooks),
            ["Apple", "mango", "zebra"]
        );
        assert_eq!(
            titles(&state.library.filtered_audiobooks),
            ["Apple", "mango"]
        );

        state.library.table_state.sort_ascending = false;
        sort_audiobooks(&mut state);
        assert_eq!(
            titles(&state.library.audiobooks),
            ["zebra", "mango", "Apple"]
        );
    }

    #[test]
    fn sorts_by_listening_progress() {
        use crate::state::ListeningProgress;
        let mut state = AppState::default();
        let mut books = vec![book("Unstarted"), book("Finished"), book("Halfway")];
        for b in &mut books {
            b.duration_seconds = Some(100);
        }
        let (finished, halfway) = (books[1].id.clone(), books[2].id.clone());
        state.library.audiobooks = books;
        state.library.progress.insert(
            finished,
            ListeningProgress {
                position_seconds: 0,
                completed: true,
            },
        );
        state.library.progress.insert(
            halfway,
            ListeningProgress {
                position_seconds: 50,
                completed: false,
            },
        );
        state.library.table_state.sort_column = "progress".into();

        sort_audiobooks(&mut state);
        assert_eq!(
            titles(&state.library.audiobooks),
            ["Unstarted", "Halfway", "Finished"]
        );
    }
}
