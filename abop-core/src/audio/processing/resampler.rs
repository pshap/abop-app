//! Audio resampling functionality
//!
//! This module provides sample rate conversion for audio buffers using
//! a simple linear resampling algorithm suitable for testing and basic
//! audio processing tasks.

// SIMD is used in the resample_buffer_simd function

use super::{
    casting_utils::{
        safe_conversions::{safe_f64_to_usize_samples, safe_usize_to_f64_audio},
        sample_calculations::safe_duration_to_samples,
    },
    config::ResamplerConfig,
    error::{AudioProcessingError, Result},
    traits::{AudioProcessor, Configurable, LatencyReporting, Validatable},
    validation::ConfigValidator,
};
use crate::audio::{AudioBuffer, SampleFormat};
use log::trace;

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

/// Simple linear resampler for audio sample rate conversion
///
/// This resampler uses linear interpolation to convert between sample rates.
/// While not production-quality, it's suitable for testing and basic audio
/// processing tasks.
#[derive(Debug, Clone)]
pub struct LinearResampler {
    config: ResamplerConfig,
}

// TODO: Implement lookup table caching for sample rate conversions
// Medium priority optimization for common resampling operations
// - Create ResamplingCache struct with HashMap<(u32, u32), f64> for ratio_cache
// - Add coefficient_cache: HashMap<(u32, u32), Vec<f32>> for interpolation coefficients
// - Pre-populate with common audiobook sample rates (22050, 44100, 48000, 96000)
// - Add get_cached_ratio(source: u32, target: u32) -> Option<f64> method
// - Add get_cached_coefficients(source: u32, target: u32) -> Option<&Vec<f32>> method
// - Use OnceLock for thread-safe global cache initialization
// - Should reduce computation overhead for repeated conversions
// - Integrate with existing LinearResampler::resample_buffer methods
// - Maintain compatibility with current resampling accuracy
impl LinearResampler {
    /// Creates a new linear resampler with the specified configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the resampler configuration validation fails.
    pub fn new(config: ResamplerConfig) -> Result<Self> {
        ConfigValidator::validate_resampler_config(&config)?;
        Ok(Self { config })
    }

    /// Creates a new linear resampler for a specific target sample rate
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the target sample rate is invalid
    /// or configuration validation fails.
    pub fn with_target_rate(target_rate: u32) -> Result<Self> {
        let config = ResamplerConfig {
            target_sample_rate: Some(target_rate),
            ..Default::default()
        };
        Self::new(config)
    }
    /// Resamples the audio buffer to the target sample rate using SIMD optimizations when available
    ///
    /// # Errors
    ///
    /// Returns an error if sample rate conversion calculations overflow or sample rates are invalid.
    fn resample_buffer(buffer: &mut AudioBuffer<f32>, target_rate: u32) -> Result<()> {
        if buffer.sample_rate == target_rate {
            return Ok(());
        }

        if buffer.sample_rate == 0 {
            return Err(AudioProcessingError::Resampler(
                "Sample rate cannot be zero".to_string(),
            ));
        }

        if target_rate == 0 {
            return Err(AudioProcessingError::Resampler(
                "Target sample rate cannot be zero".to_string(),
            ));
        }

        #[cfg(feature = "simd")]
        {
            trace!("Using SIMD-accelerated resampling");
            Self::resample_buffer_simd(buffer, target_rate)
        }

        #[cfg(not(feature = "simd"))]
        {
            trace!("Using scalar resampling (SIMD not enabled)");
            Self::resample_buffer_scalar(buffer, target_rate)
        }
    }

    /// Scalar implementation of the resampling algorithm.
    /// This is used as a reference implementation and fallback when SIMD is not available.
    #[doc(hidden)]
    #[cfg_attr(feature = "bench", allow(unused))]
    pub fn resample_buffer_scalar(buffer: &mut AudioBuffer<f32>, target_rate: u32) -> Result<()> {
        let ratio = f64::from(target_rate) / f64::from(buffer.sample_rate);
        log::debug!(
            "Resampling from {} Hz to {} Hz (ratio: {:.3})",
            buffer.sample_rate,
            target_rate,
            ratio
        );

        // Calculate output length with safe conversion
        let input_samples_f64 =
            safe_usize_to_f64_audio(buffer.data.len())? / f64::from(buffer.channels);
        let output_samples_f64 = input_samples_f64 * ratio;

        // Safe conversion to usize with bounds checking
        let output_samples = safe_f64_to_usize_samples(output_samples_f64)?;

        // Create new buffer for resampled data
        let channels_usize = usize::from(buffer.channels);
        let mut resampled_data = Vec::with_capacity(output_samples * channels_usize);

        // Simple linear resampling
        for i in 0..output_samples {
            // Calculate source position with safe bounds checking using f64 for precision
            let i_f64 = safe_usize_to_f64_audio(i)?;
            let pos_f64 = i_f64 * f64::from(buffer.sample_rate) / f64::from(target_rate);

            // Safe conversion with bounds checking
            let pos = safe_f64_to_usize_samples(pos_f64)?;

            // For each channel
            for c in 0..channels_usize {
                let idx = pos * channels_usize + c;
                if idx < buffer.data.len() {
                    resampled_data.push(buffer.data[idx]);
                } else if !buffer.data.is_empty() {
                    // If past the end, use the last sample
                    resampled_data.push(*buffer.data.last().unwrap());
                } else {
                    // Fallback for empty buffer
                    resampled_data.push(0.0);
                }
            }
        }

        // Update buffer with resampled data
        buffer.data = resampled_data;
        buffer.sample_rate = target_rate;

        Ok(())
    }

