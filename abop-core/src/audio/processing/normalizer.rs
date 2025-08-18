//! Audio normalization functionality
//!
//! This module provides volume normalization for audio buffers using
//! various normalization algorithms including peak normalization and
//! RMS-based normalization.

use super::{
    config::NormalizerConfig,
    error::Result,
    traits::{AudioProcessor, Configurable, LatencyReporting, Validatable},
    validation::ConfigValidator,
};
use crate::audio::AudioBuffer;

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

/// Audio normalizer for volume normalization
///
/// Supports various normalization algorithms including peak normalization
/// and RMS-based normalization with configurable target levels and headroom.
#[derive(Debug, Clone)]
pub struct AudioNormalizer {
    config: NormalizerConfig,
}

impl AudioNormalizer {
    /// Creates a new audio normalizer with the specified configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the normalizer configuration validation fails.
    pub fn new(config: NormalizerConfig) -> Result<Self> {
        ConfigValidator::validate_normalizer_config(&config)?;
        Ok(Self { config })
    }

    /// Creates a new audio normalizer with peak normalization to a specific level
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the target level is invalid or configuration validation fails.
    pub fn with_peak_target(target_db: f32) -> Result<Self> {
        let config = NormalizerConfig {
            algorithm: super::config::NormalizationAlgorithm::Peak,
            target_loudness: target_db,
            ..Default::default()
        };
        Self::new(config)
    }

    /// Creates a new audio normalizer with RMS normalization to a specific level
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the target level is invalid or configuration validation fails.
    pub fn with_rms_target(target_db: f32) -> Result<Self> {
        let config = NormalizerConfig {
            algorithm: super::config::NormalizationAlgorithm::Rms,
            target_loudness: target_db,
            ..Default::default()
        };
        Self::new(config)
    }
    /// Normalizes the audio buffer according to the configuration
    ///
    /// # Returns
    /// * `Result<()>` - Always returns `Ok(())` as this operation cannot fail
    ///
    /// Note: The `Result` return type is maintained for compatibility with the `AudioProcessor` trait.
    #[allow(clippy::unnecessary_wraps)]
    fn normalize_buffer(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if buffer.data.is_empty() {
            return Ok(());
        }

        match self.config.algorithm {
            super::config::NormalizationAlgorithm::Peak => {
                self.normalize_peak(buffer);
            }
            super::config::NormalizationAlgorithm::Rms => {
                self.normalize_rms(buffer);
            }
            super::config::NormalizationAlgorithm::Lufs => {
                self.normalize_lufs(buffer);
            }
        }

        Ok(())
    }

    /// Applies peak normalization to the buffer
    fn normalize_peak(&self, buffer: &mut AudioBuffer<f32>) {
        let max_sample = self.find_peak_simd(&buffer.data);
        if max_sample > 0.0 && max_sample < 1.0 {
            // Convert target dB to linear scale with headroom
            let target_linear = 10.0f32.powf(self.config.target_loudness / 20.0);
            let headroom_factor = 10.0f32.powf(-self.config.headroom_db / 20.0);
            let gain = (target_linear * headroom_factor) / max_sample;

            log::debug!(
                "Peak normalizing with gain: {:.2} dB (max sample: {:.4})",
                20.0 * gain.log10(),
                max_sample
            );

            // Apply gain with limiter to prevent clipping
            for sample in &mut buffer.data {
                let amplified = *sample * gain;
                *sample = amplified.clamp(-1.0, 1.0);
            }

            // TODO: Implement SIMD optimization for gain application and limiting
            // - Process 8 samples at a time using f32x8 vectors
            // - Use SIMD multiplication for gain application: sample_vec * gain_vec
            // - Use SIMD clamp operations for limiting: vec.clamp(-1.0, 1.0)
            // - Should provide significant speedup for gain application
            // - Handle remainder samples with scalar code
            // - Maintains exact compatibility with current limiting behavior
        }
    }

    /// Applies RMS normalization to the buffer
    fn normalize_rms(&self, buffer: &mut AudioBuffer<f32>) {
        let rms = Self::calculate_rms_simd(&buffer.data);
        if rms > 0.0 {
            // Convert target dB to linear scale with headroom
            let target_linear = 10.0f32.powf(self.config.target_loudness / 20.0);
            let headroom_factor = 10.0f32.powf(-self.config.headroom_db / 20.0);
            let gain = (target_linear * headroom_factor) / rms;

            log::debug!(
                "RMS normalizing with gain: {:.2} dB (RMS: {:.4})",
                20.0 * gain.log10(),
                rms
            );

            // Apply gain with SIMD-optimized limiter to prevent clipping
            Self::apply_gain_with_limiting_simd(&mut buffer.data, gain);
        }
    }

