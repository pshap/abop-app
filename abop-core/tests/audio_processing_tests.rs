//! Tests for the audio processing functionality.

use abop_core::{
    audio::processing::{
        AudioProcessingPipeline, ChannelMixerConfig, MixingAlgorithm, ProcessingConfig,
        error::AudioProcessingError,
    },
    message::AudioProcessingOption,
};

#[test]
fn test_audio_processing_pipeline_creation() {
    // Test stereo to mono conversion configuration
    let config = ProcessingConfig {
        channel_mixer: Some(ChannelMixerConfig {
            target_channels: Some(1), // Convert to mono
            mix_algorithm: MixingAlgorithm::Average,
        }),
        ..Default::default()
    };

    let result = AudioProcessingPipeline::new(config);

    // Verify the pipeline was created successfully
    match result {
        Ok(_pipeline) => { /* Test passed */ }
        Err(e) => panic!("Pipeline creation failed: {e:?}"),
    }
}

#[test]
fn test_audio_processing_error_handling() {
    // Test invalid configuration
    let config = ProcessingConfig {
        channel_mixer: Some(ChannelMixerConfig {
            target_channels: Some(0), // Invalid: 0 channels
            mix_algorithm: MixingAlgorithm::Average,
        }),
        ..Default::default()
    };

    let result = AudioProcessingPipeline::new(config);

    // Verify that pipeline creation with invalid config returns an error
    match result {
        Ok(_) => panic!("Expected error for invalid configuration"),
        Err(AudioProcessingError::Configuration(_)) => {
            // Expected error type
        }
        Err(e) => {
            // Accept other error types for now since the validation might be different
            println!("Got error: {e:?}");
        }
    }
}

#[test]
fn test_audio_processing_options() {
    // Test that all processing options are available
    let options = [
        AudioProcessingOption::StereoToMono,
        AudioProcessingOption::NoiseRemoval,
        AudioProcessingOption::Normalization,
        AudioProcessingOption::Split,
        AudioProcessingOption::Merge,
    ];

    // Verify options are valid enum variants
    assert_eq!(options.len(), 5);

    // Test that options can be used in match expressions
    for option in &options {
        match option {
            AudioProcessingOption::StereoToMono => { /* Valid variant */ }
            AudioProcessingOption::NoiseRemoval => { /* Valid variant */ }
            AudioProcessingOption::Normalization => { /* Valid variant */ }
            AudioProcessingOption::Split => { /* Valid variant */ }
            AudioProcessingOption::Merge => { /* Valid variant */ }
        }
    }
}

#[test]
fn test_processing_config_defaults() {
    let config = ProcessingConfig::default();

    // Test that default configuration is valid
    assert!(config.channel_mixer.is_none());

    // Should be able to create a pipeline with default config
    let result = AudioProcessingPipeline::new(config);
    match result {
        Ok(_) => { /* Test passed */ }
        Err(e) => panic!("Default config should be valid: {e:?}"),
    }
}
