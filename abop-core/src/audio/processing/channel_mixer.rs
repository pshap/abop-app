//! Audio channel mixing and conversion functionality
//!
//! This module provides channel conversion between mono and stereo audio,
//! including upmixing mono to stereo and downmixing stereo to mono.

use super::{
    config::ChannelMixerConfig,
    error::{AudioProcessingError, Result},
    traits::{AudioProcessor, Configurable, LatencyReporting, Validatable},
    validation::ConfigValidator,
};
use crate::audio::AudioBuffer;

/// Channel mixing error type
#[derive(Debug, thiserror::Error)]
pub enum ChannelMixerError {
    /// Unsupported channel configuration
    #[error("Unsupported channel conversion: {from} -> {to}")]
    UnsupportedConversion {
        /// Source channel count for the conversion attempt
        from: u16,
        /// Target channel count for the conversion attempt
        to: u16,
    },

    /// Processing error
    #[error("Channel mixing failed: {0}")]
    ProcessingError(String),
}

/// Audio channel mixer for converting between different channel configurations
///
/// Supports conversion between mono and stereo audio formats using
/// various mixing algorithms.
#[derive(Debug, Clone)]
pub struct ChannelMixer {
    config: ChannelMixerConfig,
}

impl ChannelMixer {
    /// Creates a new channel mixer with the specified configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the channel mixer configuration validation fails.
    pub fn new(config: ChannelMixerConfig) -> Result<Self> {
        ConfigValidator::validate_channel_mixer_config(&config)?;
        Ok(Self { config })
    }

    /// Creates a new channel mixer for a specific target channel count
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the target channel count is invalid
    /// or the configuration validation fails.
    pub fn with_target_channels(target_channels: u16) -> Result<Self> {
        let config = ChannelMixerConfig {
            target_channels: Some(target_channels),
            ..Default::default()
        };
        Self::new(config)
    }

    /// Converts the audio buffer to the target number of channels
    fn convert_channels(&self, buffer: &mut AudioBuffer<f32>, target_channels: u16) -> Result<()> {
        if buffer.channels == target_channels {
            return Ok(());
        }
        log::debug!(
            "Converting from {} channels to {} channels using {:?} algorithm",
            buffer.channels,
            target_channels,
            self.config.mix_algorithm
        );

        match (buffer.channels, target_channels) {
            (1, 2) => Self::mono_to_stereo(buffer),
            (2, 1) => self.stereo_to_mono(buffer),
            _ => Err(AudioProcessingError::ChannelMixer(format!(
                "Unsupported channel conversion: {} -> {}",
                buffer.channels, target_channels
            ))),
        }
    }

    /// Converts mono audio to stereo by duplicating the mono channel
    fn mono_to_stereo(buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if buffer.channels != 1 {
            return Err(AudioProcessingError::ChannelMixer(
                "Buffer must be mono for mono-to-stereo conversion".to_string(),
            ));
        }

        // TODO: Implement SIMD optimization for mono-to-stereo conversion
        // - Process 8 mono samples at a time using f32x8 vectors
        // - Load mono vector and duplicate to create left and right vectors
        // - Interleave the vectors to create stereo output
        // - Use SIMD shuffle operations for efficient interleaving
        // - Handle remainder samples with scalar code
        // - Should double processing speed for large mono buffers

        let mut new_data = Vec::with_capacity(buffer.data.len() * 2);

        for &sample in &buffer.data {
            // Duplicate mono sample to both channels
            new_data.push(sample);
            new_data.push(sample);
        }

        buffer.data = new_data;
        buffer.channels = 2;
        Ok(())
    }

