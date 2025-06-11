use super::{
    ProcessingConfig,
    channel_mixer::{ChannelMixerConfig, MixingAlgorithm},
    normalizer::{NormalizationAlgorithm, NormalizerConfig},
    output::{AudioFormat, BitDepth, OutputConfig},
    resampler::{ResampleQuality, ResamplerConfig},
    silence_detector::{SilenceDetectorConfig, SilenceRemovalMode},
};
use crate::audio::processing::error::Result;
use crate::audio::processing::traits::Validatable;
use std::path::PathBuf;

/// Builder for `ProcessingConfig` with fluent API
#[derive(Debug, Default)]
pub struct ProcessingConfigBuilder {
    resampler: Option<ResamplerConfig>,
    channel_mixer: Option<ChannelMixerConfig>,
    normalizer: Option<NormalizerConfig>,
    silence_detector: Option<SilenceDetectorConfig>,
    output: Option<OutputConfig>,
    num_threads: Option<usize>,
    enable_parallel: Option<bool>,
}

impl ProcessingConfigBuilder {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // High-level component setters

    /// Set resampler configuration
    #[must_use]
    pub const fn with_resampler(mut self, config: ResamplerConfig) -> Self {
        self.resampler = Some(config);
        self
    }

    /// Set channel mixer configuration
    #[must_use]
    pub const fn with_channel_mixer(mut self, config: ChannelMixerConfig) -> Self {
        self.channel_mixer = Some(config);
        self
    }

    /// Set normalizer configuration
    #[must_use]
    pub const fn with_normalizer(mut self, config: NormalizerConfig) -> Self {
        self.normalizer = Some(config);
        self
    }

    /// Set silence detector configuration
    #[must_use]
    pub const fn with_silence_detector(mut self, config: SilenceDetectorConfig) -> Self {
        self.silence_detector = Some(config);
        self
    }

    /// Set output configuration
    #[must_use]
    pub fn with_output(mut self, config: OutputConfig) -> Self {
        self.output = Some(config);
        self
    }

    // Convenience methods

    /// Configure resampling with target sample rate
    #[must_use]
    pub fn with_target_sample_rate(mut self, sample_rate: u32) -> Self {
        let resampler = self.resampler.unwrap_or_default();
        self.resampler = Some(ResamplerConfig {
            target_sample_rate: Some(sample_rate),
            ..resampler
        });
        self
    }

    /// Configure resampling quality
    #[must_use]
    pub fn with_resample_quality(mut self, quality: ResampleQuality) -> Self {
        let resampler = self.resampler.unwrap_or_default();
        self.resampler = Some(ResamplerConfig {
            quality,
            ..resampler
        });
        self
    }

    /// Configure target number of channels
    #[must_use]
    pub fn with_target_channels(mut self, channels: u16) -> Self {
        let mixer = self.channel_mixer.unwrap_or_default();
        self.channel_mixer = Some(ChannelMixerConfig {
            target_channels: Some(channels),
            ..mixer
        });
        self
    }

    /// Configure mixing algorithm
    #[must_use]
    pub fn with_mixing_algorithm(mut self, algorithm: MixingAlgorithm) -> Self {
        let mixer = self.channel_mixer.unwrap_or_default();
        self.channel_mixer = Some(ChannelMixerConfig {
            mix_algorithm: algorithm,
            ..mixer
        });
        self
    }

    /// Configure target loudness for normalization
    #[must_use]
    pub fn with_target_loudness(mut self, loudness: f32) -> Self {
        let normalizer = self.normalizer.unwrap_or_default();
        self.normalizer = Some(NormalizerConfig {
            target_loudness: loudness,
            ..normalizer
        });
        self
    }

    /// Configure normalization algorithm
    #[must_use]
    pub fn with_normalization_algorithm(mut self, algorithm: NormalizationAlgorithm) -> Self {
        let normalizer = self.normalizer.unwrap_or_default();
        self.normalizer = Some(NormalizerConfig {
            algorithm,
            ..normalizer
        });
        self
    }

    /// Configure silence threshold
    #[must_use]
    pub fn with_silence_threshold(mut self, threshold_db: f32) -> Self {
        let detector = self.silence_detector.unwrap_or_default();
        self.silence_detector = Some(SilenceDetectorConfig {
            threshold_db,
            ..detector
        });
        self
    }

    /// Configure silence removal mode
    #[must_use]
    pub fn with_silence_removal_mode(mut self, mode: SilenceRemovalMode) -> Self {
        let detector = self.silence_detector.unwrap_or_default();
        self.silence_detector = Some(SilenceDetectorConfig {
            removal_mode: mode,
            ..detector
        });
        self
    }

    /// Configure output format
    #[must_use]
    pub fn with_output_format(mut self, format: AudioFormat) -> Self {
        let output = self.output.unwrap_or_default();
        self.output = Some(OutputConfig {
            format: Some(format),
            ..output
        });
        self
    }

