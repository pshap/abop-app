//! Library scanning command handlers

use iced::Task;

use crate::library::{open_directory_dialog, scan_library};
use crate::messages::{Command as GuiCommand, Message};
use crate::state::{AppState, DirectoryInfo};
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
pub fn handle_library_command(state: &mut AppState, command: GuiCommand) -> Option<Task<Message>> {
    match command {
        GuiCommand::ScanLibrary { library_path } => {
            use abop_core::db::Database;

            state.library.start_scanning();
            log::info!(
                "Executing ScanLibrary command for path: {}",
                library_path.display()
            );

            // Move all DB operations into the async task
            Some(Task::perform(
                async move {
                    // Open centralized database synchronously in a blocking task
                    let db = match tokio::task::spawn_blocking(Database::open_app_database).await {
                        Ok(Ok(db)) => db,
                        Ok(Err(e)) => return Err(e.to_string()),
                        Err(e) => return Err(e.to_string()),
                    };

                    // Look up or create library based on the selected path
                    let library = match db.libraries().find_by_path(&library_path) {
                        Ok(Some(lib)) => {
                            log::warn!(
                                "🔍 LIBRARY COMMAND: Found existing library '{}' with path: '{}'",
                                lib.name,
                                lib.path.display()
                            );
                            lib
                        }
                        Ok(None) => {
                            log::warn!(
                                "🔍 LIBRARY COMMAND: Creating new library with path: '{}'",
                                library_path.display()
                            );
                            // Create a library name based on the directory name
                            let library_name = library_path
                                .file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or("Audiobook Library");

                            // Create library and then fetch the actual Library struct
                            let library_id = match db
                                .add_library_with_path(library_name, library_path.clone())
                            {
                                Ok(lib_id) => lib_id,
                                Err(e) => return Err(e.to_string()),
                            };

                            // Now get the actual Library struct
                            match db.libraries().find_by_id(&library_id) {
                                Ok(Some(lib)) => {
                                    log::warn!(
                                        "🔍 LIBRARY COMMAND: Created and retrieved library '{}' with path: '{}'",
                                        lib.name,
                                        lib.path.display()
                                    );
                                    lib
                                }
                                Ok(None) => {
                                    return Err("Library not found after creation".to_string());
                                }
                                Err(e) => return Err(e.to_string()),
                            }
                        }
                        Err(e) => return Err(e.to_string()),
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
