//! Result types for scanner operations

use crate::models::Audiobook;
use std::time::Duration;

/// Summary of a scan operation
#[derive(Debug, Clone)]
pub struct ScanSummary {
    /// Number of files successfully processed
    pub processed: usize,
    /// Number of errors encountered
    pub errors: usize,
    /// Total duration of the scan
    pub duration: Duration,
    /// New audiobooks discovered
    pub new_files: Vec<Audiobook>,
    /// Updated audiobooks
    pub updated_files: Vec<Audiobook>,
}

impl ScanSummary {
    /// Creates a new empty scan summary
    ///
    /// # Returns
    ///
    /// A new ScanSummary instance with all counters initialized to zero
    /// and empty vectors for new and updated files
    pub fn new() -> Self {
        Self {
            processed: 0,
            errors: 0,
            duration: Duration::default(),
            new_files: Vec::new(),
            updated_files: Vec::new(),
        }
    }

    /// Gets the total number of files that were processed during the scan
    ///
    /// This includes both successfully processed files and files that encountered errors
    ///
    /// # Returns
    ///
    /// The sum of processed and error counts
    pub fn total_files(&self) -> usize {
        self.processed + self.errors
    }
}

impl Default for ScanSummary {
    fn default() -> Self {
        Self::new()
    }
}