    /// Configure output directory
    #[must_use]
    pub fn with_output_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        let output = self.output.unwrap_or_default();
        self.output = Some(OutputConfig {
            output_dir: Some(dir.into()),
            ..output
        });
        self
    }

    /// Configure filename suffix
    #[must_use]
    pub fn with_filename_suffix<S: Into<String>>(mut self, suffix: S) -> Self {
        let output = self.output.unwrap_or_default();
        self.output = Some(OutputConfig {
            filename_suffix: suffix.into(),
            ..output
        });
        self
    }

    /// Set number of threads for processing
    #[must_use]
    pub const fn with_num_threads(mut self, threads: usize) -> Self {
        self.num_threads = Some(threads);
        self
    }

    /// Enable or disable parallel processing
    #[must_use]
    pub const fn with_parallel_processing(mut self, enable: bool) -> Self {
        self.enable_parallel = Some(enable);
        self
    }

    /// Build the `ProcessingConfig`
    #[must_use]
    pub fn build(self) -> ProcessingConfig {
        ProcessingConfig {
            resampler: self.resampler,
            channel_mixer: self.channel_mixer,
            normalizer: self.normalizer,
            silence_detector: self.silence_detector,
            output: self.output.unwrap_or_default(),
            num_threads: self.num_threads,
            enable_parallel: self.enable_parallel.unwrap_or(true),
        }
    }

    /// Build and validate the `ProcessingConfig`
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if any of the
    /// configured components have invalid parameters or if component configurations
    /// are incompatible with each other.
    pub fn build_validated(self) -> Result<ProcessingConfig> {
        let config = self.build();
        config.validate()?;
        Ok(config)
    }

    // Convenience methods for common use cases

    /// Configure simple resampling with default quality
    #[must_use]
    pub fn with_simple_resampling(mut self, target_sample_rate: u32) -> Self {
        self.resampler = Some(
            ResamplerConfig::builder()
                .with_sample_rate(target_sample_rate)
                .build(),
        );
        self
    }

    /// Configure for podcast processing with standard settings
    #[must_use]
    pub fn for_podcast_processing(mut self) -> Self {
        self.resampler = Some(
            ResamplerConfig::builder()
                .with_sample_rate(44100)
                .with_quality(ResampleQuality::Medium)
                .build(),
        );

        self.channel_mixer = Some(
            ChannelMixerConfig::builder()
                .with_target_channels(1) // Mono for podcasts
                .build(),
        );

        self.normalizer = Some(
            NormalizerConfig::builder()
                .with_target_loudness(-16.0) // Standard podcast loudness
                .with_limiting(true)
                .build(),
        );

        self.silence_detector = Some(
            SilenceDetectorConfig::builder()
                .with_threshold_db(-40.0)
                .with_removal_mode(SilenceRemovalMode::LeadingTrailing)
                .build(),
        );

        self.output = Some(
            OutputConfig::builder()
                .with_format(AudioFormat::Mp3)
                .with_filename_suffix("_podcast")
                .build(),
        );

        self.enable_parallel = Some(true);
        self
    }

    /// Configure for music mastering with high-quality settings
    #[must_use]
    pub fn for_music_mastering(mut self) -> Self {
        self.resampler = Some(
            ResamplerConfig::builder()
                .with_sample_rate(48000)
                .with_quality(ResampleQuality::High)
                .build(),
        );

        self.normalizer = Some(
            NormalizerConfig::builder()
                .with_target_loudness(-14.0) // Standard for music streaming
                .with_algorithm(NormalizationAlgorithm::Lufs)
                .with_limiting(true)
                .with_headroom(1.0)
                .build(),
        );

        self.output = Some(
            OutputConfig::builder()
                .with_format(AudioFormat::Wav)
                .with_bit_depth(BitDepth::TwentyFour)
                .with_filename_suffix("_mastered")
                .build(),
        );

        self.enable_parallel = Some(true);
        self
    }

    /// Configure for voice recognition preprocessing
    #[must_use]
    pub fn for_voice_recognition(mut self) -> Self {
        self.resampler = Some(
            ResamplerConfig::builder()
                .with_sample_rate(16000) // Common for speech recognition
                .build(),
        );

        self.channel_mixer = Some(
            ChannelMixerConfig::builder()
                .with_target_channels(1) // Mono for voice recognition
                .build(),
        );

        self.normalizer = Some(
            NormalizerConfig::builder()
                .with_target_loudness(-24.0)
                .build(),
        );

        self.silence_detector = Some(
            SilenceDetectorConfig::builder()
                .with_threshold_db(-30.0)
                .with_removal_mode(SilenceRemovalMode::All)
                .build(),
        );

        self.output = Some(
            OutputConfig::builder()
                .with_format(AudioFormat::Wav)
                .with_filename_suffix("_voice")
                .build(),
        );

        self
    }
}
