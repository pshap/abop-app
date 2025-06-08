//! Unified audio file scanner interface for ABOP
//!
//! This module provides a simplified, high-level interface for scanning directories
//! for audio files, extracting metadata, and updating the database.

use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};

use crate::{
    db::Database,
    models::Library,
    scanner::{
        config::ScannerConfig,
        error::{ScanError, ScanResult},
        orchestrator::{ScanOptions, ScanOrchestrator},
        performance::PerformanceMonitor,
        progress::{ChannelReporter, ProgressReporter, ScanProgress},
        result::ScanSummary,
    },
};
use iced::Task;

/// Supported audio file extensions for scanning
pub const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["mp3", "m4a", "m4b", "flac", "ogg", "wav", "aac"];

/// Progress updates for scanning operations (legacy compatibility)
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
        let orchestrator = self.create_orchestrator(options);
        orchestrator.scan(ScanOptions::default()).await
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
        
        let orchestrator = self.create_orchestrator(options)
            .with_progress_reporter(progress_reporter);
        
        orchestrator.scan(ScanOptions::default()).await
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
    fn create_orchestrator(&self, options: ScanOptions) -> ScanOrchestrator {
        let mut orchestrator = ScanOrchestrator::new(
            self.db.clone(),
            self.library.clone(),
            self.config.clone(),
        );

        if options.enable_monitoring {
            if let Some(monitor) = &self.performance_monitor {
                orchestrator = orchestrator.with_performance_monitor(monitor.clone());
            }
        }

        orchestrator = orchestrator.with_cancellation_token(self.cancel_token.clone());
        orchestrator
    }

    // Legacy compatibility methods - deprecated but maintained for backwards compatibility
    
    /// Legacy async scan method - use scan() instead
    #[deprecated(note = "Use scan() method instead")]
    pub async fn scan_async(
        &self,
        progress_tx: Option<mpsc::Sender<ScanProgress>>,
    ) -> ScanResult<ScanSummary> {
        match progress_tx {
            Some(tx) => self.scan_with_progress(tx).await,
            None => self.scan(ScanOptions::default()).await,
        }
    }

    /// Legacy enhanced scan method - use scan_with_progress() instead
    #[deprecated(note = "Use scan_with_progress() method instead")]
    pub async fn scan_async_enhanced(
        &self,
        progress_tx: mpsc::Sender<ScanProgress>,
    ) -> ScanResult<LibraryScanResult> {
        let summary = self.scan_with_progress(progress_tx).await?;
        
        // Convert ScanSummary to LibraryScanResult for compatibility
        Ok(LibraryScanResult {
            processed_count: summary.processed,
            error_count: summary.errors,
            audiobooks: summary.new_files,
            scan_duration: summary.scan_duration,
        })
    }

    /// Legacy task creation method - use scan_task() instead
    #[deprecated(note = "Use scan_task_with_progress() method instead")]
    pub fn scan_with_task(
        &self,
        progress_tx: mpsc::Sender<ScanProgress>,
    ) -> Task<ScanResult<ScanSummary>> {
        self.scan_task_with_progress(progress_tx)
    }

    /// Legacy task creation method - use scan_task_with_progress() instead
    #[deprecated(note = "Use scan_task_with_progress() method instead")]
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
