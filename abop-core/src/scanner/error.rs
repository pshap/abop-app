//! Error types for scanner operations

use thiserror::Error;
use std::path::PathBuf;
use std::time::Duration;

/// Errors that can occur during scanning operations
#[derive(Error, Debug)]
pub enum ScanError {
    /// I/O error during file operations
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Error processing audio metadata
    #[error("Metadata error: {0}")]
    Metadata(String),
    
    /// Database operation failed
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    /// Operation was cancelled
    #[error("Scan was cancelled")]
    Cancelled,
    
    /// Operation timed out
    #[error("Operation timed out after {:?}", .0)]
    Timeout(Duration),
    
    /// Invalid file format
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    
    /// Invalid path
    #[error("Invalid path: {0:?}")]
    InvalidPath(PathBuf),
    
    /// Channel communication error
    #[error("Channel error: {0}")]
    Channel(String),
    
    /// Task join error
    #[error("Task error: {0}")]
    Task(String),
}

/// Result type for scan operations
pub type ScanResult<T = ()> = std::result::Result<T, ScanError>;

/// Extension trait for adding context to Results
pub trait Context<T, E> {
    fn context<C>(self, context: C) -> Result<T, ScanError>
    where
        C: std::fmt::Display + Send + Sync + 'static;
}

impl<T, E> Context<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, ScanError>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| ScanError::Metadata(format!("{}: {}", context, e)))
    }
}

// Conversions from common error types
impl From<tokio::sync::mpsc::error::SendError<crate::scanner::progress::ScanProgress>> for ScanError {
    fn from(_: tokio::sync::mpsc::error::SendError<crate::scanner::progress::ScanProgress>) -> Self {
        ScanError::Channel("Failed to send progress update".into())
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
