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
use std::sync::{Arc, Mutex};

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

/// A thread-safe pool of reusable audio buffers to reduce allocation overhead
/// during batch processing operations.
pub struct AudioBufferPool<T> {
    /// The pool of available buffers
    buffers: Arc<Mutex<Vec<AudioBuffer<T>>>>,
    /// The capacity of each buffer in the pool
    buffer_capacity: usize,
}

impl<T: Clone + Default> AudioBufferPool<T> {
    /// Creates a new buffer pool with the specified size and capacity
    ///
    /// # Arguments
    ///
    /// * `pool_size` - The number of buffers to pre-allocate
    /// * `buffer_capacity` - The capacity of each buffer in elements
    ///
    /// # Returns
    ///
    /// A new `AudioBufferPool` instance
    #[must_use]
    pub fn new(pool_size: usize, buffer_capacity: usize) -> Self {
        let mut buffers = Vec::with_capacity(pool_size);

        // Pre-allocate buffers with the specified capacity
        for _ in 0..pool_size {
            let mut data = Vec::with_capacity(buffer_capacity);
            // Initialize with default values to ensure capacity
            data.resize(buffer_capacity, T::default());
            // Clear to keep capacity but remove elements
            data.clear();

            buffers.push(AudioBuffer::new(
                data,
                SampleFormat::F32, // Default format, will be overridden when used
                44100,             // Default sample rate, will be overridden when used
                2,                 // Default channels, will be overridden when used
            ));
        }

        Self {
            buffers: Arc::new(Mutex::new(buffers)),
            buffer_capacity,
        }
    }

    /// Acquires a buffer from the pool, or creates a new one if none are available
    ///
    /// # Returns
    ///
    /// An `AudioBuffer<T>` that can be used for processing
    #[must_use] pub fn acquire(&self) -> AudioBuffer<T> {
        // Try to get a buffer from the pool
        if let Ok(mut buffers) = self.buffers.lock()
            && let Some(mut buffer) = buffers.pop()
        {
            // Clear any existing data but keep capacity
            buffer.data.clear();
            return buffer;
        }

        // If we couldn't get a buffer from the pool, create a new one
        AudioBuffer::new(
            Vec::with_capacity(self.buffer_capacity),
            SampleFormat::F32, // Default format, will be overridden when used
            44100,             // Default sample rate, will be overridden when used
            2,                 // Default channels, will be overridden when used
        )
    }

    /// Releases a buffer back to the pool
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to return to the pool
    pub fn release(&self, mut buffer: AudioBuffer<T>) {
        // Clear the buffer data but keep capacity
        buffer.data.clear();

        // Return the buffer to the pool
        if let Ok(mut buffers) = self.buffers.lock() {
            buffers.push(buffer);
        }
    }
}
