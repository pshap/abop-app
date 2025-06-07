//! Audio processing module for ABOP
//!
//! This module provides functionality for decoding, processing, and analyzing audio files.

pub mod decoder;
pub mod metadata;
pub mod player;
pub mod processing;

// Re-export the public API
pub use decoder::AudioDecoder;
pub use metadata::AudioMetadata;
pub use player::{AudioPlayer, PlayerState};
pub use processing::{
    AudioNormalizer, AudioProcessingPipeline, ChannelMixer, ChannelMixerConfig, LinearResampler,
    MixingAlgorithm, NormalizerConfig, OutputConfig, ProcessingConfig, ResamplerConfig,
    SilenceDetector, SilenceDetectorConfig,
};

use std::path::Path;

/// Supported audio file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioFormat {
    /// MP3 audio
    Mp3,
    /// AAC audio (usually in .m4a or .m4b containers)
    Aac,
    /// FLAC audio
    Flac,
    /// Ogg Vorbis audio
    Ogg,
    /// WAV audio
    Wav,
    /// Opus audio (usually in .opus or .ogg containers)
    Opus,
}

impl AudioFormat {
    /// Gets the default file extension for this format
    #[must_use]
    pub const fn extension(&self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
            Self::Aac => "m4a",
            Self::Flac => "flac",
            Self::Ogg => "ogg",
            Self::Wav => "wav",
            Self::Opus => "opus",
        }
    }

    /// Gets the MIME type for this format
    #[must_use]
    pub const fn mime_type(&self) -> &'static str {
        match self {
            Self::Mp3 => "audio/mpeg",
            Self::Aac => "audio/mp4",
            Self::Flac => "audio/flac",
            Self::Ogg => "audio/ogg",
            Self::Wav => "audio/wav",
            Self::Opus => "audio/opus",
        }
    }

    /// Detects the audio format from a file extension
    #[must_use]
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp3" => Some(Self::Mp3),
            "m4a" | "m4b" | "aac" => Some(Self::Aac),
            "flac" => Some(Self::Flac),
            "ogg" | "oga" => Some(Self::Ogg),
            "wav" | "wave" => Some(Self::Wav),
            "opus" => Some(Self::Opus),
            _ => None,
        }
    }

    /// Detects the audio format from a file path
    #[must_use]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }
}

/// Represents an audio stream with its properties
#[derive(Debug, Clone, PartialEq)]
pub struct AudioStream {
    /// The sample rate in Hz
    pub sample_rate: u32,
    /// The number of channels
    pub channels: u16,
    /// The sample format
    pub sample_format: SampleFormat,
    /// The duration in seconds, if known
    pub duration: Option<f64>,
}

/// Represents the format of audio samples
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SampleFormat {
    /// 8-bit unsigned integer
    U8,
    /// 16-bit unsigned integer
    U16,
    /// 24-bit unsigned integer
    U24,
    /// 32-bit unsigned integer
    U32,
    /// 16-bit signed integer
    S16,
    /// 24-bit signed integer
    S24,
    /// 32-bit signed integer
    S32,
    /// 32-bit floating point
    F32,
    /// 64-bit floating point
    F64,
}

impl SampleFormat {
    /// Gets the size in bytes of a single sample of this format
    #[must_use]
    pub const fn sample_size(&self) -> usize {
        match self {
            Self::U8 => 1,
            Self::U16 | Self::S16 => 2,
            Self::U24 | Self::S24 => 3,
            Self::U32 | Self::S32 | Self::F32 => 4,
            Self::F64 => 8,
        }
    }

    /// Gets the number of bits per sample for this format
    #[must_use]
    pub const fn bits_per_sample(&self) -> u32 {
        match self {
            Self::U8 => 8,
            Self::U16 | Self::S16 => 16,
            Self::U24 | Self::S24 => 24,
            Self::U32 | Self::S32 | Self::F32 => 32,
            Self::F64 => 64,
        }
    }
}

/// Represents a segment of audio data with a specific sample type
#[derive(Debug, Clone)]
pub struct AudioBuffer<T> {
    /// The audio data
    pub data: Vec<T>,
    /// The format of the audio data
    pub format: SampleFormat,
    /// The sample rate in Hz
    pub sample_rate: u32,
    /// The number of channels
    pub channels: u16,
}

impl<T> AudioBuffer<T> {
    /// Creates a new audio buffer    #[`must_use`]
    #[must_use]
    pub const fn new(data: Vec<T>, format: SampleFormat, sample_rate: u32, channels: u16) -> Self {
        Self {
            data,
            format,
            sample_rate,
            channels,
        }
    }

    /// Gets the duration of the audio buffer in seconds
    #[must_use]
    pub fn duration(&self) -> f64 {
        // For interleaved audio data, the number of frames is the total number of samples
        // divided by the number of channels
        let num_frames = self.data.len() / self.channels as usize;

        // The cast from usize to f64 may lose precision for very large values (> 2^53)
        // but is acceptable for duration calculations where slight precision loss is not critical
        #[allow(clippy::cast_precision_loss)]
        let num_frames_f64 = num_frames as f64;

        num_frames_f64 / f64::from(self.sample_rate)
    }
}

// TODO: Implement AudioBufferPool for memory optimization
// High priority memory pool implementation needed for batch processing
// - Create AudioBufferPool<T> struct with Arc<Mutex<Vec<AudioBuffer<T>>>>
// - Implement new(pool_size: usize, buffer_capacity: usize) method
// - Add acquire() -> Option<AudioBuffer<T>> method for getting pooled buffers
// - Add release(buffer: AudioBuffer<T>) method for returning buffers to pool
// - Clear buffer data but keep capacity when releasing
// - Thread-safe implementation for use in BatchProcessor parallel operations
// - Pool size should be 16 buffers with 1MB capacity each for optimal performance
// - Integrate with existing BatchProcessor::process_files_parallel method
// - Reduces allocation overhead during bulk audio processing operations
