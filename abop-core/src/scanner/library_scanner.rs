//! Audio file scanner module for ABOP
//!
//! This module provides functionality to scan directories for audio files,
//! extract metadata, and update the database with the found files.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use futures::{StreamExt, stream};
use tokio::sync::{Semaphore, mpsc};
use tokio_util::sync::CancellationToken;
use tracing::warn;
use walkdir::WalkDir;

use crate::{
    audio::AudioMetadata,
    db::Database,
    error::{AppError, Result},
    models::{Audiobook, Library},
    scanner::{
        config::ScannerConfig,
        error::{ScanError, ScanResult},
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
#[derive(Clone)]
pub struct LibraryScanner {
    /// The database connection
    db: Database,
    /// The library being scanned
    library: Library,
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

    /// Unified scanning interface with progress reporting
    ///
    /// This method provides a single entry point for all scanning operations,
    /// supporting both synchronous and asynchronous progress reporting through
    /// callbacks or channels.
    ///
    /// # Arguments
    ///
    /// * `progress_handler` - Optional callback for progress updates
    /// * `message_handler` - Optional callback for completion message
    ///
    /// # Returns
    ///
    /// A Task that yields either progress updates or the final scan result
    pub fn scan_with_progress<F, M>(
        &self,
        progress_handler: Option<F>,
        message_handler: Option<M>,
    ) -> Task<LibraryScanResult>
    where
        F: Fn(f32) + Send + Sync + 'static + Clone,
        M: Fn(LibraryScanResult) + Send + Sync + 'static,
    {
        let db = self.db.clone();
        let library = self.library.clone();
        let _config = self.config.clone();
        let performance_monitor = self.performance_monitor.clone();
        let cancel_token = self.cancel_token.clone();

        Task::perform(
            async move {
                let start_time = std::time::Instant::now();
                let semaphore = Arc::new(Semaphore::new(_config.max_concurrent_tasks));
                let db_semaphore = Arc::new(Semaphore::new(_config.max_concurrent_db_operations));

                // Initial progress
                if let Some(ref handler) = progress_handler {
                    handler(0.0);
                }

                // Discover files
                let audio_files =
                    Self::find_audio_files_in_path_async(&library.path, &_config.extensions).await;
                if audio_files.is_empty() {
                    if let Some(ref handler) = progress_handler {
                        handler(1.0);
                    }
                    return LibraryScanResult::new();
                }

                let total_files = audio_files.len();

                // Process files
                let results: Vec<_> = stream::iter(
                    audio_files
                        .into_iter()
                        .enumerate()
                        .map(|(i, p)| (i, p.to_path_buf())),
                )
                .map(|(index, path)| {
                    let semaphore = semaphore.clone();
                    let db_semaphore = db_semaphore.clone();
                    let db = db.clone();
                    let library_id = library.id.clone();
                    let _config = _config.clone();
                    let performance_monitor = performance_monitor.as_deref();
                    let progress_handler = progress_handler.clone();
                    let cancel_token = cancel_token.clone();

                    async move {
                        // Check for cancellation
                        if cancel_token.is_cancelled() {
                            return Err(ScanError::Cancelled);
                        }

                        let _permit = semaphore
                            .acquire()
                            .await
                            .map_err(|_| ScanError::Cancelled)?;

                        // Process the file
                        let result = match Self::extract_audiobook_metadata_with_monitoring(
                            &library_id,
                            &path,
                            performance_monitor,
                        ) {
                            Ok(audiobook) => {
                                // Acquire database semaphore before database operation
                                let _db_permit = db_semaphore
                                    .acquire()
                                    .await
                                    .map_err(|_| ScanError::Cancelled)?;

                                // Convert the Result to a Future
                                match tokio::task::spawn_blocking({
                                    let db = db.clone();
                                    let audiobook = audiobook.clone();
                                    move || db.add_audiobook(&audiobook)
                                })
                                .await
                                {
                                    Ok(Ok(())) => Ok(audiobook),
                                    Ok(Err(e)) => match e {
                                        AppError::Database(e) => Err(ScanError::Database(e)),
                                        AppError::Io(e) => Err(ScanError::Io(e)),
                                        e => Err(ScanError::Metadata(e.to_string())),
                                    },
                                    Err(e) => Err(ScanError::Task(e.to_string())),
                                }
                            }
                            Err(e) => match e {
                                AppError::Database(e) => Err(ScanError::Database(e)),
                                AppError::Io(e) => Err(ScanError::Io(e)),
                                e => Err(ScanError::Metadata(e.to_string())),
                            },
                        };

                        // Update progress
                        if let Some(ref handler) = progress_handler {
                            let progress = (index + 1) as f32 / total_files as f32;
                            handler(progress);
                        }

                        result
                    }
                })
                .buffer_unordered(_config.max_concurrent_tasks)
                .collect()
                .await;

                // Collect results
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
                if let Some(ref handler) = progress_handler {
                    handler(1.0);
                }

                // Send completion message if handler provided
                if let Some(ref handler) = message_handler {
                    handler(scan_result.clone());
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

    /// Finds all audio files in the library directory
    pub fn find_audio_files(&self) -> Vec<PathBuf> {
        Self::find_audio_files_in_path(&self.library.path, &self.config.extensions)
    }

    /// Finds all audio files in a given directory path
    pub fn find_audio_files_in_path(path: &Path, extensions: &[String]) -> Vec<PathBuf> {
        println!("DEBUG: Scanning path: {}", path.display());
        println!("DEBUG: Looking for extensions: {extensions:?}");

        let results: Vec<PathBuf> = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(entry) => {
                    println!("DEBUG: Processing entry: {}", entry.path().display());
                    Some(entry)
                }
                Err(e) => {
                    log::warn!("Error reading directory entry: {e}");
                    None
                }
            })
            .filter(|e| {
                let is_file = e.file_type().is_file();
                if is_file {
                    println!("DEBUG: Found file: {}", e.path().display());
                }
                is_file
            })
            .filter_map(|entry| {
                let path = entry.path();
                if let Some(ext) = path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(str::to_lowercase)
                {
                    println!("DEBUG: Extension: '{ext}', checking against {extensions:?}");
                    if extensions.contains(&ext) {
                        println!("DEBUG: MATCH! Adding file: {}", path.display());
                        Some(entry.into_path())
                    } else {
                        println!("DEBUG: No match for extension '{ext}'");
                        None
                    }
                } else {
                    println!("DEBUG: No extension or not valid UTF-8");
                    None
                }
            })
            .collect();

        println!("DEBUG: Scan complete. Found {} files", results.len());
        results
    }

    /// Finds all audio files in the library directory asynchronously
    pub async fn find_audio_files_async(&self) -> Vec<PathBuf> {
        Self::find_audio_files_in_path_async(&self.library.path, &self.config.extensions).await
    }

    /// Finds all audio files in a given directory path asynchronously
    pub async fn find_audio_files_in_path_async(
        path: &Path,
        extensions: &[String],
    ) -> Vec<PathBuf> {
        let path = path.to_path_buf();
        let extensions = extensions.to_vec();

        tokio::task::spawn_blocking(move || Self::find_audio_files_in_path(&path, &extensions))
            .await
            .unwrap_or_else(|e| {
                warn!("Failed to discover audio files: {e}");
                Vec::new()
            })
    }

    /// Asynchronously scans the library with progress reporting
    ///
    /// # Arguments
    ///
    /// * `progress_tx` - Optional channel sender for progress updates
    ///
    /// # Returns
    ///
    /// A Result containing either a ScanSummary or a ScanError
    pub async fn scan_async(
        &self,
        progress_tx: Option<mpsc::UnboundedSender<ScanProgress>>,
    ) -> ScanResult<ScanSummary> {
        let start_time = std::time::Instant::now();
        let mut summary = ScanSummary::new();

        // Send initial progress
        if let Some(tx) = &progress_tx {
            let _ = tx.send(ScanProgress::Started {
                total_files: 0, // Will be updated after file discovery
            });
        }

        // Find audio files
        let audio_files = self.find_audio_files_async().await;
        let total_files = audio_files.len();

        // Update total files in progress
        if let Some(tx) = &progress_tx {
            let _ = tx.send(ScanProgress::Started { total_files });
        }

        // Process files with concurrency control
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_tasks));
        let db_semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_db_operations));
        let mut tasks = Vec::new();

        for (index, path) in audio_files.into_iter().enumerate() {
            // Check for cancellation
            if self.cancel_token.is_cancelled() {
                return Err(ScanError::Cancelled);
            }

            let semaphore = semaphore.clone();
            let db_semaphore = db_semaphore.clone();
            let db = self.db.clone();
            let library_id = self.library.id.clone();
            let progress_tx = progress_tx.clone();
            let performance_monitor = self.performance_monitor.clone();

            let task = tokio::spawn(async move {
                let _permit = semaphore
                    .acquire()
                    .await
                    .map_err(|_| ScanError::Cancelled)?;
                let performance_monitor = performance_monitor.as_deref();
                // Process the file
                let result = match Self::extract_audiobook_metadata_with_monitoring(
                    &library_id,
                    &path,
                    performance_monitor,
                ) {
                    Ok(audiobook) => {
                        // Acquire database semaphore before database operation
                        let _db_permit = db_semaphore
                            .acquire()
                            .await
                            .map_err(|_| ScanError::Cancelled)?;

                        // Convert the Result to a Future
                        match tokio::task::spawn_blocking({
                            let db = db.clone();
                            let audiobook = audiobook.clone();
                            move || db.add_audiobook(&audiobook)
                        })
                        .await
                        {
                            Ok(Ok(())) => Ok(audiobook),
                            Ok(Err(e)) => match e {
                                AppError::Database(e) => Err(ScanError::Database(e)),
                                AppError::Io(e) => Err(ScanError::Io(e)),
                                e => Err(ScanError::Metadata(e.to_string())),
                            },
                            Err(e) => Err(ScanError::Task(e.to_string())),
                        }
                    }
                    Err(e) => match e {
                        AppError::Database(e) => Err(ScanError::Database(e)),
                        AppError::Io(e) => Err(ScanError::Io(e)),
                        e => Err(ScanError::Metadata(e.to_string())),
                    },
                };

                // Send progress update
                if let Some(tx) = &progress_tx {
                    let progress = ScanProgress::FileProcessed {
                        current: index + 1,
                        total: total_files,
                        file_name: path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        progress_percentage: (index + 1) as f32 / total_files as f32 * 100.0,
                    };
                    let _ = tx.send(progress);
                }

                result
            });

            tasks.push(task);
        }

        // Collect results
        for task in tasks {
            match task.await {
                Ok(Ok(audiobook)) => {
                    summary.processed += 1;
                    summary.new_files.push(audiobook);
                }
                Ok(Err(e)) => {
                    summary.errors += 1;
                    warn!("Error processing file: {}", e);
                }
                Err(e) => {
                    summary.errors += 1;
                    warn!("Task error: {}", e);
                }
            }
        }

        summary.duration = start_time.elapsed();

        // Send completion progress
        if let Some(tx) = &progress_tx {
            let _ = tx.send(ScanProgress::Complete {
                processed: summary.processed,
                errors: summary.errors,
                duration: summary.duration,
            });
        }

        Ok(summary)
    }

    /// Cancels any ongoing scan operation
    pub fn cancel_scan(&self) {
        self.cancel_token.cancel();
    }

    /// Creates a new scan task for Iced integration
    pub fn scan_with_tasks(&self, audio_files: Vec<PathBuf>) -> Task<LibraryScanResult> {
        let db = self.db.clone();
        let library = self.library.clone();
        let config = self.config.clone();
        let performance_monitor = self.performance_monitor.clone();
        let cancel_token = self.cancel_token.clone();

        Task::perform(
            async move {
                let start_time = std::time::Instant::now();
                let mut result = LibraryScanResult::new();
                let semaphore = Arc::new(Semaphore::new(config.max_concurrent_tasks));
                let db_semaphore = Arc::new(Semaphore::new(config.max_concurrent_db_operations));

                let mut tasks = Vec::new();
                for path in audio_files {
                    if cancel_token.is_cancelled() {
                        return result;
                    }

                    let semaphore = semaphore.clone();
                    let db_semaphore = db_semaphore.clone();
                    let db = db.clone();
                    let library_id = library.id.clone();
                    let performance_monitor = performance_monitor.clone();

                    let task = tokio::spawn(async move {
                        let _permit = semaphore
                            .acquire()
                            .await
                            .map_err(|_| ScanError::Cancelled)?;
                        let performance_monitor = performance_monitor.as_deref();
                        match Self::extract_audiobook_metadata_with_monitoring(
                            &library_id,
                            &path,
                            performance_monitor,
                        ) {
                            Ok(audiobook) => {
                                // Acquire database semaphore before database operation
                                let _db_permit = db_semaphore
                                    .acquire()
                                    .await
                                    .map_err(|_| ScanError::Cancelled)?;

                                tokio::task::spawn_blocking({
                                    let db = db.clone();
                                    let audiobook = audiobook.clone();
                                    move || db.add_audiobook(&audiobook)
                                })
                                .await
                                .map_err(|e| ScanError::Task(e.to_string()))?
                                .map_err(|e| match e {
                                    AppError::Database(e) => ScanError::Database(e),
                                    AppError::Io(e) => ScanError::Io(e),
                                    e => ScanError::Metadata(e.to_string()),
                                })?;

                                Ok(audiobook)
                            }
                            Err(e) => match e {
                                AppError::Database(e) => Err(ScanError::Database(e)),
                                AppError::Io(e) => Err(ScanError::Io(e)),
                                e => Err(ScanError::Metadata(e.to_string())),
                            },
                        }
                    });

                    tasks.push(task);
                }

                for task in tasks {
                    match task.await {
                        Ok(Ok(audiobook)) => {
                            result.processed_count += 1;
                            result.audiobooks.push(audiobook);
                        }
                        Ok(Err(e)) => {
                            result.error_count += 1;
                            warn!("Error processing file: {}", e);
                        }
                        Err(e) => {
                            result.error_count += 1;
                            warn!("Task error: {}", e);
                        }
                    }
                }

                result.scan_duration = start_time.elapsed();
                result
            },
            |result| result,
        )
    }

    /// Creates a new scan task with progress reporting for Iced integration
    pub fn scan_with_tasks_and_progress<F>(
        &self,
        audio_files: Vec<PathBuf>,
        progress_handler: F,
    ) -> Task<LibraryScanResult>
    where
        F: Fn(f32) + Send + Sync + 'static + Clone,
    {
        let db = self.db.clone();
        let library = self.library.clone();
        let config = self.config.clone();
        let performance_monitor = self.performance_monitor.clone();
        let cancel_token = self.cancel_token.clone();

        Task::perform(
            async move {
                let start_time = std::time::Instant::now();
                let mut result = LibraryScanResult::new();
                let semaphore = Arc::new(Semaphore::new(config.max_concurrent_tasks));
                let db_semaphore = Arc::new(Semaphore::new(config.max_concurrent_db_operations));
                let total_files = audio_files.len();

                // Initial progress
                progress_handler(0.0);

                let mut tasks = Vec::new();
                for (index, path) in audio_files.into_iter().enumerate() {
                    if cancel_token.is_cancelled() {
                        return result;
                    }

                    let semaphore = semaphore.clone();
                    let db_semaphore = db_semaphore.clone();
                    let db = db.clone();
                    let library_id = library.id.clone();
                    let performance_monitor = performance_monitor.clone();
                    let progress_handler = progress_handler.clone();

                    let task = tokio::spawn(async move {
                        let _permit = semaphore
                            .acquire()
                            .await
                            .map_err(|_| ScanError::Cancelled)?;
                        let performance_monitor = performance_monitor.as_deref();
                        let file_result = match Self::extract_audiobook_metadata_with_monitoring(
                            &library_id,
                            &path,
                            performance_monitor,
                        ) {
                            Ok(audiobook) => {
                                // Acquire database semaphore before database operation
                                let _db_permit = db_semaphore
                                    .acquire()
                                    .await
                                    .map_err(|_| ScanError::Cancelled)?;

                                tokio::task::spawn_blocking({
                                    let db = db.clone();
                                    let audiobook = audiobook.clone();
                                    move || db.add_audiobook(&audiobook)
                                })
                                .await
                                .map_err(|e| ScanError::Task(e.to_string()))?
                                .map_err(|e| match e {
                                    AppError::Database(e) => ScanError::Database(e),
                                    AppError::Io(e) => ScanError::Io(e),
                                    e => ScanError::Metadata(e.to_string()),
                                })?;

                                Ok(audiobook)
                            }
                            Err(e) => match e {
                                AppError::Database(e) => Err(ScanError::Database(e)),
                                AppError::Io(e) => Err(ScanError::Io(e)),
                                e => Err(ScanError::Metadata(e.to_string())),
                            },
                        };

                        // Update progress
                        let progress = (index + 1) as f32 / total_files as f32;
                        progress_handler(progress);

                        file_result
                    });

                    tasks.push(task);
                }

                for task in tasks {
                    match task.await {
                        Ok(Ok(audiobook)) => {
                            result.processed_count += 1;
                            result.audiobooks.push(audiobook);
                        }
                        Ok(Err(e)) => {
                            result.error_count += 1;
                            warn!("Error processing file: {}", e);
                        }
                        Err(e) => {
                            result.error_count += 1;
                            warn!("Task error: {}", e);
                        }
                    }
                }

                // Final progress
                progress_handler(1.0);

                result.scan_duration = start_time.elapsed();
                result
            },
            |result| result,
        )
    }

    /// Enhanced async scan operation with modern async patterns
    /// Implements the specifications from the thread pool refactoring roadmap
    pub async fn scan_async_enhanced(
        &self,
        progress_tx: mpsc::Sender<ScanProgress>,
    ) -> ScanResultType<LibraryScanResult> {
        let start_time = std::time::Instant::now();
        let mut scan_result = LibraryScanResult::new();
        
        // Find all audio files
        let files = self.find_audio_files();
        let total_files = files.len();
        
        // Send initial progress
        progress_tx.send(ScanProgress::Started { total_files }).await
            .map_err(|_| ScanError::Cancelled)?;
        
        // Process files in parallel with backpressure
        let (result_tx, mut result_rx) = mpsc::channel(100);
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_tasks));
        
        let process_task = tokio::spawn({
            let files = files.clone();
            let semaphore = semaphore.clone();
            let library_id = self.library.id.clone();
            let cancel_token = self.cancel_token.clone();
            let performance_monitor = self.performance_monitor.clone();
            let progress_tx = progress_tx.clone();
            
            async move {
                stream::iter(files.into_iter().enumerate())
                    .for_each_concurrent(Some(self.config.max_concurrent_tasks), |(index, path)| {
                        let semaphore = semaphore.clone();
                        let result_tx = result_tx.clone();
                        let progress_tx = progress_tx.clone();
                        let cancel_token = cancel_token.clone();
                        let library_id = library_id.clone();
                        let monitor = performance_monitor.as_deref();
                        
                        async move {
                            // Check for cancellation
                            if cancel_token.is_cancelled() {
                                return;
                            }
                            
                            // Acquire semaphore permit
                            let _permit = match semaphore.acquire().await {
                                Ok(p) => p,
                                Err(_) => return,
                            };
                            
                            // Process file with performance monitoring
                            let result = Self::extract_audiobook_metadata_with_monitoring(&library_id, &path, monitor);
                            
                            // Send result
                            let _ = result_tx.send(result).await;
                            
                            // Update progress with detailed information
                            let progress = (index + 1) as f32 / total_files as f32;
                            let file_name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown")
                                .to_string();
                                
                            let _ = progress_tx.send(ScanProgress::FileProcessed {
                                current: index + 1,
                                total: total_files,
                                file_name,
                                progress_percentage: progress,
                            }).await;
                        }
                    })
                    .await;
            }
        });
        
        // Process results with batch processing
        let process_results = async {
            let mut batch = Vec::with_capacity(self.config.batch_size);
            let mut processed_count = 0;
            let mut error_count = 0;
            let progress_tx = progress_tx.clone();
            
            while let Some(result) = result_rx.recv().await {
                match result {
                    Ok(audiobook) => {
                        batch.push(audiobook);
                        processed_count += 1;
                        
                        // Process batch if full
                        if batch.len() >= self.config.batch_size {
                            if let Err(e) = self.process_batch(&batch).await {
                                tracing::error!("Failed to add batch: {}", e);
                                error_count += batch.len();
                            } else {
                                // Send batch committed progress
                                let _ = progress_tx.send(ScanProgress::BatchCommitted {
                                    count: batch.len(),
                                    total_processed: processed_count,
                                }).await;
                            }
                            
                            scan_result.audiobooks.extend(batch.drain(..));
                        }
                    }
                    Err(_) => {
                        error_count += 1;
                    }
                }
            }
            
            // Process remaining items in final batch
            if !batch.is_empty() {
                if let Err(e) = self.process_batch(&batch).await {
                    tracing::error!("Failed to add final batch: {}", e);
                    error_count += batch.len();
                } else {
                    let _ = progress_tx.send(ScanProgress::BatchCommitted {
                        count: batch.len(),
                        total_processed: processed_count,
                    }).await;
                }
                
                scan_result.audiobooks.extend(batch);
            }
            
            scan_result.processed_count = processed_count;
            scan_result.error_count = error_count;
            
            Ok::<_, ScanError>(())
        };
        
        // Wait for both tasks to complete
        let (process_results, _) = tokio::join!(
            process_results,
            process_task
        );
        
        // Check for cancellation
        if self.cancel_token.is_cancelled() {
            let duration = start_time.elapsed();
            let _ = progress_tx.send(ScanProgress::Cancelled {
                processed: scan_result.processed_count,
                duration,
            }).await;
            return Err(ScanError::Cancelled);
        }
        
        process_results?;
        
        // Calculate final duration and send completion
        scan_result.scan_duration = start_time.elapsed();
        
        let _ = progress_tx.send(ScanProgress::Complete {
            processed: scan_result.processed_count,
            errors: scan_result.error_count,
            duration: scan_result.scan_duration,
        }).await;
        
        // Log performance summary if monitoring is enabled
        if let Some(monitor) = &self.performance_monitor {
            monitor.log_summary();
        }
        
        Ok(scan_result)
    }
    
    /// Process a batch of audiobooks asynchronously
    async fn process_batch(&self, batch: &[Audiobook]) -> ScanResultType<()> {
        // For now, process items individually but this could be optimized
        // with a batch database insert method in the future
        for audiobook in batch {
            self.db.add_audiobook(audiobook)
                .map_err(|e| match e {
                    crate::error::AppError::Database(rusqlite_err) => ScanError::Database(rusqlite_err),
                    other => ScanError::Database(rusqlite::Error::SqliteFailure(
                        rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                        Some(format!("Database operation failed: {}", other))
                    ))
                })?;
        }
        Ok(())
    }

    /// Creates an enhanced Task-based scan with modern async patterns and cancellation
    pub fn scan_async_task(
        &self,
        progress_tx: mpsc::Sender<ScanProgress>,
    ) -> Task<ScanResultType<LibraryScanResult>> {
        let scanner = self.clone();
        
        Task::perform(
            async move {
                scanner.scan_async_enhanced(progress_tx).await
            },
            |result| result,
        )
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
