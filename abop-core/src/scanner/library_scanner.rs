//! Unified audio file scanner interface for ABOP
//!
//! This module provides a simplified, high-level interface for scanning directories
//! for audio files, extracting metadata, and updating the database.

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::{
    db::Database,
    models::Library,
    scanner::{
        config::ScannerConfig,
        error::ScanResult,
        orchestrator::{ScanOptions, ScanOrchestrator},
        performance::PerformanceMonitor,
        progress::{ChannelReporter, ScanProgress},
        result::ScanSummary,
    },
};
use iced::Task;

/// Supported audio file extensions for scanning
pub const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["mp3", "m4a", "m4b", "flac", "ogg", "wav", "aac"];

/// Simplified, high-level interface for library scanning operations
#[derive(Clone)]
pub struct LibraryScanner {
    /// The database connection
    db: Arc<Database>,
    /// The library being scanned
    library: Library,
    /// Configuration for scanning operations
    config: ScannerConfig,
    /// Performance monitor for tracking operation times
    performance_monitor: Option<Arc<PerformanceMonitor>>,
    /// Flag to indicate if the scan should be cancelled
    cancelled: Arc<AtomicBool>,
}

impl LibraryScanner {
    /// Creates a new `LibraryScanner` for the given library
    #[must_use]
    pub fn new(db: Database, library: Library) -> Self {
        Self {
            db: Arc::new(db),
            library,
            config: ScannerConfig::default(),
            performance_monitor: Some(Arc::new(PerformanceMonitor::new())),
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Creates a new scanner with custom configuration
    #[must_use]
    pub fn with_config(mut self, config: ScannerConfig) -> Self {
        self.config = config;
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

    /// Cancels any ongoing scan operations
    pub fn cancel_scan(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// Primary scan method - unified interface for all scanning operations
    pub fn scan(&self, options: ScanOptions) -> ScanResult<ScanSummary> {
        let orchestrator = self.create_orchestrator(&options);
        // Direct call to synchronous orchestrator
        orchestrator.scan(options)
    }

    /// Scan with progress reporting
    pub fn scan_with_progress(
        &self,
        progress_tx: std::sync::mpsc::Sender<ScanProgress>,
    ) -> ScanResult<ScanSummary> {
        // Convert std::sync::mpsc to tokio::sync::mpsc for the orchestrator
        let (tokio_tx, mut tokio_rx) = tokio::sync::mpsc::channel(100);
        let progress_reporter = Arc::new(ChannelReporter::new(tokio_tx));

        let options = ScanOptions {
            enable_progress: true,
            enable_monitoring: self.performance_monitor.is_some(),
            batch_size: None,     // Use config default
            max_concurrent: None, // Use config default
            parallel: true,       // Enable parallel processing
        };

        let orchestrator = self
            .create_orchestrator(&options)
            .with_progress_reporter(progress_reporter);

        // Forward progress updates from tokio channel to std channel
        let std_tx = progress_tx.clone();
        tokio::spawn(async move {
            while let Some(progress) = tokio_rx.recv().await {
                if std_tx.send(progress).is_err() {
                    break; // Receiver dropped
                }
            }
        });

        orchestrator.scan(options)
    }

    /// Performs a synchronous scan of the library
    pub fn scan_sync(&self) -> ScanResult<ScanSummary> {
        self.scan(ScanOptions::default())
    }

    /// Performs a synchronous scan with progress reporting
    pub fn scan_with_progress_sync(
        &self,
        progress_tx: std::sync::mpsc::Sender<ScanProgress>,
    ) -> ScanResult<ScanSummary> {
        self.scan_with_progress(progress_tx)
    }

    /// Creates a task for scanning the library (Iced integration)
    /// This runs the scan in a background task to prevent UI freezing
    pub fn scan_task(&self) -> Task<ScanResult<ScanSummary>> {
        let scanner = self.clone();

        Task::perform(
            async move {
                // Run CPU-bound work in a blocking task
                tokio::task::spawn_blocking(move || scanner.scan_sync())
                    .await
                    .map_err(|e| {
                        crate::error::AppError::Threading(format!("Scan task panicked: {e}"))
                    })?
            },
            |result| result,
        )
    }

    /// Creates a task for scanning with progress reporting (Iced integration)
    /// This runs the scan in a background task with progress updates to prevent UI freezing
    pub fn scan_task_with_progress(
        &self,
        progress_tx: std::sync::mpsc::Sender<ScanProgress>,
    ) -> Task<ScanResult<ScanSummary>> {
        let scanner = self.clone();

        Task::perform(
            async move {
                // Run CPU-bound work in a blocking task
                tokio::task::spawn_blocking(move || scanner.scan_with_progress_sync(progress_tx))
                    .await
                    .map_err(|e| {
                        crate::error::AppError::Threading(format!("Scan task panicked: {e}"))
                    })?
            },
            |result| result,
        )
    }

    /// Performs an async scan with progress reporting
    /// This version can be awaited and runs the scan in background
    pub async fn scan_async(&self, options: ScanOptions) -> ScanResult<ScanSummary> {
        let scanner = self.clone();

        tokio::task::spawn_blocking(move || scanner.scan(options))
            .await
            .map_err(|e| crate::error::AppError::Threading(format!("Scan task panicked: {e}")))?
    }

    /// Performs an async scan with progress reporting via channel
    pub async fn scan_with_progress_async(
        &self,
        options: ScanOptions,
        progress_tx: tokio::sync::mpsc::Sender<ScanProgress>,
    ) -> ScanResult<ScanSummary> {
        let scanner = self.clone();

        tokio::task::spawn_blocking(move || {
            let progress_reporter = Arc::new(ChannelReporter::new(progress_tx));
            let scan_options = ScanOptions {
                enable_progress: true,
                enable_monitoring: scanner.performance_monitor.is_some(),
                batch_size: options.batch_size,
                max_concurrent: options.max_concurrent,
                parallel: options.parallel,
            };

            let orchestrator = scanner
                .create_orchestrator(&scan_options)
                .with_progress_reporter(progress_reporter);

            orchestrator.scan(scan_options)
        })
        .await
        .map_err(|e| crate::error::AppError::Threading(format!("Scan task panicked: {e}")))?
    }

    /// Creates an orchestrator with the current scanner configuration
    fn create_orchestrator(&self, options: &ScanOptions) -> ScanOrchestrator {
        let mut orchestrator =
            ScanOrchestrator::new(self.db.clone(), self.library.clone(), self.config.clone());

        if options.enable_monitoring
            && let Some(monitor) = &self.performance_monitor
        {
            orchestrator = orchestrator.with_performance_monitor(monitor.clone());
        }

        orchestrator = orchestrator.with_cancellation_token(self.cancelled.clone());
        orchestrator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    #[tokio::test]
    async fn test_scanner_creation() {
        let temp_dir = tempdir().unwrap();
        let _library = Library {
            id: "test".to_string(),
            name: "Test Library".to_string(),
            path: temp_dir.path().to_path_buf(),
        };

        // This would need a proper database for full testing
        // let scanner = LibraryScanner::new(db, library);
        // assert!(scanner.get_performance_monitor().is_some());
    }

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
    }
}
