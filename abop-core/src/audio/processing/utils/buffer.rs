//! Audio buffer validation utilities

use crate::audio::AudioBuffer;
use crate::audio::processing::error::{AudioProcessingError, Result};

/// Validates basic buffer properties
///
/// # Errors
///
/// Returns [`AudioProcessingError`] if buffer has invalid properties such as:
/// - Zero channels
/// - Zero sample rate
/// - Empty data
/// - Data length not divisible by channel count
pub fn validate_buffer<T>(buffer: &AudioBuffer<T>) -> Result<()> {
    if buffer.channels == 0 {
        return Err(AudioProcessingError::buffer("Buffer has no channels"));
    }
    if buffer.sample_rate == 0 {
        return Err(AudioProcessingError::buffer("Invalid sample rate: 0"));
    }
    if buffer.data.is_empty() {
        return Err(AudioProcessingError::buffer("Buffer is empty"));
    }
    // Check if data length matches expected size
    let channels_usize = usize::from(buffer.channels);
    let _expected_samples = buffer.data.len() / channels_usize;
    if !buffer.data.len().is_multiple_of(channels_usize) {
        return Err(AudioProcessingError::buffer(format!(
            "Buffer data length {} is not divisible by channel count {}",
            buffer.data.len(),
            buffer.channels
        )));
    }

    Ok(())
}

/// Validates buffer for specific sample count
///
/// # Errors
///
/// Returns [`AudioProcessingError`] if buffer is invalid or if actual sample count doesn't match expected count.
pub fn validate_buffer_size<T>(buffer: &AudioBuffer<T>, expected_samples: usize) -> Result<()> {
    validate_buffer(buffer)?;

    let actual_samples = buffer.data.len() / usize::from(buffer.channels);
    if actual_samples != expected_samples {
        return Err(AudioProcessingError::buffer(format!(
            "Expected {expected_samples} samples, got {actual_samples}"
        )));
    }

    Ok(())
}

/// Validates that two buffers have compatible formats
///
/// # Errors
///
/// Returns [`AudioProcessingError`] if either buffer is invalid or if buffers have incompatible sample rates or channel counts.
pub fn validate_buffer_compatibility<T, U>(
    buffer1: &AudioBuffer<T>,
    buffer2: &AudioBuffer<U>,
) -> Result<()> {
    validate_buffer(buffer1)?;
    validate_buffer(buffer2)?;

    if buffer1.sample_rate != buffer2.sample_rate {
        return Err(AudioProcessingError::buffer(format!(
            "Sample rate mismatch: {} != {}",
            buffer1.sample_rate, buffer2.sample_rate
        )));
    }

    if buffer1.channels != buffer2.channels {
        return Err(AudioProcessingError::buffer(format!(
            "Channel count mismatch: {} != {}",
            buffer1.channels, buffer2.channels
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::{AudioBuffer, SampleFormat};

    #[test]
    fn test_validate_buffer_valid() {
        let buffer = AudioBuffer {
            data: vec![0.0f32; 1000],
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 2,
        };

        assert!(validate_buffer(&buffer).is_ok());
    }

    #[test]
    fn test_validate_buffer_no_channels() {
        let buffer = AudioBuffer {
            data: vec![0.0f32; 1000],
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 0, // Invalid
        };

        assert!(validate_buffer(&buffer).is_err());
    }

    #[test]
    fn test_validate_buffer_size_mismatch() {
        let buffer = AudioBuffer {
            data: vec![0.0f32; 1000],
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 2,
        };

        // 1000 interleaved samples across 2 channels = 500 samples per channel
        assert!(validate_buffer_size(&buffer, 500).is_ok());
        assert!(validate_buffer_size(&buffer, 1000).is_err());
        assert!(validate_buffer_size(&buffer, 2000).is_err());
    }

    #[test]
    fn test_validate_buffer_compatibility_mismatch() {
        let buffer1 = AudioBuffer {
            data: vec![0.0f32; 1000],
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 2,
        };

        let buffer2 = AudioBuffer {
            data: vec![0.0f32; 1000],
            format: SampleFormat::F32,
            sample_rate: 48000, // Different sample rate
            channels: 2,
        };

        assert!(validate_buffer_compatibility(&buffer1, &buffer2).is_err());
    }
}
