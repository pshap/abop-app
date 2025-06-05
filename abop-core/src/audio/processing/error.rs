//! Centralized error handling for audio processing

use thiserror::Error;

/// Result type for audio processing operations
pub type Result<T> = std::result::Result<T, AudioProcessingError>;

/// Result type that includes processing metadata
pub type AudioProcessingResult<T> = Result<T>;

/// Specialized error types for audio processing operations
#[derive(Error, Debug, Clone)]
pub enum AudioProcessingError {
    /// Channel mixer related errors
    #[error("Channel mixer error: {0}")]
    ChannelMixer(String),

    /// Audio normalizer related errors
    #[error("Normalizer error: {0}")]
    Normalizer(String),

    /// Resampler related errors
    #[error("Resampler error: {0}")]
    Resampler(String),

    /// Silence detector related errors
    #[error("Silence detector error: {0}")]
    SilenceDetector(String),
    /// File I/O related errors
    #[error("File I/O error: {0}")]
    FileIo(String),

    /// Configuration validation errors
    #[error("Configuration validation error: {0}")]
    Configuration(String),

    /// Audio processing pipeline errors
    #[error("Pipeline error: {0}")]
    Pipeline(String),

    /// Buffer validation errors
    #[error("Buffer validation error: {0}")]
    BufferValidation(String),

    /// Sample rate validation errors
    #[error("Sample rate validation error: {0}")]
    SampleRateValidation(String),

    /// Channel count validation errors
    #[error("Channel count validation error: {0}")]
    ChannelValidation(String),

    /// Invalid input errors
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    /// Processing timeout errors
    #[error("Processing timeout: operation took longer than {timeout_ms}ms")]
    Timeout {
        /// Timeout duration in milliseconds
        timeout_ms: u64,
    },

    /// Memory allocation errors
    #[error("Memory allocation error: {0}")]
    Memory(String),

    /// Parallel processing errors
    #[error("Parallel processing error: {0}")]
    Parallel(String),
}

impl From<AudioProcessingError> for crate::error::AppError {
    fn from(err: AudioProcessingError) -> Self {
        Self::Audio(err.to_string())
    }
}

impl From<crate::error::AppError> for AudioProcessingError {
    fn from(error: crate::error::AppError) -> Self {
        match error {
            crate::error::AppError::Audio(msg) => Self::Pipeline(msg),
            crate::error::AppError::Io(e) => Self::FileIo(e.to_string()),
            crate::error::AppError::Database(msg) => {
                Self::Pipeline(format!("Database error: {msg}"))
            }
            crate::error::AppError::Config(msg) => Self::Configuration(msg),
            _ => Self::Pipeline(error.to_string()),
        }
    }
}

impl From<std::io::Error> for AudioProcessingError {
    fn from(error: std::io::Error) -> Self {
        Self::FileIo(error.to_string())
    }
}

impl AudioProcessingError {
    /// Creates a configuration error with formatted message
    #[must_use]
    pub fn config<T: std::fmt::Display>(msg: T) -> Self {
        Self::Configuration(msg.to_string())
    }

    /// Creates a pipeline error with formatted message
    #[must_use]
    pub fn pipeline<T: std::fmt::Display>(msg: T) -> Self {
        Self::Pipeline(msg.to_string())
    }

    /// Creates a buffer validation error with formatted message
    #[must_use]
    pub fn buffer<T: std::fmt::Display>(msg: T) -> Self {
        Self::BufferValidation(msg.to_string())
    }

    /// Creates a memory error with formatted message
    #[must_use]
    pub fn memory<T: std::fmt::Display>(msg: T) -> Self {
        Self::Memory(msg.to_string())
    }
}
