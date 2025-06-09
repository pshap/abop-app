//! Orchestration layer for coordinating scanning operations
//!
//! This module provides high-level coordination between core scanning,
//! database operations, progress reporting, and performance monitoring.

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Instant;
use tracing::{error, info};

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
        log::warn!("🔍 SCAN ORCHESTRATOR: Starting scan for library '{}' with path: '{}', library ID: '{}'", 
                   self.library.name, self.library.path.display(), self.library.id);
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

        let mut processed_audiobooks = Vec::new();
        let mut error_count = 0;

        // Determine batch size
        let batch_size = options.batch_size.unwrap_or(self.config.batch_size);
        info!("Processing in batches of {} files", batch_size);

        // Process files in batches
        for (batch_index, file_chunk) in files.chunks(batch_size).enumerate() {
            if self.cancelled.load(Ordering::Relaxed) {
                info!("Scan operation was cancelled");
                return Err(ScanError::Cancelled);
            }

            info!("Processing batch {}", batch_index + 1);
            let batch_start_time = Instant::now();

            // Process files sequentially within the batch (truly synchronous)
            let mut batch_audiobooks = Vec::new();
            for (file_index, path) in file_chunk.iter().enumerate() {
                let overall_index = batch_index * batch_size + file_index;

                // Report progress
                if options.enable_progress
                    && let Some(reporter) = &self.progress_reporter
                {
                    let progress = overall_index as f32 / total_files as f32;
                    let rt = tokio::runtime::Handle::try_current();
                    if let Ok(rt) = rt {
                        rt.block_on(async { reporter.report_progress(progress).await });
                    }
                }

                // Process the file using extract_metadata (now synchronous)
                match self.core_scanner.extract_metadata(&self.library.id, path) {
                    Ok(audiobook) => {
                        batch_audiobooks.push(audiobook);
                    }
                    Err(e) => {
                        error!("Error processing file {}: {}", path.display(), e);
                        error_count += 1;
                    }
                }
            }

            // Persist the batch to the database
            if !batch_audiobooks.is_empty() {
                if let Err(e) = self.persist_batch(&batch_audiobooks) {
                    error!("Error persisting batch: {}", e);
                    error_count += batch_audiobooks.len(); // Consider all items in batch as errors
                    continue;
                }
                processed_audiobooks.extend(batch_audiobooks);
            }

            info!(
                "Processed batch {} in {:.2?} ({} items)",
                batch_index + 1,
                batch_start_time.elapsed(),
                processed_audiobooks.len()
                    - (batch_index * batch_size).min(processed_audiobooks.len())
            );
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_scan_orchestrator() {
        // Test setup would go here
    }
}
