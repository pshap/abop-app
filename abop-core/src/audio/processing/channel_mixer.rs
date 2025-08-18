//! Audio channel mixing and conversion functionality
//!
//! This module provides channel conversion from stereo to mono audio only.

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

/// Audio channel mixer for converting stereo to mono
///
/// Supports conversion from stereo to mono audio formats using
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
            (2, 1) => self.stereo_to_mono(buffer),
            _ => Err(AudioProcessingError::ChannelMixer(format!(
                "Unsupported channel conversion: {} -> {}",
                buffer.channels, target_channels
            ))),
        }
    }

    // Mono to stereo conversion has been removed as per requirements

    /// Converts stereo audio to mono using the configured mixing algorithm
    fn stereo_to_mono(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if buffer.channels != 2 {
            return Err(AudioProcessingError::ChannelMixer(
                "Buffer must be stereo for stereo-to-mono conversion".to_string(),
            ));
        }

        let new_data = self.stereo_to_mono_simd(&buffer.data);

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

    /// SIMD-optimized stereo-to-mono conversion
    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
    fn stereo_to_mono_simd(&self, stereo_data: &[f32]) -> Vec<f32> {
        use std::simd::prelude::*;
        
        let mut new_data = Vec::with_capacity(stereo_data.len() / 2);
        
        const SIMD_LANES: usize = 8;
        let stereo_pairs_per_chunk = SIMD_LANES / 2; // 4 stereo pairs per chunk
        let samples_per_chunk = stereo_pairs_per_chunk * 2; // 8 samples per chunk
        
        let simd_chunks = stereo_data.len() / samples_per_chunk;
        let remainder_start = simd_chunks * samples_per_chunk;
        
        // SIMD processing for bulk stereo pairs
        for chunk_idx in 0..simd_chunks {
            let start = chunk_idx * samples_per_chunk;
            let chunk_data = &stereo_data[start..start + samples_per_chunk];
            
            // Separate left and right channel samples
            let mut left_samples = [0.0f32; 4];
            let mut right_samples = [0.0f32; 4];
            
            for i in 0..stereo_pairs_per_chunk {
                left_samples[i] = chunk_data[i * 2];
                right_samples[i] = chunk_data[i * 2 + 1];
            }
            
            // Convert to SIMD vectors (using f32x4 for 4 stereo pairs)
            let left_vec = f32x4::from_array(left_samples);
            let right_vec = f32x4::from_array(right_samples);
            
            // Apply mixing algorithm using SIMD
            let mono_vec = match self.config.mix_algorithm {
                super::config::MixingAlgorithm::Average => {
                    (left_vec + right_vec) * f32x4::splat(0.5)
                }
                super::config::MixingAlgorithm::LeftOnly => left_vec,
                super::config::MixingAlgorithm::RightOnly => right_vec,
                super::config::MixingAlgorithm::WeightedSum {
                    left_weight,
                    right_weight,
                } => {
                    left_vec * f32x4::splat(left_weight) + right_vec * f32x4::splat(right_weight)
                }
            };
            
            // Store results
            let mono_results = mono_vec.to_array();
            new_data.extend_from_slice(&mono_results);
        }
        
        // Handle remainder samples with scalar processing
        for chunk in stereo_data[remainder_start..].chunks(2) {
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
        
        new_data
    }

    /// Fallback stereo-to-mono conversion for non-SIMD targets
    #[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
    fn stereo_to_mono_simd(&self, stereo_data: &[f32]) -> Vec<f32> {
        let mut new_data = Vec::with_capacity(stereo_data.len() / 2);

        for chunk in stereo_data.chunks(2) {
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
        
        new_data
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
    use crate::test_utils::audio::{create_stereo_test_buffer, create_test_buffer};

    #[test]
    fn test_channel_mixer_creation() {
        let config = ChannelMixerConfig {
            target_channels: Some(2),
            ..Default::default()
        };
        let mixer = ChannelMixer::new(config);
        assert!(mixer.is_ok());
    }

    // Mono to stereo test removed as per requirements

    #[test]
    fn test_stereo_to_mono() {
        let mut buffer = create_stereo_test_buffer(44100, 0.1);
        let mut mixer = ChannelMixer::new(ChannelMixerConfig {
            target_channels: Some(1),
            ..Default::default()
        })
        .unwrap();

        let result = mixer.process(&mut buffer);
        assert!(result.is_ok());
        assert_eq!(buffer.channels, 1);
    }

    #[test]
    fn test_stereo_to_quad() {
        let mut buffer = create_stereo_test_buffer(44100, 0.1);
        let mut mixer = ChannelMixer::new(ChannelMixerConfig {
            target_channels: Some(4),
            ..Default::default()
        })
        .unwrap();

        let result = mixer.process(&mut buffer);
        // This should fail because stereo-to-quad conversion is not supported
        assert!(result.is_err());
        // Channel count should remain unchanged when conversion fails
        assert_eq!(buffer.channels, 2);
    }

    #[test]
    fn test_quad_to_stereo() {
        let mut buffer = create_test_buffer(44100, 4, 0.1, Some(0.5));
        let mut mixer = ChannelMixer::new(ChannelMixerConfig {
            target_channels: Some(2),
            ..Default::default()
        })
        .unwrap();

        let result = mixer.process(&mut buffer);
        // This should fail because quad-to-stereo conversion is not supported
        assert!(result.is_err());
        // Channel count should remain unchanged when conversion fails
        assert_eq!(buffer.channels, 4);
    }

    #[test]
    fn test_invalid_channel_count() {
        // This test should verify that creating a mixer with 0 channels fails during construction
        let result = ChannelMixer::new(ChannelMixerConfig {
            target_channels: Some(0),
            ..Default::default()
        });
        assert!(
            result.is_err(),
            "Creating mixer with 0 channels should fail during validation"
        );
    }

    #[test]
    fn test_mixer_reset() {
        let mut mixer = ChannelMixer::default();
        mixer.reset();
        assert!(mixer.validate().is_ok());
    }
}
