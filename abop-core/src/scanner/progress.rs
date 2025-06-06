//! Progress reporting system for scanner operations

use std::time::Duration;
use async_trait::async_trait;

/// Events emitted during scanning to report progress
#[derive(Debug, Clone)]
pub enum ScanProgress {
    /// Scan has started with total number of files to process
    Started {
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
#[async_trait]
pub trait ProgressReporter: Send + Sync + 'static {
    /// Called when a progress event occurs
    async fn report(&self, progress: ScanProgress) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Implementation that sends progress over a channel
pub struct ChannelReporter<T> {
    sender: tokio::sync::mpsc::Sender<T>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: From<ScanProgress> + Send + Sync + 'static> ChannelReporter<T> {
    pub fn new(sender: tokio::sync::mpsc::Sender<T>) -> Self {
        Self {
            sender,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<T: From<ScanProgress> + Send + Sync + 'static> ProgressReporter for ChannelReporter<T> {
    async fn report(&self, progress: ScanProgress) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.sender.send(progress.into()).await?;
        Ok(())
    }
}

/// Implementation that logs progress updates
pub struct LoggingReporter {
    level: log::Level,
}

impl LoggingReporter {
    pub fn new(level: log::Level) -> Self {
        Self { level }
    }
}

#[async_trait]
impl ProgressReporter for LoggingReporter {
    async fn report(&self, progress: ScanProgress) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match progress {
            ScanProgress::Started { total_files } => {
                log::log!(self.level, "Starting scan of {} files", total_files);
            }
            ScanProgress::FileProcessed { current, total, file_name, progress_percentage } => {
                log::log!(
                    self.level,
                    "Processed {}/{} files ({}%) - {}",
                    current,
                    total,
                    (progress_percentage * 100.0) as u32,
                    file_name
                );
            }
            ScanProgress::BatchCommitted { count, total_processed } => {
                log::log!(
                    self.level,
                    "Committed batch of {} files (total: {})",
                    count,
                    total_processed
                );
            }
            ScanProgress::Complete { processed, errors, duration } => {
                log::log!(
                    self.level,
                    "Scan completed: {} files processed, {} errors, duration: {:?}",
                    processed,
                    errors,
                    duration
                );
            }
            ScanProgress::Cancelled { processed, duration } => {
                log::log!(
                    self.level,
                    "Scan cancelled after processing {} files in {:?}",
                    processed,
                    duration
                );
            }
        }
        Ok(())
    }
}
