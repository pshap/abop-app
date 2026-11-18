//! Orchestration layer for coordinating scanning operations
//!
//! This module provides high-level coordination between core scanning,
//! database operations, progress reporting, and performance monitoring.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Instant;
use tracing::{error, info, warn};

use crate::{
    db::Database,
    models::{Audiobook, Library},
    scanner::{
        config::ScannerConfig,
        core_scanner::CoreScanner,
        error::{ScanError, ScanResult},
        performance::PerformanceMonitor,
        progress::ProgressReporter,
        result::ScanSummary,
    },
};

use rayon::prelude::*;

/// Options for scan operations
#[derive(Debug, Clone)]
pub struct ScanOptions {
    /// Whether to enable progress reporting
    pub enable_progress: bool,
    /// Whether to enable performance monitoring
    pub enable_monitoring: bool,
    /// Custom batch size (if None, uses config default)
    pub batch_size: Option<usize>,
    /// Maximum concurrent operations (if None, uses config default)
    pub max_concurrent: Option<usize>,
    /// Whether to use parallel processing
    pub parallel: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            enable_progress: true,
            enable_monitoring: true,
            batch_size: None,
            max_concurrent: None,
            parallel: true,
        }
    }
}

/// High-level orchestrator for scan operations
pub struct ScanOrchestrator {
    /// Core scanner for file operations
    core_scanner: CoreScanner,
    /// Database for persistence
    database: Arc<Database>,
    /// Library being scanned
    library: Library,
    /// Progress reporter
    progress_reporter: Option<Arc<dyn ProgressReporter>>,
    /// Performance monitor
    performance_monitor: Option<Arc<PerformanceMonitor>>,
    /// Cancellation token
    cancelled: Arc<AtomicBool>,
    /// Configuration
    config: ScannerConfig,
}

impl ScanOrchestrator {
    /// Creates a new scan orchestrator
    #[must_use]
    pub fn new(database: Arc<Database>, library: Library, config: ScannerConfig) -> Self {
        let core_scanner = CoreScanner::with_config(config.clone());

        Self {
            core_scanner,
            database,
            library,
            progress_reporter: None,
            performance_monitor: None,
            cancelled: Arc::new(AtomicBool::new(false)),
            config,
        }
    }

    /// Sets the progress reporter
    pub fn with_progress_reporter(mut self, reporter: Arc<dyn ProgressReporter>) -> Self {
        self.progress_reporter = Some(reporter);
        self
    }

    /// Sets the performance monitor
    #[must_use]
    pub fn with_performance_monitor(mut self, monitor: Arc<PerformanceMonitor>) -> Self {
        self.performance_monitor = Some(monitor);
        self
    }

    /// Sets the cancellation token
    pub fn with_cancellation_token(mut self, token: Arc<AtomicBool>) -> Self {
        self.cancelled = token;
        self
    }

    /// Persists a batch of audiobooks to the database
    fn persist_batch(&self, audiobooks: &[Audiobook]) -> crate::error::Result<()> {
        self.database.add_audiobooks_bulk(audiobooks)
    }

