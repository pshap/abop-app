//! Library scanning functionality

use abop_core::{
    db::{Database, DatabaseConfig}, 
    models::{Audiobook, Library}, 
    scanner::{LibraryScanner, progress::ScanProgress as CoreScanProgress},
    error::{AppError, Result}
};
use iced::Task;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::mpsc;
use abop_core::scanner::progress::ScanProgress;
use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{Alignment, Element, Length, Theme};

use crate::messages::Message;

/// Progress callback type for scanning operations
pub type ProgressCallback = Box<dyn Fn(f32) + Send + Sync>;

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
pub async fn scan_library(
    db: Database,
    library: Library,
    progress_tx: mpsc::Sender<ScanProgress>,
) -> Task<Result<ScanResult>> {
    Task::spawn(async move {
        let repository = Arc::new(AudiobookRepository::new(db));
        let scanner = LibraryScanner::new(
            repository,
            4, // max_workers
            100, // batch_size
        );
        
        let result = scanner.scan_async(progress_tx).await?;
        Ok(result)
    })
}

/// Scans a library directory with progress updates
pub fn scan_library_with_progress(
    library: Library,
    db: Database,
) -> Task<Result<ScanResult, AppError>> {
    scan_library(library, db, None)
}

/// Scan library with enhanced progress tracking and ETA calculation
pub fn scan_library_with_enhanced_progress(
    library_path: PathBuf, 
    db: Database, 
    library: Library,
    progress_callback: impl Fn(ScanProgress) + Send + Sync + 'static
) -> Task<Result<ScanResult>> {
    Task::perform(
        async move {
            let scanner = LibraryScanner::new(db, library);
            let (progress_tx, mut progress_rx) = mpsc::channel(100);
            
            // Start the scan in a background task
            let scan_handle = tokio::spawn(async move {
                scanner.scan_async(progress_tx).await
            });

            // Process progress updates
            let mut scan_result = ScanResult {
                audiobooks: Vec::new(),
                scan_duration: Duration::from_secs(0),
                processed_count: 0,
                error_count: 0,
            };

            let mut progress = ScanProgress::new(0);
            let start_time = std::time::Instant::now();

            while let Some(progress_event) = progress_rx.recv().await {
                match progress_event {
                    CoreScanProgress::Started { total_files } => {
                        progress = ScanProgress::new(total_files);
                        scan_result.processed_count = 0;
                        scan_result.error_count = 0;
                    }
                    CoreScanProgress::FileProcessed { current, total, file_name, progress_percentage } => {
                        let elapsed = start_time.elapsed();
                        let throughput = current as f64 / elapsed.as_secs_f64();
                        progress.update(current, Some(file_name), throughput);
                        progress_callback(progress.clone());
                        scan_result.processed_count = current;
                    }
                    CoreScanProgress::BatchCommitted { count, total_processed } => {
                        scan_result.processed_count = total_processed;
                    }
                    CoreScanProgress::Complete { processed, errors, duration } => {
                        scan_result.processed_count = processed;
                        scan_result.error_count = errors;
                        scan_result.scan_duration = duration;
                        break;
                    }
                    CoreScanProgress::Cancelled { processed, duration } => {
                        scan_result.processed_count = processed;
                        scan_result.scan_duration = duration;
                        return Err(AppError::Scan(abop_core::scanner::error::ScanError::Cancelled));
                    }
                }
            }

            // Get final result
            match scan_handle.await {
                Ok(Ok(summary)) => {
                    scan_result.processed_count = summary.processed;
                    scan_result.error_count = summary.errors;
                    Ok(scan_result)
                }
                Ok(Err(e)) => Err(AppError::Scan(e)),
                Err(e) => Err(AppError::Threading(e.to_string())),
            }
        },
        |result| result
    )
}

