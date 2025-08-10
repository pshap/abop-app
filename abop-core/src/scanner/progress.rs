//! Progress reporting for scanner operations

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Represents the progress of a scanning operation
#[derive(Debug, Clone)]
pub enum ScanProgress {
    /// Scanning has started
    Started {
        /// Total number of files to process
        total_files: usize,
    },
    /// A file has been processed
    FileProcessed {
        /// Current file number (1-based)
        current: usize,
        /// Total number of files
        total: usize,
        /// Name of the file being processed
        file_name: String,
        /// Progress percentage (0.0 to 1.0)
        progress_percentage: f32,
    },
    /// A batch of files has been committed to the database
    BatchCommitted {
        /// Number of files in this batch
        count: usize,
        /// Total number of files processed so far
        total_processed: usize,
    },
    /// Scanning has completed
    Complete {
        /// Number of files successfully processed
        processed: usize,
        /// Number of files that had errors
        errors: usize,
        /// Total duration of the scan
        duration: std::time::Duration,
    },
    /// Scanning was cancelled
    Cancelled {
        /// Number of files processed before cancellation
        processed: usize,
        /// Duration until cancellation
        duration: std::time::Duration,
    },
}

/// Trait for reporting scan progress
#[async_trait]
pub trait ProgressReporter: Send + Sync {
    /// Report that scanning has started
    async fn report_started(&self, total_files: usize);

    /// Report that a file has been processed
    async fn report_file_processed(&self, current: usize, total: usize, file_name: String);

    /// Report scanning progress as a percentage
    async fn report_progress(&self, progress: f32);

    /// Report that scanning has completed
    async fn report_complete(&self, processed: usize, errors: usize, duration: std::time::Duration);
}

/// Progress reporter that sends updates through a channel
#[derive(Debug, Clone)]
pub struct ChannelReporter {
    /// Channel sender for progress updates
    tx: mpsc::Sender<ScanProgress>,
}

