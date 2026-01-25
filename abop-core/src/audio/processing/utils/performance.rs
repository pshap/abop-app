//! Performance utilities for timing and estimation

use crate::audio::AudioBuffer;
use crate::utils::casting::safe_usize_to_f64_audio;
use std::time::Duration;

// Re-export: unified Timer from utils
pub use crate::utils::timer::Timer;

/// Estimate processing time based on buffer size and sample rate
#[must_use]
pub fn estimate_processing_time<T>(buffer: &AudioBuffer<T>) -> Duration {
    let samples_per_channel = buffer.data.len() / usize::from(buffer.channels); // Use safe casting utility for duration calculation
    let duration_seconds =
        safe_usize_to_f64_audio(samples_per_channel) / f64::from(buffer.sample_rate);

    // Rough estimate: processing takes about 10% of audio duration minimum
    let base_processing_time = Duration::from_secs_f64(duration_seconds * 0.1);

    // Add overhead based on channel count and sample rate
    let channel_overhead = Duration::from_millis(u64::from(buffer.channels) * 10);
    let sample_rate_overhead = Duration::from_millis(u64::from(buffer.sample_rate) / 1000);

    base_processing_time + channel_overhead + sample_rate_overhead
}

/// Estimate memory usage for processing
#[must_use]
pub const fn estimate_memory_usage<T>(buffer: &AudioBuffer<T>) -> usize {
    std::mem::size_of::<T>() * buffer.data.len() * 2 // Double for processing overhead
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::{AudioBuffer, SampleFormat};

    #[test]
    fn test_estimate_processing_time() {
        let buffer = AudioBuffer {
            data: vec![0.0f32; 44100], // 1 second of audio at 44.1kHz
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 2,
        };

        let duration = estimate_processing_time(&buffer);
        // Should be around 100ms (0.1s base + overhead)
        assert!(duration.as_millis() >= 100);
        assert!(duration.as_millis() <= 500); // Reasonable upper bound
    }

    #[test]
    fn test_estimate_memory_usage() {
        let buffer_f32 = AudioBuffer {
            data: vec![0.0f32; 1000],
            format: SampleFormat::F32,
            sample_rate: 44100,
            channels: 2,
        };

        let buffer_i16 = AudioBuffer {
            data: vec![0i16; 1000],
            format: SampleFormat::I16,
            sample_rate: 44100,
            channels: 2,
        };

        // f32: 1000 samples * 2 channels * 4 bytes = 8000 bytes
        assert_eq!(estimate_memory_usage(&buffer_f32), 16000); // *2 for overhead

        // i16: 1000 samples * 2 channels * 2 bytes = 4000 bytes
        assert_eq!(estimate_memory_usage(&buffer_i16), 8000); // *2 for overhead
    }
}