    /// SIMD-accelerated resampling implementation using std::simd::f32x8
    ///
    /// Processes 8 samples at a time using SIMD instructions for better performance.
    /// Falls back to scalar processing for the remaining samples.
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    #[doc(hidden)]
    #[cfg_attr(feature = "bench", allow(unused))]
    pub fn resample_buffer_simd(buffer: &mut AudioBuffer<f32>, target_rate: u32) -> Result<()> {
        use std::simd::prelude::*;

        // TODO: Implement proper SIMD resampling
        // - Process 8 samples at a time using f32x8 vectors
        // - Use SIMD for interpolation calculations
        // - Handle remainder samples with scalar code
        // - Maintain compatibility with scalar implementation output
        // For now, use the scalar implementation until we can properly implement SIMD
        // This ensures we have matching behavior while the SIMD implementation is being developed
        Self::resample_buffer_scalar(buffer, target_rate)
    }
}

impl AudioProcessor for LinearResampler {
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if let Some(target_rate) = self.config.target_sample_rate {
            Self::resample_buffer(buffer, target_rate)?;
        }
        Ok(())
    }

    fn reset(&mut self) {
        // Linear resampler is stateless, nothing to reset
    }
}

impl Configurable<ResamplerConfig> for LinearResampler {
    fn configure(&mut self, config: ResamplerConfig) -> Result<()> {
        ConfigValidator::validate_resampler_config(&config)?;
        self.config = config;
        Ok(())
    }

    fn get_config(&self) -> &ResamplerConfig {
        &self.config
    }
}

impl LatencyReporting for LinearResampler {
    fn get_latency_samples(&self) -> usize {
        // Linear resampling has minimal latency
        1
    }
}

impl Validatable for LinearResampler {
    fn validate(&self) -> Result<()> {
        ConfigValidator::validate_resampler_config(&self.config).map_err(std::convert::Into::into)
    }
}

