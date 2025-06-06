//! Library scanning functionality for the GUI

use abop_core::{
    db::Database,
    error::Result,
    models::{Audiobook, Library},
    scanner::LibraryScanner,
};
use iced::Element;
use iced::widget::{column, progress_bar, text};
use std::path::PathBuf;

use crate::messages::Message;

/// Result of a library scan operation including timing information
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// List of successfully scanned audiobooks
    pub audiobooks: Vec<Audiobook>,
    /// Total time taken for the scan
    pub scan_duration: std::time::Duration,
    /// Number of files processed
    pub processed_count: usize,
    /// Number of files that had errors
    pub error_count: usize,
}

/// Open a directory dialog and return the selected path
pub async fn open_directory_dialog() -> Option<PathBuf> {
    use rfd::AsyncFileDialog;

    AsyncFileDialog::new()
        .set_title("Select Audiobook Library Directory")
        .pick_folder()
        .await
        .map(|handle| handle.path().to_path_buf())
}

/// Scans a library directory and returns a task that will complete with the scan result
pub async fn scan_library(db: Database, library: Library) -> Result<ScanResult> {
    let scanner = LibraryScanner::new(db, library);
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Start scan with progress reporting
    let scan_task = tokio::spawn(async move { scanner.scan_async(Some(tx)).await });

    // Collect progress updates
    let mut result = ScanResult {
        audiobooks: Vec::new(),
        scan_duration: std::time::Duration::from_secs(0),
        processed_count: 0,
        error_count: 0,
    };

    while let Some(progress) = rx.recv().await {
        match progress {
            abop_core::scanner::progress::ScanProgress::Complete {
                processed,
                errors,
                duration,
            } => {
                result.processed_count = processed;
                result.error_count = errors;
                result.scan_duration = duration;
                break;
            }
            _ => continue,
        }
    }

    // Wait for scan to complete
    match scan_task.await {
        Ok(Ok(summary)) => {
            result.audiobooks = summary.new_files;
            Ok(result)
        }
        Ok(Err(e)) => Err(e.into()),
        Err(e) => Err(abop_core::error::AppError::from(e)),
    }
}

/// Scans a directory quickly to get basic information
pub async fn scan_directory_async(path: PathBuf) -> Result<DirectoryInfo> {
    let start_time = std::time::Instant::now();
    let mut book_count = 0;

    // Walk directory and count audio files
    for entry in walkdir::WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file()
            && let Some(ext) = entry.path().extension()
            && abop_core::scanner::SUPPORTED_AUDIO_EXTENSIONS
                .contains(&ext.to_string_lossy().as_ref())
        {
            book_count += 1;
        }
    }

    Ok(DirectoryInfo {
        path,
        last_scan: std::time::SystemTime::now(),
        book_count,
        scan_duration: start_time.elapsed(),
    })
}

/// Information about a directory
#[derive(Debug, Clone)]
pub struct DirectoryInfo {
    /// Path to the directory
    pub path: PathBuf,
    /// Last time the directory was scanned
    pub last_scan: std::time::SystemTime,
    /// Number of audiobooks found in the directory
    pub book_count: usize,
    /// Time taken to scan the directory
    pub scan_duration: std::time::Duration,
}

/// Progress tracking for scanner operations
#[derive(Debug, Clone)]
pub struct ScannerProgress {
    /// Current progress percentage (0.0 to 1.0)
    progress: Option<f32>,
    /// Current state of the scanner
    state: abop_core::scanner::ScannerState,
    /// Number of files processed so far
    current_count: usize,
    /// Total number of files to process
    total_count: usize,
    /// Name of the file currently being processed
    current_file: Option<String>,
    /// Number of errors encountered during scanning
    error_count: usize,
}

impl Default for ScannerProgress {
    fn default() -> Self {
        Self {
            progress: None,
            state: abop_core::scanner::ScannerState::Idle,
            current_count: 0,
            total_count: 0,
            current_file: None,
            error_count: 0,
        }
    }
}

impl ScannerProgress {
    /// Creates a new scanner progress tracker in its default state
    pub fn new() -> Self {
        Self::default()
    }

    /// Renders the current scan progress as a UI element
    pub fn view(&self) -> Element<'_, Message> {
        match self.state {
            abop_core::scanner::ScannerState::Idle => column![].into(),
            _ => {
                let progress_percentage = if self.total_count > 0 {
                    self.current_count as f32 / self.total_count as f32
                } else {
                    0.0
                };

                column![
                    progress_bar(0.0..=1.0, progress_percentage),
                    text(format!(
                        "Processed {}/{} files",
                        self.current_count, self.total_count
                    )),
                    if let Some(file) = &self.current_file {
                        text(format!("Current file: {file}"))
                    } else {
                        text("")
                    },
                    if self.error_count > 0 {
                        text(format!("Errors: {}", self.error_count))
                    } else {
                        text("")
                    },
                ]
                .spacing(10)
                .padding(20)
                .into()
            }
        }
    }

    /// Updates the progress state with a new progress event
    pub fn update(&mut self, progress: f32) {
        self.progress = Some(progress);
        if progress >= 1.0 {
            self.state = abop_core::scanner::ScannerState::Complete;
        } else {
            self.state = abop_core::scanner::ScannerState::Scanning;
        }
    }

    /// Updates the current file being processed
    pub fn update_current_file(&mut self, file: String) {
        self.current_file = Some(file);
    }

    /// Increments the error count
    pub fn increment_error_count(&mut self) {
        self.error_count += 1;
    }

    /// Resets the progress state
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Starts a new library scan operation
///
/// # Arguments
///
/// * `db` - Database connection to use for storing scan results
/// * `library` - Library configuration for the scan
///
/// # Returns
///
/// A Result indicating success or failure of the scan operation
pub async fn start_scan(db: Database, library: Library) -> Result<()> {
    let scanner = LibraryScanner::new(db, library);
    // Note: LibraryScanner doesn't have scan_directory method, using scan_async instead
    let _result = scanner.scan_async(None).await?;
    Ok(())
}

/// Cancels an ongoing library scan operation
///
/// # Arguments
///
/// * `db` - Database connection used by the scanner
/// * `library` - Library being scanned
///
/// # Returns
///
/// A Result indicating success or failure of the cancellation
pub async fn cancel_scan(db: Database, library: Library) -> Result<()> {
    let scanner = LibraryScanner::new(db, library);
    scanner.cancel_scan();
    Ok(())
}
