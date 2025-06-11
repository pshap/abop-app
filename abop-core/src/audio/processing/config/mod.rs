//! Audio processing configuration
//!
//! This module provides a modular configuration system for audio processing components.
//! Each component has its own configuration type with a builder for easy construction.
//!
//! # Architecture
//!
//! The configuration system is organized into the following components:
//!
//! - `ProcessingConfig`: The main configuration container that holds all component configs
//! - Component-specific configs: `ResamplerConfig`, `ChannelMixerConfig`, etc.
//! - Builder types: Each config has a corresponding builder with a fluent API
//!
//! # Usage Examples
//!
//! ## Using the main builder
//!
//! ```
//! use abop_core::audio::processing::config::ProcessingConfig;
//!
//! let config = ProcessingConfig::builder()
//!     .with_target_sample_rate(48000)
//!     .with_target_channels(1)
//!     .with_target_loudness(-18.0)
//!     .with_output_format(abop_core::audio::processing::config::AudioFormat::Wav)
//!     .build();
//! ```
//!
//! ## Using component-specific builders
//!
//! ```
//! use abop_core::audio::processing::config::{
//!     ProcessingConfig, ResamplerConfig, NormalizerConfig
//! };
//!
//! let resampler = ResamplerConfig::builder()
//!     .with_sample_rate(48000)
//!     .with_quality(abop_core::audio::processing::config::ResampleQuality::High)
//!     .build();
//!
//! let normalizer = NormalizerConfig::builder()
//!     .with_target_loudness(-18.0)
//!     .with_limiting(true)
//!     .build();
//!
//! let config = ProcessingConfig::builder()
//!     .with_resampler(resampler)
//!     .with_normalizer(normalizer)
//!     .build();
//! ```

use serde::{Deserialize, Serialize};

// Re-export all public types
pub use self::{
    channel_mixer::{ChannelMixerConfig, ChannelMixerConfigBuilder, MixingAlgorithm},
    normalizer::{NormalizationAlgorithm, NormalizerConfig, NormalizerConfigBuilder},
    output::{AudioFormat, BitDepth, OutputConfig, OutputConfigBuilder},
    resampler::{ResampleQuality, ResamplerConfig, ResamplerConfigBuilder},
    silence_detector::{SilenceDetectorConfig, SilenceDetectorConfigBuilder, SilenceRemovalMode},
};

pub use builder::ProcessingConfigBuilder;

use super::error::Result;
use super::traits::Validatable;

/// Configuration for audio processing operations
///
/// This is the main configuration container that holds all component-specific
/// configurations for the audio processing pipeline. Each component configuration
/// is optional, allowing for flexible pipeline construction.
///
/// # Examples
///
/// ```
/// use abop_core::audio::processing::config::ProcessingConfig;
///
/// // Create a default configuration
/// let config = ProcessingConfig::default();
///
/// // Or use the builder pattern
/// let config = ProcessingConfig::builder()
///     .with_target_sample_rate(48000)
///     .with_target_channels(1)
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    /// Resampling configuration (optional)
    /// When present, enables sample rate conversion in the processing pipeline.
    pub resampler: Option<ResamplerConfig>,

    /// Channel mixing configuration (optional)
    /// When present, enables channel mixing operations (e.g., stereo to mono).
    pub channel_mixer: Option<ChannelMixerConfig>,

    /// Audio normalization configuration (optional)
    /// When present, enables loudness normalization and peak limiting.
    pub normalizer: Option<NormalizerConfig>,

    /// Silence detection and removal configuration (optional)
    /// When present, enables detection and optional removal of silent segments.
    pub silence_detector: Option<SilenceDetectorConfig>,

    /// Output formatting configuration
    /// Controls the output format, bit depth, and file naming.
    pub output: OutputConfig,

    /// Number of threads to use for processing (optional)
    /// When None, the system will determine the optimal thread count.
    pub num_threads: Option<usize>,

    /// Whether to enable parallel processing
    /// When true, processing will use multiple threads when available.
    pub enable_parallel: bool,
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            resampler: None,
            channel_mixer: None,
            normalizer: None,
            silence_detector: None,
            output: OutputConfig::default(),
            num_threads: None,
            enable_parallel: true,
        }
    }
}

impl Validatable for ProcessingConfig {
    fn validate(&self) -> Result<()> {
        // Validate each component configuration if present
        if let Some(ref resampler) = self.resampler {
            resampler.validate()?;
        }
        if let Some(ref channel_mixer) = self.channel_mixer {
            channel_mixer.validate()?;
        }
        if let Some(ref normalizer) = self.normalizer {
            normalizer.validate()?;
        }
        if let Some(ref silence_detector) = self.silence_detector {
            silence_detector.validate()?;
        }

        // Output config is always required
        self.output.validate()?;

        // Validate thread count if specified
        if let Some(threads) = self.num_threads {
            validation::positive(&threads, "Number of threads")?;
        }

        Ok(())
    }
}

impl ProcessingConfig {
    /// Creates a new builder for `ProcessingConfig`
    ///
    /// This method returns a builder that provides a fluent API for constructing
    /// a `ProcessingConfig` with custom settings. The builder pattern makes it easy
    /// to create complex configurations with clear, readable code.
    ///
    /// # Returns
    ///
    /// A new `ProcessingConfigBuilder` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use abop_core::audio::processing::config::ProcessingConfig;
    ///
    /// let config = ProcessingConfig::builder()
    ///     .with_target_sample_rate(48000)
    ///     .with_target_channels(1)
    ///     .build();
    /// ```
    #[must_use]
    pub fn builder() -> ProcessingConfigBuilder {
        ProcessingConfigBuilder::new()
    }
}

// Submodules
mod builder;
mod channel_mixer;
/// Normalizer configuration module.
mod normalizer;
/// Output configuration module.
mod output;
/// Resampler configuration module.
mod resampler;
/// Silence detector configuration module.
pub mod silence_detector;
/// Validation configuration module.
pub mod validation;
