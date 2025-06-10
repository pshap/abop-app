//! Audio processing module
//!
//! This module provides audio processing components and utilities for
//! manipulating audio buffers, including resampling, normalization,
//! silence detection, and channel mixing.

/// Batch processor for handling multiple audio files.
pub mod batch_processor;
pub mod casting_utils;
/// Channel mixer configuration and implementation.
pub mod channel_mixer;
/// Audio processing configuration module.
pub mod config;
pub mod error;
/// File I/O operations for audio processing.
pub mod file_io;
/// Normalizer configuration and implementation.
pub mod normalizer;
/// Audio processing pipeline implementation.
pub mod pipeline;
/// Resampler configuration and implementation.
pub mod resampler;
/// Silence detector configuration and implementation.
pub mod silence_detector;
/// Processing traits and common interfaces.
pub mod traits;
/// Utility functions for audio processing.
pub mod utils;
/// Validation functionality for processing configuration.
pub mod validation;

// Re-export main types
pub use self::config::silence_detector::SilenceRemovalMode;
pub use self::pipeline::AudioProcessingPipeline;

// Re-export config types
pub use self::config::{
    ChannelMixerConfig, MixingAlgorithm, NormalizerConfig, OutputConfig, ProcessingConfig,
    ResamplerConfig, SilenceDetectorConfig,
};

// Re-export processor types
pub use self::channel_mixer::ChannelMixer;
pub use self::normalizer::AudioNormalizer;
pub use self::resampler::LinearResampler;
pub use self::silence_detector::SilenceDetector;

// Re-export validation types
pub use self::validation::ConfigValidator;

#[cfg(test)]
mod tests {
    use crate::AudioBuffer;
    use crate::audio::processing::error::Result;
    use crate::audio::processing::traits::{AudioProcessor, LatencyReporting, Validatable};
    use crate::test_utils::audio::create_test_buffer;

    /// A dummy processor for testing audio processing traits
    #[derive(Default)]
    struct DummyProcessor {
        process_count: usize,
    }

    impl AudioProcessor for DummyProcessor {
        fn process(&mut self, _buffer: &mut AudioBuffer<f32>) -> Result<()> {
            self.process_count += 1;
            Ok(())
        }

        fn reset(&mut self) {
            self.process_count = 0;
        }
    }

    impl LatencyReporting for DummyProcessor {
        fn get_latency_samples(&self) -> usize {
            0
        }
    }

    impl Validatable for DummyProcessor {
        fn validate(&self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_audio_processor_trait() -> Result<()> {
        let mut buffer = create_test_buffer(44100, 2, 0.1, Some(0.5));
        let mut processor = DummyProcessor::default();
        processor.process(&mut buffer)?;
        assert_eq!(processor.process_count, 1);
        Ok(())
    }

    #[test]
    fn test_audio_processor_reset() -> Result<()> {
        let mut processor = DummyProcessor::default();
        processor.reset();
        assert_eq!(processor.process_count, 0);
        Ok(())
    }

    #[test]
    fn test_audio_processor_validation() -> Result<()> {
        let processor = DummyProcessor::default();
        processor.validate()?;
        Ok(())
    }

    #[test]
    fn test_audio_processor_latency() {
        let processor = DummyProcessor::default();
        assert_eq!(processor.get_latency_samples(), 0);
    }
}
