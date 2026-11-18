//! Normalizer error types

/// Normalization error type
#[derive(Debug, thiserror::Error)]
pub enum NormalizerError {
    /// Invalid normalization parameter
    #[error("Invalid normalization parameter: {0}")]
    InvalidParameter(String),

    /// Processing error
    #[error("Normalization failed: {0}")]
    ProcessingError(String),
}
