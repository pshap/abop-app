//! Core linear resampler struct and basic functionality

use super::super::{
    config::ResamplerConfig,
    error::{AudioProcessingError, Result},
    validation::ConfigValidator,
};
use super::types::ResampleBufferFn;
use crate::audio::AudioBuffer;

/// Simple linear resampler for audio sample rate conversion
///
/// This resampler uses linear interpolation to convert between sample rates.
/// While not production-quality, it's suitable for testing and basic audio
/// processing tasks.
#[derive(Debug, Clone)]
pub struct LinearResampler {
    pub(crate) config: ResamplerConfig,
    pub(crate) resample_fn: ResampleBufferFn,
}

// Note: Lookup table caching for resampling would use HashMap<(u32, u32), f64> for ratios
// Medium priority optimization: Cache common sample rate conversions (22050, 44100, 48000, 96000)
// Implementation would pre-compute conversion ratios and interpolation coefficients
// Performance improvement: Faster resampling for repeated conversions between same rates
// - Use OnceLock for thread-safe global cache initialization
// - Should reduce computation overhead for repeated conversions
// - Integrate with existing LinearResampler::resample_buffer methods
// - Maintain compatibility with current resampling accuracy

impl LinearResampler {
    /// Creates a new linear resampler with specified configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if resampler configuration validation fails.
    pub fn new(config: ResamplerConfig) -> Result<Self> {
        ConfigValidator::validate_resampler_config(&config)?;

        let resample_fn = if cfg!(feature = "simd") {
            ResampleBufferFn::Simd
        } else {
            ResampleBufferFn::Scalar
        };

        Ok(Self {
            config,
            resample_fn,
        })
    }

    /// Creates a new linear resampler for a specific target sample rate
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if target sample rate is invalid
    /// or configuration validation fails.
    pub fn with_target_rate(target_rate: u32) -> Result<Self> {
        let config = ResamplerConfig {
            target_sample_rate: Some(target_rate),
            ..Default::default()
        };
        Self::new(config)
    }

    /// Resamples audio buffer to target sample rate using appropriate implementation
    ///
    /// # Errors
    ///
    /// Returns an error if sample rate conversion calculations overflow or sample rates are invalid.
    pub fn resample_buffer(buffer: &mut AudioBuffer<f32>, target_rate: u32) -> Result<()> {
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

        match ResampleBufferFn::default() {
            ResampleBufferFn::Simd => {
                log::trace!("Using SIMD-accelerated resampling");
                super::simd::resample_buffer_simd(buffer, target_rate)
            }
            ResampleBufferFn::Scalar => {
                log::trace!("Using scalar resampling (SIMD not enabled)");
                super::scalar::resample_buffer_scalar(buffer, target_rate)
            }
        }
    }

    /// Get the current resample function type
    pub fn resample_fn(&self) -> ResampleBufferFn {
        self.resample_fn
    }

    /// Set the resample function type (for testing purposes)
    #[cfg(test)]
    pub fn set_resample_fn(&mut self, fn_type: ResampleBufferFn) {
        self.resample_fn = fn_type;
    }
}