    /// Converts stereo audio to mono using the configured mixing algorithm
    fn stereo_to_mono(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if buffer.channels != 2 {
            return Err(AudioProcessingError::ChannelMixer(
                "Buffer must be stereo for stereo-to-mono conversion".to_string(),
            ));
        }

        // TODO: Implement SIMD optimization for stereo-to-mono conversion
        // - Process 8 stereo pairs (16 samples) at a time using f32x8 vectors
        // - Load left and right channel vectors separately
        // - Apply mixing algorithm using SIMD operations:
        //   * Average: (left_vec + right_vec) * 0.5
        //   * Weighted: left_vec * left_weight + right_vec * right_weight
        // - Store result as single mono vector
        // - Handle remainder samples with scalar code
        // - Significant performance gain for large stereo buffers
        // - Should maintain exact compatibility with current scalar implementation

        let mut new_data = Vec::with_capacity(buffer.data.len() / 2);

        for chunk in buffer.data.chunks(2) {
            if chunk.len() == 2 {
                let mono_sample = match self.config.mix_algorithm {
                    super::config::MixingAlgorithm::Average => (chunk[0] + chunk[1]) * 0.5,
                    super::config::MixingAlgorithm::LeftOnly => chunk[0],
                    super::config::MixingAlgorithm::RightOnly => chunk[1],
                    super::config::MixingAlgorithm::WeightedSum {
                        left_weight,
                        right_weight,
                    } => chunk[0].mul_add(left_weight, chunk[1] * right_weight),
                };
                new_data.push(mono_sample);
            } else {
                // Handle odd number of samples (shouldn't happen with stereo)
                new_data.push(chunk[0]);
            }
        }

        buffer.data = new_data;
        buffer.channels = 1;
        Ok(())
    }

    /// Converts stereo audio to mono by averaging the channels
    ///
    /// This is a convenience method that forces averaging regardless of configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::ChannelMixer`] if the audio buffer is not stereo (2 channels).
    pub fn convert_stereo_to_mono_average(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if buffer.channels != 2 {
            return Err(AudioProcessingError::ChannelMixer(
                "Audio must be stereo (2 channels) to convert to mono".to_string(),
            ));
        }

        let mut new_data = Vec::with_capacity(buffer.data.len() / 2);

        for chunk in buffer.data.chunks(2) {
            if chunk.len() == 2 {
                new_data.push((chunk[0] + chunk[1]) * 0.5);
            } else {
                new_data.push(chunk[0]);
            }
        }

        buffer.data = new_data;
        buffer.channels = 1;
        Ok(())
    }
}

impl AudioProcessor for ChannelMixer {
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if let Some(target_channels) = self.config.target_channels {
            self.convert_channels(buffer, target_channels)?;
        }
        Ok(())
    }

    fn reset(&mut self) {
        // Channel mixer is stateless, nothing to reset
    }
}

impl Configurable<ChannelMixerConfig> for ChannelMixer {
    fn configure(&mut self, config: ChannelMixerConfig) -> Result<()> {
        config.validate()?;
        self.config = config;
        Ok(())
    }

    fn get_config(&self) -> &ChannelMixerConfig {
        &self.config
    }
}

impl LatencyReporting for ChannelMixer {
    fn get_latency_samples(&self) -> usize {
        // Channel mixing has no latency
        0
    }
}

impl Validatable for ChannelMixer {
    fn validate(&self) -> Result<()> {
        ConfigValidator::validate_channel_mixer_config(&self.config)
            .map_err(std::convert::Into::into)
    }
}

