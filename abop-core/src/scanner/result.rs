//! Result types for scanner operations

use std::time::{Duration, Instant};
use crate::models::Audiobook;

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
    pub fn new() -> Self {
        Self {
            processed: 0,
            errors: 0,
            duration: Duration::default(),
            new_files: Vec::new(),
            updated_files: Vec::new(),
        }
    }
    
    pub fn total_files(&self) -> usize {
        self.processed + self.errors
    }
}

impl Default for ScanSummary {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal result structure for scan operations
#[derive(Debug)]
pub(crate) struct InternalScanResult {
    pub processed_count: usize,
    pub error_count: usize,
    pub scan_duration: Duration,
    pub start_time: Instant,
}

impl InternalScanResult {
    pub fn new() -> Self {
        Self {
            processed_count: 0,
            error_count: 0,
            scan_duration: Duration::default(),
            start_time: Instant::now(),
        }
    }
    
    pub fn into_summary(self, new_files: Vec<Audiobook>, updated_files: Vec<Audiobook>) -> ScanSummary {
        ScanSummary {
            processed: self.processed_count,
            errors: self.error_count,
            duration: self.scan_duration,
            new_files,
            updated_files,
        }
    }
}
