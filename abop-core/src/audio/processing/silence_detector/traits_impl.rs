//! Trait implementations for SilenceDetector

use super::{
    super::{
        error::Result,
        traits::{AudioProcessor, Configurable, LatencyReporting, Validatable},
        validation::ConfigValidator,
    },
    core::SilenceDetector,
};

impl AudioProcessor for SilenceDetector {
    fn process(&mut self, buffer: &mut crate::audio::AudioBuffer<f32>) -> Result<()> {
        self.process_silence(buffer)
    }

    fn reset(&mut self) {
        // Silence detector is stateless, nothing to reset
    }
}

impl Configurable<super::super::config::SilenceDetectorConfig> for SilenceDetector {
    fn configure(&mut self, config: super::super::config::SilenceDetectorConfig) -> Result<()> {
        ConfigValidator::validate_silence_detector_config(&config)?;
        self.config = config;
        Ok(())
    }

    fn get_config(&self) -> &super::super::config::SilenceDetectorConfig {
        &self.config
    }
}

impl LatencyReporting for SilenceDetector {
    fn get_latency_samples(&self) -> usize {
        // Silence detection has no latency
        0
    }
}

impl Validatable for SilenceDetector {
    fn validate(&self) -> Result<()> {
        ConfigValidator::validate_silence_detector_config(&self.config)
            .map_err(std::convert::Into::into)
    }
}
