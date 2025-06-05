//! Library scanning command handlers

use iced::Task;

use crate::library::open_directory_dialog;
use crate::messages::{Command, Message};
use crate::state::{DirectoryInfo, UiState};
use std::path::PathBuf;
use std::time::SystemTime;

/// Scans a directory asynchronously and returns metadata about the scan
pub async fn scan_directory_async(path: PathBuf) -> Result<DirectoryInfo, String> {
    let start = std::time::Instant::now();
    let mut book_count = 0;

    let entries = std::fs::read_dir(&path).map_err(|e| e.to_string())?;

    for entry in entries.flatten() {
        if let Some(ext) = entry.path().extension()
            && matches!(ext.to_str(), Some("mp3" | "m4a" | "flac" | "wav" | "ogg"))
        {
            book_count += 1;
        }
    }

    Ok(DirectoryInfo {
        path,
        last_scan: SystemTime::now(),
        book_count,
        scan_duration: start.elapsed(),
    })
}

/// Handles library-related commands
#[must_use]
pub fn handle_library_command(state: &mut UiState, command: Command) -> Option<Task<Message>> {
    match command {
        Command::ScanLibrary { library_path } => {
            use abop_core::db::Database;
            use std::fs;
            use std::path::PathBuf;

            state.scanning = true;
            state.scan_progress = Some(0.0);
            log::info!(
                "Executing ScanLibrary command for path: {}",
                library_path.display()
            );

            // Prepare DB path
            let data_dir = dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("abop");
            if let Err(e) = fs::create_dir_all(&data_dir) {
                log::error!("Failed to create data dir: {e}");
                return Some(Task::perform(
                    async move { Err(format!("Failed to create data dir: {e}")) },
                    Message::ScanComplete,
                ));
            }
            let db_path = data_dir.join("library.db");

            // Open DB and look up or create Library synchronously
            let db = match Database::open(&db_path) {
                Ok(db) => db,
                Err(e) => {
                    return Some(Task::perform(
                        async move { Err(e.to_string()) },
                        Message::ScanComplete,
                    ));
                }
            };
            let library = match db.libraries().find_by_name("Default Library") {
                Ok(Some(lib)) => lib,
                Ok(None) => match db.add_library("Default Library", &library_path) {
                    Ok(lib) => lib,
                    Err(e) => {
                        return Some(Task::perform(
                            async move { Err(e.to_string()) },
                            Message::ScanComplete,
                        ));
                    }
                },
                Err(e) => {
                    return Some(Task::perform(
                        async move { Err(e.to_string()) },
                        Message::ScanComplete,
                    ));
                }
            };

            // Create progress-enabled scanner and launch scan with progress reporting
            let scanner = abop_core::scanner::LibraryScanner::new(db, library);
            let audio_files = scanner.find_audio_files();
            
            // Create a Task that emits progress updates and final result
            Some(
                scanner
                    .scan_with_tasks_and_progress(audio_files, |_progress| {
                        // This will be handled via message subscription pattern
                        // The actual progress updates will be sent through a channel
                    })
                    .map(|core_result| {
                        // Convert abop_core::scanner::ScanResult to crate::library::ScanResult
                        let gui_result = crate::library::ScanResult {
                            audiobooks: core_result.audiobooks,
                            scan_duration: core_result.scan_duration,
                            processed_count: core_result.processed_count,
                            error_count: core_result.error_count,
                            performance_monitor: None,
                        };
                        Message::ScanComplete(Ok(gui_result))
                    })
            )
        }
        Command::BrowseDirectory => {
            log::info!("Executing BrowseDirectory command");
            Some(Task::perform(
                open_directory_dialog(),
                Message::DirectorySelected,
            ))
        }
        Command::QuickScanDirectory { directory_path } => {
            log::info!(
                "Executing QuickScanDirectory command for path: {}",
                directory_path.display()
            );
            Some(Task::perform(
                scan_directory_async(directory_path),
                Message::QuickScanComplete,
            ))
        }
        _ => None, // Not a library command
    }
}
