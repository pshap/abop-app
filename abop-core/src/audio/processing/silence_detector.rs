//! Audio silence detection and removal functionality
//!
//! This module provides silence detection and removal capabilities for audio buffers,
//! including configurable thresholds and minimum duration requirements.

use super::{
    casting_utils::{
        safe_conversions::safe_progress,
        sample_calculations::{safe_duration_to_samples, safe_samples_to_duration},
    },
    config::{SilenceDetectorConfig, SilenceRemovalMode},
    error::Result,
    traits::{AudioProcessor, Configurable, LatencyReporting, Validatable},
    validation::ConfigValidator,
};
use crate::audio::AudioBuffer;

/// Silence detection error type
#[derive(Debug, thiserror::Error)]
pub enum SilenceDetectorError {
    /// Invalid silence parameter
    #[error("Invalid silence parameter: {0}")]
    InvalidParameter(String),

    /// Processing error
    #[error("Silence detection failed: {0}")]
    ProcessingError(String),
}

/// Represents a segment of silence in an audio buffer
#[derive(Debug, Clone, PartialEq)]
pub struct SilenceSegment {
    /// Start position in samples
    pub start: usize,
    /// End position in samples (exclusive)
    pub end: usize,
    /// Duration in seconds
    pub duration_secs: f32,
}

/// Audio silence detector and remover
///
/// Detects and optionally removes silence from audio buffers based on
/// configurable threshold and minimum duration parameters.
#[derive(Debug, Clone)]
pub struct SilenceDetector {
    config: SilenceDetectorConfig,
}

