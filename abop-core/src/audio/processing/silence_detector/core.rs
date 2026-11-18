//! Core silence detector implementation

use super::{
    super::{
        casting_utils::error_conversion::cast_to_audio_error,
        config::{SilenceDetectorConfig, SilenceRemovalMode},
        error::Result,
        validation::ConfigValidator,
    },
    types::SilenceSegment,
};
use crate::audio::AudioBuffer;
use crate::utils::casting::domain::audio::{
    safe_duration_to_samples, safe_progress, safe_samples_to_duration,
};

/// Audio silence detector and remover
///
/// Detects and optionally removes silence from audio buffers based on
/// configurable threshold and minimum duration parameters.
#[derive(Debug, Clone)]
pub struct SilenceDetector {
    pub(crate) config: SilenceDetectorConfig,
}

impl SilenceDetector {
    /// Creates a new silence detector with specified configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if configuration
    /// parameters are invalid (e.g., invalid threshold or duration values).
    pub fn new(config: SilenceDetectorConfig) -> Result<Self> {
        ConfigValidator::validate_silence_detector_config(&config)?;
        Ok(Self { config })
    }

    /// Creates a new silence detector with specific threshold and minimum duration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if threshold or
    /// minimum duration parameters are invalid.
    pub fn with_params(threshold_db: f32, min_duration_secs: f32) -> Result<Self> {
        let config = SilenceDetectorConfig {
            threshold_db,
            min_duration: std::time::Duration::from_secs_f32(min_duration_secs),
            ..Default::default()
        };
        Self::new(config)
    }

    /// Gets the current configuration
    pub fn get_config(&self) -> &SilenceDetectorConfig {
        &self.config
    }

    /// Updates the configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the new configuration is invalid.
    pub fn update_config(&mut self, config: SilenceDetectorConfig) -> Result<()> {
        ConfigValidator::validate_silence_detector_config(&config)?;
        self.config = config;
        Ok(())
    }

    /// Processes audio buffer to remove silence according to configuration
    pub fn process_silence(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if buffer.data.is_empty() {
            return Ok(());
        }

        match self.config.removal_mode {
            SilenceRemovalMode::LeadingTrailing => self.remove_leading_trailing_silence(buffer),
            SilenceRemovalMode::All => self.remove_all_silence(buffer),
            SilenceRemovalMode::None => {
                // Just detect but don't remove
                let _segments = self.detect_silence_segments(buffer)?;
                Ok(())
            }
        }
    }

    /// Removes silence from beginning and end of audio buffer
    fn remove_leading_trailing_silence(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        let threshold = super::helpers::db_to_linear(self.config.threshold_db);
        let min_samples =
            safe_duration_to_samples(self.config.min_duration.as_secs_f32(), buffer.sample_rate)
                .map_err(cast_to_audio_error)?;

        // Find the first non-silent sample
        let start = super::helpers::find_first_non_silent_sample(&buffer.data, threshold);

        // Find the last non-silent sample
        let end = super::helpers::find_last_non_silent_sample(&buffer.data, threshold);

        // Check if we have enough silence to remove
        let leading_silence = start;
        let trailing_silence = buffer.data.len() - end - 1;

        if leading_silence >= min_samples || trailing_silence >= min_samples {
            let new_start = if leading_silence >= min_samples {
                start
            } else {
                0
            };
            let new_end = if trailing_silence >= min_samples {
                end + 1
            } else {
                buffer.data.len()
            };
            if new_start < new_end {
                let removed_samples = buffer.data.len() - (new_end - new_start);
                let duration = safe_samples_to_duration(removed_samples, buffer.sample_rate)
                    .map_err(cast_to_audio_error)?;
                log::debug!(
                    "Removed {removed_samples} samples of leading/trailing silence ({duration:.3}s)"
                );

                buffer.data = buffer.data[new_start..new_end].to_vec();
            }
        }

        Ok(())
    }

    /// Removes all silence segments from audio buffer
    fn remove_all_silence(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        let segments = self.detect_silence_segments(buffer)?;

        if segments.is_empty() {
            return Ok(());
        }

        let mut new_data = Vec::new();
        let mut last_end = 0;

        for segment in &segments {
            // Add non-silent audio before this silence segment
            new_data.extend_from_slice(&buffer.data[last_end..segment.start]);
            last_end = segment.end;
        }

        // Add remaining non-silent audio after last silence segment
        if last_end < buffer.data.len() {
            new_data.extend_from_slice(&buffer.data[last_end..]);
        }
        let removed_samples = buffer.data.len() - new_data.len();
        if removed_samples > 0 {
            let duration = safe_samples_to_duration(removed_samples, buffer.sample_rate)
                .map_err(cast_to_audio_error)?;
            log::debug!(
                "Removed {} silence segments totaling {} samples ({:.3}s)",
                segments.len(),
                removed_samples,
                duration
            );
            buffer.data = new_data;
        }

        Ok(())
    }

