//! Scanner module for ABOP
//!
//! This module provides functionality to scan directories for audio files,
//! extract metadata, and update the database with the found files.
//!
//! The module supports both traditional rayon-based parallel processing and
//! a modern async/await implementation using Tokio and Iced's Task system
//! for fine-grained control over scanning operations.

pub mod library_scanner;
pub mod thread_pool;
pub mod error;
pub mod progress;
pub mod config;
pub mod result;
pub mod performance;

pub use library_scanner::{LibraryScanner, ScanProgressUpdate, LibraryScanResult, SUPPORTED_AUDIO_EXTENSIONS};
pub use thread_pool::{ScanningThreadPool, ThreadPoolConfig, ScanTask, ScanTaskResult, ScanProgress as ThreadPoolScanProgress};
pub use error::{ScanError, ScanResult};
pub use progress::{ScanProgress, ProgressReporter, ChannelReporter};
pub use performance::{PerformanceMonitor, PerformanceMetrics, OperationType, SlowOperation};
pub use config::*;
pub use result::*;
