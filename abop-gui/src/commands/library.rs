//! Library scanning command handlers

use iced::Task;

use crate::library::{open_directory_dialog, scan_library};
use crate::messages::{Command as GuiCommand, Message};
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
pub fn handle_library_command(state: &mut UiState, command: GuiCommand) -> Option<Task<Message>> {
    match command {
        GuiCommand::ScanLibrary { library_path } => {
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

            // Move all DB operations into the async task
            Some(Task::perform(
                async move {
                    // Open DB asynchronously
                    let db =
                        match tokio::task::spawn_blocking(move || Database::open(&db_path)).await {
                            Ok(Ok(db)) => db,
                            Ok(Err(e)) => return Err(e.to_string()),
                            Err(e) => return Err(format!("Failed to open database: {e}")),
                        };

                    // Look up or create library asynchronously
                    let library = match tokio::task::spawn_blocking({
                        let db = db.clone();
                        move || db.libraries().find_by_name("Default Library")
                    })
                    .await
                    {
                        Ok(Ok(Some(lib))) => lib,
                        Ok(Ok(None)) => {
                            match tokio::task::spawn_blocking({
                                let db = db.clone();
                                let path = library_path.clone();
                                move || db.add_library("Default Library", path)
                            })
                            .await
                            {
                                Ok(Ok(lib)) => lib,
                                Ok(Err(e)) => return Err(e.to_string()),
                                Err(e) => return Err(format!("Failed to create library: {e}")),
                            }
                        }
                        Ok(Err(e)) => return Err(e.to_string()),
                        Err(e) => return Err(format!("Failed to find library: {e}")),
                    };

                    // Use our new unified scanning interface
                    let scan_result = scan_library(db, library).await;
                    match scan_result {
                        Ok(result) => Ok(result),
                        Err(e) => Err(e.to_string()),
                    }
                },
                Message::ScanComplete,
            ))
        }
        GuiCommand::BrowseDirectory => {
            log::info!("Executing BrowseDirectory command");
            Some(Task::perform(
                open_directory_dialog(),
                Message::DirectorySelected,
            ))
        }
        GuiCommand::QuickScanDirectory { directory_path } => {
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
