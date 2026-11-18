//! Data update message handlers
//!
//! Handles messages that require async operations to update application data

use crate::commands::library::scan_directory_async;
use crate::messages::Message;
use crate::state::AppState;
use abop_core::services::RenamingService;
use iced::Task;
use std::sync::Arc;

/// Handles GUI messages that require async operations
#[must_use]
pub fn handle_gui_message(state: &mut AppState, message: Message) -> Option<Task<Message>> {
    match message {
        Message::DirectorySelected(path) => {
            if let Some(p) = &path {
                log::info!("Library directory selected: {}", p.display());
            }

            path.map(|path| {
                Task::perform(
                    async move {
                        match scan_directory_async(path.clone()).await {
                            Ok(info) => Message::QuickScanComplete(Ok(info)),
                            Err(e) => Message::QuickScanComplete(Err(e)),
                        }
                    },
                    |message| message,
                )
            })
        }
        Message::QuickScanComplete(result) => {
            match result {
                Ok(info) => {
                    log::info!("Library path set to {}", info.path.display());

                    // Update the library path to the newly selected directory
                    let library_path = info.path.clone();
                    state.library.set_library_path(library_path.clone());

                    // Add to recent directories, keeping only the latest entry per path
                    state
                        .library
                        .recent_directories
                        .retain(|dir| dir.path != info.path);
                    state.library.recent_directories.push(info);

                    // Selecting a library folder should show its contents right away
                    Some(Task::done(Message::command(
                        crate::messages::Command::scan_library(library_path),
                    )))
                }
                Err(error) => {
                    state
                        .ui
                        .notify_error(format!("Couldn't open folder: {error}"));
                    Some(Task::none())
                }
            }
        }
        Message::ScanComplete(result) => {
            match result {
                Ok(scan_result) => {
                    // Update state with scan results
                    state.library.set_audiobooks(scan_result.audiobooks);
                    crate::utils::sort_audiobooks(state);
                    let preload = Task::batch([preload_cover_art(state), load_progress()]);
                    // scan_progress is cleared automatically by complete_scanning()
                    state.library.complete_scanning();
                    Some(preload)
                }
                Err(error) => {
                    state.library.error_scanning();
                    state.ui.notify_error(format!("Scan failed: {error}"));
                    Some(Task::none())
                }
            }
        }
        Message::ScanProgress(progress) => {
            state.library.update_scan_progress(progress);
            Some(Task::none())
        }
        Message::CommitRename(id, new_name) => handle_commit_rename(state, id, new_name),
        Message::LibraryLoaded(path, result) => Some(apply_loaded_library(state, path, result)),
        Message::ProgressLoaded(result) => {
            match result {
                Ok(progress) => {
                    state.library.progress = progress;
                    // Progress may affect the current sort order
                    crate::utils::sort_audiobooks(state);
                }
                Err(error) => log::warn!("Could not load listening progress: {error}"),
            }
            Some(Task::none())
        }
        Message::RenameFinished(result) => {
            apply_rename_result(state, result);
            Some(Task::none())
        }
        _ => None,
    }
}

fn handle_commit_rename(
    state: &mut AppState,
    id: String,
    new_name: String,
) -> Option<Task<Message>> {
    log::info!("Committing rename for {id} to {new_name}");
    state.ui.renaming_id = None;

    // Find the audiobook in state to get current path
    let audiobook = state
        .library
        .audiobooks
        .iter()
        .find(|a| a.id == id)?
        .clone();

    Some(Task::perform(
        async move {
            tokio::task::spawn_blocking(move || {
                let db = crate::database::app_database()?;
                let repo = Arc::new(db.audiobook_repository());
                let service = RenamingService::new(repo);

                service.rename_audiobook(&audiobook, &new_name)
            })
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())
        },
        Message::RenameFinished,
    ))
}

/// Update the library with a renamed audiobook, or report why the rename failed
fn apply_rename_result(state: &mut AppState, result: Result<abop_core::models::Audiobook, String>) {
    let updated = match result {
        Ok(updated) => updated,
        Err(error) => {
            state.ui.notify_error(format!("Rename failed: {error}"));
            return;
        }
    };
    log::info!("Renamed audiobook to {}", updated.path.display());

    let Some(existing) = state
        .library
        .audiobooks
        .iter_mut()
        .find(|b| b.id == updated.id)
    else {
        return;
    };
    let old_path = std::mem::replace(&mut existing.path, updated.path.clone());
    existing.updated_at = updated.updated_at;
    if let Some(book) = state
        .library
        .filtered_audiobooks
        .iter_mut()
        .find(|b| b.id == updated.id)
    {
        book.path = updated.path.clone();
    }
    if state.player.current_playing_file.as_ref() == Some(&old_path) {
        state.player.current_playing_file = Some(updated.path.clone());
    }

    let file_name = updated
        .path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    state.ui.notify_info(format!("Renamed to “{file_name}”"));
}

