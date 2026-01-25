//! Silence detector error types and data structures

use crate::audio::processing::error::AudioProcessingError;
use crate::audio::processing::error::Result;
use crate::audio::processing::traits::Validatable;

/// Silence detection error type
#[derive(Debug, thiserror::Error)]
pub enum SilenceDetectorError {
    /// Invalid silence parameter
    #[error("Invalid silence parameter: {0}")]
    InvalidParameter(String),

    /// Processing error
    #[error("Silence detection failed: {0}")]
    ProcessingError(String),
}

/// Represents a segment of silence in an audio buffer
#[derive(Debug, Clone, PartialEq)]
pub struct SilenceSegment {
    /// Start position in samples
    pub start: usize,
    /// End position in samples (exclusive)
    pub end: usize,
    /// Duration in seconds
    pub duration_secs: f32,
}

/// Helper functions for silence detection
impl SilenceSegment {
    /// Creates a new silence segment
    pub fn new(start: usize, end: usize, duration_secs: f32) -> Self {
        Self {
            start,
            end,
            duration_secs,
        }
    }

    /// Gets the duration of the silence segment in samples
    pub fn duration_samples(&self) -> usize {
        self.end - self.start
    }

    /// Validates the silence segment
    pub fn validate(&self) -> Result<()> {
        if self.start >= self.end {
            return Err(AudioProcessingError::Configuration(
                "Silence segment start must be less than end".to_string(),
            )
            .into());
        }
        if self.duration_secs < 0.0 {
            return Err(AudioProcessingError::Configuration(
                "Silence segment duration must be non-negative".to_string(),
            )
            .into());
        }
        Ok(())
    }
}

/// Trait implementations for silence detector validation
impl Validatable for SilenceSegment {
    fn validate(&self) -> Result<()> {
        self.validate()
    }
}
