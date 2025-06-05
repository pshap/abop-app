//! Library scanning command handlers

use iced::Task;

use crate::library::{open_directory_dialog, scan_library_async};
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
            state.scanning = true;
            state.scan_progress = Some(0.0);
            // Remove duplicate status message - the StatusDisplay component already shows "Scanning library..." when scanning is true
            log::info!(
                "Executing ScanLibrary command for path: {}",
                library_path.display()
            );
            Some(Task::perform(
                scan_library_async(library_path),
                Message::ScanComplete,
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
