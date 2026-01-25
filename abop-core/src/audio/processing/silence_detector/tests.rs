//! Tests for silence detector
#[cfg(test)]
use crate::audio::processing::traits::core::{AudioProcessor, Validatable};
#[cfg(test)]
use crate::audio::processing::traits::reporting::LatencyReporting;
#[cfg(test)]
use crate::audio::SilenceDetector;
#[cfg(test)]
use crate::test_utils::audio::create_test_buffer_with_silence;
#[cfg(test)]
use crate::utils::casting::safe_samples_to_duration;

#[test]
fn test_silence_detector_creation() {
    let config = super::super::config::SilenceDetectorConfig {
        threshold_db: -60.0,
        min_duration: std::time::Duration::from_secs_f32(0.1),
        removal_mode: super::super::config::SilenceRemovalMode::None,
        ..Default::default()
    };
    let detector = SilenceDetector::new(config);
    assert!(detector.is_ok());
}

#[test]
fn test_detect_silence() {
    let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
    let config = super::super::config::SilenceDetectorConfig {
        threshold_db: -60.0,
        min_duration: std::time::Duration::from_secs_f32(0.1),
        removal_mode: super::super::config::SilenceRemovalMode::None,
        ..Default::default()
    };

    let mut detector = SilenceDetector::new(config).unwrap();
    let result = detector.process(&mut buffer);

    assert!(result.is_ok());
    let segments = detector.detect_silence_segments(&buffer).unwrap();
    assert!(!segments.is_empty());

    // Check that we found silence segment
    let silence_segment = segments.first().unwrap();
    let start_secs =
        safe_samples_to_duration(silence_segment.start, buffer.sample_rate).unwrap_or(0.0);
    assert!((start_secs - 0.3).abs() < 0.01);
    assert!((silence_segment.duration_secs - 0.2).abs() < 0.01);
}

#[test]
fn test_detect_silence_with_noise_floor() {
    let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
    // Add some noise to silence segment (convert -50dB to linear amplitude)
    let noise_amplitude = super::helpers::db_to_linear(-50.0);
    let silence_start = (0.3 * 44100.0) as usize;
    let silence_end = ((0.3 + 0.2) * 44100.0) as usize;
    for i in silence_start..silence_end {
        buffer.data[i] = noise_amplitude; // Proper noise floor amplitude
    }

    let config = super::super::config::SilenceDetectorConfig {
        threshold_db: -40.0, // Higher threshold than noise floor
        min_duration: std::time::Duration::from_secs_f32(0.1),
        removal_mode: super::super::config::SilenceRemovalMode::None,
        ..Default::default()
    };

    let mut detector = SilenceDetector::new(config).unwrap();
    let result = detector.process(&mut buffer);

    assert!(result.is_ok());
    let segments = detector.detect_silence_segments(&buffer).unwrap();
    assert!(
        !segments.is_empty(),
        "Should detect silence segment with noise floor below threshold"
    );

    // Check that we found silence segment
    let silence_segment = segments.first().unwrap();
    let start_secs =
        safe_samples_to_duration(silence_segment.start, buffer.sample_rate).unwrap_or(0.0);
    assert!(
        (start_secs - 0.3).abs() < 0.02,
        "Silence start time should be approximately 0.3s"
    );
    assert!(
        (silence_segment.duration_secs - 0.2).abs() < 0.02,
        "Silence duration should be approximately 0.2s"
    );
}

#[test]
fn test_detect_silence_with_long_segment() {
    let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.4); // 40% silence
    let config = super::super::config::SilenceDetectorConfig {
        threshold_db: -60.0,
        min_duration: std::time::Duration::from_secs_f32(0.1),
        removal_mode: super::super::config::SilenceRemovalMode::None,
        ..Default::default()
    };

    let mut detector = SilenceDetector::new(config).unwrap();
    let result = detector.process(&mut buffer);

    assert!(result.is_ok());
    let segments = detector.detect_silence_segments(&buffer).unwrap();
    assert!(!segments.is_empty());

    // Check that we found silence segment
    let silence_segment = segments.first().unwrap();
    let start_secs =
        safe_samples_to_duration(silence_segment.start, buffer.sample_rate).unwrap_or(0.0);
    assert!((start_secs - 0.3).abs() < 0.01);
    assert!((silence_segment.duration_secs - 0.4).abs() < 0.01);
}

#[test]
fn test_detect_silence_with_multiple_segments() {
    let mut buffer_with_silence = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
    let mut buffer_no_silence = create_test_buffer_with_silence(44100, 1, 1.0, 0.0, 0.0);

    let config = super::super::config::SilenceDetectorConfig {
        threshold_db: -60.0,
        min_duration: std::time::Duration::from_secs_f32(0.1),
        removal_mode: super::super::config::SilenceRemovalMode::None,
        ..Default::default()
    };

    let mut detector = SilenceDetector::new(config).unwrap();

    // Process both buffers
    let result_with_silence = detector.process(&mut buffer_with_silence);
    let result_no_silence = detector.process(&mut buffer_no_silence);

    assert!(result_with_silence.is_ok());
    assert!(result_no_silence.is_ok());

    let silence_segments_with = detector
        .detect_silence_segments(&buffer_with_silence)
        .unwrap();
    let silence_segments_without = detector
        .detect_silence_segments(&buffer_no_silence)
        .unwrap();

    assert!(!silence_segments_with.is_empty());
    assert!(silence_segments_without.is_empty());
}

