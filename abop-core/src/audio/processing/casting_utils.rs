//! Audio processing casting utilities (consolidated)
//!
//! This module re-exports safe casting utilities from the unified casting module
//! and provides audio-specific convenience functions.

// Re-export from unified casting system
pub use crate::utils::casting::domain::audio::*;
pub use crate::utils::casting::{CastError, CastResult, CastingBuilder, DomainCastError};

use crate::audio::processing::error::{AudioProcessingError, Result};

/// Audio-specific error conversion utilities
pub mod error_conversion {
    use super::{AudioProcessingError, DomainCastError, Result};

    /// Convert a DomainCastError to an AudioProcessingError
    #[must_use] pub fn cast_to_audio_error(err: DomainCastError) -> AudioProcessingError {
        AudioProcessingError::buffer(format!("Casting error: {err}"))
    }

    /// Convert a casting result to an audio processing result
    pub fn cast_result_to_audio<T>(result: super::CastResult<T>) -> Result<T> {
        result.map_err(|e| cast_to_audio_error(e.into()))
    }
}

/// Legacy compatibility - these functions now delegate to the unified casting system
pub mod safe_conversions {
    use super::*;
    use crate::utils::casting::domain::{audio as unified_audio, db};

    /// Safe conversion from database count to usize
    #[must_use] pub fn safe_db_count_to_usize(count: i64) -> usize {
        db::safe_db_count_to_usize(count)
    }
    /// Safe conversion from f64 to usize for sample indices
    pub fn safe_f64_to_usize_samples(value: f64) -> Result<usize> {
        unified_audio::safe_f64_to_usize_samples(value)
            .map_err(|e| error_conversion::cast_to_audio_error(e.into()))
    }

    /// Safe conversion from usize to f64 for audio calculations  
    #[must_use] pub fn safe_usize_to_f64_audio(value: usize) -> f64 {
        unified_audio::safe_usize_to_f64_audio(value)
    }

    /// Safe conversion from f64 to usize for resampling operations
    /// Allows truncation of fractional parts, which is normal for resampling
    pub fn safe_f64_to_usize_resampling(value: f64) -> Result<usize> {
        unified_audio::safe_f64_to_usize_resampling(value)
            .map_err(|e| error_conversion::cast_to_audio_error(e.into()))
    }
}

/// Sample calculation utilities
pub mod sample_calculations {
    use super::*;
    use crate::utils::casting::domain::audio as unified_audio;

    /// Calculate sample rate as f32 with safety checks
    pub const fn sample_rate_to_f32(rate: u32) -> Result<f32> {
        Ok(rate as f32)
    }

    /// Safe conversion from duration in seconds to sample count
    pub fn safe_duration_to_samples(duration_secs: f32, sample_rate: u32) -> Result<usize> {
        unified_audio::safe_duration_to_samples(duration_secs, sample_rate)
            .map_err(error_conversion::cast_to_audio_error)
    }

    /// Safe conversion from sample count to duration in seconds
    pub fn safe_samples_to_duration(samples: usize, sample_rate: u32) -> Result<f32> {
        unified_audio::safe_samples_to_duration(samples, sample_rate)
            .map_err(error_conversion::cast_to_audio_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_conversion() {
        let cast_error = DomainCastError::Generic(CastError::Overflow);
        let audio_error = error_conversion::cast_to_audio_error(cast_error);
        assert!(audio_error.to_string().contains("Casting error"));
    }

    #[test]
    fn test_safe_conversions_delegation() {
        // Test that our functions properly delegate to the unified system
        assert_eq!(safe_conversions::safe_db_count_to_usize(-1), 0);
        assert_eq!(safe_conversions::safe_db_count_to_usize(42), 42);

        let result = safe_conversions::safe_usize_to_f64_audio(100);
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_sample_calculations_delegation() {
        // Test sample rate conversion
        let rate_result = sample_calculations::sample_rate_to_f32(44100);
        assert!(rate_result.is_ok());
        assert_eq!(rate_result.unwrap(), 44100.0);

        // Test duration to samples conversion
        let samples_result = sample_calculations::safe_duration_to_samples(1.0, 44100);
        assert!(samples_result.is_ok());
        assert_eq!(samples_result.unwrap(), 44100);
    }
}
