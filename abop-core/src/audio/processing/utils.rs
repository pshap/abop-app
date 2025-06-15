//! Common utilities for audio processing

use super::error::{AudioProcessingError, Result};
use crate::audio::AudioBuffer;
use std::time::Duration;

// Re-export commonly used functions at module level
pub use buffer::validate_buffer;
pub use performance::{estimate_memory_usage, estimate_processing_time};
pub use timing::Timer;

/// Common buffer validation utilities
pub mod buffer {
    use super::{AudioBuffer, AudioProcessingError, Result};

    /// Validates basic buffer properties
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the buffer has invalid properties such as:
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
        if buffer.data.len() % channels_usize != 0 {
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
    /// Returns [`AudioProcessingError`] if the buffer is invalid or if the actual sample count doesn't match the expected count.
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
    /// Returns [`AudioProcessingError`] if either buffer is invalid or if the buffers have incompatible sample rates or channel counts.
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
}

/// Sample rate validation utilities
pub mod sample_rate {
    use super::{AudioProcessingError, Result};

    /// Common sample rate constants
    pub const MIN_SAMPLE_RATE: u32 = 8_000;
    /// Maximum supported sample rate
    pub const MAX_SAMPLE_RATE: u32 = 192_000;
    /// Standard audio sample rates
    pub const STANDARD_RATES: &[u32] = &[
        8_000, 11_025, 16_000, 22_050, 44_100, 48_000, 88_200, 96_000, 176_400, 192_000,
    ];

    /// Validates sample rate is within acceptable range
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::SampleRateValidation`] if the sample rate is outside the valid range.
    pub fn validate_sample_rate(sample_rate: u32) -> Result<()> {
        if !(MIN_SAMPLE_RATE..=MAX_SAMPLE_RATE).contains(&sample_rate) {
            return Err(AudioProcessingError::SampleRateValidation(format!(
                "Sample rate {sample_rate} outside valid range [{MIN_SAMPLE_RATE}, {MAX_SAMPLE_RATE}]"
            )));
        }
        Ok(())
    }

    /// Checks if sample rate is a standard/common rate
    #[must_use]
    pub fn is_standard_sample_rate(sample_rate: u32) -> bool {
        STANDARD_RATES.contains(&sample_rate)
    }

    /// Finds the closest standard sample rate
    #[must_use]
    pub fn closest_standard_rate(sample_rate: u32) -> u32 {
        STANDARD_RATES
            .iter()
            .min_by_key(|&&rate| {
                let rate_i64 = i64::from(rate);
                let sample_rate_i64 = i64::from(sample_rate);
                (rate_i64 - sample_rate_i64).abs()
            })
            .copied()
            .unwrap_or(44100)
    }

    /// Validates sample rate conversion ratio
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::SampleRateValidation`] if either sample rate is invalid.
    pub fn validate_conversion_ratio(input_rate: u32, output_rate: u32) -> Result<()> {
        validate_sample_rate(input_rate)?;
        validate_sample_rate(output_rate)?;

        let ratio = if input_rate > output_rate {
            f64::from(input_rate) / f64::from(output_rate)
        } else {
            f64::from(output_rate) / f64::from(input_rate)
        };

        // Warn for extreme ratios that might cause quality issues
        if ratio > 8.0 {
            log::warn!(
                "Large sample rate conversion ratio: {ratio:.2}x ({input_rate}Hz -> {output_rate}Hz)"
            );
        }

        Ok(())
    }
}

/// Channel count validation utilities
pub mod channels {
    use super::{AudioProcessingError, Result};

    /// Maximum supported channel count
    pub const MAX_CHANNELS: u16 = 32;

    /// Common channel configurations
    pub const MONO: u16 = 1;
    /// Stereo channel count
    pub const STEREO: u16 = 2;
    /// 5.1 surround sound channel count
    pub const SURROUND_5_1: u16 = 6;
    /// 7.1 surround sound channel count
    pub const SURROUND_7_1: u16 = 8;

