//! Library scanning functionality

use abop_core::{
    db::Database,
    error::{AppError, Result},
    models::{Audiobook, Library},
    scanner::{LibraryScanner, progress::ScanProgress},
};
use iced::Element;
use iced::Task;
use iced::widget::{column, progress_bar, text};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::messages::Message;

/// Progress callback type for scanning operations

/// Result of a library scan operation including timing information
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// List of successfully scanned audiobooks
    pub audiobooks: Vec<Audiobook>,
    /// Total time taken for the scan
    pub scan_duration: Duration,
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
pub fn scan_library(
    db: Database,
    library: Library,
    progress_tx: Option<mpsc::UnboundedSender<ScanProgress>>,
) -> Task<Result<ScanResult>> {
    Task::perform(
        async move {
            let scanner = LibraryScanner::new(db, library);

            let result = scanner.scan_async(progress_tx).await?;

            // Convert ScanSummary to ScanResult
            let scan_result = ScanResult {
                audiobooks: result.new_files,
                scan_duration: result.duration,
                processed_count: result.processed,
                error_count: result.errors,
            };

            Ok(scan_result)
        },
        |result| result,
    )
}

/// Scans a library directory with progress updates
pub fn scan_library_with_progress(db: Database, library: Library) -> Task<Result<ScanResult>> {
    scan_library(db, library, None)
}

/// Scan library with enhanced progress tracking and ETA calculation
pub fn scan_library_with_enhanced_progress(
    _library_path: PathBuf,
    db: Database,
    library: Library,
    progress_callback: impl Fn(ScanProgress) + Send + Sync + 'static,
) -> Task<Result<ScanResult>> {
    Task::perform(
        async move {
            let scanner = LibraryScanner::new(db, library);
            let (progress_tx, mut progress_rx) = mpsc::unbounded_channel();

            // Start the scan in a background task
            let scan_handle =
                tokio::spawn(async move { scanner.scan_async(Some(progress_tx)).await });

            // Process progress updates
            let mut scan_result = ScanResult {
                audiobooks: Vec::new(),
                scan_duration: Duration::from_secs(0),
                processed_count: 0,
                error_count: 0,
            };

            let _start_time = std::time::Instant::now();
            while let Some(progress_event) = progress_rx.recv().await {
                let should_break = matches!(&progress_event, ScanProgress::Complete { .. });

                match &progress_event {
                    ScanProgress::Started { total_files: _ } => {
                        scan_result.processed_count = 0;
                        scan_result.error_count = 0;
                    }
                    ScanProgress::FileProcessed {
                        current,
                        total: _,
                        file_name: _,
                        progress_percentage: _,
                    } => {
                        scan_result.processed_count = *current;
                    }
                    ScanProgress::BatchCommitted {
                        count: _,
                        total_processed,
                    } => {
                        scan_result.processed_count = *total_processed;
                    }
                    ScanProgress::Complete {
                        processed,
                        errors,
                        duration,
                    } => {
                        scan_result.processed_count = *processed;
                        scan_result.error_count = *errors;
                        scan_result.scan_duration = *duration;
                    }
                    ScanProgress::Cancelled {
                        processed,
                        duration,
                    } => {
                        scan_result.processed_count = *processed;
                        scan_result.scan_duration = *duration;
                        progress_callback(progress_event);
                        return Err(AppError::Scan(
                            abop_core::scanner::error::ScanError::Cancelled,
                        ));
                    }
                }
                progress_callback(progress_event);

                if should_break {
                    break;
                }
            }

            // Get final result
            match scan_handle.await {
                Ok(Ok(summary)) => {
                    scan_result.audiobooks = summary.new_files;
                    scan_result.processed_count = summary.processed;
                    scan_result.error_count = summary.errors;
                    scan_result.scan_duration = summary.duration;
                    Ok(scan_result)
                }
                Ok(Err(e)) => Err(AppError::Scan(e)),
                Err(e) => Err(AppError::Threading(e.to_string().into())),
            }
        },
        |result| result,
    )
}

/// Progress tracking and display for library scanning operations
///
/// This struct maintains the current state of a library scan operation and provides
/// methods to update the progress and render a UI representation of the scan status.
pub struct ScannerProgress {
    /// Most recent progress event from the scanner
    progress: Option<ScanProgress>,
    /// Current state of the scanner (idle, scanning, etc.)
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
    ///
    /// # Returns
    ///
    /// A new ScannerProgress instance with all counters initialized to zero
    /// and the scanner in the Idle state
    pub fn new() -> Self {
        Self::default()
    }

    /// Renders the current scan progress as a UI element
    ///
    /// # Returns
    ///
    /// An iced Element containing a progress bar and status information
    /// about the current scan operation. Returns an empty column if the
    /// scanner is idle.
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
                    text("Scanning Library...").size(20),
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
    ///
    /// # Arguments
    ///
    /// * `progress` - New progress event from the scanner
    pub fn update(&mut self, progress: ScanProgress) {
        match &progress {
            ScanProgress::Started { total_files } => {
                self.total_count = *total_files;
                self.current_count = 0;
                self.error_count = 0;
                self.current_file = None;
            }
            ScanProgress::FileProcessed {
                current,
                total,
                file_name,
                ..
            } => {
                self.current_count = *current;
                self.total_count = *total;
                self.current_file = Some(file_name.clone());
            }
            ScanProgress::BatchCommitted {
                total_processed, ..
            } => {
                self.current_count = *total_processed;
            }
            ScanProgress::Complete {
                processed, errors, ..
            } => {
                self.current_count = *processed;
                self.error_count = *errors;
            }
            ScanProgress::Cancelled { processed, .. } => {
                self.current_count = *processed;
            }
        }
        self.progress = Some(progress);
    }

    /// Updates the scanner state
    ///
    /// # Arguments
    ///
    /// * `state` - New state for the scanner
    pub fn set_state(&mut self, state: abop_core::scanner::ScannerState) {
        self.state = state;
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