    /// Performs a complete scan operation (synchronous)
    pub fn scan(&self, options: ScanOptions) -> ScanResult<ScanSummary> {
        log::debug!(
            "SCAN ORCHESTRATOR: Starting scan for library '{}' with path: '{}', library ID: '{}'",
            self.library.name,
            self.library.path.display(),
            self.library.id
        );
        let start_time = Instant::now();

        // Discover all audio files
        let files = self.core_scanner.discover_files(&self.library.path)?;
        let total_files = files.len();

        info!("Discovered {} files to process", total_files);

        // Report scan start
        if options.enable_progress
            && let Some(reporter) = &self.progress_reporter
        {
            // For synchronous operation, we'll use a blocking approach for progress reporting
            let rt = tokio::runtime::Handle::try_current();
            if let Ok(rt) = rt {
                rt.block_on(async { reporter.report_started(total_files).await });
            }
        }

        // Books already in the database for this library, by path, so rescans
        // keep stable ids and we can prune files that no longer exist
        let repository = self.database.audiobook_repository();
        let existing: HashMap<PathBuf, Audiobook> = repository
            .find_by_library(&self.library.id)
            .map(|books| books.into_iter().map(|b| (b.path.clone(), b)).collect())
            .unwrap_or_else(|e| {
                warn!("Could not load existing audiobooks: {e}");
                HashMap::new()
            });

        let mut processed_audiobooks = Vec::new();
        let mut error_count = 0;
        let mut persist_failed = false;

        // Determine batch size
        let batch_size = options.batch_size.unwrap_or(self.config.batch_size);
        info!("Processing in batches of {} files", batch_size);

        // Process files in batches with parallel processing
        for (batch_index, file_chunk) in files.chunks(batch_size).enumerate() {
            if self.cancelled.load(Ordering::Relaxed) {
                info!("Scan operation was cancelled");
                return Err(ScanError::Cancelled);
            }

            info!(
                "Processing batch {} ({} files)",
                batch_index + 1,
                file_chunk.len()
            );
            let batch_start_time = Instant::now();

            // Clone necessary data for parallel processing
            let library_id = self.library.id.clone();

            // Process files in parallel using rayon
            let batch_results: Vec<crate::error::Result<Audiobook>> = file_chunk
                .par_iter()
                .map(|path| self.core_scanner.extract_metadata(&library_id, path))
                .collect();

            // Separate successes from errors
            let mut batch_audiobooks = Vec::new();
            for (file_index, result) in batch_results.into_iter().enumerate() {
                let overall_index = batch_index * batch_size + file_index;

                // Report progress (only when processing completes for a file)
                if options.enable_progress
                    && let Some(reporter) = &self.progress_reporter
                {
                    let progress =
                        crate::utils::casting::safe_progress(overall_index + 1, total_files)
                            .unwrap_or(1.0);
                    let rt = tokio::runtime::Handle::try_current();
                    if let Ok(rt) = rt {
                        rt.block_on(async { reporter.report_progress(progress).await });
                    }
                }

                match result {
                    Ok(mut audiobook) => {
                        if let Some(known) = existing.get(&audiobook.path) {
                            audiobook.id.clone_from(&known.id);
                            audiobook.created_at = known.created_at;
                        }
                        batch_audiobooks.push(audiobook);
                    }
                    Err(e) => {
                        // Categorize errors for better logging
                        let error_msg = e.to_string();
                        if error_msg.contains("end of stream")
                            || error_msg.contains("Failed to probe audio format")
                        {
                            warn!("Skipping file with unreadable format: {}", e);
                        } else {
                            error!("Error processing file: {}", e);
                        }
                        error_count += 1;
                    }
                }
            }

            // Persist the batch to the database
            let audiobook_count = batch_audiobooks.len();
            if !batch_audiobooks.is_empty() {
                if let Err(e) = self.persist_batch(&batch_audiobooks) {
                    error!("Error persisting batch: {}", e);
                    error_count += batch_audiobooks.len(); // Consider all items in batch as errors
                    persist_failed = true;
                    continue;
                }
                processed_audiobooks.extend(batch_audiobooks);
            }

            info!(
                "Processed batch {} in {:.2?} ({} audiobooks)",
                batch_index + 1,
                batch_start_time.elapsed(),
                audiobook_count
            );
        }

        // Forget books whose files are gone. Nothing is pruned if saving failed
        // or the library folder itself is missing (e.g. an unmounted drive).
        // A book this scan didn't see is only removed once its file is
        // confirmed missing: it may sit in a subfolder that wasn't scanned
        // (subfolders turned off) or couldn't be read (permissions, I/O error),
        // and deleting it would also delete its listening progress.
        if !persist_failed && self.library.path.is_dir() {
            let found: HashSet<&PathBuf> = files.iter().collect();
            for (path, book) in &existing {
                if !found.contains(path) && is_confirmed_missing(path) {
                    match repository.delete(&book.id) {
                        Ok(_) => info!("Removed missing audiobook {}", path.display()),
                        Err(e) => {
                            warn!("Could not remove missing audiobook {}: {e}", path.display())
                        }
                    }
                }
            }
        }

        // Report completion
        let duration = start_time.elapsed();
        if options.enable_progress
            && let Some(reporter) = &self.progress_reporter
        {
            let rt = tokio::runtime::Handle::try_current();
            if let Ok(rt) = rt {
                rt.block_on(async {
                    reporter
                        .report_complete(processed_audiobooks.len(), error_count, duration)
                        .await
                });
            }
        }

        Ok(ScanSummary {
            new_files: processed_audiobooks,
            scan_duration: duration,
            processed: total_files,
            errors: error_count,
        })
    }
}

/// Whether `path` definitely no longer exists (errors such as permission
/// denied count as "might still exist")
fn is_confirmed_missing(path: &std::path::Path) -> bool {
    matches!(path.try_exists(), Ok(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::config::ScannerConfig;

    fn scan(db: &Arc<Database>, library: &Library, recursive: bool) -> ScanSummary {
        let config = ScannerConfig {
            recursive,
            ..ScannerConfig::default()
        };
        ScanOrchestrator::new(Arc::clone(db), library.clone(), config)
            .scan(ScanOptions {
                enable_progress: false,
                enable_monitoring: false,
                ..ScanOptions::default()
            })
            .unwrap()
    }

    fn fixture(dest: &std::path::Path) {
        std::fs::create_dir_all(dest.parent().unwrap()).unwrap();
        std::fs::copy(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/tagged.mp3"),
            dest,
        )
        .unwrap();
    }

    #[test]
    fn prunes_only_books_whose_files_are_gone() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("library");
        let top = root.join("top.mp3");
        let nested = root.join("Author/nested.mp3");
        let doomed = root.join("Author/deleted.mp3");
        for path in [&top, &nested, &doomed] {
            fixture(path);
        }
        let db = Arc::new(Database::open(dir.path().join("test.db")).unwrap());
        let mut library = Library::new("lib", &root);
        library.id = db.add_library(&library).unwrap();

        assert_eq!(scan(&db, &library, true).new_files.len(), 3);
        let count = || {
            db.audiobook_repository()
                .find_by_library(&library.id)
                .unwrap()
                .len()
        };

        // Subfolders off: nested books aren't seen but still exist, so they stay
        scan(&db, &library, false);
        assert_eq!(count(), 3);

        // A file that really was deleted is forgotten
        std::fs::remove_file(&doomed).unwrap();
        scan(&db, &library, true);
        assert_eq!(count(), 2);
    }

    #[test]
    fn missing_means_confirmed_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(is_confirmed_missing(&dir.path().join("nope.mp3")));
        assert!(!is_confirmed_missing(dir.path()));
    }
}