/// Handles core operation messages that require async operations
pub fn handle_core_operation(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::ScanComplete(Ok(result)) => {
            log::info!(
                "Scan complete: {} books found in {:?}",
                result.audiobooks.len(),
                result.scan_duration
            );
            // Update library state with successful scan results
            state.library.set_audiobooks(result.audiobooks);
            crate::utils::sort_audiobooks(state);
            state.library.complete_scanning();
            // Clear progress cache
            state.progress_cache.clear_scan_cache();
            preload_cover_art(state)
        }
        Message::ScanComplete(Err(e)) => {
            log::error!("Scan failed: {e}");
            // Handle failed scan
            state.library.error_scanning();
            // Clear progress cache
            state.progress_cache.clear_scan_cache();
            Task::none()
        }
        Message::StateSaveComplete(Ok(_)) => {
            log::info!("State save complete");
            state.tasks.complete_saving();
            Task::none()
        }
        Message::StateSaveComplete(Err(e)) => {
            log::error!("State save failed: {e}");
            state.tasks.complete_saving();
            Task::none()
        }
        _ => Task::none(),
    }
}

/// Load saved listening progress in the background
fn load_progress() -> Task<Message> {
    Task::perform(
        crate::audio::player::load_all_progress(),
        Message::ProgressLoaded,
    )
}

/// Show a library loaded from the database, or scan it if it was never scanned
fn apply_loaded_library(
    state: &mut AppState,
    path: std::path::PathBuf,
    result: Result<Option<Vec<abop_core::models::Audiobook>>, String>,
) -> Task<Message> {
    match result {
        Ok(Some(audiobooks)) => {
            log::info!(
                "Loaded {} audiobooks for {}",
                audiobooks.len(),
                path.display()
            );
            state.library.set_library_path(path);
            state.library.set_audiobooks(audiobooks);
            crate::utils::sort_audiobooks(state);
            state.library.complete_scanning();
            Task::batch([preload_cover_art(state), load_progress()])
        }
        Ok(None) => Task::done(Message::command(crate::messages::Command::scan_library(
            path,
        ))),
        Err(error) => {
            state
                .ui
                .notify_error(format!("Couldn't load library: {error}"));
            Task::none()
        }
    }
}

/// Decode cover thumbnails on a background thread so the table never blocks on it
///
/// The view shows placeholders until this finishes; the returned message triggers a redraw.
fn preload_cover_art(state: &AppState) -> Task<Message> {
    let covers: Vec<(String, Vec<u8>)> = state
        .library
        .audiobooks
        .iter()
        .filter_map(|book| Some((book.id.clone(), book.cover_art.clone()?)))
        .collect();
    if covers.is_empty() {
        return Task::none();
    }

    let cache = Arc::clone(&state.library.image_cache);
    Task::perform(
        async move {
            tokio::task::spawn_blocking(move || {
                cache.preload(
                    covers
                        .iter()
                        .map(|(id, data)| (id.as_str(), data.as_slice())),
                    crate::components::cover_art::TABLE_THUMBNAIL_SIZE,
                )
            })
            .await
            .unwrap_or_default()
        },
        |decoded| {
            log::debug!("Preloaded {decoded} cover thumbnails");
            Message::NoOp
        },
    )
}

#[cfg(test)]
mod rename_tests {
    use super::*;
    use abop_core::models::Audiobook;

    fn state_with_book() -> AppState {
        let mut state = AppState::default();
        let mut book = Audiobook::new("lib", "/books/Old.mp3");
        book.id = "b1".into();
        state.library.audiobooks.push(book);
        state
    }

    #[test]
    fn successful_rename_updates_paths_everywhere() {
        let mut state = state_with_book();
        state.player.current_playing_file = Some("/books/Old.mp3".into());
        let mut renamed = state.library.audiobooks[0].clone();
        renamed.path = "/books/New.mp3".into();

        apply_rename_result(&mut state, Ok(renamed));

        assert_eq!(
            state.library.audiobooks[0].path,
            std::path::Path::new("/books/New.mp3")
        );
        assert_eq!(
            state.player.current_playing_file.as_deref(),
            Some(std::path::Path::new("/books/New.mp3"))
        );
        assert_eq!(
            state.ui.notice.as_ref().map(|n| n.kind),
            Some(crate::state::NoticeKind::Info)
        );
    }

    #[test]
    fn failed_rename_reports_error_and_keeps_path() {
        let mut state = state_with_book();
        apply_rename_result(&mut state, Err("Destination already exists".into()));
        assert_eq!(
            state.library.audiobooks[0].path,
            std::path::Path::new("/books/Old.mp3")
        );
        let notice = state.ui.notice.expect("error should be shown");
        assert_eq!(notice.kind, crate::state::NoticeKind::Error);
        assert!(notice.message.contains("Destination already exists"));
    }
}
