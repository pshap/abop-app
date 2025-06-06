//! Error handling for the ABOP core
use hound;
use thiserror::Error;
use std::io;
use serde_json;

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
#[derive(Error, Debug)]
pub enum AppError {
    /// I/O related errors (file system, network, etc.)
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Database errors from rusqlite
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

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
    WavFile(#[from] hound::Error),

    /// TOML deserialization error
    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
    /// TOML serialization error
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
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
    
    /// Error during scanning operations
    #[error("Scan error: {0}")]
    Scan(#[from] crate::scanner::error::ScanError),

    /// Internal errors (custom string context)
    #[error("Internal error: {0}")]
    Internal(String),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Unknown errors
    #[error("Unknown error: {0}")]
    Unknown(String),
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

impl AppError {
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("IO error during scan: {0}")]
    Io(#[from] io::Error),

    #[error("Database error during scan: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Processing error: {0}")]
    ProcessingError(String),

    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    #[error("Metadata extraction failed: {0}")]
    MetadataError(String),

    #[error("Scan cancelled")]
    Cancelled,

    #[error("Scan paused")]
    Paused,

    #[error("Unknown scan error: {0}")]
    Unknown(String),
}
