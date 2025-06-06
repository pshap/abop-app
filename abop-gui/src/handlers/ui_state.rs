//! UI state message handlers
//!
//! Handles messages that only affect UI state without requiring async operations

use iced::Task;
use abop_core::error::AppError;
use crate::messages::Message;
use crate::state::UiState;
use abop_core::models::Audiobook;
use abop_core::scanner::progress::ScanProgress;
use crate::components::task_manager::TaskManager;

/// Handles pure UI state changes that don't require async operations
#[must_use]
pub fn handle_ui_message(state: &mut UiState, message: Message) -> Option<Task<Message>> {
    match message {
        Message::ShowSettings => {
            state.settings_open = true;
            Some(Task::none())
        }
        Message::CloseSettings => {
            state.settings_open = false;
            Some(Task::none())
        }
        Message::SelectRecentDirectory(path) => {
            log::info!("Selected recent directory: {}", path.display());
            state.library_path = path;
            Some(Task::none())
        }
        Message::SetTheme(theme) => {
            state.theme_mode = theme;
            log::info!("Theme changed to: {theme:?}");
            Some(Task::none())
        }
        Message::PlayPause => {
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
                    if let Some(audiobook) =
                        state.audiobooks.iter().find(|ab| ab.path == *current_file)
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
        Message::Stop => {
            log::info!("Stop button pressed");
            crate::audio::player::stop_audio();
            state.player_state = abop_core::PlayerState::Stopped;
            Some(Task::none())
        }
        Message::Previous => {
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
        Message::Next => {
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
        Message::QuickScanComplete(result) => {
            match result {
                Ok(directory_info) => {
                    log::info!(
                        "Quick scan completed for {}: {} books found in {:?}",
                        directory_info.path.display(),
                        directory_info.book_count,
                        directory_info.scan_duration
                    );

                    // Update or add to recent directories
                    if let Some(existing) = state
                        .recent_directories
                        .iter_mut()
                        .find(|dir| dir.path == directory_info.path)
                    {
                        *existing = directory_info;
                    } else {
                        state.recent_directories.push(directory_info);
                        // Keep only the 10 most recent directories
                        if state.recent_directories.len() > 10 {
                            state.recent_directories.remove(0);
                        }
                    }
                }
                Err(error) => {
                    log::error!("Quick scan failed: {error}");
                }
            }
            Some(Task::none())
        }
        Message::ResetRedrawFlag => {
            state.needs_redraw = false;
            Some(Task::none())
        }
        Message::StartTask(task_type) => {
            let task = TaskManager::create_task(task_type);
            state.active_task = Some(task);
            Some(Task::none())
        }
        Message::TaskProgress { task_id, progress, status } => {
            if let Some(task) = &mut state.active_task {
                if task.id == task_id {
                    task.progress = progress;
                    task.status = status;
                }
            }
            Some(Task::none())
        }
        Message::TaskComplete { task_id, status } => {
            if let Some(mut task) = state.active_task.take() {
                if task.id == task_id {
                    task.is_completed = true;
                    task.is_running = false;
                    task.status = status;
                    task.end_time = Some(chrono::Local::now());
                    state.recent_tasks.push(task);
                }
            }
            Some(Task::none())
        }
        Message::TaskFailed { task_id, error } => {
            if let Some(mut task) = state.active_task.take() {
                if task.id == task_id {
                    task.is_running = false;
                    task.error = Some(error);
                    task.end_time = Some(chrono::Local::now());
                    state.recent_tasks.push(task);
                }
            }
            Some(Task::none())
        }
        Message::CancelTask => {
            if let Some(mut task) = state.active_task.take() {
                task.is_running = false;
                task.error = Some("Cancelled by user".to_string());
                task.end_time = Some(chrono::Local::now());
                state.recent_tasks.push(task);
            }
            Some(Task::none())
        }
        Message::ToggleTaskHistory => {
            state.show_task_history = !state.show_task_history;
            Some(Task::none())
        }
        Message::ClearTaskHistory => {
            state.recent_tasks.clear();
            Some(Task::none())
        }
        Message::ScanProgress(progress) => {
            match progress {
                ScanProgress::Started { total_files } => {
                    state.processing_status = Some(format!("Starting scan of {} files...", total_files));
                    state.scan_progress = Some(0.0);
                }
                ScanProgress::FileProcessed { current, total, file_name, progress_percentage } => {
                    state.processing_status = Some(format!("Processing {} ({}/{})", file_name, current, total));
                    state.scan_progress = Some(progress_percentage);
                }
                ScanProgress::BatchCommitted { count, total_processed } => {
                    state.processing_status = Some(format!(
                        "Committed batch of {} files (total: {})",
                        count, total_processed
                    ));
                }
                ScanProgress::Complete { processed, errors, duration } => {
                    state.scanning = false;
                    state.processing_status = Some(format!(
                        "Scan complete: {} files processed ({} errors) in {:.2?}",
                        processed, errors, duration
                    ));
                    state.scan_progress = Some(1.0);
                }
                ScanProgress::Cancelled { processed, duration } => {
                    state.scanning = false;
                    state.processing_status = Some(format!(
                        "Scan cancelled after processing {} files in {:.2?}",
                        processed, duration
                    ));
                    state.scan_progress = Some(0.0);
                }
            }
            Some(Task::none())
        }
        Message::ScanComplete(result) => {
            match result {
                Ok(audiobooks) => {
                    state.scanning = false;
                    state.processing_status = Some("Scan completed successfully".to_string());
                    state.scan_progress = Some(1.0);
                    state.audiobooks = audiobooks;
                }
                Err(e) => {
                    state.scanning = false;
                    state.processing_status = Some(format!("Scan failed: {}", e));
                    state.scan_progress = Some(0.0);
                }
            }
            Some(Task::none())
        }
        Message::ScanCancelled => {
            state.scanning = false;
            state.processing_status = Some("Scan cancelled".to_string());
            state.scan_progress = Some(0.0);
            Some(Task::none())
        }
        Message::Error(e) => {
            state.scanning = false;
            state.processing_status = Some(format!("Error: {}", e));
            state.scan_progress = Some(0.0);
            Some(Task::none())
        }
        Message::Close => {
            Some(Task::none())
        }
        _ => None, // Not a UI message
    }
}

/// Handles scan completion
pub fn handle_scan_complete(state: &mut UiState, result: Result<(), AppError>) {
    state.scanning = false;
    match result {
        Ok(()) => {
            state.processing_status = Some("Scan completed successfully".to_string());
            state.scan_progress = Some(1.0);
        }
        Err(e) => {
            state.processing_status = Some(format!("Scan failed: {}", e));
            state.scan_progress = Some(0.0);
        }
    }
}
