//! Trait implementations for LinearResampler

use super::super::{
    config::ResamplerConfig,
    error::Result,
    traits::{AudioProcessor, Configurable, LatencyReporting, Validatable},
};
use super::{core::LinearResampler, types::ResampleBufferFn};

impl AudioProcessor for LinearResampler {
    fn process(&mut self, buffer: &mut crate::audio::AudioBuffer<f32>) -> Result<()> {
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
        super::super::validation::ConfigValidator::validate_resampler_config(&config)?;
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
        super::super::validation::ConfigValidator::validate_resampler_config(&self.config)
            .map_err(std::convert::Into::into)
    }
}

impl Default for LinearResampler {
    /// Creates a new linear resampler with default configuration
    fn default() -> Self {
        let config = ResamplerConfig::default();
        let resample_fn = ResampleBufferFn::default();
        Self {
            config,
            resample_fn,
        }
    }
}
