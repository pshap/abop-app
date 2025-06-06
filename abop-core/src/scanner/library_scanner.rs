//! Audio file scanner module for ABOP
//!
//! This module provides functionality to scan directories for audio files,
//! extract metadata, and update the database with the found files.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use futures::{StreamExt, stream};
use rayon::prelude::*;
use tokio::sync::{Semaphore, mpsc};
use tokio_util::sync::CancellationToken;
use tracing::{Level, debug, error, info, instrument, span, warn};
use walkdir::WalkDir;

use crate::{
    audio::AudioMetadata,
    db::Database,
    error::Result,
    models::{Audiobook, Library},
    scanner::{
        config::ScannerConfig,
        error::{ScanError, ScanResult as ScanResultType},
        performance::{OperationType, PerformanceMonitor},
        progress::ScanProgress,
        result::ScanSummary,
    },
};
use iced::Task;

/// Supported audio file extensions for scanning
pub const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["mp3", "m4a", "m4b", "flac", "ogg", "wav", "aac"];

/// Progress updates for scanning operations
#[derive(Debug, Clone)]
pub enum ScanProgressUpdate {
    /// Scanning started with total file count
    Started {
        /// Total number of files that will be processed in this scan
        total_files: usize,
    },
    /// Individual file processed
    FileProcessed {
        /// Current file number being processed (1-based index)
        current: usize,
        /// Total number of files to process
        total: usize,
        /// Name of the file currently being processed
        file_name: String,
        /// Overall progress percentage (0.0 to 100.0)
        progress_percentage: f32,
    },
    /// Scanning completed
    Complete {
        /// Number of files successfully processed
        processed: usize,
        /// Number of files that encountered errors during processing
        errors: usize,
        /// Total duration of the scan operation
        duration: std::time::Duration,
    },
}

/// Scans a library directory for audio files and updates the database
pub struct LibraryScanner {
    /// The database connection
    db: Database,
    /// The library being scanned
    library: Library,
    /// File extensions to include in the scan (using static strings for efficiency)
    extensions: &'static [&'static str],
    /// Configuration for async scanning operations
    config: ScannerConfig,
    /// Cancellation token for async operations
    cancel_token: CancellationToken,
    /// Performance monitor for tracking operation times
    performance_monitor: Option<Arc<PerformanceMonitor>>,
}

impl LibraryScanner {
    /// Creates a new `LibraryScanner` for the given library
    #[must_use]
    pub fn new(db: Database, library: Library) -> Self {
        Self {
            db,
            library,
            extensions: SUPPORTED_AUDIO_EXTENSIONS,
            config: ScannerConfig::default(),
            cancel_token: CancellationToken::new(),
            performance_monitor: Some(Arc::new(PerformanceMonitor::new())),
        }
    }

    /// Creates a new scanner with custom configuration
    pub fn with_config(mut self, config: ScannerConfig) -> Self {
        self.config = config;
        self
    }

    /// Enables performance monitoring for this scanner
    pub fn with_performance_monitoring(mut self) -> Self {
        self.performance_monitor = Some(Arc::new(PerformanceMonitor::new()));
        self
    }

    /// Disables performance monitoring for this scanner
    pub fn without_performance_monitoring(mut self) -> Self {
        self.performance_monitor = None;
        self
    }

    /// Gets the performance monitor if enabled
    pub fn get_performance_monitor(&self) -> Option<Arc<PerformanceMonitor>> {
        self.performance_monitor.clone()
    }

