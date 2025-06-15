//! Audio test utilities for creating test buffers and other audio test helpers
//!
//! This module provides utilities for creating test audio buffers and other
//! audio-related test helpers. It consolidates test buffer creation functions
//! from various audio processing modules.

use crate::audio::AudioBuffer;
use crate::audio::SampleFormat;
use crate::utils::casting::domain::audio::{safe_duration_to_samples, safe_samples_to_duration};

/// Creates a test audio buffer with a sine wave
///
/// # Arguments
/// * `sample_rate` - Sample rate in Hz
/// * `channels` - Number of channels
/// * `duration_secs` - Duration in seconds
/// * `amplitude` - Optional amplitude (defaults to 0.5)
///
/// # Returns
/// An `AudioBuffer` containing a sine wave test signal
#[must_use]
pub fn create_test_buffer(
    sample_rate: u32,
    channels: u16,
    duration_secs: f32,
    amplitude: Option<f32>,
) -> AudioBuffer<f32> {
    let amplitude = amplitude.unwrap_or(0.5);
    let num_samples = safe_duration_to_samples(duration_secs, sample_rate).unwrap_or(0);
    let channels_usize = usize::from(channels);
    let mut data = Vec::with_capacity(num_samples * channels_usize);

    // Generate a simple sine wave test signal
    for i in 0..num_samples {
        let t = safe_samples_to_duration(i, sample_rate).unwrap_or(0.0);
        let sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin() * amplitude;

        // Duplicate for each channel
        for _ in 0..channels {
            data.push(sample);
        }
    }

    AudioBuffer {
        data,
        format: SampleFormat::F32,
        sample_rate,
        channels,
    }
}

/// Creates a test audio buffer with a silence segment
///
/// # Arguments
/// * `sample_rate` - Sample rate in Hz
/// * `channels` - Number of channels
/// * `duration_secs` - Total duration in seconds
/// * `silence_start` - Start time of silence in seconds
/// * `silence_duration` - Duration of silence in seconds
///
/// # Returns
/// An `AudioBuffer` containing a sine wave with a silence segment
#[must_use]
pub fn create_test_buffer_with_silence(
    sample_rate: u32,
    channels: u16,
    duration_secs: f32,
    silence_start: f32,
    silence_duration: f32,
) -> AudioBuffer<f32> {
    let num_samples = safe_duration_to_samples(duration_secs, sample_rate).unwrap_or(0);
    let silence_start_sample = safe_duration_to_samples(silence_start, sample_rate).unwrap_or(0);
    let silence_samples = safe_duration_to_samples(silence_duration, sample_rate).unwrap_or(0);
    let mut data = Vec::with_capacity(num_samples * usize::from(channels));

    for i in 0..num_samples {
        // Calculate time in seconds for this sample
        let t = i as f32 / sample_rate as f32;
        let is_silent = i >= silence_start_sample && i < silence_start_sample + silence_samples;

        let sample = if is_silent {
            0.0
        } else {
            (t * 440.0 * 2.0 * std::f32::consts::PI).sin() * 0.5
        };

        // Duplicate for each channel
        for _ in 0..channels {
            data.push(sample);
        }
    }

    AudioBuffer {
        data,
        format: SampleFormat::F32,
        sample_rate,
        channels,
    }
}

/// Creates a test audio buffer with stereo channels
///
/// # Arguments
/// * `sample_rate` - Sample rate in Hz
/// * `duration_secs` - Duration in seconds
///
/// # Returns
/// An `AudioBuffer` containing stereo test signals
#[must_use]
pub fn create_stereo_test_buffer(sample_rate: u32, duration_secs: f32) -> AudioBuffer<f32> {
    let num_samples = safe_duration_to_samples(duration_secs, sample_rate).unwrap_or(0);
    let mut data = Vec::with_capacity(num_samples * 2); // Generate a simple sine wave test signal
    for i in 0..num_samples {
        let t = safe_samples_to_duration(i, sample_rate).unwrap_or(0.0);
        // Add phase offset to right channel to ensure it's never identical to left
        let left_sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin() * 0.5;
        let right_sample = (t * 880.0 * 2.0)
            .mul_add(std::f32::consts::PI, std::f32::consts::PI / 4.0)
            .sin()
            * 0.3;
        data.push(left_sample);
        data.push(right_sample);
    }

    AudioBuffer {
        data,
        format: SampleFormat::F32,
        sample_rate,
        channels: 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_buffer() {
        let buffer = create_test_buffer(44100, 2, 0.1, None);
        assert_eq!(buffer.sample_rate, 44100);
        assert_eq!(buffer.channels, 2);
        assert!(!buffer.data.is_empty());
    }

    #[test]
    fn test_create_test_buffer_with_silence() {
        let buffer = create_test_buffer_with_silence(44100, 1, 1.0, 0.3, 0.2);
        assert_eq!(buffer.sample_rate, 44100);
        assert_eq!(buffer.channels, 1);

        // Check that silence segment exists
        let silence_start = (0.3 * 44100.0) as usize;
        let silence_end = ((0.3 + 0.2) * 44100.0) as usize;
        for i in silence_start..silence_end {
            assert!(buffer.data[i].abs() < 0.001);
        }
    }

    #[test]
    fn test_create_stereo_test_buffer() {
        let buffer = create_stereo_test_buffer(44100, 0.1);
        assert_eq!(buffer.sample_rate, 44100);
        assert_eq!(buffer.channels, 2);
        assert_eq!(buffer.data.len() % 2, 0);

        // Check that left and right channels are different
        for i in (0..buffer.data.len()).step_by(2) {
            assert_ne!(buffer.data[i], buffer.data[i + 1]);
        }
    }
}