impl SilenceDetector {
    /// Creates a new silence detector with the specified configuration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if the configuration
    /// parameters are invalid (e.g., invalid threshold or duration values).
    pub fn new(config: SilenceDetectorConfig) -> Result<Self> {
        ConfigValidator::validate_silence_detector_config(&config)?;
        Ok(Self { config })
    }
    /// Creates a new silence detector with specific threshold and minimum duration
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidConfiguration`] if the threshold or
    /// minimum duration parameters are invalid.
    pub fn with_params(threshold_db: f32, min_duration_secs: f32) -> Result<Self> {
        let config = SilenceDetectorConfig {
            threshold_db,
            min_duration: std::time::Duration::from_secs_f32(min_duration_secs),
            ..Default::default()
        };
        Self::new(config)
    }

    /// Processes the audio buffer to remove silence according to configuration
    fn process_silence(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        if buffer.data.is_empty() {
            return Ok(());
        }

        match self.config.removal_mode {
            SilenceRemovalMode::LeadingTrailing => {
                self.remove_leading_trailing_silence(buffer)
            }
            SilenceRemovalMode::All => self.remove_all_silence(buffer),
            SilenceRemovalMode::None => {
                // Just detect but don't remove
                let _segments = self.detect_silence_segments(buffer)?;
                Ok(())
            }
        }
    }

    /// Removes silence from the beginning and end of the audio buffer
    fn remove_leading_trailing_silence(&self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        let threshold = Self::db_to_linear(self.config.threshold_db);
        let min_samples =
            safe_duration_to_samples(self.config.min_duration.as_secs_f32(), buffer.sample_rate)?;

        // Find the first non-silent sample
        let start = Self::find_first_non_silent_sample(&buffer.data, threshold);

        // Find the last non-silent sample
        let end = Self::find_last_non_silent_sample(&buffer.data, threshold);

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
                let duration = safe_samples_to_duration(removed_samples, buffer.sample_rate)?;
                log::debug!(
                    "Removed {removed_samples} samples of leading/trailing silence ({duration:.3}s)"
                );

                buffer.data = buffer.data[new_start..new_end].to_vec();
            }
        }

        Ok(())
    }

    /// Removes all silence segments from the audio buffer
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

        // Add remaining non-silent audio after the last silence segment
        if last_end < buffer.data.len() {
            new_data.extend_from_slice(&buffer.data[last_end..]);
        }

        let removed_samples = buffer.data.len() - new_data.len();
        if removed_samples > 0 {
            let duration = safe_samples_to_duration(removed_samples, buffer.sample_rate)?;
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

    /// Detects all silence segments in the audio buffer
    ///
    /// # Errors
    ///
    /// Returns an error if the sample rate conversion fails or duration calculations overflow.
    pub fn detect_silence_segments(
        &self,
        buffer: &AudioBuffer<f32>,
    ) -> Result<Vec<SilenceSegment>> {
        if buffer.data.is_empty() {
            return Ok(Vec::new());
        }
        let threshold = Self::db_to_linear(self.config.threshold_db);
        let min_samples =
            safe_duration_to_samples(self.config.min_duration.as_secs_f32(), buffer.sample_rate)?;
        let mut segments = Vec::new();
        let mut in_silence = false;
        let mut silence_start = 0;

        for (i, &sample) in buffer.data.iter().enumerate() {
            let is_silent = sample.abs() <= threshold;

            if is_silent && !in_silence {
                // Start of silence
                in_silence = true;
                silence_start = i;
            } else if !is_silent && in_silence {
                // End of silence
                let silence_length = i - silence_start;
                if silence_length >= min_samples {
                    let duration_secs =
                        safe_samples_to_duration(silence_length, buffer.sample_rate)?;
                    segments.push(SilenceSegment {
                        start: silence_start,
                        end: i,
                        duration_secs,
                    });
                }
                in_silence = false;
            }
        }

        // Handle silence at the end of the buffer
        if in_silence {
            let silence_length = buffer.data.len() - silence_start;
            if silence_length >= min_samples {
                let duration_secs = safe_samples_to_duration(silence_length, buffer.sample_rate)?;
                segments.push(SilenceSegment {
                    start: silence_start,
                    end: buffer.data.len(),
                    duration_secs,
                });
            }
        }

        Ok(segments)
    }

    /// Finds the first non-silent sample in the audio data
    fn find_first_non_silent_sample(data: &[f32], threshold: f32) -> usize {
        data.iter().position(|&s| s.abs() > threshold).unwrap_or(0)
    }

    /// Finds the last non-silent sample in the audio data
    fn find_last_non_silent_sample(data: &[f32], threshold: f32) -> usize {
        data.iter()
            .rposition(|&s| s.abs() > threshold)
            .unwrap_or_else(|| data.len().saturating_sub(1))
    }

    /// Converts decibel value to linear amplitude
    fn db_to_linear(db: f32) -> f32 {
        10.0f32.powf(db / 20.0)
    }

    /// Calculates the percentage of silence in the audio buffer
    ///
    /// # Errors
    ///
    /// Returns an error if silence detection fails due to invalid sample rate or duration calculations.
    pub fn calculate_silence_percentage(&self, buffer: &AudioBuffer<f32>) -> Result<f32> {
        if buffer.data.is_empty() {
            return Ok(0.0);
        }

        let segments = self.detect_silence_segments(buffer)?;
        let total_silence_samples: usize = segments.iter().map(|s| s.end - s.start).sum();

        // Safe casting for silence percentage calculation
        let silence_ratio = safe_progress(total_silence_samples, buffer.data.len())?;
        Ok(silence_ratio * 100.0)
    }

    /// Checks if the audio buffer contains significant silence
    ///
    /// # Errors
    ///
    /// Returns false if silence detection fails due to invalid sample rate or duration calculations.
    #[must_use]
    pub fn has_significant_silence(&self, buffer: &AudioBuffer<f32>) -> bool {
        self.detect_silence_segments(buffer)
            .is_ok_and(|segments| !segments.is_empty())
    }

    /// Gets the total duration of silence in the buffer
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

impl AudioProcessor for SilenceDetector {
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) -> Result<()> {
        self.process_silence(buffer)
    }

    fn reset(&mut self) {
        // Silence detector is stateless, nothing to reset
    }
}

impl Configurable<SilenceDetectorConfig> for SilenceDetector {
    fn configure(&mut self, config: SilenceDetectorConfig) -> Result<()> {
        ConfigValidator::validate_silence_detector_config(&config)?;
        self.config = config;
        Ok(())
    }

    fn get_config(&self) -> &SilenceDetectorConfig {
        &self.config
    }
}

impl LatencyReporting for SilenceDetector {
    fn get_latency_samples(&self) -> usize {
        // Silence detection has no latency
        0
    }
}

impl Validatable for SilenceDetector {
    fn validate(&self) -> Result<()> {
        ConfigValidator::validate_silence_detector_config(&self.config)
            .map_err(std::convert::Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::audio::{create_test_buffer, create_test_buffer_with_silence};

    #[test]
    fn test_silence_detector_creation() {
        let config = SilenceDetectorConfig {
            threshold_db: -60.0,
            min_duration: std::time::Duration::from_secs_f32(0.1),
            removal_mode: SilenceRemovalMode::None,
            ..Default::default()
        };
        let detector = SilenceDetector::new(config);
        assert!(detector.is_ok());
    }

    #[test]
    fn test_detect_silence() {
        let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
        let config = SilenceDetectorConfig {
            threshold_db: -60.0,
            min_duration: std::time::Duration::from_secs_f32(0.1),
            removal_mode: SilenceRemovalMode::None,
            ..Default::default()
        };

        let mut detector = SilenceDetector::new(config).unwrap();
        let result = detector.process(&mut buffer);

        assert!(result.is_ok());
        let segments = detector.detect_silence_segments(&buffer).unwrap();
        assert!(!segments.is_empty());

        // Check that we found the silence segment
        let silence_segment = segments.first().unwrap();
        let start_secs = safe_samples_to_duration(silence_segment.start, buffer.sample_rate).unwrap_or(0.0);
        assert!((start_secs - 0.3).abs() < 0.01);
        assert!((silence_segment.duration_secs - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_detect_silence_with_noise_floor() {
        let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
        // Add some noise to the silence segment
        let silence_start = (0.3 * 44100.0) as usize;
        let silence_end = ((0.3 + 0.2) * 44100.0) as usize;
        for i in silence_start..silence_end {
            buffer.data[i] = -50.0; // -50dB noise floor
        }

        let config = SilenceDetectorConfig {
            threshold_db: -40.0, // Higher threshold
            min_duration: std::time::Duration::from_secs_f32(0.1),
            removal_mode: SilenceRemovalMode::None,
            ..Default::default()
        };

        let mut detector = SilenceDetector::new(config).unwrap();
        let result = detector.process(&mut buffer);

        assert!(result.is_ok());
        let segments = detector.detect_silence_segments(&buffer).unwrap();
        assert!(!segments.is_empty());

        // Check that we found the silence segment
        let silence_segment = segments.first().unwrap();
        let start_secs = safe_samples_to_duration(silence_segment.start, buffer.sample_rate).unwrap_or(0.0);
        assert!((start_secs - 0.3).abs() < 0.01);
        assert!((silence_segment.duration_secs - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_detect_silence_with_long_segment() {
        let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.4); // 40% silence
        let config = SilenceDetectorConfig {
            threshold_db: -60.0,
            min_duration: std::time::Duration::from_secs_f32(0.1),
            removal_mode: SilenceRemovalMode::None,
            ..Default::default()
        };

        let mut detector = SilenceDetector::new(config).unwrap();
        let result = detector.process(&mut buffer);

        assert!(result.is_ok());
        let segments = detector.detect_silence_segments(&buffer).unwrap();
        assert!(!segments.is_empty());

        // Check that we found the silence segment
        let silence_segment = segments.first().unwrap();
        let start_secs = safe_samples_to_duration(silence_segment.start, buffer.sample_rate).unwrap_or(0.0);
        assert!((start_secs - 0.3).abs() < 0.01);
        assert!((silence_segment.duration_secs - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_detect_silence_with_multiple_segments() {
        let mut buffer_with_silence = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
        let mut buffer_no_silence = create_test_buffer_with_silence(44100, 1, 1.0, 0.0, 0.0);

        let config = SilenceDetectorConfig {
            threshold_db: -60.0,
            min_duration: std::time::Duration::from_secs_f32(0.1),
            removal_mode: SilenceRemovalMode::None,
            ..Default::default()
        };

        let mut detector = SilenceDetector::new(config).unwrap();

        // Process both buffers
        let result_with_silence = detector.process(&mut buffer_with_silence);
        let result_no_silence = detector.process(&mut buffer_no_silence);

        assert!(result_with_silence.is_ok());
        assert!(result_no_silence.is_ok());

        let silence_segments_with = detector.detect_silence_segments(&buffer_with_silence).unwrap();
        let silence_segments_without = detector.detect_silence_segments(&buffer_no_silence).unwrap();

        assert!(!silence_segments_with.is_empty());
        assert!(silence_segments_without.is_empty());
    }

    #[test]
    fn test_detect_silence_with_stereo() {
        let mut buffer = create_test_buffer_with_silence(44100, 2, 1.0, 0.3, 0.2);
        let config = SilenceDetectorConfig {
            threshold_db: -60.0,
            min_duration: std::time::Duration::from_secs_f32(0.1),
            removal_mode: SilenceRemovalMode::None,
            ..Default::default()
        };

        let mut detector = SilenceDetector::new(config).unwrap();
        let result = detector.process(&mut buffer);

        assert!(result.is_ok());
        let segments = detector.detect_silence_segments(&buffer).unwrap();
        assert!(!segments.is_empty());

        // Check that we found the silence segment
        let silence_segment = segments.first().unwrap();
        let start_secs = safe_samples_to_duration(silence_segment.start, buffer.sample_rate).unwrap_or(0.0);
        assert!((start_secs - 0.3).abs() < 0.01);
        assert!((silence_segment.duration_secs - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_silence_detector_latency() {
        let detector = SilenceDetector::default();
        assert_eq!(detector.get_latency_samples(), 0);
    }

    #[test]
    fn test_silence_detector_validation() {
        let detector = SilenceDetector::default();
        assert!(detector.validate().is_ok());
    }

    #[test]
    fn test_silence_detector_leading_trailing() {
        let config = SilenceDetectorConfig {
            threshold_db: -60.0,
            min_duration: std::time::Duration::from_millis(100),
            removal_mode: SilenceRemovalMode::LeadingTrailing,
            ..Default::default()
        };
        let mut detector = SilenceDetector::new(config).unwrap();
        let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
        let result = detector.process(&mut buffer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_silence_detector_all() {
        let config = SilenceDetectorConfig {
            threshold_db: -60.0,
            min_duration: std::time::Duration::from_millis(100),
            removal_mode: SilenceRemovalMode::All,
            ..Default::default()
        };
        let mut detector = SilenceDetector::new(config).unwrap();
        let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
        let result = detector.process(&mut buffer);
        assert!(result.is_ok());
    }
}
