use serde::{Deserialize, Serialize};

use crate::audio::processing::error::Result;
use crate::audio::processing::traits::Validatable;

/// Configuration for channel mixing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMixerConfig {
    /// Target number of output channels (optional)
    pub target_channels: Option<u16>,
    /// Algorithm to use for mixing channels
    pub mix_algorithm: MixingAlgorithm,
}

impl Default for ChannelMixerConfig {
    fn default() -> Self {
        Self {
            target_channels: Some(1), // Mono (stereo to mono conversion only)
            mix_algorithm: MixingAlgorithm::Average,
        }
    }
}

impl Validatable for ChannelMixerConfig {
    fn validate(&self) -> Result<()> {
        // Use the validation utilities for consistent error messages
        use super::validation;

        // Validate target channels if specified
        if let Some(channels) = self.target_channels {
            validation::range(&channels, &1, &32, "Target channels")?;
        }

        // Validate weights for weighted sum algorithm
        if let MixingAlgorithm::WeightedSum {
            left_weight,
            right_weight,
        } = self.mix_algorithm
        {
            validation::range(&left_weight, &0.0, &1.0, "Left weight")?;
            validation::range(&right_weight, &0.0, &1.0, "Right weight")?;
        }

        Ok(())
    }
}

/// Algorithms for mixing channels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MixingAlgorithm {
    /// Average left and right channels
    Average,
    /// Use only the left channel
    LeftOnly,
    /// Use only the right channel
    RightOnly,
    /// Weighted sum of left and right channels
    WeightedSum {
        /// Weight for the left channel (0.0 to 1.0)
        left_weight: f32,
        /// Weight for the right channel (0.0 to 1.0)
        right_weight: f32,
    },
}

/// Builder for `ChannelMixerConfig`
#[derive(Debug, Default)]
pub struct ChannelMixerConfigBuilder {
    target_channels: Option<u16>,
    mix_algorithm: Option<MixingAlgorithm>,
}

impl ChannelMixerConfigBuilder {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the target number of channels
    #[must_use]
    pub const fn with_target_channels(mut self, channels: u16) -> Self {
        self.target_channels = Some(channels);
        self
    }

    /// Set the mixing algorithm
    #[must_use]
    pub const fn with_mixing_algorithm(mut self, algorithm: MixingAlgorithm) -> Self {
        self.mix_algorithm = Some(algorithm);
        self
    }

    /// Use average mixing algorithm
    #[must_use]
    pub const fn with_average_mixing(mut self) -> Self {
        self.mix_algorithm = Some(MixingAlgorithm::Average);
        self
    }

    /// Use left-only mixing algorithm
    #[must_use]
    pub const fn with_left_only_mixing(mut self) -> Self {
        self.mix_algorithm = Some(MixingAlgorithm::LeftOnly);
        self
    }

    /// Use right-only mixing algorithm
    #[must_use]
    pub const fn with_right_only_mixing(mut self) -> Self {
        self.mix_algorithm = Some(MixingAlgorithm::RightOnly);
        self
    }

    /// Use weighted sum mixing algorithm
    #[must_use]
    pub const fn with_weighted_mixing(mut self, left_weight: f32, right_weight: f32) -> Self {
        self.mix_algorithm = Some(MixingAlgorithm::WeightedSum {
            left_weight,
            right_weight,
        });
        self
    }

    /// Build the `ChannelMixerConfig`
    #[must_use]
    pub fn build(self) -> ChannelMixerConfig {
        ChannelMixerConfig {
            target_channels: self.target_channels,
            mix_algorithm: self.mix_algorithm.unwrap_or(MixingAlgorithm::Average),
        }
    }

    /// Build and validate the `ChannelMixerConfig`
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if the target
    /// channel count or mixing algorithm parameters are invalid.
    pub fn build_validated(self) -> Result<ChannelMixerConfig> {
        let config = self.build();
        config.validate()?;
        Ok(config)
    }
}

impl ChannelMixerConfig {
    /// Create a new builder for `ChannelMixerConfig`
    #[must_use]
    pub fn builder() -> ChannelMixerConfigBuilder {
        ChannelMixerConfigBuilder::new()
    }
}
