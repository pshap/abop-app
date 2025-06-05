//! Scanner module for ABOP
//!
//! This module provides functionality to scan directories for audio files,
//! extract metadata, and update the database with the found files.
//!
//! The module supports both traditional rayon-based parallel processing and
//! a configurable thread pool approach for more fine-grained control over
//! scanning operations.

pub mod library_scanner;
pub mod thread_pool;

pub use library_scanner::*;
pub use thread_pool::*;
