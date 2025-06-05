//! Modular audio processing components
//!
//! This module provides a collection of specialized audio processing components
//! that can be used individually or combined to create complex audio processing
//! pipelines. Each component follows the Single Responsibility Principle and
//! implements common traits for consistency.

pub mod batch_processor;
/// Safe casting utilities for audio processing
pub mod casting_utils;
pub mod channel_mixer;
/// Configuration types and validation for audio processing
pub mod config;
/// Centralized error handling for audio processing
pub mod error;
pub mod file_io;
pub mod normalizer;
pub mod pipeline;
pub mod resampler;
pub mod silence_detector;
pub mod traits;
/// Shared utilities for audio processing
pub mod utils;
/// Configuration validation system
pub mod validation;

pub use channel_mixer::{ChannelMixer, ChannelMixerError};
pub use config::{
    ChannelMixerConfig, MixingAlgorithm, NormalizationAlgorithm, NormalizerConfig, OutputConfig,
    ProcessingConfig, ProcessingConfigBuilder, ResampleQuality, ResamplerConfig,
    SilenceDetectorConfig, SilenceRemovalMode,
};
pub use error::{AudioProcessingError, AudioProcessingResult};
pub use normalizer::{AudioNormalizer, NormalizerError};
pub use resampler::{LinearResampler, ResamplerError};
pub use silence_detector::{SilenceDetector, SilenceDetectorError, SilenceSegment};
pub use traits::*;
pub use utils::*;
pub use validation::ConfigValidator;

/// Re-export main types
pub use self::pipeline::AudioProcessingPipeline;

#[cfg(test)]
mod tests {
    use crate::AudioBuffer;
    // use std::path::Path; // Removed unused import

    use super::*;
    use crate::audio::SampleFormat;

    fn create_test_buffer(sample_rate: u32, channels: u16, duration_secs: f32) -> AudioBuffer<f32> {
        let num_samples = (sample_rate as f32 * duration_secs) as usize;
        let mut data = Vec::with_capacity(num_samples * channels as usize);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin() * 0.5;

            for _ in 0..channels {
                data.push(sample);
            }
        }

        AudioBuffer {
            data,
            format: SampleFormat::F32,
            sample_rate,
            channels,
        }
    }

    #[test]
    fn test_pipeline_creation() {
        let config = ProcessingConfig::default();
        let pipeline = AudioProcessingPipeline::new(config);
        assert!(pipeline.is_ok());
    }

    #[test]
    fn test_pipeline_default() {
        let pipeline = AudioProcessingPipeline::default();
        assert!(pipeline.validate().is_ok());
    }

    #[test]
    fn test_pipeline_with_settings() {
        let pipeline = AudioProcessingPipeline::with_settings(Some(48000), Some(1), true, false);
        assert!(pipeline.is_ok());
    }

    #[test]
    fn test_process_buffer() {
        let mut buffer = create_test_buffer(44100, 2, 0.1);
        let mut pipeline =
            AudioProcessingPipeline::with_settings(Some(22050), Some(1), true, false).unwrap();

        let result = pipeline.process_buffer(&mut buffer);
        assert!(result.is_ok());

        // Check that processing was applied
        assert_eq!(buffer.sample_rate, 22050);
        assert_eq!(buffer.channels, 1);
    }

    #[test]
    fn test_pipeline_reset() {
        let mut pipeline = AudioProcessingPipeline::default();
        pipeline.reset(); // Should not panic
        assert!(pipeline.validate().is_ok());
    }
    #[test]
    fn test_pipeline_configuration() {
        let mut pipeline = AudioProcessingPipeline::default();
        let new_config = ProcessingConfig {
            resampler: Some(ResamplerConfig {
                target_sample_rate: Some(48000),
                ..Default::default()
            }),
            ..Default::default()
        };

        let result = pipeline.configure(new_config.clone());
        assert!(result.is_ok());
        assert_eq!(
            pipeline
                .get_config()
                .resampler
                .as_ref()
                .unwrap()
                .target_sample_rate,
            Some(48000)
        );
    }

    // #[test]
    // fn test_determine_output_path() {
    //     let pipeline = AudioProcessingPipeline::default();
    //     let input_path = Path::new("test.wav");
    //
    //     let output_path = pipeline.determine_output_path(input_path);
    //     assert!(output_path.is_ok());
    //
    //     let output_path = output_path.unwrap();
    //     assert!(output_path.to_string_lossy().contains("_processed"));
    // }
}
