//! Audio processing validation module
//!
//! This module provides comprehensive validation for audio processing configurations,
//! file operations, and parameter ranges.

pub mod error_handling;
pub mod file_validators;
pub mod format_validators;
pub mod range_validators;

// Re-export key validation functions and types for backward compatibility
pub use error_handling::{ValidationError, ValidationResult};
pub use file_validators::*;
pub use format_validators::ConfigValidator;
pub use range_validators::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;
    use std::time::Duration;

    #[test]
    fn test_validate_processing_config_valid() {
        let config = super::super::config::ProcessingConfig {
            resampler: Some(super::super::config::ResamplerConfig {
                target_sample_rate: Some(44100),
                ..Default::default()
            }),
            channel_mixer: Some(super::super::config::ChannelMixerConfig {
                target_channels: Some(2),
                ..Default::default()
            }),
            normalizer: Some(super::super::config::NormalizerConfig {
                target_loudness: -23.0,
                ..Default::default()
            }),
            silence_detector: Some(super::super::config::SilenceDetectorConfig {
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
        let config = super::super::config::ProcessingConfig {
            num_threads: Some(0), // Invalid: zero threads
            ..Default::default()
        };

        assert!(ConfigValidator::validate_processing_config(&config).is_err());
    }

    #[test]
    fn test_validate_sample_rate() {
        assert!(validate_sample_rate(44100).is_ok());
        assert!(validate_sample_rate(0).is_err());
        assert!(validate_sample_rate(300000).is_err());
    }

    #[test]
    fn test_validate_channels() {
        assert!(validate_channels(2).is_ok());
        assert!(validate_channels(0).is_err());
        assert!(validate_channels(16).is_err());
    }

    #[test]
    fn test_validate_bit_depth() {
        assert!(validate_bit_depth(16).is_ok());
        assert!(validate_bit_depth(24).is_ok());
        assert!(validate_bit_depth(32).is_ok());
        assert!(validate_bit_depth(8).is_err());
    }

    #[test]
    fn test_validate_file_exists() {
        let valid_file = tempfile::NamedTempFile::new().unwrap();
        let invalid_path = "/nonexistent/file.mp3";

        // Note: These tests may fail on different systems, adjust as needed
        assert!(validate_file_exists(valid_file.path()).is_ok());
        assert!(validate_file_exists(invalid_path).is_err());
        // Directories exist but are not files
        assert!(validate_file_exists(std::env::temp_dir()).is_err());
    }

    #[test]
    fn test_validate_audio_extension() {
        assert!(validate_audio_extension("test.mp3").is_ok());
        assert!(validate_audio_extension("test.wav").is_ok());
        assert!(validate_audio_extension("test.txt").is_err());
        assert!(validate_audio_extension("test").is_err());
    }

    #[test]
    fn test_validate_metadata_completeness() {
        // Valid metadata
        let result = validate_metadata_completeness(
            Some("Test Title"),
            Some("Test Artist"),
            Some(Duration::from_secs(180)),
        );
        assert!(result.is_ok());

        // Missing duration
        let result = validate_metadata_completeness(
            Some("Test Title"),
            Some("Test Artist"),
            None,
        );
        assert!(result.is_err());

        // Zero duration
        let result = validate_metadata_completeness(
            Some("Test Title"),
            Some("Test Artist"),
            Some(Duration::from_secs(0)),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_error_handling() {
        let error = ValidationError::new("test_field", "Test validation error");
        let app_error: AppError = error.into();
        
        assert!(matches!(app_error, AppError::Audio(_)));
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
        let config = super::super::config::ProcessingConfig::default();
        // Verify that validate_config is an alias for validate_processing_config
        let result1 = ConfigValidator::validate_config(&config);
        let result2 = ConfigValidator::validate_processing_config(&config);

        assert_eq!(result1.is_ok(), result2.is_ok());
    }
}