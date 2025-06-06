//! Library scanning command handlers

use iced::Task;

use crate::library::open_directory_dialog;
use crate::messages::{Command, Message};
use crate::state::{DirectoryInfo, UiState};
use std::path::PathBuf;
use std::time::SystemTime;
use abop_core::{
    db::{Database, DatabaseConfig},
    models::Library,
    error::AppError,
    scanner::progress::ScanProgress,
};

/// Scans a directory asynchronously and returns metadata about the scan
pub async fn scan_directory_async(path: PathBuf) -> Result<DirectoryInfo, String> {
    let start = std::time::Instant::now();
    let mut book_count = 0;

    let entries = std::fs::read_dir(&path).map_err(|e| e.to_string())?;    for entry in entries.flatten() {
        if let Some(ext) = entry.path().extension() {
            if matches!(ext.to_str(), Some("mp3" | "m4a" | "flac" | "wav" | "ogg")) {
                book_count += 1;
            }
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
            let db = match Database::new(DatabaseConfig {
                path: db_path.to_string_lossy().into_owned(),
                enhanced: true,
            }) {
                Ok(db) => db,
                Err(e) => {
                    return Some(Task::perform(
                        async move { Err(e.to_string()) },
                        Message::ScanComplete,
                    ));
                }
            };

            // Create a new library
            let library = Library {
                id: uuid::Uuid::new_v4().to_string(),
                name: "Default Library".to_string(),
                path: library_path,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // Add the library to the database
            if let Err(e) = db.add_library(&library) {
                return Some(Task::perform(
                    async move { Err(e.to_string()) },
                    Message::ScanComplete,
                ));
            }

            // Create scanner and launch scan with progress reporting
            let scanner = abop_core::scanner::LibraryScanner::new(db, library);
            
            // Create a Task that emits progress updates and final result
            Some(Task::perform(
                async move {
                    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
                    let scan_result = scanner.scan_async(tx).await?;
                    Ok(scan_result)
                },
                |result| match result {
                    Ok(summary) => Message::ScanComplete(Ok(summary)),
                    Err(e) => Message::ScanComplete(Err(e.to_string())),
                },
            ))
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

/// Start scanning a library
pub fn start_scan(library: Library, db: Database) -> Task<Message> {
    Task::perform(
        async move {
            let scanner = abop_core::scanner::LibraryScanner::new(db, library);
            let (tx, _rx) = tokio::sync::mpsc::channel(100);
            let result = scanner.scan_async(tx).await?;
            Ok(result)
        },
        |result| match result {
            Ok(summary) => Message::ScanComplete(Ok(summary)),
            Err(e) => Message::ScanComplete(Err(e.to_string())),
        },
    )
}

/// Cancel the current scan
pub fn cancel_scan() -> Task<Message> {
    Task::perform(
        async move { () },
        |_| Message::ScanCancelled,
    )
}
