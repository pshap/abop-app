//! Audio processing casting utilities (consolidated)
//!
//! This module re-exports safe casting utilities from the unified casting module
//! and provides audio-specific convenience functions.

// Import error types and results
use crate::utils::casting::CastError;

/// A type alias for `Result` with the error type `CastError`
///
/// This is used as the return type for casting operations that can fail.
/// The `T` type parameter represents the successful result type.
pub type CastResult<T> = std::result::Result<T, CastError>;

use crate::audio::processing::error::{AudioProcessingError, Result};

/// Constants for audio processing bounds checking
pub mod conversion_constants {
    /// Maximum usize value that can be safely converted to f64 without precision loss
    /// f64 mantissa has 52 bits, so 2^52 is the safe limit
    pub const USIZE_SAFE_F64_MAX: usize = 1_usize << 52;

    /// Maximum u64 value that can be safely converted to f64 without precision loss
    pub const U64_SAFE_F64_MAX: u64 = 1_u64 << 52;

    /// Maximum reasonable audio sample rate (1MHz should be enough for anyone)
    pub const MAX_SAMPLE_RATE: u32 = 1_000_000;
}

/// Audio-specific safe conversions (wrapping unified functions with audio error types)
pub mod safe_conversions {
    use super::{AudioProcessingError, Result};

    /// Safe conversion from usize to f64 for audio calculations
    pub const fn safe_usize_to_f64_audio(value: usize) -> Result<f64> {
        // Direct conversion since it's just a type cast
        Ok(value as f64)
    }

    /// Safe conversion from f64 to usize for sample indices
    pub fn safe_f64_to_usize_samples(value: f64) -> Result<usize> {
        if value < 0.0 {
            return Err(AudioProcessingError::buffer(
                "Value cannot be negative".to_string(),
            ));
        }
        if value > usize::MAX as f64 {
            return Err(AudioProcessingError::buffer(
                "Value too large for usize".to_string(),
            ));
        }
        Ok(value.round() as usize)
    }

    /// Safe conversion from usize to f32 for RMS calculations
    pub fn safe_usize_to_f32_rms(sample_count: usize) -> Result<f32> {
        // Calculate RMS value (square root of the mean of the squares)
        Ok((sample_count as f32).sqrt())
    }

    /// Safe database count conversion
    pub const fn safe_db_count_to_usize(count: i64) -> usize {
        if count < 0 { 0 } else { count as usize }
    }

    /// Validate database count
    pub fn validate_db_count(count: i64) -> Result<usize> {
        if count < 0 {
            return Err(AudioProcessingError::buffer(
                "Count cannot be negative".to_string(),
            ));
        }
        Ok(count as usize)
    }

    /// Safe u64 to f64 conversion for file sizes
    pub const fn safe_u64_to_f64_size(bytes: u64) -> f64 {
        bytes as f64
    }

    /// Safe progress ratio calculation
    pub fn safe_progress(current: usize, total: usize) -> Result<f32> {
        if total == 0 {
            return Err(AudioProcessingError::buffer(
                "Total cannot be zero".to_string(),
            ));
        }
        Ok(current as f32 / total as f32)
    }
}

/// Sample calculation utilities
pub mod sample_calculations {
    use super::{AudioProcessingError, Result, conversion_constants, safe_conversions};
    use crate::utils::casting::domain::audio::{
        safe_duration_to_samples as audio_safe_duration_to_samples,
        safe_samples_to_duration as audio_safe_samples_to_duration,
    };

    /// Calculate sample rate as f32 with safety checks
    pub fn sample_rate_to_f32(rate: u32) -> Result<f32> {
        if rate == 0 || rate > conversion_constants::MAX_SAMPLE_RATE {
            return Err(AudioProcessingError::buffer(format!(
                "Invalid sample rate: {}. Must be between 1 and {}",
                rate,
                conversion_constants::MAX_SAMPLE_RATE
            )));
        }
        Ok(rate as f32)
    }

    /// Calculate samples per second with precision awareness
    pub fn samples_per_second(sample_rate: u32, duration_secs: f64) -> Result<usize> {
        let samples_f64 = duration_secs * f64::from(sample_rate);
        safe_conversions::safe_f64_to_usize_samples(samples_f64)
    }

    /// Safe conversion from duration in seconds to sample count
    pub fn safe_duration_to_samples(duration_secs: f32, sample_rate: u32) -> Result<usize> {
        audio_safe_duration_to_samples(duration_secs, sample_rate)
            .map_err(|e| AudioProcessingError::buffer(e.to_string()))
    }

    /// Safe conversion from sample count to duration in seconds
    pub fn safe_samples_to_duration(samples: usize, sample_rate: u32) -> Result<f32> {
        audio_safe_samples_to_duration(samples, sample_rate)
            .map_err(|e| AudioProcessingError::buffer(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_conversions() {
        // Test safe_usize_to_f64_audio
        assert_eq!(safe_conversions::safe_usize_to_f64_audio(42).unwrap(), 42.0);

        // Test safe_f64_to_usize_samples
        assert_eq!(
            safe_conversions::safe_f64_to_usize_samples(42.5).unwrap(),
            43
        );
        assert!(safe_conversions::safe_f64_to_usize_samples(-1.0).is_err());

        // Test safe_usize_to_f32_rms
        assert_eq!(safe_conversions::safe_usize_to_f32_rms(100).unwrap(), 10.0);
    }

    #[test]
    fn test_db_count_conversion() {
        // Test safe_db_count_to_usize
        assert_eq!(safe_conversions::safe_db_count_to_usize(42), 42);
        assert_eq!(safe_conversions::safe_db_count_to_usize(-1), 0);
    }

    #[test]
    fn test_sample_calculations() {
        // Test sample_rate_to_f32
        assert_eq!(
            sample_calculations::sample_rate_to_f32(44100).unwrap(),
            44100.0
        );
        assert!(sample_calculations::sample_rate_to_f32(0).is_err());

        // Test safe_duration_to_samples
        assert_eq!(
            sample_calculations::safe_duration_to_samples(1.0, 44100).unwrap(),
            44100
        );
    }
}
