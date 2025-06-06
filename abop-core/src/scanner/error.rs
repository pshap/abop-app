//! Error types for scanner operations

use thiserror::Error;
use std::path::PathBuf;
use std::time::Duration;
use std::io;
use rusqlite;
use std::sync::Arc;

/// Errors that can occur during scanning operations
#[derive(Debug, Error, Clone)]
pub enum ScanError {
    /// I/O error during file operations
    #[error("IO error during scan: {0}")]
    Io(#[from] Arc<io::Error>),
    
    /// Invalid file path
    #[error("Invalid file path: {0}")]
    InvalidPath(PathBuf),
    
    /// Unsupported file type
    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),
    
    /// Metadata extraction failed
    #[error("Metadata extraction failed: {0}")]
    MetadataError(String),
    
    /// Database error
    #[error("Database error during scan: {0}")]
    Database(#[from] Arc<rusqlite::Error>),
    
    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),
    
    /// Operation cancelled
    #[error("Scan cancelled")]
    Cancelled,
    
    /// Operation timed out
    #[error("Operation timed out after {:?}", .0)]
    Timeout(Duration),
    
    /// Invalid file format
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),
    
    /// Task execution failed
    #[error("Task failed: {0}")]
    Task(String),
    
    /// Scan paused
    #[error("Scan paused")]
    Paused,
    
    /// Unknown scan error
    #[error("Unknown scan error: {0}")]
    Unknown(String),
}

/// Result type for scan operations
pub type ScanResult<T> = Result<T, ScanError>;

/// Extension trait for adding context to Results
pub trait Context<T, E> {
    fn context<C>(self, context: C) -> ScanResult<T>
    where
        C: std::fmt::Display + Send + Sync + 'static;
}

impl<T, E> Context<T, E> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> ScanResult<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| ScanError::MetadataError(format!("{}: {}", context, e)))
    }
}

// Conversions from common error types
impl From<tokio::sync::mpsc::error::SendError<crate::scanner::progress::ScanProgress>> for ScanError {
    fn from(_: tokio::sync::mpsc::error::SendError<crate::scanner::progress::ScanProgress>) -> Self {
        ScanError::Task("Failed to send progress update".into())
    }
}

impl From<tokio::task::JoinError> for ScanError {
    fn from(err: tokio::task::JoinError) -> Self {
        if err.is_cancelled() {
            ScanError::Cancelled
        } else if err.is_panic() {
            ScanError::Task("Task panicked".into())
        } else {
            ScanError::Task("Task failed to complete".into())
        }
    }
}

impl From<io::Error> for ScanError {
    fn from(err: io::Error) -> Self {
        ScanError::Io(Arc::new(err))
    }
}

impl From<rusqlite::Error> for ScanError {
    fn from(err: rusqlite::Error) -> Self {
        ScanError::Database(Arc::new(err))
    }
}
