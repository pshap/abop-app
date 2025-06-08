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
    error::Result,
    models::{Audiobook, Library},
    scanner::{
        config::ScannerConfig,
        error::{ScanError, ScanResult},
        performance::{OperationType, PerformanceMonitor},
        progress::{ChannelReporter, ProgressReporter, ScanProgress},
        result::ScanSummary,
        task_manager::TaskManager,
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
    /// Configuration for scanning operations
    config: ScannerConfig,
    /// Task manager for async operations
    task_manager: TaskManager,
    /// Performance monitor for tracking operation times
    performance_monitor: Option<Arc<PerformanceMonitor>>,
    /// Cancel token for cancelling operations
    cancel_token: CancellationToken,
}

impl LibraryScanner {
    /// Creates a new `LibraryScanner` for the given library
    #[must_use]
    pub fn new(db: Database, library: Library) -> Self {
        Self {
            db,
            library,
            config: ScannerConfig::default(),
            task_manager: TaskManager::new(),
            performance_monitor: Some(Arc::new(PerformanceMonitor::new())),
            cancel_token: CancellationToken::new(),
        }
    }

    /// Creates a new scanner with custom configuration    #[must_use]
    pub fn with_config(mut self, config: ScannerConfig) -> Self {
        let max_concurrent_tasks = config.max_concurrent_tasks;
        let max_concurrent_db_operations = config.max_concurrent_db_operations;
        self.config = config;
        self.task_manager =
            TaskManager::with_settings(max_concurrent_tasks, max_concurrent_db_operations);
        self
    }

    /// Enables performance monitoring for this scanner
    #[must_use]
    pub fn with_performance_monitoring(mut self) -> Self {
        self.performance_monitor = Some(Arc::new(PerformanceMonitor::new()));
        self
    }

    /// Disables performance monitoring for this scanner
    #[must_use]
    pub fn without_performance_monitoring(mut self) -> Self {
        self.performance_monitor = None;
        self
    }

    /// Gets the performance monitor if enabled
    #[must_use]
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
        // Set file size for MD3 data clarity
        if let Ok(meta) = std::fs::metadata(path) {
            audiobook.size_bytes = Some(meta.len());
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

    /// Cancels any ongoing scan operations
    pub fn cancel_scan(&self) {
        self.task_manager.cancel();
    }

    /// Scans the library directory for audio files and updates the database
    pub async fn scan_async(
        &self,
        progress_tx: Option<mpsc::Sender<ScanProgress>>,
    ) -> ScanResult<ScanSummary> {
        let config = self.config.clone(); // Clone config to avoid move issues
        let library_id = self.library.id.clone();
        let _library_path = self.library.path.clone();
        let _db = self.db.clone();
        let cancel_token = self.cancel_token.clone();
        let performance_monitor = self.performance_monitor.clone();

        // Create progress reporter if channel provided
        let progress_reporter = progress_tx.map(|tx| {
            let reporter = ChannelReporter::new(tx);
            Arc::new(reporter) as Arc<dyn ProgressReporter>
        });

        // Find all audio files
        let audio_files = self.find_audio_files();
        let total_files = audio_files.len();

        // Report scan start
        if let Some(reporter) = &progress_reporter {
            reporter.report_started(total_files).await;
        }

        let start_time = std::time::Instant::now();
        let mut new_files = Vec::new();
        let mut errors = 0;

        // Process files in batches
        for chunk in audio_files.chunks(config.batch_size) {
            if cancel_token.is_cancelled() {
                return Err(ScanError::Cancelled);
            }

            let mut batch = Vec::with_capacity(chunk.len());
            for path in chunk {
                match Self::extract_audiobook_metadata_with_monitoring(
                    &library_id,
                    path,
                    performance_monitor.as_deref(),
                ) {
                    Ok(mut audiobook) => {
                        audiobook.library_id = library_id.clone();
                        batch.push(audiobook);
                    }
                    Err(e) => {
                        warn!("Error processing {}: {}", path.display(), e);
                        errors += 1;
                    }
                }
            }

            // Process batch
            if !batch.is_empty() {
                self.process_batch(&batch).await?;
                new_files.extend(batch);
            } // Report progress
            if let Some(reporter) = &progress_reporter {
                reporter
                    .report_file_processed(new_files.len(), total_files, "Processing files".into())
                    .await;
            }
        }

        let duration = start_time.elapsed();

        // Report completion
        let processed = new_files.len();
        if let Some(reporter) = &progress_reporter {
            reporter.report_complete(processed, errors, duration).await;
        }

        Ok(ScanSummary {
            new_files,
            scan_duration: duration,
            processed,
            errors,
        })
    }

    /// Creates a task for scanning the library
    pub fn scan_with_task(
        &self,
        progress_tx: mpsc::Sender<ScanProgress>,
    ) -> Task<ScanResult<ScanSummary>> {
        let scanner = self.clone();
        Task::perform(
            async move { scanner.scan_async(Some(progress_tx)).await },
            |result| result,
        )
    }

    /// Processes a batch of audiobooks
    async fn process_batch(&self, batch: &[Audiobook]) -> ScanResult<()> {
        let db = self.db.clone();
        let library_id = self.library.id.clone();

        // Use a semaphore to limit concurrent DB operations
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_db_operations));
        let mut handles = Vec::with_capacity(batch.len());

