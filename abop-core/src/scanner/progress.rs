//! Progress reporting system for scanner operations

use std::time::Duration;

/// Events emitted during scanning to report progress
#[derive(Debug, Clone)]
pub enum ScanProgress {
    /// Scan has started with total number of files to process
    Started {
        /// Total number of files that will be processed in this scan operation
        total_files: usize,
    },

    /// A file has been processed
    FileProcessed {
        /// Current file number being processed
        current: usize,
        /// Total number of files to process
        total: usize,
        /// Name of the file being processed
        file_name: String,
        /// Progress percentage (0.0 to 1.0)
        progress_percentage: f32,
    },

    /// A batch of files has been committed to the database
    BatchCommitted {
        /// Number of items in this batch
        count: usize,
        /// Total processed so far
        total_processed: usize,
    },

    /// Scan has completed
    Complete {
        /// Number of files successfully processed
        processed: usize,
        /// Number of errors encountered
        errors: usize,
        /// Total duration of the scan
        duration: Duration,
    },

    /// Scan was cancelled
    Cancelled {
        /// Number of files processed before cancellation
        processed: usize,
        /// Duration before cancellation
        duration: Duration,
    },
}

/// Trait for types that can receive scan progress updates
#[async_trait::async_trait]
pub trait ProgressReporter: Send + Sync + 'static {
    /// Called when a progress event occurs
    async fn report(
        &self,
        progress: ScanProgress,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Implementation that sends progress over a channel
pub struct ChannelReporter<T> {
    /// Sender channel for progress updates
    sender: tokio::sync::mpsc::Sender<T>,
    /// Phantom data to hold the type parameter
    _phantom: std::marker::PhantomData<T>,
}

impl<T: From<ScanProgress> + Send + Sync + 'static> ChannelReporter<T> {
    /// Creates a new channel reporter that will send progress updates over the given channel
    ///
    /// # Arguments
    ///
    /// * `sender` - A channel sender that can send messages of type T, where T implements From<ScanProgress>
    ///
    /// # Returns
    ///
    /// A new ChannelReporter instance that will convert ScanProgress events into type T
    /// and send them over the provided channel
    pub fn new(sender: tokio::sync::mpsc::Sender<T>) -> Self {
        Self {
            sender,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T: From<ScanProgress> + Send + Sync + 'static> ProgressReporter for ChannelReporter<T> {
    async fn report(
        &self,
        progress: ScanProgress,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.sender.send(progress.into()).await?;
        Ok(())
    }
}