    /// Applies LUFS normalization to the buffer (simplified implementation)
    fn normalize_lufs(&self, buffer: &mut AudioBuffer<f32>) {
        self.normalize_rms(buffer);
    }

    /// SIMD-optimized peak finding for audio data
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    fn find_peak_simd(&self, data: &[f32]) -> f32 {
        use std::simd::prelude::*;
        
        if data.is_empty() {
            return 0.0;
        }

        const SIMD_LANES: usize = 8;
        let chunks = data.len() / SIMD_LANES;
        let remainder_start = chunks * SIMD_LANES;
        
        // SIMD processing for bulk data
        let mut max_vec = f32x8::splat(0.0);
        
        for chunk_idx in 0..chunks {
            let start = chunk_idx * SIMD_LANES;
            let chunk_data = &data[start..start + SIMD_LANES];
            
            // Load 8 samples into SIMD vector
            let sample_vec = f32x8::from_slice(chunk_data);
            
            // Take absolute value and find max
            let abs_vec = sample_vec.abs();
            max_vec = max_vec.simd_max(abs_vec);
        }
        
        // Horizontal reduction to find maximum across lanes
        let max_array = max_vec.to_array();
        let mut simd_max = max_array[0];
        for &val in &max_array[1..] {
            simd_max = simd_max.max(val);
        }
        
        // Handle remainder samples with scalar processing
        let scalar_max = data[remainder_start..]
            .iter()
            .fold(0.0f32, |max, &s| max.max(s.abs()));
        
        simd_max.max(scalar_max)
    }

    /// Fallback peak finding for non-SIMD targets
    #[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
    fn find_peak_simd(&self, data: &[f32]) -> f32 {
        data.iter().fold(0.0f32, |max, &s| max.max(s.abs()))
    }

    /// SIMD-optimized RMS calculation for audio data
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    fn calculate_rms_simd(data: &[f32]) -> f32 {
        use std::simd::prelude::*;
        
        if data.is_empty() {
            return 0.0;
        }

        const SIMD_LANES: usize = 8;
        let chunks = data.len() / SIMD_LANES;
        let remainder_start = chunks * SIMD_LANES;
        
        // SIMD processing for bulk squares calculation
        let mut sum_vec = f32x8::splat(0.0);
        
        for chunk_idx in 0..chunks {
            let start = chunk_idx * SIMD_LANES;
            let chunk_data = &data[start..start + SIMD_LANES];
            
            // Load 8 samples into SIMD vector
            let sample_vec = f32x8::from_slice(chunk_data);
            
            // Square the samples: sample_vec * sample_vec
            let squared_vec = sample_vec * sample_vec;
            sum_vec += squared_vec;
        }
        
        // Horizontal sum reduction across lanes
        let sum_array = sum_vec.to_array();
        let mut simd_sum = 0.0f32;
        for &val in &sum_array {
            simd_sum += val;
        }
        
        // Handle remainder samples with scalar processing
        let scalar_sum: f32 = data[remainder_start..]
            .iter()
            .map(|&s| s * s)
            .sum();
        
        let total_sum = simd_sum + scalar_sum;
        
        // Calculate RMS with safe conversion
        #[allow(clippy::cast_precision_loss)]
        let data_len_f32 = if data.len() <= (1_usize << 24) {
            data.len() as f32
        } else {
            // For very large arrays, we accept potential precision loss
            data.len() as f32
        };
        
        (total_sum / data_len_f32).sqrt()
    }

    /// Fallback RMS calculation for non-SIMD targets
    #[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
    fn calculate_rms_simd(data: &[f32]) -> f32 {
        Self::calculate_rms(data)
    }

    /// SIMD-optimized gain application with limiting
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    fn apply_gain_with_limiting_simd(data: &mut [f32], gain: f32) {
        use std::simd::prelude::*;
        
        if data.is_empty() {
            return;
        }

        const SIMD_LANES: usize = 8;
        let chunks = data.len() / SIMD_LANES;
        let remainder_start = chunks * SIMD_LANES;
        
        let gain_vec = f32x8::splat(gain);
        let min_vec = f32x8::splat(-1.0);
        let max_vec = f32x8::splat(1.0);
        
        // SIMD processing for bulk gain application
        for chunk_idx in 0..chunks {
            let start = chunk_idx * SIMD_LANES;
            let chunk_data = &mut data[start..start + SIMD_LANES];
            
            // Load 8 samples into SIMD vector
            let sample_vec = f32x8::from_slice(chunk_data);
            
            // Apply gain and clamp to [-1.0, 1.0]
            let amplified_vec = sample_vec * gain_vec;
            let limited_vec = amplified_vec.simd_clamp(min_vec, max_vec);
            
            // Store results back
            limited_vec.copy_to_slice(chunk_data);
        }
        
        // Handle remainder samples with scalar processing
        for sample in &mut data[remainder_start..] {
            let amplified = *sample * gain;
            *sample = amplified.clamp(-1.0, 1.0);
        }
    }

