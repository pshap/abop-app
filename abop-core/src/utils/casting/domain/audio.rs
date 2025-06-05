//! Audio-specific conversion utilities

use super::super::error::{CastError, DomainCastError};
use log::warn;

/// Safe conversion from usize to f64 for audio sample calculations
///
/// # Note
/// f64 can exactly represent all integers up to 2^53. For values within
/// this range, the conversion is exact. For larger values, some precision
/// loss may occur, but this is acceptable for audio processing.
pub fn safe_usize_to_f64_audio(value: usize) -> f64 {
    const MAX_EXACT: usize = 1 << 53; // 2^53

    if value > MAX_EXACT {
        warn!("Converting large usize value {value} to f64 may lose precision for audio samples");
    }

    value as f64
}

/// Safe conversion from f64 to usize for audio sample indices
///
/// # Errors
/// Returns an error if:
/// - The input value is negative
/// - The input value is not finite (NaN or infinity)
/// - The input value exceeds the maximum value that can be represented by `usize`
pub fn safe_f64_to_usize_samples(value: f64) -> Result<usize, CastError> {
    if !value.is_finite() {
        return Err(CastError::NotFinite(value));
    }

    if value < 0.0 {
        return Err(CastError::NegativeValue(value.to_string()));
    }

    if value > usize::MAX as f64 {
        return Err(CastError::ValueTooLarge(
            value.to_string(),
            usize::MAX.to_string(),
        ));
    }

    // Check for fractional part
    if value.fract() != 0.0 {
        return Err(CastError::PrecisionLoss(value));
    }

    // Safe to cast since we've checked the bounds
    Ok(value as usize)
}

/// Safe conversion from usize to f32 for RMS calculations
///
/// # Errors
/// Returns an error if precision loss would be significant
///
/// # Note
/// f32 can exactly represent all integers up to 2^24. For larger values,
/// precision loss occurs, which may not be acceptable for some calculations.
pub fn safe_usize_to_f32_rms(sample_count: usize) -> Result<f32, CastError> {
    const MAX_EXACT: usize = 1 << 24; // 2^24

    if sample_count > MAX_EXACT {
        return Err(CastError::PrecisionLoss(sample_count as f64));
    }

    Ok(sample_count as f32)
}

/// Safe u64 to f64 conversion for file sizes
///
/// # Note
/// For very large file sizes, some precision loss may occur, but this is
/// acceptable for display purposes.
pub fn safe_u64_to_f64_size(bytes: u64) -> f64 {
    const MAX_EXACT: u64 = 1 << 53; // 2^53

    if bytes > MAX_EXACT {
        warn!("Converting large u64 value {bytes} to f64 may lose precision for file size");
    }

    bytes as f64
}

/// Safe conversion from duration in seconds to sample count
///
/// # Errors
/// Returns domain-specific audio errors for better error handling
pub fn safe_duration_to_samples(
    duration_secs: f32,
    sample_rate: u32,
) -> Result<usize, DomainCastError> {
    use crate::utils::casting::error::domain::AudioCastError;

    if sample_rate == 0 || sample_rate > 192_000 {
        return Err(AudioCastError::InvalidSampleRate(sample_rate).into());
    }

    if !duration_secs.is_finite() || duration_secs < 0.0 {
        return Err(AudioCastError::InvalidDuration(duration_secs).into());
    }

    let samples_f64 = f64::from(duration_secs) * f64::from(sample_rate);

    if samples_f64 > usize::MAX as f64 {
        return Err(AudioCastError::SampleCountOutOfRange(usize::MAX).into());
    }

    // Safe to cast since we've checked the bounds
    Ok(samples_f64 as usize)
}

/// Safe conversion from sample count to duration in seconds
///
/// # Errors
/// Returns domain-specific audio errors
pub fn safe_samples_to_duration(samples: usize, sample_rate: u32) -> Result<f32, DomainCastError> {
    use crate::utils::casting::error::domain::AudioCastError;

    if sample_rate == 0 || sample_rate > 192_000 {
        return Err(AudioCastError::InvalidSampleRate(sample_rate).into());
    }

    let duration = samples as f64 / f64::from(sample_rate);

    // Check for overflow in f32
    if duration > f32::MAX as f64 || duration < f32::MIN as f64 {
        return Err(AudioCastError::InvalidDuration(duration as f32).into());
    }

    Ok(duration as f32)
}

