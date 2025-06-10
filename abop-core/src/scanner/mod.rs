//! Scanner module for ABOP
//!
//! This module provides functionality to scan directories for audio files,
//! extract metadata, and update the database with the found files.
//!
//! The module uses a modern async/await implementation with Tokio and Iced's Task system
//! for fine-grained control over scanning operations.

mod config;
mod constants;
mod core_scanner;
pub mod error;
mod file_discovery;
mod library_scanner;
mod orchestrator;
mod performance;
pub mod progress;
mod result;
mod state;
mod task_manager;

pub use config::*;
pub use constants::*;
pub use core_scanner::CoreScanner;
pub use error::{ScanError, ScanResult};
pub use file_discovery::FileDiscoverer;
pub use library_scanner::{LibraryScanner, SUPPORTED_AUDIO_EXTENSIONS};
pub use orchestrator::{ScanOptions, ScanOrchestrator};
pub use performance::{OperationType, PerformanceMetrics, PerformanceMonitor, SlowOperation};
pub use progress::{ChannelReporter, ProgressReporter, ScanProgress};
pub use result::*;
pub use state::ScannerState;
pub use task_manager::TaskManager;

// Re-export common types for convenience
pub use crate::db::Database;
pub use crate::error::AppError;
pub use crate::models::{Audiobook, Library};