    /// Fallback gain application for non-SIMD targets
    #[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
    fn apply_gain_with_limiting_simd(data: &mut [f32], gain: f32) {
        for sample in data {
            let amplified = *sample * gain;
            *sample = amplified.clamp(-1.0, 1.0);
        }
    }

    /// Calculates the RMS (Root Mean Square) value of the audio data
    fn calculate_rms(data: &[f32]) -> f32 {
        if data.is_empty() {
            return 0.0;
        }

        // TODO: Implement SIMD optimization for RMS calculation
        // - Process 8 samples at a time using f32x8 vectors
        // - Use SIMD multiplication for squaring: sample_vec * sample_vec
        // - Use horizontal sum reduction to accumulate squares efficiently
        // - Should provide 4-8x speedup for RMS calculation on large buffers
        // - Handle remainder samples with scalar code
        // - Maintain exact numerical compatibility with current RMS calculation

        #[allow(clippy::cast_precision_loss)]
        let sum_squares: f32 = data.iter().map(|&s| s * s).sum();

        // Use safe conversion for sample count, with fallback for very large arrays
        #[allow(clippy::cast_precision_loss)]
        let data_len_f32 = if data.len() <= (1_usize << 24) {
            data.len() as f32
        } else {
            // For extremely large arrays, use f64 intermediate calculation
            #[allow(clippy::cast_possible_truncation)]
            return (f64::from(sum_squares) / (data.len() as f64)).sqrt() as f32;
        };

        (sum_squares / data_len_f32).sqrt()
    }

    /// Calculates the peak level in dB
    #[must_use]
    pub fn calculate_peak_db(&self, buffer: &AudioBuffer<f32>) -> f32 {
        let max_sample = self.find_peak_simd(&buffer.data);

        if max_sample > 0.0 {
            20.0 * max_sample.log10()
        } else {
            -96.0 // Representing digital silence
        }
    }

    /// Calculates the RMS level in dB
    #[must_use]
    pub fn calculate_rms_db(&self, buffer: &AudioBuffer<f32>) -> f32 {
        let rms = Self::calculate_rms(&buffer.data);
        if rms > 0.0 {
            20.0 * rms.log10()
        } else {
            -96.0 // Representing digital silence
        }
    }

