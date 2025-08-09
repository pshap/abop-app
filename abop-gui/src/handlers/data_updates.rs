//! Data update message handlers
//!
//! Handles messages that require async operations to update application data

use crate::commands::library::scan_directory_async;
use crate::messages::Message;
use crate::state::AppState;
use iced::Task;


/// Handles GUI messages that require async operations
#[must_use]
pub fn handle_gui_message(state: &mut AppState, message: Message) -> Option<Task<Message>> {
    match message {
        Message::DirectorySelected(path) => {
            if let Some(p) = &path {
                log::warn!("📨 DIRECTORY SELECTED MESSAGE: {}", p.display());
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
                    log::warn!("💾 STATE UPDATE: library_path = {}", info.path.display());

                    // Update the library path to the newly selected directory
                    state.library.set_library_path(info.path.clone());

                    // Add to recent directories
                    state.library.recent_directories.push(info);

                    // Quick scan complete - just update UI state, don't auto-start full scan
                    // The user can manually click the scan button if they want a full scan
                    Some(Task::none())
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
                    state.library.scan_progress = None;
                    state.library.complete_scanning();
                    Some(Task::none())
                }
                Err(_e) => {
                    state.library.error_scanning();
                    Some(Task::none())
                }
            }
        }
        Message::ScanProgress(progress) => {
            state.library.scan_progress = Some(progress);
            // Update cached progress text using the new progress cache
            state.progress_cache.get_scan_progress_text(progress);
            Some(Task::none())
        }
        Message::ScanProgressEnhanced(progress) => {
            state.library.scanner_progress = Some(progress);
            Some(Task::none())
        }
        _ => None,
    }
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
            state.library.complete_scanning();
            // Clear progress cache
            state.progress_cache.clear_scan_cache();
            Task::none()
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
