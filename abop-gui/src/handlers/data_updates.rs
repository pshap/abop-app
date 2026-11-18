//! Data update message handlers
//!
//! Handles messages that require async operations to update application data

use crate::commands::library::scan_directory_async;
use crate::messages::Message;
use crate::state::AppState;
use abop_core::db::Database;
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
                Err(_e) => {
                    log::error!("Quick scan failed: {_e}");
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
                    let preload = preload_cover_art(state);
                    // scan_progress is cleared automatically by complete_scanning()
                    state.library.complete_scanning();
                    Some(preload)
                }
                Err(_e) => {
                    state.library.error_scanning();
                    Some(Task::none())
                }
            }
        }
        Message::ScanProgress(progress) => {
            state.library.update_scan_progress(progress);
            Some(Task::none())
        }
        Message::CommitRename(id, new_name) => handle_commit_rename(state, id, new_name),
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
                let db = Database::open_app_database()?;
                let repo = Arc::new(db.audiobook_repository());
                let service = RenamingService::new(repo);

                service.rename_audiobook(&audiobook, &new_name)
            })
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())
        },
        |result| match result {
            Ok(updated) => {
                log::info!("Rename successful: {}", updated.path.display());
                // We could send a message to update local state,
                // but for now let's just trigger a re-scan or partial update
                Message::NoOp // Or Message::RenameSuccess(updated)
            }
            Err(e) => {
                log::error!("Rename failed: {e}");
                Message::NoOp
            }
        },
    ))
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