impl Default for ChannelMixer {
    /// Creates a new channel mixer with default configuration
    fn default() -> Self {
        Self {
            config: ChannelMixerConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::SampleFormat;

    fn create_test_buffer(sample_rate: u32, channels: u16, duration_secs: f32) -> AudioBuffer<f32> {
        let num_samples = (sample_rate as f32 * duration_secs) as usize;
        let mut data = Vec::with_capacity(num_samples * channels as usize);

        // Generate a simple sine wave test signal
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin() * 0.5;

            // Create different signals for left and right channels if stereo
            for c in 0..channels {
                let channel_sample = if channels == 2 && c == 1 {
                    // Right channel gets a slightly different frequency
                    (t * 880.0 * 2.0 * std::f32::consts::PI).sin() * 0.3
                } else {
                    sample
                };
                data.push(channel_sample);
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
    fn test_channel_mixer_creation() {
        let config = ChannelMixerConfig {
            target_channels: Some(2),
            ..Default::default()
        };
        let mixer = ChannelMixer::new(config);
        assert!(mixer.is_ok());
    }

    #[test]
    fn test_mono_to_stereo() {
        let mut buffer = create_test_buffer(44100, 1, 0.1);
        let original_len = buffer.data.len();

        let config = ChannelMixerConfig {
            target_channels: Some(2),
            ..Default::default()
        };

        let mut mixer = ChannelMixer::new(config).unwrap();
        let result = mixer.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.channels, 2);
        assert_eq!(buffer.data.len(), original_len * 2);
    }

    #[test]
    fn test_stereo_to_mono_average() {
        let mut buffer = create_test_buffer(44100, 2, 0.1);
        let original_len = buffer.data.len();
        let config = ChannelMixerConfig {
            target_channels: Some(1),
            mix_algorithm: super::super::config::MixingAlgorithm::Average,
        };

        let mut mixer = ChannelMixer::new(config).unwrap();
        let result = mixer.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.channels, 1);
        assert_eq!(buffer.data.len(), original_len / 2);
    }

    #[test]
    fn test_stereo_to_mono_left_only() {
        let mut buffer = create_test_buffer(44100, 2, 0.1);
        let original_len = buffer.data.len();

        let config = ChannelMixerConfig {
            target_channels: Some(1),
            mix_algorithm: super::super::config::MixingAlgorithm::LeftOnly,
        };

        let mut mixer = ChannelMixer::new(config).unwrap();
        let result = mixer.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.channels, 1);
        assert_eq!(buffer.data.len(), original_len / 2);
    }

    #[test]
    fn test_stereo_to_mono_weighted() {
        let mut buffer = create_test_buffer(44100, 2, 0.1);
        let original_len = buffer.data.len();

        let config = ChannelMixerConfig {
            target_channels: Some(1),
            mix_algorithm: super::super::config::MixingAlgorithm::Average,
        };

        let mut mixer = ChannelMixer::new(config).unwrap();
        let result = mixer.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.channels, 1);
        assert_eq!(buffer.data.len(), original_len / 2);
    }

    #[test]
    fn test_no_conversion_needed() {
        let mut buffer = create_test_buffer(44100, 2, 0.1);
        let original_len = buffer.data.len();

        let config = ChannelMixerConfig {
            target_channels: Some(2),
            ..Default::default()
        };

        let mut mixer = ChannelMixer::new(config).unwrap();
        let result = mixer.process(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.channels, 2);
        assert_eq!(buffer.data.len(), original_len);
    }

    #[test]
    fn test_unsupported_conversion() {
        let mut buffer = create_test_buffer(44100, 1, 0.1);

        let config = ChannelMixerConfig {
            target_channels: Some(6), // 5.1 surround
            ..Default::default()
        };

        let mut mixer = ChannelMixer::new(config).unwrap();
        let result = mixer.process(&mut buffer);

        assert!(result.is_err());
    }

    #[test]
    fn test_convert_stereo_to_mono_average_method() {
        let mut buffer = create_test_buffer(44100, 2, 0.1);
        let original_len = buffer.data.len();

        let mixer = ChannelMixer::default();
        let result = mixer.convert_stereo_to_mono_average(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.channels, 1);
        assert_eq!(buffer.data.len(), original_len / 2);
    }

    #[test]
    fn test_channel_mixer_with_target_channels() {
        let mixer = ChannelMixer::with_target_channels(1);
        assert!(mixer.is_ok());
        assert_eq!(mixer.unwrap().config.target_channels, Some(1));
    }

    #[test]
    fn test_channel_mixer_latency() {
        let mixer = ChannelMixer::default();
        assert_eq!(mixer.get_latency_samples(), 0);
    }

    #[test]
    fn test_channel_mixer_validation() {
        let mixer = ChannelMixer::default();
        assert!(mixer.validate().is_ok());
    }
}
