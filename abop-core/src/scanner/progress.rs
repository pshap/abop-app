//! Progress reporting system for scanner operations

use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::info;

use crate::scanner::error::ScanError;

/// Events emitted during scanning to report progress
#[derive(Debug, Clone, Default)]
pub struct ScanProgress {
    pub files_processed: usize,
    pub total_files: usize,
    pub current_file: Option<PathBuf>,
    pub errors: Vec<ScanError>,
}

/// Trait for types that can receive scan progress updates
pub trait ProgressReporter {
    fn report_progress(&self, progress: &ScanProgress);
}

/// Implementation that sends progress over a channel
pub struct ChannelReporter {
    tx: mpsc::Sender<ScanProgress>,
}

impl ChannelReporter {
    pub fn new(tx: mpsc::Sender<ScanProgress>) -> Self {
        Self { tx }
    }
}

impl ProgressReporter for ChannelReporter {
    fn report_progress(&self, progress: &ScanProgress) {
        if let Err(e) = self.tx.try_send(progress.clone()) {
            info!("Failed to send progress update: {}", e);
        }
    }
}

/// Implementation that logs progress updates
pub struct LoggingReporter;

impl ProgressReporter for LoggingReporter {
    fn report_progress(&self, progress: &ScanProgress) {
        if let Some(file) = &progress.current_file {
            info!(
                "Processing file {}/{}: {}",
                progress.files_processed, progress.total_files, file.display()
            );
        }
        for error in &progress.errors {
            info!("Scan error: {}", error);
        }
    }
}
