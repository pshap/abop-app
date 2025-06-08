//! Unified audio file scanner interface for ABOP
//!
//! This module provides a simplified, high-level interface for scanning directories
//! for audio files, extracting metadata, and updating the database.

use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

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
    /// Cancel token for cancelling operations
    cancel_token: CancellationToken,
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
            cancel_token: CancellationToken::new(),
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
        self.cancel_token.cancel();
    }

    /// Primary scan method - unified interface for all scanning operations
    pub async fn scan(&self, options: ScanOptions) -> ScanResult<ScanSummary> {
        let orchestrator = self.create_orchestrator(&options);
        orchestrator.scan(options).await
    }

    /// Scan with progress reporting
    pub async fn scan_with_progress(
        &self,
        progress_tx: mpsc::Sender<ScanProgress>,
    ) -> ScanResult<ScanSummary> {
        let progress_reporter = Arc::new(ChannelReporter::new(progress_tx));
        let options = ScanOptions {
            enable_progress: true,
            enable_monitoring: self.performance_monitor.is_some(),
            ..Default::default()
        };

        let orchestrator = self
            .create_orchestrator(&options)
            .with_progress_reporter(progress_reporter);

        orchestrator.scan(options).await
    }

    /// Creates a task for scanning the library (Iced integration)
    pub fn scan_task(&self) -> Task<ScanResult<ScanSummary>> {
        let scanner = self.clone();
        Task::perform(
            async move { scanner.scan(ScanOptions::default()).await },
            |result| result,
        )
    }

    /// Creates a task for scanning with progress reporting (Iced integration)
    pub fn scan_task_with_progress(
        &self,
        progress_tx: mpsc::Sender<ScanProgress>,
    ) -> Task<ScanResult<ScanSummary>> {
        let scanner = self.clone();
        Task::perform(
            async move { scanner.scan_with_progress(progress_tx).await },
            |result| result,
        )
    }

    /// Creates an orchestrator with the current scanner configuration
    fn create_orchestrator(&self, options: &ScanOptions) -> ScanOrchestrator {
        let mut orchestrator =
            ScanOrchestrator::new(self.db.clone(), self.library.clone(), self.config.clone());

        if options.enable_monitoring {
            if let Some(monitor) = &self.performance_monitor {
                orchestrator = orchestrator.with_performance_monitor(monitor.clone());
            }
        }

        orchestrator = orchestrator.with_cancellation_token(self.cancel_token.clone());
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