/// Convenience wrapper for safe_duration_to_samples
///
/// # Errors
/// Returns domain-specific audio errors
pub fn duration_to_samples(duration: f32, sample_rate: u32) -> Result<usize, DomainCastError> {
    safe_duration_to_samples(duration, sample_rate)
}

/// Convenience wrapper for safe_samples_to_duration
///
/// # Errors
/// Returns domain-specific audio errors
pub fn samples_to_duration(samples: usize, sample_rate: u32) -> Result<f32, DomainCastError> {
    safe_samples_to_duration(samples, sample_rate)
}

/// Convenience wrapper for safe_usize_to_f64_audio
#[must_use]
pub fn samples_to_f64(samples: usize) -> f64 {
    safe_usize_to_f64_audio(samples)
}

/// Convenience wrapper for safe_f64_to_usize_samples
///
/// # Errors
/// Returns casting errors for invalid conversions
pub fn f64_to_samples(value: f64) -> Result<usize, super::super::CastError> {
    safe_f64_to_usize_samples(value)
}

/// Safe progress ratio calculation
///
/// # Errors
/// Returns domain-specific audio errors
pub fn safe_progress(current: usize, total: usize) -> Result<f32, DomainCastError> {
    use crate::utils::casting::error::domain::AudioCastError;

    if total == 0 {
        return Err(AudioCastError::InvalidDuration(0.0).into());
    }

    if current > total {
        return Err(AudioCastError::SampleCountOutOfRange(current).into());
    }

    Ok(current as f32 / total as f32)
}

/// Validate audio channel count for audiobook processing
///
/// # Errors
/// Returns domain-specific audio errors for invalid channel counts
pub fn validate_channel_count(channels: u16) -> Result<u16, DomainCastError> {
    use crate::utils::casting::error::domain::AudioCastError;

    match channels {
        0 => Err(AudioCastError::InvalidChannelCount(channels).into()),
        1..=8 => Ok(channels), // Reasonable range for audiobooks
        _ => Err(AudioCastError::InvalidChannelCount(channels).into()),
    }
}

/// Validate bit depth for audio processing
///
/// # Errors
/// Returns domain-specific audio errors for invalid bit depths
pub fn validate_bit_depth(bits: u8) -> Result<u8, DomainCastError> {
    use crate::utils::casting::error::domain::AudioCastError;

    match bits {
        8 | 16 | 24 | 32 => Ok(bits),
        _ => Err(AudioCastError::InvalidBitDepth(bits).into()),
    }
}

/// Validate audiobook duration (reasonable bounds)
///
/// # Errors
/// Returns domain-specific audio errors for invalid durations
pub fn validate_audiobook_duration(duration_secs: f32) -> Result<f32, DomainCastError> {
    use crate::utils::casting::error::domain::AudioCastError;

    if !duration_secs.is_finite() || duration_secs < 0.0 {
        return Err(AudioCastError::InvalidDuration(duration_secs).into());
    }

    // Reasonable bounds: 1 second to 100 hours
    if !(1.0..=360000.0).contains(&duration_secs) {
        return Err(AudioCastError::InvalidDuration(duration_secs).into());
    }

    Ok(duration_secs)
}

/// Validate sample rate for audiobook processing
///
/// # Errors
/// Returns domain-specific audio errors for invalid sample rates
pub fn validate_sample_rate_audiobook(sample_rate: u32) -> Result<u32, DomainCastError> {
    use crate::utils::casting::error::domain::AudioCastError;

    // Common audiobook sample rates
    match sample_rate {
        8000 | 11025 | 16000 | 22050 | 44100 | 48000 | 96000 => Ok(sample_rate),
        _ if (8000..=192000).contains(&sample_rate) => Ok(sample_rate), // Allow reasonable range
        _ => Err(AudioCastError::InvalidSampleRate(sample_rate).into()),
    }
}
