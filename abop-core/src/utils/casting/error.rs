//! Error types for casting operations

use thiserror::Error;

/// Main error type for casting operations
#[derive(Debug, Error)]
pub enum CastError {
    /// Value is not finite (NaN or infinity)
    #[error("Value is not finite: {0}")]
    NotFinite(f64),

    /// Negative value where non-negative is required
    #[error("Negative value not allowed: {0}")]
    NegativeValue(String),

    /// Value exceeds maximum allowed
    #[error("Value {0} exceeds maximum allowed: {1}")]
    ValueTooLarge(String, String),

    /// Precision would be lost in conversion
    #[error("Precision would be lost in conversion of {0}")]
    PrecisionLoss(f64),

    /// Integer overflow occurred
    #[error("Integer overflow during conversion")]
    Overflow,

    /// Underlying I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid input parameter
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Domain-specific error types
#[derive(Debug, Error)]
pub enum DomainCastError {
    /// Generic casting error
    #[error("Casting error: {0}")]
    Generic(CastError),

    /// Audio-specific errors
    #[error("Audio conversion error: {0}")]
    Audio(#[from] domain::AudioCastError),

    /// Database-specific errors
    #[error("Database conversion error: {0}")]
    Database(#[from] domain::DatabaseCastError),

    /// UI-specific errors
    #[error("UI conversion error: {0}")]
    Ui(#[from] domain::UiCastError),

    /// File-related errors
    #[error("File conversion error: {0}")]
    File(#[from] domain::FileCastError),
}

impl From<CastError> for DomainCastError {
    fn from(err: CastError) -> Self {
        Self::Generic(err)
    }
}

/// Domain-specific error types
pub mod domain {
    use super::CastError;
    use thiserror::Error;

    /// Audio-specific conversion errors
    #[derive(Debug, Error)]
    pub enum AudioCastError {
        /// Invalid sample rate
        #[error("Invalid sample rate: {0} Hz")]
        InvalidSampleRate(u32),

        /// Invalid audio duration
        #[error("Invalid audio duration: {0} seconds")]
        InvalidDuration(f32),

        /// Sample count out of range
        #[error("Sample count out of range: {0}")]
        SampleCountOutOfRange(usize),

        /// Invalid channel count
        #[error("Invalid channel count: {0}")]
        InvalidChannelCount(u16),

        /// Invalid bit depth
        #[error("Invalid bit depth: {0}")]
        InvalidBitDepth(u8),

        /// Generic casting error
        #[error("Casting error: {0}")]
        Casting(#[from] CastError),
    }

    /// Database-specific conversion errors
    #[derive(Debug, Error)]
    pub enum DatabaseCastError {
        /// Count out of range
        #[error("Count out of range: {0}")]
        CountOutOfRange(i64),

        /// Value too large for database
        #[error("Value too large for database: {0}")]
        ValueTooLarge(i64),

        /// Generic casting error
        #[error("Casting error: {0}")]
        Casting(#[from] CastError),
    }

    /// UI-specific conversion errors
    #[derive(Debug, Error)]
    pub enum UiCastError {
        /// Invalid UI spacing value
        #[error("Invalid UI spacing: {0}")]
        InvalidSpacing(f32),

        /// Invalid animation duration value
        #[error("Invalid animation duration: {0}")]
    InvalidDuration(f32),

        /// Invalid color value
        #[error("Invalid color value: {0}")]
        InvalidColor(String),

        /// Generic casting error
        #[error("Casting error: {0}")]
        Casting(#[from] CastError),
    }

    /// File-related conversion errors
    #[derive(Debug, Error)]
    pub enum FileCastError {
        /// File size too large
        #[error("File size too large: {0} bytes")]
        FileTooLarge(u64),

        /// Invalid file path
        #[error("Invalid file path: {0}")]
        InvalidPath(String),

        /// Generic casting error
        #[error("Casting error: {0}")]
        Casting(#[from] CastError),
    }
}