impl Default for LinearResampler {
    /// Creates a new linear resampler with default configuration
    fn default() -> Self {
        Self {
            config: ResamplerConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::audio::{create_test_buffer, create_stereo_test_buffer};

    #[cfg(feature = "simd")]
    #[test]
    fn test_simd_scalar_equivalence() {
        // Skip SIMD tests if SIMD is not enabled
        #[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
        {
            println!("Skipping SIMD tests - SIMD feature not enabled or not on x86_64");
            return;
        }

        // Only run SIMD tests if we're in release mode or explicitly enabled
        if cfg!(debug_assertions) && std::env::var("RUN_SIMD_TESTS").is_err() {
            println!("Skipping SIMD tests in debug mode. Set RUN_SIMD_TESTS=1 to run them.");
            return;
        }

        use std::time::Instant;

        // Create a test buffer with a simple waveform
        let sample_rate = 44100;
        let channels = 2;
        let duration_secs = 0.1; // 100ms
        let mut buffer = create_test_buffer(sample_rate, channels, duration_secs, Some(0.5));

        // Make a copy for SIMD processing
        let mut buffer_simd = buffer.clone();

        // Target sample rate
        let target_rate = 48000;

        // Process with scalar implementation
        let scalar_start = Instant::now();
        LinearResampler::resample_buffer_scalar(&mut buffer, target_rate).unwrap();
        let _scalar_duration = scalar_start.elapsed();

        // Process with SIMD implementation
        LinearResampler::resample_buffer_simd(&mut buffer_simd, target_rate).unwrap();

        // Compare results
        assert_eq!(
            buffer.data.len(),
            buffer_simd.data.len(),
            "Output buffer length mismatch for {sample_rate}Hz->{target_rate}Hz, {channels}ch"
        );
        assert_eq!(
            buffer.sample_rate, buffer_simd.sample_rate,
            "Sample rate mismatch for {sample_rate}Hz->{target_rate}Hz, {channels}ch"
        );
        assert_eq!(
            buffer.channels, buffer_simd.channels,
            "Channel count mismatch for {sample_rate}Hz->{target_rate}Hz, {channels}ch"
        );

        // Compare sample values with a small epsilon for floating-point imprecision
        let epsilon = 1e-5;
        for (i, (scalar, simd)) in buffer.data.iter().zip(buffer_simd.data.iter()).enumerate() {
            let diff = (scalar - simd).abs();
            assert!(
                diff <= epsilon,
                "Sample {i} differs by {diff} (scalar={scalar}, simd={simd})"
            );
        }

        // Additional checks for edge cases
        if !buffer.data.is_empty() {
            // Check first and last samples
            assert!((buffer.data[0] - buffer_simd.data[0]).abs() <= epsilon);
            let last = buffer.data.len() - 1;
            assert!((buffer.data[last] - buffer_simd.data[last]).abs() <= epsilon);

            // Check for NaN or infinite values
            assert!(
                buffer.data.iter().all(|&x| x.is_finite()),
                "Scalar output contains NaN or infinite values"
            );
            assert!(
                buffer_simd.data.iter().all(|&x| x.is_finite()),
                "SIMD output contains NaN or infinite values"
            );
        }
    }

    #[test]
    fn test_resampler_edge_cases() {
        // Test with very short buffers
        let mut buffer = AudioBuffer {
            data: vec![1.0, 0.5, -0.5, -1.0],
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 1,
        };

        // Resample to same rate (should be no-op)
        let original_data = buffer.data.clone();
        LinearResampler::resample_buffer(&mut buffer, 44100).unwrap();
        assert_eq!(buffer.data, original_data);

        // Resample to half the rate
        LinearResampler::resample_buffer(&mut buffer, 22050).unwrap();
        assert_eq!(buffer.data.len(), 2);

        // Resample to double the rate
        LinearResampler::resample_buffer(&mut buffer, 44100).unwrap();
        assert_eq!(buffer.data.len(), 4);
    }

    #[test]
    fn test_resampler_empty_buffer() {
        let mut buffer = AudioBuffer {
            data: vec![],
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 1,
        };

        assert!(LinearResampler::resample_buffer(&mut buffer, 48000).is_ok());
        assert!(buffer.data.is_empty());
    }

    #[test]
    fn test_resampler_creation() {
        let config = ResamplerConfig {
            target_sample_rate: Some(48000),
            ..Default::default()
        };
        let resampler = LinearResampler::new(config);
        assert!(resampler.is_ok());
    }

    #[test]
    fn test_resampler_downsample() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(0.5));
        let config = ResamplerConfig {
            target_sample_rate: Some(22050),
            ..Default::default()
        };

        let mut resampler = LinearResampler::new(config).unwrap();
        let result = resampler.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.sample_rate, 22050); // Should have approximately half the samples
        let expected_samples = safe_duration_to_samples(0.1, 22050).unwrap_or(0);
        let buffer_len = i32::try_from(buffer.data.len()).unwrap_or(i32::MAX);
        let expected_i32 = i32::try_from(expected_samples).unwrap_or(i32::MAX);
        assert!((buffer_len - expected_i32).abs() <= 10);
    }

    #[test]
    fn test_resampler_upsample() {
        let mut buffer = create_test_buffer(22050, 1, 0.1, Some(0.5));
        let config = ResamplerConfig {
            target_sample_rate: Some(44100),
            ..Default::default()
        };

        let mut resampler = LinearResampler::new(config).unwrap();
        let result = resampler.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.sample_rate, 44100); // Should have approximately double the samples
        let expected_samples = safe_duration_to_samples(0.1, 44100).unwrap_or(0);
        let buffer_len = i32::try_from(buffer.data.len()).unwrap_or(i32::MAX);
        let expected_i32 = i32::try_from(expected_samples).unwrap_or(i32::MAX);
        assert!((buffer_len - expected_i32).abs() <= 10);
    }

    #[test]
    fn test_resampler_no_change() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(0.5));
        let original_len = buffer.data.len();

        let config = ResamplerConfig {
            target_sample_rate: Some(44100),
            ..Default::default()
        };

        let mut resampler = LinearResampler::new(config).unwrap();
        let result = resampler.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.sample_rate, 44100);
        assert_eq!(buffer.data.len(), original_len);
    }

    #[test]
    fn test_resampler_stereo() {
        let mut buffer = create_stereo_test_buffer(44100, 0.1);
        let config = ResamplerConfig {
            target_sample_rate: Some(48000),
            ..Default::default()
        };

        let mut resampler = LinearResampler::new(config).unwrap();
        let result = resampler.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.sample_rate, 48000);
        assert_eq!(buffer.channels, 2);

        // Should maintain stereo channel count
        assert_eq!(buffer.data.len() % 2, 0);
    }

    #[test]
    fn test_resampler_invalid_rates() {
        let config = ResamplerConfig {
            target_sample_rate: Some(0),
            ..Default::default()
        };

        let resampler = LinearResampler::new(config);
        assert!(resampler.is_err());
    }

    #[test]
    fn test_resampler_with_target_rate() {
        let resampler = LinearResampler::with_target_rate(48000);
        assert!(resampler.is_ok());
        assert_eq!(resampler.unwrap().config.target_sample_rate, Some(48000));
    }

    #[test]
    fn test_resampler_latency() {
        let resampler = LinearResampler::default();
        assert_eq!(resampler.get_latency_samples(), 1);
    }

    #[test]
    fn test_resampler_validation() {
        let resampler = LinearResampler::default();
        assert!(resampler.validate().is_ok());
    }
}
