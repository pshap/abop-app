//! Error types for scanner operations

use crate::db::error::DatabaseError;
use crate::error::AppError;
use std::path::PathBuf;
use std::time::Duration;
use thiserror::Error;

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
    Database(String),

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
    /// Adds context to an error result, converting it into a [`ScanError`].
    ///
    /// This method allows adding descriptive context to any error that can be converted
    /// into a [`ScanError`]. The context is typically a string that describes the operation
    /// that failed, making error messages more informative.
    ///
    /// # Arguments
    ///
    /// * `context` - A displayable value that provides context about where the error occurred
    ///
    /// # Returns
    ///
    /// A [`Result`] containing either the success value or a [`ScanError`] with the added context
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
        self.map_err(|e| ScanError::Metadata(format!("{context}: {e}")))
    }
}

// Conversions from common error types
impl From<tokio::sync::mpsc::error::SendError<crate::scanner::progress::ScanProgress>>
    for ScanError
{
    fn from(
        _: tokio::sync::mpsc::error::SendError<crate::scanner::progress::ScanProgress>,
    ) -> Self {
        Self::Channel("Failed to send progress update".into())
    }
}

impl From<tokio::task::JoinError> for ScanError {
    fn from(err: tokio::task::JoinError) -> Self {
        if err.is_cancelled() {
            Self::Cancelled
        } else if err.is_panic() {
            Self::Task("Task panicked".into())
        } else {
            Self::Task("Task failed to complete".into())
        }
    }
}

impl From<AppError> for ScanError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Io(e) => Self::Metadata(format!("I/O error: {e}")),
            AppError::Database(e) => match e {
                DatabaseError::Sqlite(e) => Self::Database(e),
                _ => Self::Metadata(e.to_string()),
            },
            AppError::Scan(e) => Self::Metadata(e),
            AppError::InvalidData(msg) => Self::Metadata(msg),
            AppError::Cancelled => Self::Cancelled,
            AppError::Timeout {
                operation: _,
                timeout_ms: _,
                elapsed_ms,
            } => Self::Timeout(Duration::from_millis(elapsed_ms)),
            AppError::Metadata(msg) => Self::Metadata(msg),
            AppError::Library(msg) => Self::Metadata(format!("Library error: {msg}")),
            AppError::Progress(msg) => Self::Channel(msg),
            AppError::Task(msg) => Self::Task(msg),
            _ => Self::Metadata(err.to_string()),
        }
    }
}

impl From<tokio::sync::AcquireError> for ScanError {
    fn from(_: tokio::sync::AcquireError) -> Self {
        Self::Cancelled
    }
}

impl From<rusqlite::Error> for ScanError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database(err.to_string())
    }
}