    /// Applies a simple limiter to prevent clipping
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::Normalizer`] if the limiting operation fails
    /// or buffer parameters are invalid.
    pub fn apply_limiter(&self, buffer: &mut AudioBuffer<f32>, threshold: f32) -> Result<()> {
        let threshold = threshold.clamp(0.1, 1.0);

        for sample in &mut buffer.data {
            *sample = sample.clamp(-threshold, threshold);
        }

        Ok(())
    }
}

impl AudioProcessor for AudioNormalizer {
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        self.normalize_buffer(buffer)
    }

    fn reset(&mut self) {
        // Normalizer is stateless, nothing to reset
    }
}

impl Configurable<NormalizerConfig> for AudioNormalizer {
    fn configure(&mut self, config: NormalizerConfig) -> Result<()> {
        ConfigValidator::validate_normalizer_config(&config)?;
        self.config = config;
        Ok(())
    }

    fn get_config(&self) -> &NormalizerConfig {
        &self.config
    }
}

impl LatencyReporting for AudioNormalizer {
    fn get_latency_samples(&self) -> usize {
        // Normalization has no latency for simple algorithms
        // LUFS normalization might require lookahead in a full implementation
        0
    }
}

impl Validatable for AudioNormalizer {
    fn validate(&self) -> Result<()> {
        ConfigValidator::validate_normalizer_config(&self.config).map_err(std::convert::Into::into)
    }
}

impl Default for AudioNormalizer {
    /// Creates a new audio normalizer with default configuration
    fn default() -> Self {
        Self {
            config: NormalizerConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::SampleFormat;
    use crate::test_utils::audio::create_test_buffer;

    #[test]
    fn test_normalizer_creation() {
        let config = NormalizerConfig {
            target_loudness: -16.0,
            ..Default::default()
        };
        let normalizer = AudioNormalizer::new(config);
        assert!(normalizer.is_ok());
    }

    #[test]
    fn test_peak_normalization() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(0.1)); // Quiet signal
        let config = NormalizerConfig {
            algorithm: super::super::config::NormalizationAlgorithm::Peak,
            target_loudness: -6.0, // Target -6dB
            headroom_db: 1.0,      // 1dB headroom
            ..Default::default()
        };

        let mut normalizer = AudioNormalizer::new(config).unwrap();
        let result = normalizer.process(&mut buffer);

        assert!(result.is_ok());

        // Check that the peak is now close to the target with headroom
        let max_sample = buffer.data.iter().fold(0.0f32, |max, &s| max.max(s.abs()));
        let peak_db = 20.0 * max_sample.log10();

        // Should be close to -7dB (-6dB target - 1dB headroom)
        assert!((peak_db - (-7.0)).abs() < 1.0);
    }

    #[test]
    fn test_rms_normalization() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(0.1)); // Quiet signal
        let config = NormalizerConfig {
            algorithm: super::super::config::NormalizationAlgorithm::Rms,
            target_loudness: -12.0,
            headroom_db: 2.0,
            ..Default::default()
        };

        let mut normalizer = AudioNormalizer::new(config).unwrap();
        let result = normalizer.process(&mut buffer);

        assert!(result.is_ok());

        // Check that the signal was amplified
        let max_sample = buffer.data.iter().fold(0.0f32, |max, &s| max.max(s.abs()));
        assert!(max_sample > 0.1); // Should be louder than original
    }

    #[test]
    fn test_empty_buffer_unchanged() {
        let mut buffer = AudioBuffer {
            data: Vec::new(),
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 1,
        };

        let config = NormalizerConfig::default();
        let mut normalizer = AudioNormalizer::new(config).unwrap();
        let result = normalizer.process(&mut buffer);

        assert!(result.is_ok());

        // Buffer should remain empty
        assert!(buffer.data.is_empty());
    }

    #[test]
    fn test_calculate_peak_db() {
        let buffer = create_test_buffer(44100, 1, 0.1, Some(0.5));
        let normalizer = AudioNormalizer::default();

        let peak_db = normalizer.calculate_peak_db(&buffer);
        let expected_db = 20.0 * 0.5f32.log10(); // About -6dB

        assert!((peak_db - expected_db).abs() < 0.1);
    }

    #[test]
    fn test_calculate_rms_db() {
        let buffer = create_test_buffer(44100, 1, 0.1, Some(0.5));
        let normalizer = AudioNormalizer::default();

        let rms_db = normalizer.calculate_rms_db(&buffer);

        // RMS of a sine wave is amplitude / sqrt(2)
        let expected_rms = 0.5 / (2.0_f32).sqrt();
        let expected_db = 20.0 * expected_rms.log10();

        assert!((rms_db - expected_db).abs() < 0.5);
    }

    #[test]
    fn test_limiter() {
        let mut buffer = create_test_buffer(44100, 1, 0.1, Some(1.5)); // Clipped signal
        let normalizer = AudioNormalizer::default();

        let result = normalizer.apply_limiter(&mut buffer, 0.9);
        assert!(result.is_ok());

        // Check that no sample exceeds the threshold
        let max_sample = buffer.data.iter().fold(0.0f32, |max, &s| max.max(s.abs()));
        assert!(max_sample <= 0.9);
    }

    #[test]
    fn test_with_peak_target() {
        let normalizer = AudioNormalizer::with_peak_target(-3.0);
        assert!(normalizer.is_ok());

        let normalizer = normalizer.unwrap();
        assert_eq!(normalizer.config.target_loudness, -3.0);
        assert!(matches!(
            normalizer.config.algorithm,
            super::super::config::NormalizationAlgorithm::Peak
        ));
    }

    #[test]
    fn test_with_rms_target() {
        let normalizer = AudioNormalizer::with_rms_target(-12.0);
        assert!(normalizer.is_ok());

        let normalizer = normalizer.unwrap();
        assert_eq!(normalizer.config.target_loudness, -12.0);
        assert!(matches!(
            normalizer.config.algorithm,
            super::super::config::NormalizationAlgorithm::Rms
        ));
    }

    #[test]
    fn test_empty_buffer() {
        let mut buffer = AudioBuffer {
            data: Vec::new(),
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 1,
        };

        let mut normalizer = AudioNormalizer::default();
        let result = normalizer.process(&mut buffer);

        assert!(result.is_ok());
        assert!(buffer.data.is_empty());
    }

    #[test]
    fn test_normalizer_latency() {
        let normalizer = AudioNormalizer::default();
        assert_eq!(normalizer.get_latency_samples(), 0);
    }

    #[test]
    fn test_normalizer_validation() {
        let normalizer = AudioNormalizer::default();
        assert!(normalizer.validate().is_ok());
    }
}