#[test]
fn test_detect_silence_with_stereo() {
    let mut buffer = create_test_buffer_with_silence(44100, 2, 1.0, 0.3, 0.2);
    let config = super::super::config::SilenceDetectorConfig {
        threshold_db: -60.0,
        min_duration: std::time::Duration::from_secs_f32(0.1),
        removal_mode: super::super::config::SilenceRemovalMode::None,
        ..Default::default()
    };

    let mut detector = SilenceDetector::new(config).unwrap();
    let result = detector.process(&mut buffer);

    assert!(result.is_ok());
    let segments = detector.detect_silence_segments(&buffer).unwrap();
    assert!(
        !segments.is_empty(),
        "Should detect silence segment in stereo buffer"
    );

    // Check that we found silence segment
    let silence_segment = segments.first().unwrap();
    let start_secs = safe_samples_to_duration(
        silence_segment.start / buffer.channels as usize,
        buffer.sample_rate,
    )
    .unwrap_or(0.0);

    assert!(
        (start_secs - 0.3).abs() < 0.02,
        "Silence start time should be approximately 0.3s, but was {start_secs:.3}s"
    );
    assert!(
        (silence_segment.duration_secs - 0.2).abs() < 0.02,
        "Silence duration should be approximately 0.2s, but was {:.3}s",
        silence_segment.duration_secs
    );
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
    let config = super::super::config::SilenceDetectorConfig {
        threshold_db: -60.0,
        min_duration: std::time::Duration::from_millis(100),
        removal_mode: super::super::config::SilenceRemovalMode::LeadingTrailing,
        ..Default::default()
    };
    let mut detector = SilenceDetector::new(config).unwrap();
    let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
    let result = detector.process(&mut buffer);
    assert!(result.is_ok());
}

#[test]
fn test_silence_detector_all() {
    let config = super::super::config::SilenceDetectorConfig {
        threshold_db: -60.0,
        min_duration: std::time::Duration::from_millis(100),
        removal_mode: super::super::config::SilenceRemovalMode::All,
        ..Default::default()
    };
    let mut detector = SilenceDetector::new(config).unwrap();
    let mut buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
    let result = detector.process(&mut buffer);
    assert!(result.is_ok());
}

#[test]
fn test_silence_segment_validation() {
    // Valid segment
    let segment = super::types::SilenceSegment::new(100, 200, 1.0);
    assert!(segment.validate().is_ok());

    // Invalid segment (start >= end)
    let invalid_segment = super::types::SilenceSegment::new(200, 100, 1.0);
    assert!(invalid_segment.validate().is_err());

    // Invalid segment (negative duration)
    let negative_segment = super::types::SilenceSegment::new(100, 200, -1.0);
    assert!(negative_segment.validate().is_err());
}

#[test]
fn test_silence_percentage_calculation() {
    let buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
    let detector = SilenceDetector::default();

    let percentage = detector.calculate_silence_percentage(&buffer).unwrap();
    assert!(percentage > 0.0); // Should detect some silence
    assert!(percentage <= 100.0); // Should not exceed 100%
}

#[test]
fn test_has_significant_silence() {
    let buffer_with_silence = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
    let buffer_no_silence = create_test_buffer_with_silence(44100, 1, 1.0, 0.0, 0.0);
    let detector = SilenceDetector::default();

    assert!(detector.has_significant_silence(&buffer_with_silence));
    assert!(!detector.has_significant_silence(&buffer_no_silence));
}

#[test]
fn test_total_silence_duration() {
    let buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
    let detector = SilenceDetector::default();

    let total_duration = detector.get_total_silence_duration(&buffer);
    assert!((total_duration - 0.2).abs() < 0.01); // Should be approximately 0.2s
}

#[test]
fn test_helper_functions() {
    use super::helpers::*;

    // Test db conversion
    assert!((db_to_linear(0.0) - 1.0).abs() < f32::EPSILON);
    assert!((linear_to_db(1.0) - 0.0).abs() < f32::EPSILON);

    // Test sample finding
    let data = vec![0.0, 0.0, 0.001, 0.0];
    assert_eq!(find_first_non_silent_sample(&data, 0.0005), 2);
    assert_eq!(find_last_non_silent_sample(&data, 0.0005), 2);

    // Test frame silence detection
    let silent_frame = vec![0.0, 0.0, 0.0];
    let noisy_frame = vec![0.0, 0.002, 0.0];
    assert!(is_frame_silent(&silent_frame, 0.001));
    assert!(!is_frame_silent(&noisy_frame, 0.001));
}
