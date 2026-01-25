//! Core audio normalizer struct and basic functionality

use super::super::{config::NormalizerConfig, error::Result, validation::ConfigValidator};
use crate::audio::AudioBuffer;

/// Audio normalizer for volume normalization
///
/// Supports various normalization algorithms including peak normalization
/// and RMS-based normalization with configurable target levels and headroom.
#[derive(Debug, Clone)]
pub struct AudioNormalizer {
    pub(crate) config: NormalizerConfig,
}

impl AudioNormalizer {
    /// Creates a new audio normalizer with the specified configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if normalizer configuration validation fails.
    pub fn new(config: NormalizerConfig) -> Result<Self> {
        ConfigValidator::validate_normalizer_config(&config)?;
        Ok(Self { config })
    }

    /// Creates a new audio normalizer with peak normalization to a specific level
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if target level is invalid or configuration validation fails.
    pub fn with_peak_target(target_db: f32) -> Result<Self> {
        let config = NormalizerConfig {
            algorithm: super::super::config::NormalizationAlgorithm::Peak,
            target_loudness: target_db,
            ..Default::default()
        };
        Self::new(config)
    }

    /// Creates a new audio normalizer with RMS normalization to a specific level
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if target level is invalid or configuration validation fails.
    pub fn with_rms_target(target_db: f32) -> Result<Self> {
        let config = NormalizerConfig {
            algorithm: super::super::config::NormalizationAlgorithm::Rms,
            target_loudness: target_db,
            ..Default::default()
        };
        Self::new(config)
    }

    /// Normalizes audio buffer according to the configuration
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Always returns `Ok(())` as this operation cannot fail
    ///
    /// Note: The `Result` return type is maintained for compatibility with the `AudioProcessor` trait.
    #[allow(clippy::unnecessary_wraps)]
    pub fn normalize_buffer(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if buffer.data.is_empty() {
            return Ok(());
        }

        match self.config.algorithm {
            super::super::config::NormalizationAlgorithm::Peak => {
                self.normalize_peak(buffer);
            }
            super::super::config::NormalizationAlgorithm::Rms => {
                self.normalize_rms(buffer);
            }
            super::super::config::NormalizationAlgorithm::Lufs => {
                self.normalize_lufs(buffer);
            }
        }

        Ok(())
    }

    /// Applies peak normalization to the buffer
    fn normalize_peak(&self, buffer: &mut AudioBuffer<f32>) {
        let max_sample = super::peak_detection::find_peak_simd(&buffer.data);
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

            // Note: SIMD optimization for gain application would use portable_simd feature
            // Implementation would use f32x8 vectors for parallel gain and limiting operations
            // Performance improvement: Significant speedup for audio normalization
            // Currently using scalar implementation for compatibility
            // - Maintains exact compatibility with current limiting behavior
        }
    }

    /// Applies RMS normalization to the buffer
    fn normalize_rms(&self, buffer: &mut AudioBuffer<f32>) {
        let rms = super::rms_calculation::calculate_rms_simd(&buffer.data);
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
            super::gain_application::apply_gain_with_limiting_simd(&mut buffer.data, gain);
        }
    }

    /// Applies LUFS normalization to the buffer (simplified implementation)
    fn normalize_lufs(&self, buffer: &mut AudioBuffer<f32>) {
        self.normalize_rms(buffer);
    }

    /// Calculates the peak level in dB
    #[must_use]
    pub fn calculate_peak_db(&self, buffer: &AudioBuffer<f32>) -> f32 {
        let max_sample = super::peak_detection::find_peak_simd(&buffer.data);

        if max_sample > 0.0 {
            20.0 * max_sample.log10()
        } else {
            -96.0 // Representing digital silence
        }
    }

    /// Calculates the RMS level in dB
    #[must_use]
    pub fn calculate_rms_db(&self, buffer: &AudioBuffer<f32>) -> f32 {
        let rms = super::rms_calculation::calculate_rms(&buffer.data);
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
    /// Returns [`AudioProcessingError::Normalizer`] if limiting operation fails
    /// or buffer parameters are invalid.
    pub fn apply_limiter(&self, buffer: &mut AudioBuffer<f32>, threshold: f32) -> Result<()> {
        let threshold = threshold.clamp(0.1, 1.0);

        for sample in &mut buffer.data {
            *sample = sample.clamp(-threshold, threshold);
        }

        Ok(())
    }

    /// Gets current configuration
    #[allow(dead_code)]
    pub(crate) fn config(&self) -> &NormalizerConfig {
        &self.config
    }
}
