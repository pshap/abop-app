//! Memory estimation utilities

use crate::audio::processing::error::{AudioProcessingError, Result};
use crate::utils::casting::domain::audio::safe_duration_to_samples;

/// Estimates memory usage for f32 audio buffer
#[must_use]
pub fn estimate_f32_buffer_size(sample_rate: u32, channels: u16, duration_secs: f32) -> usize {
    // Use safe casting utility for sample calculation
    let samples = safe_duration_to_samples(duration_secs, sample_rate).unwrap_or(0);
    samples * usize::from(channels) * std::mem::size_of::<f32>()
}

/// Estimates memory usage for i16 audio buffer
#[must_use]
pub fn estimate_i16_buffer_size(sample_rate: u32, channels: u16, duration_secs: f32) -> usize {
    // Use safe casting utility for sample calculation
    let samples = safe_duration_to_samples(duration_secs, sample_rate).unwrap_or(0);
    samples * usize::from(channels) * std::mem::size_of::<i16>()
}

/// Validates that estimated memory usage is reasonable
///
/// # Errors
///
/// Returns [`AudioProcessingError`] if estimated memory usage exceeds specified limit.
pub fn validate_memory_usage(estimated_bytes: usize, max_bytes: usize) -> Result<()> {
    if estimated_bytes > max_bytes {
        return Err(AudioProcessingError::memory(format!(
            "Estimated memory usage {}MB exceeds limit {}MB",
            estimated_bytes / 1024 / 1024,
            max_bytes / 1024 / 1024
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_f32_buffer_size() {
        let size = estimate_f32_buffer_size(44100, 2, 1.0);
        assert_eq!(size, 44100 * 2 * 4); // 1 second, stereo, f32

        let size = estimate_f32_buffer_size(48000, 1, 0.5);
        assert_eq!(size, 24000 * 1 * 4); // 0.5 second, mono, f32
    }

    #[test]
    fn test_estimate_i16_buffer_size() {
        let size = estimate_i16_buffer_size(44100, 2, 1.0);
        assert_eq!(size, 44100 * 2 * 2); // 1 second, stereo, i16

        let size = estimate_i16_buffer_size(48000, 1, 0.5);
        assert_eq!(size, 24000 * 1 * 2); // 0.5 second, mono, i16
    }

    #[test]
    fn test_validate_memory_usage() {
        // Should pass
        assert!(validate_memory_usage(1024 * 1024, 2 * 1024 * 1024).is_ok());

        // Should fail
        assert!(validate_memory_usage(2 * 1024 * 1024, 1024 * 1024).is_err());
    }

    #[test]
    fn test_validate_memory_usage_edge_cases() {
        // Exact limit should pass
        assert!(validate_memory_usage(1024 * 1024, 1024 * 1024).is_ok());

        // One byte over should fail
        assert!(validate_memory_usage(1024 * 1024 + 1, 1024 * 1024).is_err());
    }
}