    /// Detects all silence segments in audio buffer
    ///
    /// # Errors
    ///
    /// Returns an error if sample rate conversion fails or duration calculations overflow.
    pub fn detect_silence_segments(
        &self,
        buffer: &AudioBuffer<f32>,
    ) -> Result<Vec<SilenceSegment>> {
        if buffer.data.is_empty() {
            return Ok(Vec::new());
        }
        let threshold = super::helpers::db_to_linear(self.config.threshold_db);
        let min_samples =
            safe_duration_to_samples(self.config.min_duration.as_secs_f32(), buffer.sample_rate)
                .map_err(cast_to_audio_error)?;
        let mut segments = Vec::new();
        let mut in_silence = false;
        let mut silence_start_frame = 0;

        let channels = buffer.channels as usize;
        let num_frames = buffer.data.len() / channels;

        for frame in 0..num_frames {
            // Check if all channels in this frame are silent
            let mut frame_is_silent = true;
            for channel in 0..channels {
                let sample_index = frame * channels + channel;
                if sample_index < buffer.data.len() && buffer.data[sample_index].abs() > threshold {
                    frame_is_silent = false;
                    break;
                }
            }

            if frame_is_silent && !in_silence {
                // Start of silence
                in_silence = true;
                silence_start_frame = frame;
            } else if !frame_is_silent && in_silence {
                // End of silence
                let silence_length_frames = frame - silence_start_frame;
                if silence_length_frames >= min_samples {
                    let duration_secs =
                        safe_samples_to_duration(silence_length_frames, buffer.sample_rate)
                            .map_err(cast_to_audio_error)?;
                    segments.push(SilenceSegment {
                        start: silence_start_frame * channels,
                        end: frame * channels,
                        duration_secs,
                    });
                }
                in_silence = false;
            }
        } // Handle silence at end of buffer
        if in_silence {
            let silence_length_frames = num_frames - silence_start_frame;
            if silence_length_frames >= min_samples {
                let duration_secs =
                    safe_samples_to_duration(silence_length_frames, buffer.sample_rate)
                        .map_err(cast_to_audio_error)?;
                segments.push(SilenceSegment {
                    start: silence_start_frame * channels,
                    end: buffer.data.len(),
                    duration_secs,
                });
            }
        }

        Ok(segments)
    }

    /// Calculates percentage of silence in audio buffer
    ///
    /// # Errors
    ///
    /// Returns an error if silence detection fails due to invalid sample rate or duration calculations.
    pub fn calculate_silence_percentage(&self, buffer: &AudioBuffer<f32>) -> Result<f32> {
        if buffer.data.is_empty() {
            return Ok(0.0);
        }

        let segments = self.detect_silence_segments(buffer)?;
        let total_silence_samples: usize = segments.iter().map(|s| s.end - s.start).sum(); // Safe casting for silence percentage calculation
        let silence_ratio =
            safe_progress(total_silence_samples, buffer.data.len()).map_err(cast_to_audio_error)?;
        Ok(silence_ratio * 100.0)
    }

    /// Checks if audio buffer contains significant silence
    ///
    /// # Errors
    ///
    /// Returns false if silence detection fails due to invalid sample rate or duration calculations.
    #[must_use]
    pub fn has_significant_silence(&self, buffer: &AudioBuffer<f32>) -> bool {
        self.detect_silence_segments(buffer)
            .is_ok_and(|segments| !segments.is_empty())
    }

    /// Gets total duration of silence in buffer
    ///
    /// # Errors
    ///
    /// Returns 0.0 if silence detection fails due to invalid sample rate or duration calculations.
    #[must_use]
    pub fn get_total_silence_duration(&self, buffer: &AudioBuffer<f32>) -> f32 {
        self.detect_silence_segments(buffer)
            .map_or(0.0, |segments| {
                segments.iter().map(|s| s.duration_secs).sum()
            })
    }
}

impl Default for SilenceDetector {
    /// Creates a new silence detector with default configuration
    fn default() -> Self {
        Self {
            config: SilenceDetectorConfig::default(),
        }
    }
}
