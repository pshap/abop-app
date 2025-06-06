//! Scanner module for ABOP
//!
//! This module provides functionality to scan directories for audio files,
//! extract metadata, and update the database with the found files.
//!
//! The module supports both traditional rayon-based parallel processing and
//! a modern async/await implementation using Tokio and Iced's Task system
//! for fine-grained control over scanning operations.

pub mod config;
pub mod error;
pub mod library_scanner;
pub mod performance;
pub mod progress;
pub mod result;
pub mod state;
pub mod thread_pool;

pub use config::*;
pub use error::{ScanError, ScanResult};
pub use library_scanner::{
    LibraryScanResult, LibraryScanner, SUPPORTED_AUDIO_EXTENSIONS, ScanProgressUpdate,
};
pub use performance::{OperationType, PerformanceMetrics, PerformanceMonitor, SlowOperation};
pub use progress::{ChannelReporter, ProgressReporter, ScanProgress};
pub use result::*;
pub use state::ScannerState;
pub use thread_pool::{
    ScanProgress as ThreadPoolScanProgress, ScanTask, ScanTaskResult, ScanningThreadPool,
    ThreadPoolConfig,
};
