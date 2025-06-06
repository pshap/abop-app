//! Scanner module for ABOP
//!
//! This module provides functionality to scan directories for audio files,
//! extract metadata, and update the database with the found files.
//!
//! The module uses modern async/await patterns with Tokio for efficient
//! parallel processing and resource management.
//!
//! # Features
//!
//! - Asynchronous file scanning with backpressure control
//! - Configurable concurrency and batch processing
//! - Progress reporting with detailed statistics
//! - Graceful cancellation support
//! - Comprehensive error handling
//! - Memory-efficient processing
//!
//! # Example
//!
//! ```rust,no_run
//! use abop_core::scanner::{LibraryScanner, ScannerConfig};
//! use abop_core::database::Database;
//! use abop_core::models::Library;
//! use tokio::sync::mpsc;
//!
//! async fn scan_library(db: Database, library: Library) {
//!     // Create scanner with custom configuration
//!     let scanner = LibraryScanner::new(db.into(), library)
//!         .with_config(ScannerConfig {
//!             max_concurrent_tasks: 8,
//!             batch_size: 100,
//!             ..Default::default()
//!         });
//!
//!     // Create progress channel
//!     let (progress_tx, mut progress_rx) = mpsc::channel(100);
//!
//!     // Start scan in background
//!     let scan_handle = tokio::spawn(async move {
//!         scanner.scan_async(progress_tx).await
//!     });
//!
//!     // Monitor progress
//!     while let Some(progress) = progress_rx.recv().await {
//!         println!("Progress: {:?}", progress);
//!     }
//!
//!     // Get final result
//!     let result = scan_handle.await.unwrap();
//!     println!("Scan completed: {:?}", result);
//! }
//! ```

pub mod library_scanner;
pub mod error;
pub mod progress;
pub mod config;
pub mod result;

pub use library_scanner::LibraryScanner;
pub use error::{ScanError, ScanResult};
pub use progress::{ScanProgress, ProgressReporter, ChannelReporter, LoggingReporter};
pub use config::ScannerConfig;
pub use result::ScanSummary;
