//! Data update message handlers
//!
//! Handles messages that require async operations to update application data

use crate::commands::library::scan_directory_async;
use crate::messages::Message;
use crate::state::UiState;
use abop_core::scanner::ScannerState;
use iced::Task;

/// Handles GUI messages that require async operations
#[must_use]
pub fn handle_gui_message(state: &mut UiState, message: Message) -> Option<Task<Message>> {
    match message {
        Message::DirectorySelected(path) => path.map(|path| {
            Task::perform(
                async move {
                    match scan_directory_async(path.clone()).await {
                        Ok(info) => Message::QuickScanComplete(Ok(info)),
                        Err(e) => Message::QuickScanComplete(Err(e)),
                    }
                },
                |message| message,
            )
        }),
        Message::QuickScanComplete(result) => {
            match result {
                Ok(info) => {
                    // Update the library path to the newly selected directory
                    state.library_path = info.path.clone();

                    // Add to recent directories
                    state.recent_directories.push(info.clone());

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
                    state.audiobooks = scan_result.audiobooks;
                    state.scan_progress = None;
                    state.scanning = false;
                    state.scanner_state = ScannerState::Complete;
                    Some(Task::none())
                }
                Err(_e) => {
                    state.scanner_state = ScannerState::Error;
                    state.scanning = false;
                    Some(Task::none())
                }
            }
        }
        Message::ScanProgress(progress) => {
            state.scan_progress = Some(progress);
            Some(Task::none())
        }
        Message::ScanProgressEnhanced(progress) => {
            state.scanner_progress = Some(progress);
            Some(Task::none())
        }
        _ => None,
    }
}

/// Handles core operation messages that require async operations
pub fn handle_core_operation(state: &mut UiState, message: Message) -> Task<Message> {
    match message {
        Message::ScanComplete(Ok(result)) => {
            log::info!(
                "Scan complete: {} books found in {:?}",
                result.audiobooks.len(),
                result.scan_duration
            );
            state.scanning = false;
            state.scan_progress = None;
            state.audiobooks = result.audiobooks;
            state.scanner_state = ScannerState::Complete;
            Task::none()
        }
        Message::ScanComplete(Err(e)) => {
            log::error!("Scan failed: {e}");
            state.scanning = false;
            state.scan_progress = None;
            state.scanner_state = ScannerState::Error;
            Task::none()
        }
        Message::StateSaveComplete(Ok(_)) => {
            log::info!("State save complete");
            state.saving = false;
            state.save_progress = None;
            Task::none()
        }
        Message::StateSaveComplete(Err(e)) => {
            log::error!("State save failed: {e}");
            state.saving = false;
            state.save_progress = None;
            Task::none()
        }
        _ => Task::none(),
    }
}
