use serde::{Deserialize, Serialize};

use crate::audio::processing::error::Result;
use crate::audio::processing::traits::Validatable;

/// Configuration for the resampler component
///
/// This configuration controls how audio sample rate conversion is performed.
/// It allows specifying the target sample rate, quality level, and whether to
/// use anti-aliasing filtering during the resampling process.
///
/// # Examples
///
/// ```
/// use abop_core::audio::processing::config::{ResamplerConfig, ResampleQuality};
///
/// // Create with default settings (44.1kHz, medium quality)
/// let config = ResamplerConfig::default();
///
/// // Or use the builder pattern
/// let config = ResamplerConfig::builder()
///     .with_sample_rate(48000)
///     .with_quality(ResampleQuality::High)
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResamplerConfig {
    /// Target sample rate in Hz (optional)
    ///
    /// When None, no sample rate conversion will be performed.
    /// Common values: 44100 (CD quality), 48000 (professional audio),
    /// 96000 (high resolution), 16000 (voice/speech)
    pub target_sample_rate: Option<u32>,

    /// Quality setting for resampling
    ///
    /// Controls the trade-off between audio quality and processing speed.
    /// Higher quality settings use more CPU but produce better results.
    pub quality: ResampleQuality,

    /// Whether to enable anti-aliasing filter
    ///
    /// Anti-aliasing filters prevent aliasing artifacts during resampling.
    /// Should generally be enabled for best quality.
    pub enable_anti_aliasing: bool,
}

impl Default for ResamplerConfig {
    fn default() -> Self {
        Self {
            target_sample_rate: Some(44100),
            quality: ResampleQuality::Medium,
            enable_anti_aliasing: true,
        }
    }
}

impl Validatable for ResamplerConfig {
    fn validate(&self) -> Result<()> {
        if let Some(rate) = self.target_sample_rate {
            // Use the validation utilities for consistent error messages
            super::validation::range(&rate, &1, &192_000, "Target sample rate")?;
        }
        Ok(())
    }
}

/// Quality settings for resampling
///
/// This enum defines the quality levels available for the resampling algorithm.
/// Higher quality settings use more CPU resources but produce better audio results
/// with fewer artifacts.
///
/// The quality setting affects:
/// - The complexity of the interpolation algorithm
/// - The size of the filter window
/// - The precision of calculations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ResampleQuality {
    /// Low quality (fastest)
    ///
    /// Uses simple linear interpolation.
    /// Best for non-critical applications where performance is more important than quality.
    /// CPU usage: Low
    Low,

    /// Medium quality (default)
    ///
    /// Uses a balanced algorithm with good quality and reasonable performance.
    /// Suitable for most general-purpose audio processing tasks.
    /// CPU usage: Moderate
    Medium,

    /// High quality (slowest)
    ///
    /// Uses sophisticated interpolation with a large filter window.
    /// Best for professional audio work where quality is critical.
    /// CPU usage: High
    High,
}

/// Builder for `ResamplerConfig`
///
/// Provides a fluent API for constructing `ResamplerConfig` instances.
/// The builder pattern makes it easy to create configurations with
/// clear, readable code and sensible defaults for omitted values.
///
/// # Examples
///
/// ```
/// use abop_core::audio::processing::config::{ResamplerConfig, ResampleQuality};
///
/// let config = ResamplerConfig::builder()
///     .with_sample_rate(48000)
///     .with_quality(ResampleQuality::High)
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct ResamplerConfigBuilder {
    /// Target sample rate in Hz (optional)
    target_sample_rate: Option<u32>,

    /// Quality setting for resampling (optional)
    quality: Option<ResampleQuality>,

    /// Whether to enable anti-aliasing filter (optional)
    enable_anti_aliasing: Option<bool>,
}

impl ResamplerConfigBuilder {
    /// Creates a new builder with default values
    ///
    /// All settings start as None and will be populated with defaults
    /// if not explicitly set when `build()` is called.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the target sample rate for resampling
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - The target sample rate in Hz (e.g., 44100, 48000)
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    #[must_use]
    pub const fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.target_sample_rate = Some(sample_rate);
        self
    }

    /// Sets the resampling quality level
    ///
    /// # Arguments
    ///
    /// * `quality` - The quality level to use for resampling
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    #[must_use]
    pub const fn with_quality(mut self, quality: ResampleQuality) -> Self {
        self.quality = Some(quality);
        self
    }

    /// Enables or disables the anti-aliasing filter
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable the anti-aliasing filter
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    #[must_use]
    pub const fn with_anti_aliasing(mut self, enable: bool) -> Self {
        self.enable_anti_aliasing = Some(enable);
        self
    }

    /// Builds the `ResamplerConfig` with the configured settings
    #[must_use]
    pub fn build(self) -> ResamplerConfig {
        ResamplerConfig {
            target_sample_rate: self.target_sample_rate,
            quality: self.quality.unwrap_or(ResampleQuality::Medium),
            enable_anti_aliasing: self.enable_anti_aliasing.unwrap_or(true),
        }
    }

    /// Builds and validates the `ResamplerConfig`
    ///
    /// This method builds the config and then validates it to ensure
    /// all settings are valid. If validation fails, an error is returned.
    ///
    /// # Returns
    ///
    /// A Result containing either the validated `ResamplerConfig` or an error
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if the target
    /// sample rate or quality settings are invalid.
    pub fn build_validated(self) -> Result<ResamplerConfig> {
        let config = self.build();
        config.validate()?;
        Ok(config)
    }

    /// Configures settings for CD quality audio (44.1kHz)
    ///
    /// Sets the sample rate to 44.1kHz and quality to Medium,
    /// which is appropriate for most music and general audio.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    #[must_use]
    pub const fn for_cd_quality(mut self) -> Self {
        self.target_sample_rate = Some(44100);
        self.quality = Some(ResampleQuality::Medium);
        self
    }

    /// Configures settings for high-resolution audio (96kHz)
    ///
    /// Sets the sample rate to 96kHz and quality to High,
    /// which is appropriate for professional audio production.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    #[must_use]
    pub const fn for_high_res(mut self) -> Self {
        self.target_sample_rate = Some(96000);
        self.quality = Some(ResampleQuality::High);
        self
    }

    /// Configures settings for voice/speech processing (16kHz)
    ///
    /// Sets the sample rate to 16kHz and quality to Medium,
    /// which is appropriate for voice recognition and telephony.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    #[must_use]
    pub const fn for_voice(mut self) -> Self {
        self.target_sample_rate = Some(16000);
        self.quality = Some(ResampleQuality::Medium);
        self
    }
}

impl ResamplerConfig {
    /// Creates a new builder for `ResamplerConfig`
    ///
    /// This method provides access to the builder pattern for creating
    /// `ResamplerConfig` instances with a fluent API.
    ///
    /// # Returns
    ///
    /// A new `ResamplerConfigBuilder` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use abop_core::audio::processing::config::{ResamplerConfig, ResampleQuality};
    ///
    /// let config = ResamplerConfig::builder()
    ///     .with_sample_rate(48000)
    ///     .with_quality(ResampleQuality::High)
    ///     .build();
    /// ```
    #[must_use]
    pub fn builder() -> ResamplerConfigBuilder {
        ResamplerConfigBuilder::new()
    }
}
