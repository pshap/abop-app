//! Trait implementations for AudioNormalizer

use super::super::{
    config::NormalizerConfig,
    error::Result,
    traits::{AudioProcessor, Configurable, LatencyReporting, Validatable},
};
use super::core::AudioNormalizer;

impl AudioProcessor for AudioNormalizer {
    fn process(&mut self, buffer: &mut crate::audio::AudioBuffer<f32>) -> Result<()> {
        self.normalize_buffer(buffer)
    }

    fn reset(&mut self) {
        // Normalizer is stateless, nothing to reset
    }
}

impl Configurable<NormalizerConfig> for AudioNormalizer {
    fn configure(&mut self, config: NormalizerConfig) -> Result<()> {
        super::super::validation::ConfigValidator::validate_normalizer_config(&config)?;
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
        super::super::validation::ConfigValidator::validate_normalizer_config(&self.config)
            .map_err(std::convert::Into::into)
    }
}

impl Default for AudioNormalizer {
    /// Creates a new audio normalizer with default configuration
    fn default() -> Self {
        Self {
            config: super::super::config::NormalizerConfig::default(),
        }
    }
}