        for audiobook in batch {
            let db = db.clone();
            let _library_id = library_id.clone();
            let semaphore = semaphore.clone();
            let audiobook = audiobook.clone();

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await?;
                let repo = db.audiobook_repository();
                repo.upsert(&audiobook).map_err(ScanError::from)?;
                Ok::<_, ScanError>(())
            });

            handles.push(handle);
        }

        // Wait for all operations to complete
        for handle in handles {
            handle.await??;
        }

        Ok(())
    }

    /// Finds all audio files in the library directory
    pub fn find_audio_files(&self) -> Vec<PathBuf> {
        Self::find_audio_files_in_path(&self.library.path, &self.config.extensions)
    }

    /// Finds all audio files in a given directory path
    pub fn find_audio_files_in_path(path: &Path, extensions: &[String]) -> Vec<PathBuf> {
        log::debug!(
            "🔍 SCANNER DEBUG: Starting scan of path: {}",
            path.display()
        );

        let results: Vec<PathBuf> = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(entry) => {
                    let entry_path = entry.path();
                    log::debug!(
                        "🔍 SCANNER DEBUG: Examining entry: {}",
                        entry_path.display()
                    );
                    Some(entry)
                }
                Err(e) => {
                    log::warn!("Error reading directory entry: {e}");
                    None
                }
            })
            .filter(|e| e.file_type().is_file())
            .filter_map(|entry| {
                let path = entry.path();
                #[allow(clippy::option_if_let_else)]
                if let Some(ext) = path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(str::to_lowercase)
                {
                    if extensions.contains(&ext) {
                        log::debug!("🎵 SCANNER DEBUG: Found audio file: {}", path.display());
                        Some(entry.into_path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        log::debug!(
            "🔍 SCANNER DEBUG: Completed scan, found {} files",
            results.len()
        );
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

    /// Enhanced async scan operation with modern async patterns
    /// Implements the specifications from the thread pool refactoring roadmap
    pub async fn scan_async_enhanced(
        &self,
        progress_tx: mpsc::Sender<ScanProgress>,
    ) -> ScanResult<LibraryScanResult> {
        let start_time = std::time::Instant::now();
        let mut scan_result = LibraryScanResult::new();

        // Find all audio files
        let files = self.find_audio_files();
        let total_files = files.len();

        // Send initial progress
        progress_tx
            .send(ScanProgress::Started { total_files })
            .await
            .map_err(|_| ScanError::Cancelled)?;
        // Process files in parallel with backpressure
        let (result_tx, mut result_rx) = mpsc::channel(100);
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_tasks));
        let max_concurrent_tasks = self.config.max_concurrent_tasks;

        let process_task = tokio::spawn({
            let files = files.clone();
            let semaphore = semaphore.clone();
            let library_id = self.library.id.clone();
            let cancel_token = self.cancel_token.clone();
            let performance_monitor = self.performance_monitor.clone();
            let progress_tx = progress_tx.clone();

            async move {
                stream::iter(files.into_iter().enumerate())
                    .for_each_concurrent(Some(max_concurrent_tasks), |(index, path)| {
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
                            let result = Self::extract_audiobook_metadata_with_monitoring(
                                &library_id,
                                &path,
                                monitor,
                            );

                            // Send result
                            let _ = result_tx.send(result).await;

                            // Update progress with detailed information
                            let progress = (index + 1) as f32 / total_files as f32;
                            let file_name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown")
                                .to_string();

                            let _ = progress_tx
                                .send(ScanProgress::FileProcessed {
                                    current: index + 1,
                                    total: total_files,
                                    file_name,
                                    progress_percentage: progress,
                                })
                                .await;
                        }
                    })
                    .await;
            }
        });
        // Process results with batch processing
        let batch_size = self.config.batch_size;
        let process_results = async {
            let mut batch = Vec::with_capacity(batch_size);
            let mut processed_count = 0;
            let mut error_count = 0;
            let progress_tx = progress_tx.clone();

            while let Some(result) = result_rx.recv().await {
                match result {
                    Ok(audiobook) => {
                        batch.push(audiobook);
                        processed_count += 1;

                        // Process batch if full
                        if batch.len() >= batch_size {
                            if let Err(e) = self.process_batch(&batch).await {
                                tracing::error!("Failed to add batch: {}", e);
                                error_count += batch.len();
                            } else {
                                // Send batch committed progress
                                let _ = progress_tx
                                    .send(ScanProgress::BatchCommitted {
                                        count: batch.len(),
                                        total_processed: processed_count,
                                    })
                                    .await;
                            }

                            scan_result.audiobooks.append(&mut batch);
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
                    let _ = progress_tx
                        .send(ScanProgress::BatchCommitted {
                            count: batch.len(),
                            total_processed: processed_count,
                        })
                        .await;
                }

                scan_result.audiobooks.extend(batch);
            }

            scan_result.processed_count = processed_count;
            scan_result.error_count = error_count;

            Ok::<_, ScanError>(())
        };

        // Wait for both tasks to complete
        let (process_results, _) = tokio::join!(process_results, process_task);

        // Check for cancellation
        if self.cancel_token.is_cancelled() {
            let duration = start_time.elapsed();
            let _ = progress_tx
                .send(ScanProgress::Cancelled {
                    processed: scan_result.processed_count,
                    duration,
                })
                .await;
            return Err(ScanError::Cancelled);
        }

        process_results?;

        // Calculate final duration and send completion
        scan_result.scan_duration = start_time.elapsed();

        let _ = progress_tx
            .send(ScanProgress::Complete {
                processed: scan_result.processed_count,
                errors: scan_result.error_count,
                duration: scan_result.scan_duration,
            })
            .await;

        // Log performance summary if monitoring is enabled
        if let Some(monitor) = &self.performance_monitor {
            monitor.log_summary();
        }

        Ok(scan_result)
    }
    /// Creates an enhanced Task-based scan with modern async patterns and cancellation
    pub fn scan_async_task(
        &self,
        progress_tx: mpsc::Sender<ScanProgress>,
    ) -> Task<ScanResult<LibraryScanResult>> {
        let scanner = self.clone();

        Task::perform(
            async move { scanner.scan_async_enhanced(progress_tx).await },
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

    #[tokio::test]
    async fn test_scan_async() {
        let db = Database::in_memory().unwrap();
        let library = Library {
            id: "test".to_string(),
            name: "Test Library".to_string(),
            path: PathBuf::from("test_data"),
        };
        let scanner = LibraryScanner::new(db, library);
        let (tx, _rx) = mpsc::channel(100);
        let result = scanner.scan_async(Some(tx)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cancellation() {
        let db = Database::in_memory().unwrap();
        let library = Library {
            id: "test".to_string(),
            name: "Test Library".to_string(),
            path: PathBuf::from("test_data"),
        };
        let scanner = LibraryScanner::new(db, library);
        scanner.cancel_scan();
        let (tx, _rx) = mpsc::channel(100);
        let result = scanner.scan_async(Some(tx)).await;
        assert!(matches!(result, Err(ScanError::Cancelled)));
    }
}
