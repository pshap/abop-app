//! Data operation message handlers
//!
//! Handles messages that update application data and state

use iced::Task;

use crate::audio::{get_audio_player, get_player_state};
use crate::messages::Message;
use crate::state::UiState;
use crate::utils::sort_audiobooks;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Handles data updates and synchronous state changes
#[must_use]
pub fn handle_data_message(state: &mut UiState, message: Message) -> Option<Task<Message>> {
    match message {
        Message::DirectorySelected(path) => {
            match path {
                Some(directory_path) => {
                    log::info!("Directory selected: {}", directory_path.display());

                    // Add to core state's user preferences for persistence
                    state
                        .core_state
                        .user_preferences
                        .add_recent_directory(directory_path.clone());

                    // Save the updated app state to persist recent directories asynchronously
                    let core_state = state.core_state.clone();
                    log::info!(
                        "Creating save task for DirectorySelected with {} audiobooks",
                        core_state.data.audiobooks.len()
                    );

                    // Set saving state
                    state.saving = true;
                    state.save_progress = Some(0.0);

                    let save_task = Task::perform(
                        async move {
                            log::info!(
                                "Executing save_blocking_with_progress in DirectorySelected task"
                            );

                            // Create progress channel
                            let (tx, rx) = mpsc::channel();

                            // Spawn save task in background thread
                            let save_handle = std::thread::spawn(move || {
                                // The tx will be dropped at the end of this scope

                                // tx is automatically dropped when this closure ends
                                core_state.save_blocking_with_progress(Some(tx))
                            });

                            // Process progress updates
                            let _last_progress = 0.0; // Tracked for future use
                            loop {
                                match rx.try_recv() {
                                    Ok(progress) => {
                                        // Return early with progress update
                                        return Ok(Message::StateSaveProgress(progress));
                                    }
                                    Err(mpsc::TryRecvError::Empty) => {
                                        if save_handle.is_finished() {
                                            break;
                                        }
                                        thread::sleep(Duration::from_millis(50));
                                    }
                                    Err(mpsc::TryRecvError::Disconnected) => break,
                                }
                            }

                            match save_handle.join() {
                                Ok(Ok(msg)) => {
                                    log::info!("DirectorySelected save successful: {msg}");
                                    Ok(Message::StateSaveComplete(Ok(
                                        "Settings saved successfully".to_string(),
                                    )))
                                }
                                Ok(Err(e)) => {
                                    log::error!("DirectorySelected save failed: {e}");
                                    Ok(Message::StateSaveComplete(Err(e.to_string())))
                                }
                                Err(_) => {
                                    log::error!("DirectorySelected save thread panicked");
                                    Ok(Message::StateSaveComplete(Err(
                                        "Save operation failed".to_string()
                                    )))
                                }
                            }
                        },
                        |result: Result<Message, String>| match result {
                            Ok(message) => message,
                            Err(e) => Message::StateSaveComplete(Err(e)),
                        },
                    );

                    // Add to recent directories if not already there
                    if !state
                        .recent_directories
                        .iter()
                        .any(|dir| dir.path == directory_path)
                    {
                        let directory_info = crate::state::DirectoryInfo {
                            path: directory_path.clone(),
                            last_scan: std::time::SystemTime::now(),
                            book_count: 0, // Will be updated when scanning
                            scan_duration: std::time::Duration::from_secs(0),
                        };
                        state.recent_directories.insert(0, directory_info);
                        // Keep only the last 10 recent directories
                        if state.recent_directories.len() > 10 {
                            state.recent_directories.truncate(10);
                        }
                    }
                    state.library_path = directory_path;

                    // Return the save task to actually execute it
                    return Some(save_task);
                }
                None => {
                    log::info!("Directory selection cancelled");
                }
            }
            Some(Task::none())
        }
        Message::ScanComplete(result) => {
            log::info!(
                "[SCAN] ScanComplete received, result has {} items",
                result.as_ref().map(|r| r.audiobooks.len()).unwrap_or(0)
            );
            state.scanning = false;
            state.scan_progress = None;
            match result {
                Ok(scan_result) => {
                    log::info!(
                        "[SCAN] Scan successful, found {} audiobooks in {:?}",
                        scan_result.audiobooks.len(),
                        scan_result.scan_duration
                    );

                    // Add the successfully scanned directory to recent directories
                    state
                        .core_state
                        .user_preferences
                        .add_recent_directory(state.library_path.clone());

                    // Clear existing UI audiobooks data
                    state.audiobooks.clear();
                    state.audiobooks = scan_result.audiobooks.clone();

                    // *** PERSISTENCE: Update core state with scan results ***
                    // Clear existing audiobooks in core state and add new ones
                    state.core_state.data.audiobooks.clear();
                    for audiobook in &scan_result.audiobooks {
                        state.core_state.add_audiobook(audiobook.clone());
                    }

                    // Update directory scan metadata with actual results
                    let scan_end_time = std::time::SystemTime::now();
                    if let Some(dir_info) = state
                        .recent_directories
                        .iter_mut()
                        .find(|dir| dir.path == state.library_path)
                    {
                        dir_info.last_scan = scan_end_time;
                        dir_info.book_count = scan_result.audiobooks.len();
                        dir_info.scan_duration = scan_result.scan_duration;
                    }

                    // *** PERSISTENCE: Save the updated core state to disk asynchronously ***
                    // Instead of blocking the UI with save(), use Task::perform for async save
                    let core_state = state.core_state.clone();
                    log::info!(
                        "Creating save task for ScanComplete with {} audiobooks",
                        core_state.data.audiobooks.len()
                    );

                    // Set saving state
                    state.saving = true;
                    state.save_progress = Some(0.0);

                    let save_task = Task::perform(
                        async move {
                            log::info!(
                                "Executing save_blocking_with_progress in ScanComplete task"
                            );
                            let (tx, rx) = mpsc::channel();
                            let save_handle = std::thread::spawn(move || {
                                // tx is automatically dropped when this closure ends
                                core_state.save_blocking_with_progress(Some(tx))
                            });

                            // Process progress updates
                            let _last_progress = 0.0; // Tracked for future use
                            loop {
                                match rx.try_recv() {
                                    Ok(progress) => {
                                        // Return early with progress update
                                        return Ok(Message::StateSaveProgress(progress));
                                    }
                                    Err(mpsc::TryRecvError::Empty) => {
                                        if save_handle.is_finished() {
                                            break;
                                        }
                                        thread::sleep(Duration::from_millis(50));
                                    }
                                    Err(mpsc::TryRecvError::Disconnected) => break,
                                }
                            }

                            match save_handle.join() {
                                Ok(Ok(msg)) => {
                                    log::info!("ScanComplete save successful: {msg}");
                                    Ok(Message::StateSaveComplete(Ok(
                                        "Scan results saved successfully".to_string(),
                                    )))
                                }
                                Ok(Err(e)) => {
                                    log::error!("ScanComplete save failed: {e}");
                                    Ok(Message::StateSaveComplete(Err(e.to_string())))
                                }
                                Err(_) => {
                                    log::error!("ScanComplete save thread panicked");
                                    Ok(Message::StateSaveComplete(Err(
                                        "Save operation failed".to_string()
                                    )))
                                }
                            }
                        },
                        |result: Result<Message, String>| match result {
                            Ok(message) => message,
                            Err(e) => Message::StateSaveComplete(Err(e)),
                        },
                    );

                    log::info!(
                        "[SCAN] Audiobooks set in state, count: {}",
                        state.audiobooks.len()
                    );

                    // Sort the audiobooks after updating
                    crate::utils::sort_audiobooks(state);
                    log::info!(
                        "[SCAN] Audiobooks sorted, count: {}",
                        state.audiobooks.len()
                    );

                    // Clear any previous processing status on successful scan
                    state.processing_status = None;

                    // Clear any selections and reset table state
                    state.selected_audiobooks.clear();
                    state.table_state = crate::state::TableState::default();

                    log::info!(
                        "[SCAN] Table state reset, audiobooks count in state: {}",
                        state.audiobooks.len()
                    );

                    // Force a UI update by requesting a redraw
                    state.needs_redraw = true;

                    // Return both the save task and the redraw reset task
                    return Some(Task::batch([
                        save_task,
                        Task::perform(
                            async {}, // No delay needed - reset immediately
                            |()| Message::ResetRedrawFlag,
                        ),
                    ]));
                }
                Err(e) => {
                    log::error!("[SCAN] Scan failed: {e}");
                    state.processing_status = Some(format!("Scan failed: {e}"));
                }
            }
            Some(Task::none())
        }
        Message::ScanProgress(progress) => {
            // Update scan progress during scanning
            if state.scanning {
                state.scan_progress = Some(progress);
            }
            Some(Task::none())
        }
        Message::SelectAudiobook(id) => {
            if state.selected_audiobooks.contains(&id) {
                state.selected_audiobooks.remove(&id);
            } else {
                state.selected_audiobooks.insert(id);
            }
            Some(Task::none())
        }
        Message::DeselectAll => {
            state.selected_audiobooks.clear();
            Some(Task::none())
        }
        Message::SortBy(column) => {
            // If clicking the same column, toggle sort direction
            if state.table_state.sort_column == column {
                state.table_state.sort_ascending = !state.table_state.sort_ascending;
            } else {
                state.table_state.sort_column = column;
                state.table_state.sort_ascending = true;
            }
            // Apply the sorting
            sort_audiobooks(state);
            Some(Task::none())
        }
        Message::AudioProcessingComplete(result) => {
            // Clear processing state
            state.processing_audio = false;
            state.processing_progress = None;

            match result {
                Ok(message) => {
                    log::info!("Audio processing completed: {message}");
                    state.processing_status = Some(format!("✓ {message}"));
                }
                Err(error) => {
                    log::error!("Audio processing failed: {error}");
                    state.processing_status = Some(format!("✗ Error: {error}"));
                }
            }
            Some(Task::none())
        }
        Message::PlaybackStarted(result) => {
            match result {
                Ok(message) => {
                    log::info!("Audio playback started: {message}");
                    state.player_state = get_player_state();
                    state.current_playing_file = get_audio_player().get_current_file();
                }
                Err(error) => {
                    log::error!("Audio playback failed: {error}");
                    state.processing_status = Some(format!("✗ Playback Error: {error}"));
                }
            }
            Some(Task::none())
        }
        Message::PlaybackStopped => {
            log::info!("Audio playback stopped");
            state.player_state = abop_core::PlayerState::Stopped;
            state.current_playing_file = None;
            Some(Task::none())
        }
        Message::StateSaveComplete(result) => {
            match result {
                Ok(message) => {
                    log::info!("✓ State save completed successfully: {message}");
                    // Optionally update UI status if needed
                }
                Err(error) => {
                    log::error!("✗ State save failed: {error}");
                    // Optionally show error to user
                    state.processing_status = Some(format!("✗ Save Error: {error}"));
                }
            }
            Some(Task::none())
        }
        Message::StateSaveProgress(progress) => {
            // Update save progress during saving
            if state.saving {
                state.save_progress = Some(progress);
            }
            Some(Task::none())
        }
        _ => None, // Not a data message
    }
}
