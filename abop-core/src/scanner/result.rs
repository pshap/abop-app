//! Scan result types

use crate::models::Audiobook;
use std::time::Duration;

/// Summary of a library scan operation
#[derive(Debug, Clone)]
pub struct ScanSummary {
    /// New audiobooks discovered during the scan
    pub new_files: Vec<Audiobook>,
    /// Duration of the scan operation
    pub scan_duration: Duration,
}

impl ScanSummary {
    /// Create a new empty scan summary
    #[must_use]
    pub const fn new() -> Self {
        Self {
            new_files: Vec::new(),
            scan_duration: Duration::new(0, 0),
        }
    }
}

impl Default for ScanSummary {
    fn default() -> Self {
        Self::new()
    }
}
