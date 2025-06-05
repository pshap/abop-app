use serde::{Deserialize, Serialize};

use crate::audio::processing::error::Result;
use crate::audio::processing::traits::Validatable;

/// Configuration for audio normalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizerConfig {
    /// Target loudness in LUFS (should be negative)
    pub target_loudness: f32,
    /// Whether to use peak normalization
    pub use_peak_normalization: bool,
    /// Target peak level in dB (should be negative or zero)
    pub peak_level: f32,
    /// Whether to enable limiting
    pub enable_limiting: bool,
    /// Algorithm to use for normalization
    pub algorithm: NormalizationAlgorithm,
    /// Headroom in dB
    pub headroom_db: f32,
}

impl Default for NormalizerConfig {
    fn default() -> Self {
        Self {
            target_loudness: -16.0, // LUFS
            use_peak_normalization: false,
            peak_level: -1.0, // dB
            enable_limiting: true,
            algorithm: NormalizationAlgorithm::Peak,
            headroom_db: 1.0,
        }
    }
}

impl Validatable for NormalizerConfig {
    fn validate(&self) -> Result<()> {
        // Use the validation utilities for consistent error messages
        use super::validation;

        // Target loudness should be negative
        validation::negative(&self.target_loudness, "Target loudness")?;

        // Peak level should be negative or zero
        validation::less_than(&self.peak_level, &0.01, "Peak level")?;

        // Headroom must be positive
        validation::positive(&self.headroom_db, "Headroom")?;

        Ok(())
    }
}

/// Normalization algorithms
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NormalizationAlgorithm {
    /// Peak normalization
    Peak,
    /// RMS normalization
    Rms,
    /// LUFS normalization
    Lufs,
}

/// Builder for `NormalizerConfig`
#[derive(Debug, Default)]
pub struct NormalizerConfigBuilder {
    target_loudness: Option<f32>,
    use_peak_normalization: Option<bool>,
    peak_level: Option<f32>,
    enable_limiting: Option<bool>,
    algorithm: Option<NormalizationAlgorithm>,
    headroom_db: Option<f32>,
}

impl NormalizerConfigBuilder {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the target loudness in LUFS
    #[must_use]
    pub const fn with_target_loudness(mut self, loudness: f32) -> Self {
        self.target_loudness = Some(loudness);
        self
    }

    /// Enable or disable peak normalization
    #[must_use]
    pub const fn with_peak_normalization(mut self, enable: bool) -> Self {
        self.use_peak_normalization = Some(enable);
        self
    }

    /// Set the target peak level in dB
    #[must_use]
    pub const fn with_peak_level(mut self, level: f32) -> Self {
        self.peak_level = Some(level);
        self
    }

    /// Enable or disable limiting
    #[must_use]
    pub const fn with_limiting(mut self, enable: bool) -> Self {
        self.enable_limiting = Some(enable);
        self
    }

    /// Set the normalization algorithm
    #[must_use]
    pub const fn with_algorithm(mut self, algorithm: NormalizationAlgorithm) -> Self {
        self.algorithm = Some(algorithm);
        self
    }

    /// Set the headroom in dB
    #[must_use]
    pub const fn with_headroom(mut self, headroom: f32) -> Self {
        self.headroom_db = Some(headroom);
        self
    }

    /// Build the `NormalizerConfig`
    #[must_use]
    pub fn build(self) -> NormalizerConfig {
        NormalizerConfig {
            target_loudness: self.target_loudness.unwrap_or(-16.0),
            use_peak_normalization: self.use_peak_normalization.unwrap_or(false),
            peak_level: self.peak_level.unwrap_or(-1.0),
            enable_limiting: self.enable_limiting.unwrap_or(true),
            algorithm: self.algorithm.unwrap_or(NormalizationAlgorithm::Peak),
            headroom_db: self.headroom_db.unwrap_or(1.0),
        }
    }

    /// Build and validate the `NormalizerConfig`
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn build_validated(self) -> Result<NormalizerConfig> {
        let config = self.build();
        config.validate()?;
        Ok(config)
    }

    /// Configure for podcast loudness standards (-16 LUFS)
    #[must_use]
    pub const fn for_podcast(mut self) -> Self {
        self.target_loudness = Some(-16.0);
        self.algorithm = Some(NormalizationAlgorithm::Lufs);
        self.enable_limiting = Some(true);
        self
    }

    /// Configure for music streaming services (-14 LUFS)
    #[must_use]
    pub const fn for_streaming(mut self) -> Self {
        self.target_loudness = Some(-14.0);
        self.algorithm = Some(NormalizationAlgorithm::Lufs);
        self.enable_limiting = Some(true);
        self.headroom_db = Some(1.0);
        self
    }

    /// Configure for broadcast standards (-23 LUFS)
    #[must_use]
    pub const fn for_broadcast(mut self) -> Self {
        self.target_loudness = Some(-23.0);
        self.algorithm = Some(NormalizationAlgorithm::Lufs);
        self.enable_limiting = Some(true);
        self.headroom_db = Some(2.0);
        self
    }

    /// Configure for audiobook production (-18 LUFS)
    #[must_use]
    pub const fn for_audiobook(mut self) -> Self {
        self.target_loudness = Some(-18.0);
        self.algorithm = Some(NormalizationAlgorithm::Rms);
        self.enable_limiting = Some(true);
        self
    }
}

impl NormalizerConfig {
    /// Create a new builder for `NormalizerConfig`
    #[must_use]
    pub fn builder() -> NormalizerConfigBuilder {
        NormalizerConfigBuilder::new()
    }
}