/// Helper function to scan with progress reporting
async fn scan_with_progress_reporting(
    db: Database,
    library: Library,
    progress_callback: ProgressCallback,
) -> Result<Vec<Audiobook>> {
    let scanner = LibraryScanner::new(db, library);
    let (progress_tx, mut progress_rx) = mpsc::channel(100);
    
    // Start the scan in a background task
    let scan_handle = tokio::spawn(async move {
        scanner.scan_async(progress_tx).await
    });

    // Process progress updates
    let mut audiobooks = Vec::new();
    let start_time = std::time::Instant::now();

    while let Some(progress) = progress_rx.recv().await {
        match progress {
            CoreScanProgress::Started { total_files } => {
                progress_callback(0.0);
            }
            CoreScanProgress::FileProcessed { current, total, file_name, progress_percentage } => {
                progress_callback(progress_percentage);
            }
            CoreScanProgress::BatchCommitted { count, total_processed } => {
                // No progress update needed for batch commits
            }
            CoreScanProgress::Complete { processed, errors, duration } => {
                progress_callback(1.0);
                break;
            }
            CoreScanProgress::Cancelled { processed, duration } => {
                return Err(AppError::Scan(abop_core::scanner::error::ScanError::Cancelled));
            }
        }
    }

    // Get final result
    match scan_handle.await {
        Ok(Ok(summary)) => Ok(summary.audiobooks),
        Ok(Err(e)) => Err(AppError::Scan(e)),
        Err(e) => Err(AppError::Threading(e.to_string())),
    }
}

pub struct ScannerProgress {
    progress: Option<ScanProgress>,
    state: abop_core::scanner::ScannerState,
}

impl Default for ScannerProgress {
    fn default() -> Self {
        Self {
            progress: None,
            state: abop_core::scanner::ScannerState::Idle,
        }
    }
}

impl ScannerProgress {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&self) -> Element<Message> {
        match self.state {
            abop_core::scanner::ScannerState::Idle => column![].into(),
            _ => {
                let progress = self.progress.as_ref().unwrap_or(&ScanProgress::default());
                let progress_percentage = if progress.total_files > 0 {
                    progress.files_processed as f32 / progress.total_files as f32
                } else {
                    0.0
                };

                column![
                    text("Scanning Library...").size(20),
                    progress_bar(0.0..=1.0, progress_percentage),
                    text(format!(
                        "Processed {}/{} files",
                        progress.files_processed, progress.total_files
                    )),
                    if let Some(file) = &progress.current_file {
                        text(format!("Current file: {}", file.display()))
                    } else {
                        text("")
                    },
                    if !progress.errors.is_empty() {
                        text(format!("Errors: {}", progress.errors.len()))
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

    pub fn update(&mut self, progress: ScanProgress) {
        self.progress = Some(progress);
    }

    pub fn set_state(&mut self, state: abop_core::scanner::ScannerState) {
        self.state = state;
    }
}

pub async fn start_scan(
    db: Arc<AudiobookRepository>,
    library: Library,
    max_workers: usize,
    batch_size: usize,
) -> Result<()> {
    let scanner = LibraryScanner::new(db, max_workers, batch_size);
    scanner.scan_directory(library.path).await
}

pub async fn cancel_scan(
    db: Arc<AudiobookRepository>,
    library: Library,
    max_workers: usize,
    batch_size: usize,
) -> Result<()> {
    let scanner = LibraryScanner::new(db, max_workers, batch_size);
    scanner.cancel().await
}

#[derive(Debug, Clone)]
pub enum ScanProgress {
    Started { total_files: usize },
    FileProcessed { 
        current: usize,
        total: usize,
        file_name: String,
        progress_percentage: f32,
    },
    BatchCommitted { 
        count: usize,
        total_processed: usize,
    },
    Complete { 
        processed: usize,
        errors: usize,
        duration: std::time::Duration,
    },
    Cancelled { 
        processed: usize,
        duration: std::time::Duration,
    },
}

impl ScanProgress {
    pub fn new(total_files: usize) -> Self {
        Self::Started { total_files }
    }

    pub fn update_progress(&mut self, current: usize, total: usize, file_name: String) {
        let progress_percentage = if total > 0 {
            current as f32 / total as f32
        } else {
            0.0
        };
        
        *self = Self::FileProcessed {
            current,
            total,
            file_name,
            progress_percentage,
        };
    }

    pub fn complete(&mut self, processed: usize, errors: usize, duration: std::time::Duration) {
        *self = Self::Complete {
            processed,
            errors,
            duration,
        };
    }

    pub fn cancel(&mut self, processed: usize, duration: std::time::Duration) {
        *self = Self::Cancelled {
            processed,
            duration,
        };
    }
}
