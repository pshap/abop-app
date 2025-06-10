//! Error handling for the ABOP core
use crate::db::error::DatabaseError;
use crate::scanner::error::ScanError;
use hound;
use thiserror::Error;
use toml;

pub mod macros;

// Re-export commonly used macros and utilities
pub use macros::*;

/// Central error type for the ABOP application
///
/// This enum represents all possible error cases that can occur in the core library,
/// including I/O, database, audio processing, configuration, and serialization errors.
/// Each variant provides context for the error and, where possible, wraps the underlying error type.
///
/// # Examples
/// ```
/// use abop_core::error::AppError;
/// let err = AppError::Audio("Unsupported format".into());
/// ```
#[derive(Error, Debug, Clone)]
pub enum AppError {
    /// I/O related errors (file system, network, etc.)
    #[error("I/O error: {0}")]
    Io(String),

    /// Database errors from rusqlite
    #[error("Database error: {0}")]
    Database(DatabaseError),

    /// Audio processing errors (custom string context)
    #[error("Audio processing error: {0}")]
    Audio(String),

    /// Configuration errors (custom string context)
    #[error("Configuration error: {0}")]
    Config(String),

    /// Parsing errors (custom string context)
    #[error("Parse error: {0}")]
    Parse(String),

    /// Other errors not covered by specific variants
    #[error("{0}")]
    Other(String),

    /// WAV file related errors from hound
    #[error("WAV file error: {0}")]
    WavFile(String),

    /// TOML deserialization error
    #[error("TOML deserialization error: {0}")]
    TomlDe(String),
    /// TOML serialization error
    #[error("TOML serialization error: {0}")]
    TomlSer(String),
    /// Threading and concurrency errors
    #[error("Threading error: {0}")]
    Threading(String),
    /// Operation timeout error
    #[error("Operation '{operation}' timed out after {timeout_ms}ms (elapsed: {elapsed_ms}ms)")]
    Timeout {
        /// Name of the operation that timed out
        operation: String,
        /// Configured timeout in milliseconds
        timeout_ms: u64,
        /// Actual elapsed time in milliseconds
        elapsed_ms: u64,
    },

    /// Scan operation error
    #[error("Scan error: {0}")]
    Scan(String),

    /// Task join error
    #[error("Task join error: {0}")]
    TaskJoin(String),

    /// Invalid data error
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Validation failed error
    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    /// Operation cancelled error
    #[error("Operation cancelled")]
    Cancelled,

    /// Metadata error
    #[error("Metadata error: {0}")]
    Metadata(String),

    /// Library error
    #[error("Library error: {0}")]
    Library(String),

    /// Progress error
    #[error("Progress error: {0}")]
    Progress(String),

    /// Task error
    #[error("Task error: {0}")]
    Task(String),

    /// GUI error
    #[error("GUI error: {0}")]
    Gui(String),
}

/// Result type alias for fallible operations in the ABOP core
///
/// This type is used throughout the core library for functions that may fail,
/// returning either a successful value or an `AppError`.
///
/// # Examples
/// ```
/// use abop_core::error::Result;
/// fn do_work() -> Result<()> {
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, AppError>;

// Ergonomic conversions for AppError
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<hound::Error> for AppError {
    fn from(err: hound::Error) -> Self {
        Self::WavFile(err.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        Self::TomlDe(err.to_string())
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(err: toml::ser::Error) -> Self {
        Self::TomlSer(err.to_string())
    }
}

impl From<ScanError> for AppError {
    fn from(err: ScanError) -> Self {
        Self::Scan(err.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::TaskJoin(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        Self::Other(err.to_string())
    }
}

// Helper methods for error conversions
impl AppError {
    /// Convert a rusqlite::Error to AppError
    #[must_use]
    pub fn from_sqlite(err: rusqlite::Error) -> Self {
        Self::Database(DatabaseError::Sqlite(err.to_string()))
    }

    /// Convert a DatabaseError to rusqlite::Error
    #[must_use]
    pub fn to_sqlite(&self) -> rusqlite::Error {
        match self {
            Self::Database(db_err) => match db_err {
                DatabaseError::Sqlite(e) => rusqlite::Error::InvalidParameterName(e.to_string()),
                DatabaseError::ConnectionFailed(msg) => {
                    rusqlite::Error::InvalidPath(msg.clone().into())
                }
                DatabaseError::LockTimeout { timeout_ms } => rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_BUSY),
                    Some(format!("Database lock timeout after {timeout_ms}ms")),
                ),
                DatabaseError::ExecutionFailed { message } => rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_ERROR),
                    Some(message.clone()),
                ),
                _ => rusqlite::Error::InvalidParameterName(db_err.to_string()),
            },
            Self::Io(e) => rusqlite::Error::InvalidPath(e.to_string().into()),
            Self::Scan(scan_err) => rusqlite::Error::InvalidParameterName(scan_err.clone()),
            _ => rusqlite::Error::InvalidParameterName(self.to_string()),
        }
    }
}

// Helper function to convert rusqlite::Error to AppError
/// Converts rusqlite errors to AppError
///
/// This function provides a centralized way to convert SQLite errors
/// into the application's error type hierarchy.
pub fn sqlite_to_app_error(err: rusqlite::Error) -> AppError {
    AppError::from_sqlite(err)
}
