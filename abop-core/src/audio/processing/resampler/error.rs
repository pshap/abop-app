//! Resampler error types


/// Audio resampling error type
#[derive(Debug, thiserror::Error)]
pub enum ResamplerError {
    /// Invalid sample rate
    #[error("Invalid sample rate: {0}")]
    InvalidSampleRate(String),

    /// Processing error
    #[error("Resampling failed: {0}")]
    ProcessingError(String),
}
