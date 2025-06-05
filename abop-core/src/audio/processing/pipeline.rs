//! Audio processing pipeline
//!
//! This module contains the core `AudioProcessingPipeline` struct and its
//! main coordination logic for combining multiple processing components.

use super::error::{AudioProcessingError, Result};
use super::traits::{AudioProcessor, LatencyReporting, Validatable};
use super::validation::ConfigValidator;
use super::{
    AudioNormalizer, ChannelMixer, ChannelMixerConfig, LinearResampler, NormalizerConfig,
    ProcessingConfig, ResamplerConfig, SilenceDetector, SilenceDetectorConfig,
};
use crate::audio::AudioBuffer;

/// Main audio processor that orchestrates all processing components.
///
/// This processor combines resampling, channel mixing, normalization, and
/// silence detection into a single, configurable processing pipeline.
#[derive(Debug, Clone)]
pub struct AudioProcessingPipeline {
    pub(super) resampler: LinearResampler,
    pub(super) channel_mixer: ChannelMixer,
    pub(super) normalizer: AudioNormalizer,
    pub(super) silence_detector: SilenceDetector,
    pub(super) config: ProcessingConfig,
}

impl AudioProcessingPipeline {
    /// Creates a new audio processing pipeline with the specified configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the configuration is invalid or if any processing component fails to initialize.
    /// This includes validation errors for sample rates, channel counts, or component-specific parameters.
    pub fn new(config: ProcessingConfig) -> Result<Self> {
        // Use centralized validation
        ConfigValidator::validate_config(&config)?;

        let resampler = LinearResampler::new(config.resampler.clone().unwrap_or_default())
            .map_err(|e| AudioProcessingError::Resampler(e.to_string()))?;
        let channel_mixer = ChannelMixer::new(config.channel_mixer.clone().unwrap_or_default())
            .map_err(|e| AudioProcessingError::ChannelMixer(e.to_string()))?;
        let normalizer = AudioNormalizer::new(config.normalizer.clone().unwrap_or_default())
            .map_err(|e| AudioProcessingError::Normalizer(e.to_string()))?;
        let silence_detector =
            SilenceDetector::new(config.silence_detector.clone().unwrap_or_default())
                .map_err(|e| AudioProcessingError::SilenceDetector(e.to_string()))?;
        Ok(Self {
            resampler,
            channel_mixer,
            normalizer,
            silence_detector,
            config,
        })
    }