impl ChannelReporter {
    /// Create a new channel-based reporter
    #[must_use]
    pub const fn new(tx: mpsc::Sender<ScanProgress>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl ProgressReporter for ChannelReporter {
    async fn report_started(&self, total_files: usize) {
        let _ = self.tx.send(ScanProgress::Started { total_files }).await;
    }

    async fn report_file_processed(&self, current: usize, total: usize, file_name: String) {
        let progress = if total > 0 {
            current as f32 / total as f32
        } else {
            0.0
        };
        let _ = self
            .tx
            .send(ScanProgress::FileProcessed {
                current,
                total,
                file_name,
                progress_percentage: progress,
            })
            .await;
    }

    async fn report_progress(&self, progress: f32) {
        // For now, just ignore individual progress reports since we track by file count
        // Could be used to send a custom progress variant if needed
        let _ = progress; // Silence unused parameter warning
    }

    async fn report_complete(
        &self,
        processed: usize,
        errors: usize,
        duration: std::time::Duration,
    ) {
        let _ = self
            .tx
            .send(ScanProgress::Complete {
                processed,
                errors,
                duration,
            })
            .await;
    }
}

/// Progress reporter that calls a callback function
#[derive(Debug)]
pub struct CallbackReporter<F> {
    /// Callback function for progress updates
    callback: Arc<F>,
}

impl<F> CallbackReporter<F>
where
    F: Fn(f32) + Send + Sync + 'static,
{
    /// Create a new callback-based reporter
    #[must_use]
    pub fn new(callback: F) -> Self {
        Self {
            callback: Arc::new(callback),
        }
    }
}

#[async_trait]
impl<F> ProgressReporter for CallbackReporter<F>
where
    F: Fn(f32) + Send + Sync + 'static,
{
    async fn report_started(&self, _total_files: usize) {
        (self.callback)(0.0);
    }

    async fn report_file_processed(&self, current: usize, total: usize, _file_name: String) {
        let progress = current as f32 / total as f32;
        (self.callback)(progress);
    }

    async fn report_progress(&self, progress: f32) {
        (self.callback)(progress);
    }

    async fn report_complete(
        &self,
        _processed: usize,
        _errors: usize,
        _duration: std::time::Duration,
    ) {
        (self.callback)(1.0);
    }
}

/// Test reporter that collects progress updates
#[derive(Debug, Clone, Default)]
pub struct TestReporter {
    /// Collected progress updates
    updates: Arc<tokio::sync::Mutex<Vec<ScanProgress>>>,
}

#[async_trait]
impl ProgressReporter for TestReporter {
    async fn report_started(&self, total_files: usize) {
        let mut updates = self.updates.lock().await;
        updates.push(ScanProgress::Started { total_files });
    }

    async fn report_file_processed(&self, current: usize, total: usize, file_name: String) {
        let mut updates = self.updates.lock().await;
        updates.push(ScanProgress::FileProcessed {
            current,
            total,
            file_name,
            progress_percentage: 0.0,
        });
    }

    async fn report_progress(&self, _progress: f32) {
        // For test reporter, we could store progress updates if needed
        // For now, just ignore them
    }

    async fn report_complete(
        &self,
        processed: usize,
        errors: usize,
        duration: std::time::Duration,
    ) {
        let mut updates = self.updates.lock().await;
        updates.push(ScanProgress::Complete {
            processed,
            errors,
            duration,
        });
    }
}

impl TestReporter {
    /// Get the collected progress updates
    pub async fn get_updates(&self) -> Vec<ScanProgress> {
        self.updates.lock().await.clone()
    }

    /// Clear collected updates
    pub async fn clear(&self) {
        self.updates.lock().await.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channel_reporter() {
        let (tx, mut rx) = mpsc::channel(10);
        let reporter = ChannelReporter::new(tx);

        // Test started
        reporter.report_started(10).await;
        assert!(matches!(
            rx.recv().await,
            Some(ScanProgress::Started { total_files: 10 })
        ));

        // Test file processed
        reporter
            .report_file_processed(1, 10, "test.mp3".to_string())
            .await;
        assert!(matches!(
            rx.recv().await,
            Some(ScanProgress::FileProcessed {
                current: 1,
                total: 10,
                file_name: _,
                progress_percentage: _,
            })
        ));

        // Test complete
        reporter
            .report_complete(5, 1, std::time::Duration::from_secs(1))
            .await;
        assert!(matches!(
            rx.recv().await,
            Some(ScanProgress::Complete {
                processed: 5,
                errors: 1,
                duration: _,
            })
        ));
    }

    #[tokio::test]
    async fn test_callback_reporter() {
        use std::sync::{Arc, Mutex};
        let last_progress = Arc::new(Mutex::new(0.0));
        let last_progress_clone = Arc::clone(&last_progress);
        let callback = move |progress: f32| {
            *last_progress_clone.lock().unwrap() = progress;
        };
        let reporter = CallbackReporter::new(callback);

        // Test started
        reporter.report_started(10).await;
        assert_eq!(*last_progress.lock().unwrap(), 0.0);

        // Test file processed
        reporter
            .report_file_processed(5, 10, "test.mp3".to_string())
            .await;
        assert_eq!(*last_progress.lock().unwrap(), 0.5);

        // Test complete
        reporter
            .report_complete(10, 0, std::time::Duration::from_secs(1))
            .await;
        assert_eq!(*last_progress.lock().unwrap(), 1.0);
    }

    #[tokio::test]
    async fn test_test_reporter() {
        let reporter = TestReporter::default();

        // Test started
        reporter.report_started(10).await;
        assert!(matches!(
            reporter.get_updates().await[0],
            ScanProgress::Started { total_files: 10 }
        ));

        // Test file processed
        reporter
            .report_file_processed(1, 10, "test.mp3".to_string())
            .await;
        assert!(matches!(
            reporter.get_updates().await[1],
            ScanProgress::FileProcessed {
                current: 1,
                total: 10,
                file_name: _,
                progress_percentage: _,
            }
        ));

        // Test complete
        reporter
            .report_complete(5, 1, std::time::Duration::from_secs(1))
            .await;
        assert!(matches!(
            reporter.get_updates().await[2],
            ScanProgress::Complete {
                processed: 5,
                errors: 1,
                duration: _,
            }
        ));

        // Test clear
        reporter.clear().await;
        assert!(reporter.get_updates().await.is_empty());
    }
}
