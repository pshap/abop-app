//! Task management for scanner operations
//!
//! This module provides functionality for managing async scanning tasks,
//! including progress reporting, cancellation, and concurrency control.

use std::future::Future;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::{
    error::Result,
    scanner::{
        error::ScanError,
        progress::{ChannelReporter, ProgressReporter, ScanProgress},
        result::ScanSummary,
    },
};

/// Manages scanning tasks and their lifecycle
#[derive(Debug, Clone)]
pub struct TaskManager {
    /// Maximum number of concurrent file operations
    max_concurrent_tasks: usize,
    /// Maximum number of concurrent database operations
    max_concurrent_db_operations: usize,
    /// Cancellation token for async operations
    cancel_token: CancellationToken,
}

impl TaskManager {
    /// Create a new task manager with default settings
    pub fn new() -> Self {
        Self {
            max_concurrent_tasks: 4,
            max_concurrent_db_operations: 2,
            cancel_token: CancellationToken::new(),
        }
    }

    /// Create a new task manager with custom settings
    pub fn with_settings(max_concurrent_tasks: usize, max_concurrent_db_operations: usize) -> Self {
        Self {
            max_concurrent_tasks,
            max_concurrent_db_operations,
            cancel_token: CancellationToken::new(),
        }
    }

    /// Cancel all ongoing tasks
    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    /// Check if tasks have been cancelled
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancel_token.is_cancelled()
    }

    /// Get a new cancellation token
    #[must_use]
    pub fn get_cancel_token(&self) -> CancellationToken {
        self.cancel_token.clone()
    }

    /// Run a scan operation with the given reporter
    pub async fn run_scan<F, Fut>(
        &self,
        scan_fn: F,
        reporter: Arc<dyn ProgressReporter>,
    ) -> Result<ScanSummary>
    where
        F: FnOnce(Arc<dyn ProgressReporter>) -> Fut,
        Fut: Future<Output = Result<ScanSummary>>,
    {
        if self.is_cancelled() {
            return Err(ScanError::Cancelled.into());
        }

        let start_time = std::time::Instant::now();
        let result = scan_fn(reporter).await;

        match result {
            Ok(summary) => {
                let duration = start_time.elapsed();
                let processed = summary.new_files.len();
                Ok(ScanSummary {
                    new_files: summary.new_files,
                    scan_duration: duration,
                    processed,
                    errors: 0, // No errors if we reached this point
                })
            }
            Err(e) => {
                if self.is_cancelled() {
                    Err(ScanError::Cancelled.into())
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Run a scan operation with channel-based progress reporting
    pub async fn run_scan_with_channel<F, Fut>(
        &self,
        scan_fn: F,
        progress_tx: tokio::sync::mpsc::Sender<ScanProgress>,
    ) -> Result<ScanSummary>
    where
        F: FnOnce(Arc<dyn ProgressReporter>) -> Fut,
        Fut: Future<Output = Result<ScanSummary>>,
    {
        let reporter = Arc::new(ChannelReporter::new(progress_tx));
        self.run_scan(scan_fn, reporter).await
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{
        file_discovery::{DefaultFileDiscoverer, FileDiscoverer}, 
        file_processor::{DefaultFileProcessor, FileProcessor},
        progress::TestReporter,
    };
    use crate::db::Database;
    use crate::scanner::Library;
    use log::debug;

    #[tokio::test]
    async fn test_task_manager_cancellation() {
        let manager = TaskManager::new();
        assert!(!manager.is_cancelled());

        manager.cancel();
        assert!(manager.is_cancelled());
    }

    #[tokio::test]
    async fn test_run_scan() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let test_file = temp_dir.path().join("test.mp3");
        std::fs::write(&test_file, b"fake mp3 data")?;

        let library = Library {
            id: "test".to_string(),
            name: "Test Library".to_string(),
            path: temp_dir.path().to_path_buf(),
        };

        let task_manager = TaskManager::with_settings(2, 1);
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        let scan_fn = |reporter: Arc<dyn ProgressReporter>| async move {
            let discoverer = DefaultFileDiscoverer::with_default_extensions();
            let db = Database::open(":memory:")?;
            let processor = DefaultFileProcessor::new(db);
            let audio_files = discoverer.discover_files(&library.path).await?;

            if audio_files.is_empty() {
                reporter
                    .report_complete(0, 0, std::time::Duration::from_secs(0))
                    .await;
                return Ok(ScanSummary::new());
            }

            let total_files = audio_files.len();
            reporter.report_started(total_files).await;

            let mut audiobooks = Vec::new();
            let mut error_count = 0;
            let start_time = std::time::Instant::now();

            for (index, path) in audio_files.into_iter().enumerate() {
                match processor.process_file(path).await {
                    Ok(audiobook) => audiobooks.push(audiobook),
                    Err(e) => {
                        debug!("Error processing file: {}", e);
                        error_count += 1;
                    }
                }

                reporter
                    .report_file_processed(
                        index + 1,
                        total_files,
                        path.to_string_lossy().into_owned(),
                    )
                    .await;
            }

            let duration = start_time.elapsed();
            reporter
                .report_complete(audiobooks.len(), error_count, duration)
                .await;
            Ok(ScanSummary {
                new_files: audiobooks,
                scan_duration: duration,
                processed: audiobooks.len(),
                errors: error_count,
            })
        };

        let handle =
            tokio::spawn(async move { task_manager.run_scan_with_channel(scan_fn, tx).await });

        // Verify progress updates
        assert!(matches!(
            rx.recv().await,
            Some(ScanProgress::Started { total_files: 1 })
        ));
        assert!(matches!(
            rx.recv().await,
            Some(ScanProgress::FileProcessed {
                current: 1,
                total: 1,
                file_name: _,
                progress_percentage: _,
            })
        ));
        assert!(matches!(
            rx.recv().await,
            Some(ScanProgress::Complete {
                processed: 0,
                errors: 1,
                duration: _,
            })
        ));

        let result = handle.await??;
        assert_eq!(result.new_files.len(), 0);
        assert_eq!(result.scan_duration.as_secs(), 0);

        Ok(())
    }
}