    /// Process a file and save the result to the specified output path
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be processed or saved
    pub fn process_file_with_output<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
        &self,
        input_path: P,
        output_path: Q,
    ) -> Result<()> {
        use super::file_io::{AudioFileProcessor, FileProcessingOptions};

        let options = FileProcessingOptions::default();
        let mut processor = AudioFileProcessor::new(self.clone(), options);
        // Map the AppError to AudioProcessingError
        processor
            .process_file_with_output(input_path, output_path)
            .map_err(|e| AudioProcessingError::Pipeline(e.to_string()))
    }

    /// Process multiple files in parallel
    ///
    /// # Errors
    ///
    /// Returns an error if any file cannot be processed
    pub fn process_files<P: AsRef<std::path::Path> + Send + Sync>(
        &self,
        input_paths: &[P],
    ) -> Result<Vec<std::path::PathBuf>> {
        use super::batch_processor::BatchProcessor;
        use super::file_io::FileProcessingOptions;

        let options = FileProcessingOptions::default();
        let batch_processor = BatchProcessor::new(self.config.clone(), options)?;
        batch_processor.process_files(input_paths)
    }
    /// Creates a new pipeline with custom settings for common use cases
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the configuration is invalid or if any processing component fails to initialize.
    pub fn with_settings(
        target_sample_rate: Option<u32>,
        target_channels: Option<u16>,
        normalize: bool,
        remove_silence: bool,
    ) -> Result<Self> {
        let config = ProcessingConfig {
            resampler: if target_sample_rate.is_some() {
                Some(ResamplerConfig {
                    target_sample_rate,
                    ..Default::default()
                })
            } else {
                None
            },
            channel_mixer: if target_channels.is_some() {
                Some(ChannelMixerConfig {
                    target_channels,
                    ..Default::default()
                })
            } else {
                None
            },
            normalizer: if normalize {
                Some(NormalizerConfig::default())
            } else {
                None
            },
            silence_detector: if remove_silence {
                Some(SilenceDetectorConfig::default())
            } else {
                None
            },
            ..Default::default()
        };
        Self::new(config)
    }
    /// Processes an audio buffer through the entire pipeline
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if any processing step fails, such as:
    /// - Resampling failures due to invalid sample rates
    /// - Channel mixing errors for unsupported channel configurations
    /// - Normalization failures due to invalid audio data
    /// - Silence detection processing errors
    pub fn process_buffer(&mut self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        log::debug!(
            "Processing audio buffer: {} Hz, {} channels, {} samples",
            buffer.sample_rate,
            buffer.channels,
            buffer.data.len()
        );
        // Apply resampling first if needed
        if let Some(ref resampler_config) = self.config.resampler
            && resampler_config.target_sample_rate.is_some()
        {
            self.resampler.process(buffer)?;
            log::debug!(
                "After resampling: {} Hz, {} samples",
                buffer.sample_rate,
                buffer.data.len()
            );
        }
        // Apply channel mixing if needed
        if let Some(ref mixer_config) = self.config.channel_mixer
            && mixer_config.target_channels.is_some()
        {
            self.channel_mixer.process(buffer)?;
            log::debug!(
                "After channel mixing: {} channels, {} samples",
                buffer.channels,
                buffer.data.len()
            );
        }
        // Apply normalization if enabled
        if self.config.normalizer.is_some() {
            self.normalizer.process(buffer)?;
            log::debug!("After normalization: {} samples", buffer.data.len());
        }
        // Apply silence removal if enabled
        if self.config.silence_detector.is_some() {
            self.silence_detector.process(buffer)?;
            log::debug!("After silence removal: {} samples", buffer.data.len());
        }
        Ok(())
    }
    /// Gets the total latency in samples for all components
    #[must_use]
    pub fn get_total_latency_samples(&self) -> usize {
        let mut total_latency = 0;
        if self.config.resampler.is_some() {
            total_latency += self.resampler.get_latency_samples();
        }
        if self.config.channel_mixer.is_some() {
            total_latency += self.channel_mixer.get_latency_samples();
        }
        if self.config.normalizer.is_some() {
            total_latency += self.normalizer.get_latency_samples();
        }
        if self.config.silence_detector.is_some() {
            total_latency += self.silence_detector.get_latency_samples();
        }
        total_latency
    }
    /// Resets all processing components to their initial state
    pub fn reset(&mut self) {
        self.resampler.reset();
        self.channel_mixer.reset();
        self.normalizer.reset();
        self.silence_detector.reset();
    }
    /// Gets the current configuration
    #[must_use]
    pub const fn get_config(&self) -> &ProcessingConfig {
        &self.config
    }
    /// Updates the pipeline configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the new configuration is invalid or if any processing component fails to reinitialize.
    pub fn configure(&mut self, config: ProcessingConfig) -> Result<()> {
        config.validate()?;
        // Recreate components with new configuration
        if let Some(ref resampler_config) = config.resampler {
            self.resampler = LinearResampler::new(resampler_config.clone())?;
        }
        if let Some(ref mixer_config) = config.channel_mixer {
            self.channel_mixer = ChannelMixer::new(mixer_config.clone())?;
        }
        if let Some(ref normalizer_config) = config.normalizer {
            self.normalizer = AudioNormalizer::new(normalizer_config.clone())?;
        }
        if let Some(ref silence_config) = config.silence_detector {
            self.silence_detector = SilenceDetector::new(silence_config.clone())?;
        }
        self.config = config;
        Ok(())
    }
    /// Validates the current pipeline configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the current configuration is invalid.
    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        // Additional pipeline-specific validation could go here
        Ok(())
    }
}

impl Default for AudioProcessingPipeline {
    /// Creates a new audio processing pipeline with default configuration
    fn default() -> Self {
        let config = ProcessingConfig::default();
        Self::new(config).expect("Default configuration should always be valid")
    }
}