    /// Extracts audiobook metadata from a file path (no DB access, safe for parallel)
    ///
    /// # Errors
    /// Returns an error if metadata cannot be read from the file
    pub fn extract_audiobook_metadata(library_id: &str, path: &Path) -> Result<Audiobook> {
        let metadata = AudioMetadata::from_file(path).map_err(|e| {
            warn!("Error reading metadata for {}: {}", path.display(), e);
            e
        })?;

        let mut audiobook = Audiobook::new(library_id, path);
        audiobook.title = Some(metadata.title.unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown Title")
                .to_string()
        }));
        audiobook.author = metadata.artist;
        audiobook.narrator = metadata.narrator;
        audiobook.duration_seconds = metadata.duration_seconds.map(|d| {
            if d.is_nan() || d < 0.0 {
                0
            } else {
                // Safely convert f64 to u64 with explicit bounds checking
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let duration = d.round().clamp(0.0, f64::from(u32::MAX)) as u64;
                duration
            }
        });
        if let Some(cover_art) = metadata.cover_art {
            audiobook.cover_art = Some(cover_art);
        }

        Ok(audiobook)
    }

    /// Extracts audiobook metadata from a file path with performance monitoring
    ///
    /// # Errors
    /// Returns an error if metadata cannot be read from the file
    pub fn extract_audiobook_metadata_with_monitoring(
        library_id: &str,
        path: &Path,
        monitor: Option<&PerformanceMonitor>,
    ) -> Result<Audiobook> {
        let start_time = std::time::Instant::now();
        let _timer = monitor.map(|m| {
            m.start_operation(
                path.to_string_lossy().as_ref(),
                OperationType::MetadataExtraction,
            )
        });

        let result = Self::extract_audiobook_metadata(library_id, path);

        // Record the operation result
        if let Some(monitor) = monitor {
            let duration = start_time.elapsed();
            monitor.record_file_processed(duration, result.is_ok());
        }

        result
    }

    /// Scans the library directory for audio files and updates the database
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Audio file discovery fails
    /// - Metadata extraction fails
    /// - Database operations fail
    /// - File system access is denied
    #[instrument(name = "scan_library", skip(self), fields(library_name = %self.library.name, library_path = %self.library.path.display()))]
    pub fn scan(&self) -> Result<LibraryScanResult> {
        info!("Starting scan of library: {}", self.library.name);

        // Collect all audio files in the library directory
        let audio_files = self.find_audio_files();
        info!("Found {} audio files", audio_files.len());

        let library_id = self.library.id.clone();
        let performance_monitor = self.performance_monitor.as_deref();

        // Extract metadata in parallel, no DB access
        let _span = span!(Level::DEBUG, "extract_metadata_parallel").entered();
        let audiobooks: Vec<_> = audio_files
            .par_iter()
            .filter_map(|path| {
                match Self::extract_audiobook_metadata_with_monitoring(
                    &library_id,
                    path,
                    performance_monitor,
                ) {
                    Ok(book) => {
                        debug!("Successfully extracted metadata for: {}", path.display());
                        Some(book)
                    }
                    Err(e) => {
                        warn!("Error extracting metadata from {}: {}", path.display(), e);
                        None
                    }
                }
            })
            .collect();
        drop(_span);

        // Write to DB serially
        let _db_span = span!(Level::DEBUG, "database_writes").entered();
        let mut result = LibraryScanResult::new();
        for audiobook in audiobooks {
            match self.db.add_audiobook(&audiobook) {
                Ok(()) => {
                    result.processed_count += 1;
                    debug!("Successfully saved audiobook: {}", audiobook.path.display());
                    result.audiobooks.push(audiobook);
                }
                Err(e) => {
                    error!("Error saving {}: {}", audiobook.path.display(), e);
                    result.error_count += 1;
                }
            }
        }

        log::info!(
            "Scan completed. Processed: {}, Errors: {}",
            result.processed_count,
            result.error_count
        );

        // Log performance summary if monitoring is enabled
        if let Some(monitor) = &self.performance_monitor {
            monitor.log_summary();
            let recommendations = monitor.get_recommendations();
            if !recommendations.is_empty() {
                info!("Performance recommendations:");
                for rec in recommendations {
                    info!("  • {}", rec);
                }
            }
        }

        Ok(result)
    }

    /// Async Task-based scan using Iced's Task system
    pub fn scan_with_tasks(&self, paths: Vec<PathBuf>) -> Task<LibraryScanResult> {
        let db = self.db.clone();
        let library = self.library.clone();
        let performance_monitor = self.performance_monitor.clone();
        let concurrency = 8;
        let semaphore = Arc::new(Semaphore::new(concurrency));

        Task::perform(
            async move {
                let results: Vec<_> = stream::iter(paths.into_iter().enumerate())
                    .map(|(_index, path)| {
                        let semaphore = semaphore.clone();
                        let db = db.clone();
                        let library_id = library.id.clone();
                        let monitor = performance_monitor.as_deref();
                        async move {
                            let _permit = semaphore.acquire().await.unwrap();

                            // Start timing for database operations
                            let db_start = std::time::Instant::now();
                            let _db_timer = monitor.map(|m| {
                                m.start_operation(
                                    path.to_string_lossy().as_ref(),
                                    OperationType::DatabaseInsert,
                                )
                            });

                            match Self::extract_audiobook_metadata_with_monitoring(
                                &library_id,
                                &path,
                                monitor,
                            ) {
                                Ok(audiobook) => {
                                    let db_result = db.add_audiobook(&audiobook);

                                    // Record database operation performance
                                    if let Some(monitor) = monitor {
                                        let db_duration = db_start.elapsed();
                                        monitor
                                            .record_file_processed(db_duration, db_result.is_ok());
                                    }

                                    match db_result {
                                        Ok(()) => Ok(audiobook),
                                        Err(e) => Err(e),
                                    }
                                }
                                Err(e) => Err(e),
                            }
                        }
                    })
                    .buffer_unordered(concurrency)
                    .collect()
                    .await;

                let mut scan_result = LibraryScanResult::new();
                for res in results {
                    match res {
                        Ok(audiobook) => {
                            scan_result.processed_count += 1;
                            scan_result.audiobooks.push(audiobook);
                        }
                        Err(_e) => {
                            scan_result.error_count += 1;
                        }
                    }
                }

                // Log performance summary if monitoring is enabled
                if let Some(monitor) = &performance_monitor {
                    monitor.log_summary();
                }

                scan_result
            },
            |result| result,
        )
    }

    /// Async Task-based scan using Iced's Task system with progress reporting
    /// Returns a Task that yields progress updates and final result
    pub fn scan_with_tasks_and_progress(
        &self,
        paths: Vec<PathBuf>,
        progress_callback: impl Fn(f32) + Send + Sync + 'static,
    ) -> Task<LibraryScanResult> {
        let db = self.db.clone();
        let library = self.library.clone();
        let concurrency = 8;
        let semaphore = Arc::new(Semaphore::new(concurrency));
        let total_files = paths.len();
        let progress_callback = Arc::new(progress_callback);

        Task::perform(
            async move {
                let start_time = std::time::Instant::now();

                // Initial progress
                progress_callback(0.0);

                let results: Vec<_> = stream::iter(paths.into_iter().enumerate())
                    .map(|(index, path)| {
                        let semaphore = semaphore.clone();
                        let db = db.clone();
                        let library_id = library.id.clone();
                        let progress_callback = progress_callback.clone();

                        async move {
                            let _permit = semaphore.acquire().await.unwrap();

                            // Process the file
                            let result = match Self::extract_audiobook_metadata(&library_id, &path)
                            {
                                Ok(audiobook) => match db.add_audiobook(&audiobook) {
                                    Ok(()) => Ok(audiobook),
                                    Err(e) => Err(e),
                                },
                                Err(e) => Err(e),
                            };

                            // Update progress
                            let progress = (index + 1) as f32 / total_files as f32;
                            progress_callback(progress);

                            result
                        }
                    })
                    .buffer_unordered(concurrency)
                    .collect()
                    .await;

                let mut scan_result = LibraryScanResult::new();
                scan_result.scan_duration = start_time.elapsed();

                for res in results {
                    match res {
                        Ok(audiobook) => {
                            scan_result.processed_count += 1;
                            scan_result.audiobooks.push(audiobook);
                        }
                        Err(_e) => {
                            scan_result.error_count += 1;
                        }
                    }
                }

                // Final progress
                progress_callback(1.0);

                scan_result
            },
            |result| result,
        )
    }

    /// Async Task-based scan using Iced's Task system with progress messages
    /// Returns a Task that yields both progress updates and final result
    pub fn scan_with_tasks_streaming<Message>(
        &self,
        paths: Vec<PathBuf>,
        progress_message: impl Fn(f32) -> Message + Send + Sync + 'static,
        complete_message: impl Fn(LibraryScanResult) -> Message + Send + Sync + 'static,
    ) -> Task<Message>
    where
        Message: Send + 'static,
    {
        let db = self.db.clone();
        let library = self.library.clone();
        let concurrency = 8;
        let semaphore = Arc::new(Semaphore::new(concurrency));
        let total_files = paths.len();
        let progress_message = Arc::new(progress_message);
        let complete_message = Arc::new(complete_message);

        Task::perform(
            async move {
                let start_time = std::time::Instant::now();

                // Send initial progress
                let _ = progress_message(0.0);

                // Process files
                let results: Vec<_> = stream::iter(paths.into_iter().enumerate())
                    .map(|(index, path)| {
                        let semaphore = semaphore.clone();
                        let db = db.clone();
                        let library_id = library.id.clone();
                        let progress_message = progress_message.clone();

                        async move {
                            let _permit = semaphore.acquire().await.unwrap();

                            // Process the file
                            let result = match Self::extract_audiobook_metadata(&library_id, &path)
                            {
                                Ok(audiobook) => match db.add_audiobook(&audiobook) {
                                    Ok(()) => Ok(audiobook),
                                    Err(e) => Err(e),
                                },
                                Err(e) => Err(e),
                            };

                            // Update progress
                            let progress = (index + 1) as f32 / total_files as f32;
                            let _ = progress_message(progress);

                            result
                        }
                    })
                    .buffer_unordered(concurrency)
                    .collect()
                    .await;

                let mut scan_result = LibraryScanResult::new();
                scan_result.scan_duration = start_time.elapsed();

                for res in results {
                    match res {
                        Ok(audiobook) => {
                            scan_result.processed_count += 1;
                            scan_result.audiobooks.push(audiobook);
                        }
                        Err(_e) => {
                            scan_result.error_count += 1;
                        }
                    }
                }

                // Send final completion message
                complete_message(scan_result)
            },
            |message| message,
        )
    }

    /// Cancels ongoing scanning operations
    pub fn cancel_scan(&self) {
        self.cancel_token.cancel();
    }

    /// Async scan with modern async/await patterns and proper error handling
    ///
    /// # Arguments
    /// * `progress_sender` - Optional channel sender for progress updates
    ///
    /// # Errors
    /// Returns a `ScanError` if scanning fails
    #[instrument(name = "scan_async", skip(self, progress_sender), fields(library_name = %self.library.name))]
    pub async fn scan_async(
        &self,
        progress_sender: Option<mpsc::UnboundedSender<ScanProgress>>,
    ) -> ScanResultType<ScanSummary> {
        info!("Starting async scan of library: {}", self.library.name);
        let start_time = std::time::Instant::now();

        // Send start progress
        if let Some(ref sender) = progress_sender {
            let _ = sender.send(ScanProgress::Started {
                total_files: 0, // Will be updated after file discovery
            });
        }

        // Discover files with configuration filters
        let audio_files = self.discover_files().await?;
        let total_files = audio_files.len();

        info!("Found {} audio files", total_files);

        if total_files == 0 {
            let summary = ScanSummary {
                processed: 0,
                errors: 0,
                duration: start_time.elapsed(),
                new_files: Vec::new(),
                updated_files: Vec::new(),
            };

            if let Some(ref sender) = progress_sender {
                let _ = sender.send(ScanProgress::Complete {
                    processed: 0,
                    errors: 0,
                    duration: start_time.elapsed(),
                });
            }

            return Ok(summary);
        }

        // Create semaphore for concurrency control
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_tasks));
        let library_id = self.library.id.clone();

        // Process files with backpressure control
        let results = stream::iter(audio_files.into_iter().enumerate())
            .map(|(index, path)| {
                let semaphore = semaphore.clone();
                let library_id = library_id.clone();
                let cancel_token = self.cancel_token.clone();
                let config = self.config.clone();
                let progress_sender = progress_sender.clone();

                async move {
                    // Check for cancellation
                    if cancel_token.is_cancelled() {
                        return Err(ScanError::Cancelled);
                    }

                    let _permit = semaphore
                        .acquire()
                        .await
                        .map_err(|_| ScanError::Cancelled)?;

                    let result = self.process_file(&library_id, &path, &config).await;

                    // Send progress update
                    if let Some(ref sender) = progress_sender {
                        let file_name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown")
                            .to_string();

                        let progress_percentage = (index + 1) as f32 / total_files as f32;

                        let _ = sender.send(ScanProgress::FileProcessed {
                            current: index + 1,
                            total: total_files,
                            file_name,
                            progress_percentage,
                        });
                    }

                    result
                }
            })
            .buffer_unordered(self.config.max_concurrent_tasks)
            .collect::<Vec<_>>()
            .await;

        // Collect results and create summary
        let mut processed_count = 0;
        let mut error_count = 0;
        let mut new_files = Vec::new();

        for result in results {
            match result {
                Ok(audiobook) => {
                    processed_count += 1;
                    new_files.push(audiobook);
                }
                Err(ScanError::Cancelled) => {
                    // Handle cancellation
                    if let Some(ref sender) = progress_sender {
                        let _ = sender.send(ScanProgress::Cancelled {
                            processed: processed_count,
                            duration: start_time.elapsed(),
                        });
                    }

                    return Err(ScanError::Cancelled);
                }
                Err(_) => {
                    error_count += 1;
                }
            }
        }

        let summary = ScanSummary {
            processed: processed_count,
            errors: error_count,
            duration: start_time.elapsed(),
            new_files,
            updated_files: Vec::new(), // For now, we only handle new files
        };

        info!(
            "Async scan completed. Processed: {}, Errors: {}, Duration: {:?}",
            processed_count, error_count, summary.duration
        );

        // Send completion
        if let Some(ref sender) = progress_sender {
            let _ = sender.send(ScanProgress::Complete {
                processed: processed_count,
                errors: error_count,
                duration: summary.duration,
            });
        }

        Ok(summary)
    }

    /// Process a single file asynchronously
    ///
    /// # Arguments
    /// * `library_id` - The ID of the library
    /// * `path` - Path to the audio file
    /// * `config` - Scanner configuration
    ///
    /// # Errors
    /// Returns a `ScanError` if file processing fails
    async fn process_file(
        &self,
        library_id: &str,
        path: &Path,
        config: &ScannerConfig,
    ) -> ScanResultType<crate::models::Audiobook> {
        // Check file size if configured
        if config.max_file_size > 0 {
            let metadata = tokio::fs::metadata(path).await?;

            if metadata.len() > config.max_file_size {
                return Err(ScanError::Metadata(format!(
                    "File too large: {} bytes (max: {} bytes)",
                    metadata.len(),
                    config.max_file_size
                )));
            }
        }

        // Add timeout if configured
        let metadata_future = tokio::task::spawn_blocking({
            let library_id = library_id.to_string();
            let path = path.to_path_buf();
            move || Self::extract_audiobook_metadata(&library_id, &path)
        });

        let audiobook = if let Some(timeout) = config.timeout {
            tokio::time::timeout(timeout, metadata_future)
                .await
                .map_err(|_| ScanError::Timeout(timeout))?
                .map_err(|e| ScanError::Task(format!("Task failed: {e}")))?
                .map_err(|e| ScanError::Metadata(format!("Metadata extraction failed: {e}")))?
        } else {
            metadata_future
                .await
                .map_err(|e| ScanError::Task(format!("Task failed: {e}")))?
                .map_err(|e| ScanError::Metadata(format!("Metadata extraction failed: {e}")))?
        };

        // Save to database
        self.db
            .add_audiobook(&audiobook)
            .map_err(|e| ScanError::Metadata(format!("Database error: {e}")))?;

        debug!("Successfully processed file: {}", path.display());

        Ok(audiobook)
    }

    /// Discover audio files asynchronously with configuration filters
    ///
    /// # Errors
    /// Returns a `ScanError` if file discovery fails
    async fn discover_files(&self) -> ScanResultType<Vec<PathBuf>> {
        let path = self.library.path.clone();
        let extensions = self.config.extensions.clone();

        let files = tokio::task::spawn_blocking(move || {
            WalkDir::new(&path)
                .into_iter()
                .filter_map(|entry| match entry {
                    Ok(entry) => Some(entry),
                    Err(e) => {
                        warn!("Error reading directory entry: {e}");
                        None
                    }
                })
                .filter(|e| e.file_type().is_file())
                .filter_map(|entry| {
                    entry
                        .path()
                        .extension()
                        .and_then(OsStr::to_str)
                        .map(str::to_lowercase)
                        .and_then(|ext| {
                            if extensions.contains(&ext) {
                                Some(entry.into_path())
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>()
        })
        .await
        .map_err(|e| ScanError::Task(format!("File discovery failed: {e}")))?;

        Ok(files)
    }

    /// Finds all audio files in the library directory
    pub fn find_audio_files(&self) -> Vec<PathBuf> {
        Self::find_audio_files_in_path(&self.library.path, self.extensions)
    }

    /// Finds all audio files in a given directory path
    pub fn find_audio_files_in_path(path: &Path, extensions: &[&str]) -> Vec<PathBuf> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry),
                Err(e) => {
                    log::warn!("Error reading directory entry: {e}");
                    None
                }
            })
            .filter(|e| e.file_type().is_file())
            .filter_map(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(str::to_lowercase)
                    .and_then(|ext| {
                        if extensions.contains(&&*ext) {
                            Some(entry.into_path())
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}

/// Represents the result of a library scan
#[derive(Debug, Clone)]
pub struct LibraryScanResult {
    /// Number of files successfully processed
    pub processed_count: usize,
    /// Number of files that had errors
    pub error_count: usize,
    /// List of processed audiobooks
    pub audiobooks: Vec<Audiobook>,
    /// Duration of the scan
    pub scan_duration: std::time::Duration,
}

impl LibraryScanResult {
    /// Creates a new empty scan result
    #[must_use]
    pub const fn new() -> Self {
        Self {
            processed_count: 0,
            error_count: 0,
            audiobooks: Vec::new(),
            scan_duration: std::time::Duration::new(0, 0),
        }
    }
}

impl Default for LibraryScanResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_find_audio_files() {
        // Create a temporary directory with some test files
        let temp_dir = tempdir().unwrap();
        let test_files = [
            file::TEST_MP3,
            file::TEST_M4B,
            file::TEST_TXT,
            file::TEST_FLAC,
        ];

        // Create the files
        for file in &test_files {
            let path = temp_dir.path().join(file);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            let mut f = File::create(&path).unwrap();
            write!(f, "{}", file::TEST_CONTENT).unwrap();
        }

        // Create a test library
        let db = Database::open(file::MEMORY_DB).unwrap();
        let library = Library::new(library::TEST_NAME, temp_dir.path());

        // Create a scanner and find audio files
        let scanner = LibraryScanner::new(db, library);
        let audio_files = scanner.find_audio_files();

        // Should find 3 audio files (excluding the .txt file)
        assert_eq!(audio_files.len(), 3);
        assert!(audio_files.iter().any(|p| p.ends_with(file::TEST_MP3)));
        assert!(audio_files.iter().any(|p| p.ends_with(file::TEST_M4B)));
        assert!(audio_files.iter().any(|p| p.ends_with(file::TEST_FLAC)));
        assert!(!audio_files.iter().any(|p| p.ends_with(file::TEST_TXT)));
    }
}
