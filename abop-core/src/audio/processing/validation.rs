use super::{
    config::{
        ChannelMixerConfig, NormalizerConfig, ProcessingConfig, ResamplerConfig,
        SilenceDetectorConfig,
    },
    utils::{channels::validate_channels, sample_rate::validate_sample_rate},
};
use crate::error::{AppError, Result as AppResult};

/// Centralized configuration validation for audio processing
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate a complete processing configuration (alias for `validate_processing_config`)
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if any configuration parameter is invalid or if configurations are incompatible.
    pub fn validate_config(config: &ProcessingConfig) -> AppResult<()> {
        Self::validate_processing_config(config)
    }

    /// Validate a complete processing configuration
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if any configuration parameter is invalid, such as:
    /// - Invalid sample rates or channel counts
    /// - Thread count of zero or excessive values
    /// - Invalid normalization targets or mixing weights
    /// - Incompatible configuration combinations
    pub fn validate_processing_config(config: &ProcessingConfig) -> AppResult<()> {
        // Validate individual processor configurations
        if let Some(ref resampler) = config.resampler {
            Self::validate_resampler_config(resampler)?;
        }

        if let Some(ref mixer) = config.channel_mixer {
            Self::validate_channel_mixer_config(mixer)?;
        }

        if let Some(ref normalizer) = config.normalizer {
            Self::validate_normalizer_config(normalizer)?;
        }

        if let Some(ref detector) = config.silence_detector {
            Self::validate_silence_detector_config(detector)?;
        }

        // Validate thread configuration
        if let Some(threads) = config.num_threads {
            if threads == 0 {
                return Err(AppError::Audio(
                    "Number of threads must be greater than 0".to_string(),
                ));
            }
            if threads > 64 {
                log::warn!("Very high thread count specified: {threads}");
            }
        }

        // Validate configuration compatibility
        Self::validate_config_compatibility(config)?;

        Ok(())
    }

    /// Validate resampler configuration
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if the target sample rate is invalid or unsupported.
    pub fn validate_resampler_config(config: &ResamplerConfig) -> AppResult<()> {
        if let Some(rate) = config.target_sample_rate {
            validate_sample_rate(rate)?;
        }
        Ok(())
    }
    /// Validate channel mixer configuration
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if the target channel count is invalid or if mixing weights are outside the valid range (0.0-1.0).
    pub fn validate_channel_mixer_config(config: &ChannelMixerConfig) -> AppResult<()> {
        if let Some(channels) = config.target_channels {
            validate_channels(channels)?;
        }

        // Validate mixing algorithm parameters
        if let super::config::MixingAlgorithm::WeightedSum {
            left_weight,
            right_weight,
        } = config.mix_algorithm
        {
            if !(0.0..=1.0).contains(&left_weight) {
                return Err(AppError::Audio(format!(
                    "Left weight must be between 0.0 and 1.0, got: {left_weight}"
                )));
            }
            if !(0.0..=1.0).contains(&right_weight) {
                return Err(AppError::Audio(format!(
                    "Right weight must be between 0.0 and 1.0, got: {right_weight}"
                )));
            }
            
            // Validate that the sum of weights is reasonable to prevent audio clipping.
            // Rationale: In weighted sum mixing, the output amplitude is:
            // output = (left_channel * left_weight) + (right_channel * right_weight)
            // If both input channels are at maximum amplitude (1.0) and weight_sum > 1.0,
            // the output can exceed the maximum representable value, causing clipping distortion.
            // This validation ensures the mixed output stays within the valid audio range.
            let weight_sum = left_weight + right_weight;
            if weight_sum > 1.0 {
                return Err(AppError::Audio(format!(
                    "Sum of mixing weights ({weight_sum:.2}) exceeds 1.0, which may cause clipping. \
                     Individual weights: left={left_weight:.2}, right={right_weight:.2}. \
                     Consider reducing one or both weights to keep the sum ≤ 1.0"
                )));
            }
        } else {
            // Other algorithms don't require additional validation
        }

        Ok(())
    }

    /// Validate normalizer configuration
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if the target loudness is positive, peak level is positive, or headroom is negative.
    pub fn validate_normalizer_config(config: &NormalizerConfig) -> AppResult<()> {
        // Validate target loudness
        if config.target_loudness > 0.0 {
            return Err(AppError::Audio(format!(
                "Target loudness must be negative, got: {} LUFS",
                config.target_loudness
            )));
        }
        if config.target_loudness < -70.0 {
            log::warn!(
                "Very low target loudness specified: {} LUFS",
                config.target_loudness
            );
        }

        // Validate peak level
        if config.peak_level > 0.0 {
            return Err(AppError::Audio(format!(
                "Peak level must be negative or zero, got: {} dB",
                config.peak_level
            )));
        }

        // Validate headroom
        if config.headroom_db < 0.0 {
            return Err(AppError::Audio(format!(
                "Headroom must be positive, got: {} dB",
                config.headroom_db
            )));
        }

        Ok(())
    }

    /// Validate silence detector configuration
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if the threshold is positive, minimum duration is zero, or fade duration exceeds minimum duration.
    pub fn validate_silence_detector_config(config: &SilenceDetectorConfig) -> AppResult<()> {
        // Validate threshold (should be negative in dB)
        if config.threshold_db > 0.0 {
            return Err(AppError::Audio(format!(
                "Silence threshold must be negative, got: {} dB",
                config.threshold_db
            )));
        }

        // Validate minimum duration
        if config.min_duration.as_millis() == 0 {
            return Err(AppError::Audio(
                "Minimum silence duration must be greater than 0".to_string(),
            ));
        }

        if config.min_duration.as_secs() > 60 {
            log::warn!(
                "Very long minimum silence duration: {:?}",
                config.min_duration
            );
        }

        // Validate fade duration relative to min duration
        if config.fade_duration > config.min_duration {
            return Err(AppError::Audio(
                "Fade duration cannot be longer than minimum silence duration".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate configuration compatibility between different processors
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if processor configurations are incompatible with each other.
    pub fn validate_config_compatibility(config: &ProcessingConfig) -> AppResult<()> {
        // Check resampler and normalizer compatibility
        if let (Some(resampler), Some(_normalizer)) = (&config.resampler, &config.normalizer)
            && let Some(target_rate) = resampler.target_sample_rate
            && target_rate < 8000
        {
            log::warn!("Low sample rate may affect normalization accuracy: {target_rate} Hz");
        }

        // Check channel mixer and silence detector compatibility
        if let (Some(mixer), Some(_silence)) = (&config.channel_mixer, &config.silence_detector)
            && let Some(target_channels) = mixer.target_channels
            && target_channels == 1
        {
            log::info!("Mono output selected - silence detection will work on mono signal");
        }

        Ok(())
    }

    /// Get recommended configuration for common use cases
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Audio`] if the specified use case is not recognized.
    pub fn get_recommended_config(use_case: &str) -> AppResult<ProcessingConfig> {
        match use_case.to_lowercase().as_str() {
            "podcast" => Ok(ProcessingConfig {
                resampler: Some(super::config::ResamplerConfig {
                    target_sample_rate: Some(44100),
                    ..Default::default()
                }),
                channel_mixer: Some(super::config::ChannelMixerConfig {
                    target_channels: Some(1), // Mono for podcasts
                    ..Default::default()
                }),
                normalizer: Some(super::config::NormalizerConfig {
                    target_loudness: -16.0, // Standard for podcasts
                    ..Default::default()
                }),
                silence_detector: Some(super::config::SilenceDetectorConfig {
                    threshold_db: -45.0,
                    min_duration: std::time::Duration::from_millis(1000),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            "music" => Ok(ProcessingConfig {
                resampler: Some(super::config::ResamplerConfig {
                    target_sample_rate: Some(44100),
                    quality: super::config::ResampleQuality::High,
                    ..Default::default()
                }),
                channel_mixer: Some(super::config::ChannelMixerConfig {
                    target_channels: Some(2), // Keep as stereo for music processing
                    ..Default::default()
                }),
                normalizer: Some(super::config::NormalizerConfig {
                    target_loudness: -14.0, // Standard for music
                    use_peak_normalization: false,
                    ..Default::default()
                }),
                silence_detector: None, // Usually don't remove silence from music
                ..Default::default()
            }),
            "voice" => Ok(ProcessingConfig {
                resampler: Some(super::config::ResamplerConfig {
                    target_sample_rate: Some(16000), // Lower rate for voice
                    ..Default::default()
                }),
                channel_mixer: Some(super::config::ChannelMixerConfig {
                    target_channels: Some(1), // Mono for voice
                    ..Default::default()
                }),
                normalizer: Some(super::config::NormalizerConfig {
                    target_loudness: -20.0,
                    ..Default::default()
                }),
                silence_detector: Some(super::config::SilenceDetectorConfig {
                    threshold_db: -50.0,
                    min_duration: std::time::Duration::from_millis(500),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            _ => Err(AppError::Audio(format!("Unknown use case: {use_case}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::config::MixingAlgorithm;
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_validate_processing_config_valid() {
        let config = ProcessingConfig {
            resampler: Some(ResamplerConfig {
                target_sample_rate: Some(44100),
                ..Default::default()
            }),
            channel_mixer: Some(ChannelMixerConfig {
                target_channels: Some(2),
                ..Default::default()
            }),
            normalizer: Some(NormalizerConfig {
                target_loudness: -23.0,
                ..Default::default()
            }),
            silence_detector: Some(SilenceDetectorConfig {
                threshold_db: -45.0,
                min_duration: Duration::from_millis(1000),
                ..Default::default()
            }),
            num_threads: Some(4),
            ..Default::default()
        };

        assert!(ConfigValidator::validate_processing_config(&config).is_ok());
    }

    #[test]
    fn test_validate_processing_config_invalid_threads() {
        let config = ProcessingConfig {
            num_threads: Some(0), // Invalid: zero threads
            ..Default::default()
        };

        assert!(ConfigValidator::validate_processing_config(&config).is_err());
    }

    #[test]
    fn test_validate_resampler_config() {
        let valid_config = ResamplerConfig {
            target_sample_rate: Some(44100),
            ..Default::default()
        };
        assert!(ConfigValidator::validate_resampler_config(&valid_config).is_ok());

        let invalid_config = ResamplerConfig {
            target_sample_rate: Some(0), // Invalid sample rate
            ..Default::default()
        };
        assert!(ConfigValidator::validate_resampler_config(&invalid_config).is_err());
    }
    #[test]
    fn test_validate_channel_mixer_config() {
        let valid_config = ChannelMixerConfig {
            target_channels: Some(2),
            mix_algorithm: MixingAlgorithm::WeightedSum {
                left_weight: 0.7,
                right_weight: 0.3,
            },
        };
        assert!(ConfigValidator::validate_channel_mixer_config(&valid_config).is_ok());
        let invalid_config = ChannelMixerConfig {
            mix_algorithm: MixingAlgorithm::WeightedSum {
                left_weight: 1.5, // Invalid: > 1.0
                right_weight: 0.3,
            },
            ..Default::default()
        };
        assert!(ConfigValidator::validate_channel_mixer_config(&invalid_config).is_err());
        
        // Test sum validation - weights individually valid but sum > 1.0
        let invalid_sum_config = ChannelMixerConfig {
            mix_algorithm: MixingAlgorithm::WeightedSum {
                left_weight: 0.7,
                right_weight: 0.8, // Sum = 1.5 > 1.0, may cause clipping
            },
            ..Default::default()
        };
        assert!(ConfigValidator::validate_channel_mixer_config(&invalid_sum_config).is_err());
    }

    #[test]
    fn test_validate_normalizer_config() {
        let valid_config = NormalizerConfig {
            target_loudness: -23.0,
            peak_level: -1.0,
            headroom_db: 1.0,
            ..Default::default()
        };
        assert!(ConfigValidator::validate_normalizer_config(&valid_config).is_ok());

        let invalid_loudness = NormalizerConfig {
            target_loudness: 5.0, // Invalid: positive loudness
            ..Default::default()
        };
        assert!(ConfigValidator::validate_normalizer_config(&invalid_loudness).is_err());

        let invalid_peak = NormalizerConfig {
            peak_level: 2.0, // Invalid: positive peak level
            ..Default::default()
        };
        assert!(ConfigValidator::validate_normalizer_config(&invalid_peak).is_err());

        let invalid_headroom = NormalizerConfig {
            headroom_db: -1.0, // Invalid: negative headroom
            ..Default::default()
        };
        assert!(ConfigValidator::validate_normalizer_config(&invalid_headroom).is_err());
    }

    #[test]
    fn test_validate_silence_detector_config() {
        let valid_config = SilenceDetectorConfig {
            threshold_db: -45.0,
            min_duration: Duration::from_millis(1000),
            fade_duration: Duration::from_millis(100),
            ..Default::default()
        };
        assert!(ConfigValidator::validate_silence_detector_config(&valid_config).is_ok());

        let invalid_threshold = SilenceDetectorConfig {
            threshold_db: 5.0, // Invalid: positive threshold
            ..Default::default()
        };
        assert!(ConfigValidator::validate_silence_detector_config(&invalid_threshold).is_err());

        let invalid_duration = SilenceDetectorConfig {
            min_duration: Duration::from_millis(0), // Invalid: zero duration
            ..Default::default()
        };
        assert!(ConfigValidator::validate_silence_detector_config(&invalid_duration).is_err());

        let invalid_fade = SilenceDetectorConfig {
            min_duration: Duration::from_millis(100),
            fade_duration: Duration::from_millis(200), // Invalid: fade > min_duration
            ..Default::default()
        };
        assert!(ConfigValidator::validate_silence_detector_config(&invalid_fade).is_err());
    }

    #[test]
    fn test_config_compatibility() {
        let config = ProcessingConfig {
            resampler: Some(ResamplerConfig {
                target_sample_rate: Some(44100),
                ..Default::default()
            }),
            normalizer: Some(NormalizerConfig::default()),
            ..Default::default()
        };
        assert!(ConfigValidator::validate_config_compatibility(&config).is_ok());

        // Test with low sample rate - should still work but may generate warning
        let low_rate_config = ProcessingConfig {
            resampler: Some(ResamplerConfig {
                target_sample_rate: Some(4000),
                ..Default::default()
            }),
            normalizer: Some(NormalizerConfig::default()),
            ..Default::default()
        };
        assert!(ConfigValidator::validate_config_compatibility(&low_rate_config).is_ok());
    }

    #[test]
    fn test_get_recommended_config_podcast() {
        let config = ConfigValidator::get_recommended_config("podcast").unwrap();

        assert!(config.resampler.is_some());
        assert!(config.channel_mixer.is_some());
        assert!(config.normalizer.is_some());
        assert!(config.silence_detector.is_some());

        let resampler = config.resampler.unwrap();
        assert_eq!(resampler.target_sample_rate, Some(44100));

        let mixer = config.channel_mixer.unwrap();
        assert_eq!(mixer.target_channels, Some(1));

        let normalizer = config.normalizer.unwrap();
        assert_eq!(normalizer.target_loudness, -16.0);

        let detector = config.silence_detector.unwrap();
        assert_eq!(detector.threshold_db, -45.0);
    }

    #[test]
    fn test_get_recommended_config_music() {
        let config = ConfigValidator::get_recommended_config("music").unwrap();

        assert!(config.resampler.is_some());
        assert!(config.channel_mixer.is_some());
        assert!(config.normalizer.is_some());
        assert!(config.silence_detector.is_none()); // No silence removal for music

        let resampler = config.resampler.unwrap();
        assert_eq!(resampler.target_sample_rate, Some(44100));

        let mixer = config.channel_mixer.unwrap();
        assert_eq!(mixer.target_channels, Some(2)); // Stereo for music

        let normalizer = config.normalizer.unwrap();
        assert_eq!(normalizer.target_loudness, -14.0);
    }

    #[test]
    fn test_get_recommended_config_voice() {
        let config = ConfigValidator::get_recommended_config("voice").unwrap();

        assert!(config.resampler.is_some());
        assert!(config.channel_mixer.is_some());
        assert!(config.normalizer.is_some());
        assert!(config.silence_detector.is_some());

        let resampler = config.resampler.unwrap();
        assert_eq!(resampler.target_sample_rate, Some(16000)); // Lower rate for voice

        let mixer = config.channel_mixer.unwrap();
        assert_eq!(mixer.target_channels, Some(1)); // Mono for voice

        let normalizer = config.normalizer.unwrap();
        assert_eq!(normalizer.target_loudness, -20.0);

        let detector = config.silence_detector.unwrap();
        assert_eq!(detector.threshold_db, -50.0);
    }

    #[test]
    fn test_get_recommended_config_unknown() {
        let result = ConfigValidator::get_recommended_config("unknown");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_config_alias() {
        let config = ProcessingConfig::default();
        // Verify that validate_config is an alias for validate_processing_config
        let result1 = ConfigValidator::validate_config(&config);
        let result2 = ConfigValidator::validate_processing_config(&config);

        assert_eq!(result1.is_ok(), result2.is_ok());
    }
}