    /// Validates channel count
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::ChannelValidation`] if the channel count is zero or exceeds the maximum supported channels.
    pub fn validate_channels(channels: u16) -> Result<()> {
        if channels == 0 {
            return Err(AudioProcessingError::ChannelValidation(
                "Channel count cannot be zero".to_string(),
            ));
        }
        if channels > MAX_CHANNELS {
            return Err(AudioProcessingError::ChannelValidation(format!(
                "Channel count {channels} exceeds maximum {MAX_CHANNELS}"
            )));
        }
        Ok(())
    }

    /// Validates channel mixing configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::ChannelValidation`] if either channel count is invalid.
    pub fn validate_channel_mixing(input_channels: u16, output_channels: u16) -> Result<()> {
        validate_channels(input_channels)?;
        validate_channels(output_channels)?;

        // Log warnings for unusual conversions
        if input_channels > output_channels && output_channels == 1 {
            log::debug!("Downmixing {input_channels} channels to mono");
        }

        Ok(())
    }
    /// Checks if channel configuration is standard
    #[must_use]
    pub const fn is_standard_configuration(channels: u16) -> bool {
        matches!(channels, MONO | STEREO | SURROUND_5_1 | SURROUND_7_1)
    }
}

/// Memory estimation utilities
pub mod memory {
    use super::{AudioProcessingError, Result};
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
    /// Returns [`AudioProcessingError`] if the estimated memory usage exceeds the specified limit.
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
}

/// Performance timing utilities
pub mod timing {
    // Re-export the unified Timer from utils
    pub use crate::utils::timer::Timer;
}

/// Performance utilities for timing and estimation
pub mod performance {
    use super::{AudioBuffer, Duration};
    use crate::utils::casting::safe_usize_to_f64_audio;

    // Re-export the unified Timer from utils
    pub use crate::utils::timer::Timer;
    /// Estimate processing time based on buffer size and sample rate
    #[must_use]
    pub fn estimate_processing_time<T>(buffer: &AudioBuffer<T>) -> Duration {
        let samples_per_channel = buffer.data.len() / usize::from(buffer.channels); // Use safe casting utility for duration calculation
        let duration_seconds =
            safe_usize_to_f64_audio(samples_per_channel) / f64::from(buffer.sample_rate);

        // Rough estimate: processing takes about 10% of audio duration minimum
        let base_processing_time = Duration::from_secs_f64(duration_seconds * 0.1);

        // Add overhead based on channel count and sample rate
        let channel_overhead = Duration::from_millis(u64::from(buffer.channels) * 10);
        let sample_rate_overhead = Duration::from_millis(u64::from(buffer.sample_rate) / 1000);

        base_processing_time + channel_overhead + sample_rate_overhead
    }
    /// Estimate memory usage for processing
    #[must_use]
    pub const fn estimate_memory_usage<T>(buffer: &AudioBuffer<T>) -> usize {
        std::mem::size_of::<T>() * buffer.data.len() * 2 // Double for processing overhead
    }
}

/// Common audio processing constants
pub mod constants {
    /// Default processing buffer size in samples
    pub const DEFAULT_BUFFER_SIZE: usize = 4096;

    /// Default processing timeout in milliseconds
    pub const DEFAULT_TIMEOUT_MS: u64 = 30_000; // 30 seconds

    /// Default number of parallel processing threads
    pub const DEFAULT_PARALLEL_THREADS: usize = 4;

    /// Silence threshold in dB
    pub const DEFAULT_SILENCE_THRESHOLD_DB: f32 = -60.0;

    /// Default target loudness in LUFS
    pub const DEFAULT_TARGET_LUFS: f32 = -23.0;

    /// Maximum allowed audio file size in bytes (500MB)
    pub const MAX_AUDIO_FILE_SIZE: usize = 500 * 1024 * 1024;
}
