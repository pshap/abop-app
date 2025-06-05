use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::audio::processing::error::{AudioProcessingError, Result};
use crate::audio::processing::traits::Validatable;

/// Configuration for silence detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceDetectorConfig {
    /// Silence threshold in dB (should be negative)
    pub threshold_db: f32,
    /// Minimum duration of silence to detect
    pub min_duration: Duration,
    /// Whether to remove leading silence
    pub remove_leading: bool,
    /// Whether to remove trailing silence
    pub remove_trailing: bool,
    /// Whether to remove internal silence
    pub remove_internal: bool,
    /// Duration of fade applied to removed regions
    pub fade_duration: Duration,
    /// Mode for silence removal
    pub removal_mode: SilenceRemovalMode,
}

impl Default for SilenceDetectorConfig {
    fn default() -> Self {
        Self {
            threshold_db: -40.0,
            min_duration: Duration::from_millis(500),
            remove_leading: true,
            remove_trailing: true,
            remove_internal: false,
            fade_duration: Duration::from_millis(10),
            removal_mode: SilenceRemovalMode::LeadingTrailing,
        }
    }
}

impl Validatable for SilenceDetectorConfig {
    fn validate(&self) -> Result<()> {
        // Use the validation utilities for consistent error messages
        use super::validation;

        // Silence threshold should be negative
        validation::negative(&self.threshold_db, "Silence threshold")?;

        // Minimum duration must be positive
        if self.min_duration.as_millis() == 0 {
            return Err(AudioProcessingError::config(
                "Minimum silence duration must be greater than 0",
            ));
        }

        // Fade duration cannot be longer than minimum silence duration
        if self.fade_duration > self.min_duration {
            return Err(AudioProcessingError::config(
                "Fade duration cannot be longer than minimum silence duration",
            ));
        }

        Ok(())
    }
}

/// Silence removal modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SilenceRemovalMode {
    /// Do not remove silence
    None,
    /// Remove only leading and trailing silence
    LeadingTrailing,
    /// Remove all detected silence
    All,
}

/// Builder for `SilenceDetectorConfig`
#[derive(Debug, Default)]
pub struct SilenceDetectorConfigBuilder {
    threshold_db: Option<f32>,
    min_duration: Option<Duration>,
    remove_leading: Option<bool>,
    remove_trailing: Option<bool>,
    remove_internal: Option<bool>,
    fade_duration: Option<Duration>,
    removal_mode: Option<SilenceRemovalMode>,
}

impl SilenceDetectorConfigBuilder {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the silence threshold in dB
    #[must_use]
    pub const fn with_threshold_db(mut self, threshold: f32) -> Self {
        self.threshold_db = Some(threshold);
        self
    }

    /// Set the minimum duration of silence to detect
    #[must_use]
    pub const fn with_min_duration(mut self, duration: Duration) -> Self {
        self.min_duration = Some(duration);
        self
    }

    /// Enable or disable removal of leading silence
    #[must_use]
    pub const fn with_remove_leading(mut self, enable: bool) -> Self {
        self.remove_leading = Some(enable);
        self
    }

    /// Enable or disable removal of trailing silence
    #[must_use]
    pub const fn with_remove_trailing(mut self, enable: bool) -> Self {
        self.remove_trailing = Some(enable);
        self
    }

    /// Enable or disable removal of internal silence
    #[must_use]
    pub const fn with_remove_internal(mut self, enable: bool) -> Self {
        self.remove_internal = Some(enable);
        self
    }

    /// Set the fade duration for removed regions
    #[must_use]
    pub const fn with_fade_duration(mut self, duration: Duration) -> Self {
        self.fade_duration = Some(duration);
        self
    }

    /// Set the silence removal mode
    #[must_use]
    pub const fn with_removal_mode(mut self, mode: SilenceRemovalMode) -> Self {
        self.removal_mode = Some(mode);
        self
    }

    /// Build the `SilenceDetectorConfig`
    #[must_use]
    pub fn build(self) -> SilenceDetectorConfig {
        SilenceDetectorConfig {
            threshold_db: self.threshold_db.unwrap_or(-40.0),
            min_duration: self.min_duration.unwrap_or(Duration::from_millis(500)),
            remove_leading: self.remove_leading.unwrap_or(true),
            remove_trailing: self.remove_trailing.unwrap_or(true),
            remove_internal: self.remove_internal.unwrap_or(false),
            fade_duration: self.fade_duration.unwrap_or(Duration::from_millis(10)),
            removal_mode: self
                .removal_mode
                .unwrap_or(SilenceRemovalMode::LeadingTrailing),
        }
    }

    /// Build and validate the `SilenceDetectorConfig`
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if the threshold
    /// is positive or if the minimum duration is zero.
    pub fn build_validated(self) -> Result<SilenceDetectorConfig> {
        let config = self.build();
        config.validate()?;
        Ok(config)
    }

    /// Configure for podcast editing (removes leading/trailing silence)
    #[must_use]
    pub const fn for_podcast(mut self) -> Self {
        self.threshold_db = Some(-40.0);
        self.min_duration = Some(Duration::from_millis(500));
        self.remove_leading = Some(true);
        self.remove_trailing = Some(true);
        self.remove_internal = Some(false);
        self.removal_mode = Some(SilenceRemovalMode::LeadingTrailing);
        self
    }

    /// Configure for voice recognition (removes all silence)
    #[must_use]
    pub const fn for_voice_recognition(mut self) -> Self {
        self.threshold_db = Some(-30.0);
        self.min_duration = Some(Duration::from_millis(300));
        self.remove_leading = Some(true);
        self.remove_trailing = Some(true);
        self.remove_internal = Some(true);
        self.removal_mode = Some(SilenceRemovalMode::All);
        self
    }

    /// Configure for music mastering (only removes extreme silence)
    #[must_use]
    pub const fn for_music(mut self) -> Self {
        self.threshold_db = Some(-60.0);
        self.min_duration = Some(Duration::from_millis(1000));
        self.remove_leading = Some(true);
        self.remove_trailing = Some(true);
        self.remove_internal = Some(false);
        self.fade_duration = Some(Duration::from_millis(50));
        self.removal_mode = Some(SilenceRemovalMode::LeadingTrailing);
        self
    }
}

impl SilenceDetectorConfig {
    /// Create a new builder for `SilenceDetectorConfig`
    #[must_use]
    pub fn builder() -> SilenceDetectorConfigBuilder {
        SilenceDetectorConfigBuilder::new()
    }
}
