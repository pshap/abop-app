//! Specialized audio processing traits for advanced functionality
//!
//! This module defines traits for specialized processing scenarios such as
//! streaming, real-time processing, and file I/O operations.

use super::super::error::Result;
use super::core::AudioProcessor;
use crate::audio::AudioBuffer;

/// Trait for processors designed for streaming audio processing
pub trait StreamingProcessor: AudioProcessor {
    /// Process audio with streaming-specific optimizations
    ///
    /// This method should be used when processing continuous streams of audio
    /// data where latency and memory usage are critical.
    ///
    /// # Arguments
    /// * `input` - Input audio buffer
    /// * `output` - Output audio buffer (may be the same as input)
    ///
    /// # Errors
    ///
    /// Returns an error if streaming processing fails.
    fn process_streaming(
        &mut self,
        input: &AudioBuffer<f32>,
        output: &mut AudioBuffer<f32>,
    ) -> Result<()>;

    /// Set the target latency for streaming processing
    ///
    /// # Arguments
    /// * `latency_samples` - Target latency in samples
    ///
    /// # Errors
    ///
    /// Returns an error if the requested latency cannot be achieved.
    fn set_streaming_latency(&mut self, latency_samples: usize) -> Result<()>;

    /// Flush any internal buffers (end of stream)
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Trait for processors that can write processed audio to files
pub trait FileWriter {
    /// Write processed audio data to a file
    ///
    /// # Arguments
    /// * `buffer` - The audio buffer to write
    /// * `file_path` - Path where the audio should be written
    ///
    /// # Errors
    ///
    /// Returns an error if file writing fails due to I/O issues,
    /// permission problems, or format incompatibility.
    fn write_to_file(&self, buffer: &AudioBuffer<f32>, file_path: &std::path::Path) -> Result<()>;

    /// Get supported output formats
    fn supported_output_formats(&self) -> Vec<String> {
        vec!["wav".to_string()]
    }
}

