//! Audio decoding functionality using Symphonia

use std::fs::File;
use std::path::Path;

use symphonia::core::{
    audio::{AudioBufferRef, Signal}, // Only import AudioBufferRef and Signal
    codecs::{CODEC_TYPE_NULL, Decoder, DecoderOptions},
    errors::Error as SymphoniaError,
    formats::{FormatOptions, FormatReader, SeekMode, SeekTo, Track},
    io::{MediaSource, MediaSourceStream, MediaSourceStreamOptions},
    meta::MetadataOptions,
    probe::Hint,
};

use super::{AudioBuffer, AudioStream, SampleFormat};
use crate::error::{AppError, Result};

/// Audio decoder for various audio formats
pub struct AudioDecoder {
    /// The format reader for the audio file
    format: Box<dyn FormatReader>,
    /// The decoder for the audio stream
    decoder: Box<dyn Decoder>,
    /// The current track
    track: Track,
    /// The audio stream properties
    stream: AudioStream,
    /// The sample buffer for decoded audio (stored as f32 samples)
    sample_buffer: Vec<f32>,
    /// Current position in the sample buffer
    position: usize,
}

impl AudioDecoder {
    /// Creates a new `AudioDecoder` from a file path
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened, the format is not supported,
    /// or no suitable codec is found for the audio stream.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Open the file
        let file = File::open(path.as_ref()).map_err(AppError::Io)?; // Create a media source from the file
        let mss = MediaSourceStream::new(
            Box::new(file) as Box<dyn MediaSource>,
            MediaSourceStreamOptions::default(),
        );

        // Create a hint to help the format registry guess the format
        let mut hint = Hint::new();
        if let Some(ext) = path.as_ref().extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        // Use the default options for metadata and format readers
        let meta_opts = MetadataOptions::default();
        let fmt_opts = FormatOptions::default();

        // Probe the media format
        let probed = match symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)
        {
            Ok(probed) => probed,
            Err(e) => return Err(AppError::Audio(format!("Failed to probe format: {e}"))),
        };

        // Get the default track
        let track = probed
            .format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or_else(|| AppError::Audio("No supported audio tracks found".to_string()))?
            .clone();

        // Create a decoder for the track
        let dec_opts = DecoderOptions::default();
        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .map_err(|e| AppError::Audio(format!("Failed to create decoder: {e}")))?;

        // Get the codec parameters
        let codec_params = &track.codec_params;
        let sample_rate = codec_params.sample_rate.unwrap_or(44100);
        let channels = codec_params
            .channels
            .map_or(2, |c| u16::try_from(c.count()).unwrap_or(2));

        // Get the sample format from the codec parameters
        let sample_format = codec_params
            .sample_format
            .map_or(SampleFormat::F32, |fmt| match fmt {
                symphonia::core::sample::SampleFormat::U8 => SampleFormat::U8,
                symphonia::core::sample::SampleFormat::U16 => SampleFormat::U16,
                symphonia::core::sample::SampleFormat::U24 => SampleFormat::U24,
                symphonia::core::sample::SampleFormat::U32 => SampleFormat::U32,
                symphonia::core::sample::SampleFormat::S8 => {
                    // S8 not supported in local SampleFormat, map to U8 or return error if needed
                    SampleFormat::U8
                }
                symphonia::core::sample::SampleFormat::S16 => SampleFormat::S16,
                symphonia::core::sample::SampleFormat::S24 => SampleFormat::S24,
                symphonia::core::sample::SampleFormat::S32 => SampleFormat::S32,
                symphonia::core::sample::SampleFormat::F32 => SampleFormat::F32,
                symphonia::core::sample::SampleFormat::F64 => SampleFormat::F64,
            });

        // Create the audio stream
        let stream = AudioStream {
            sample_rate,
            channels,
            sample_format,
            duration: codec_params.n_frames.map(|f| {
                // Note: u64 to f64 cast may lose precision for very large values
                #[allow(clippy::cast_precision_loss)]
                let frames = f as f64;
                frames / f64::from(sample_rate)
            }),
        };

        // Create and return the decoder instance
        Ok(Self {
            format: probed.format,
            decoder,
            track: track.clone(),
            stream,
            sample_buffer: Vec::with_capacity(4096 * channels as usize),
            position: 0,
        })
    }
    /// Decodes the next packet of audio data into floating point samples
    ///
    /// # Errors
    ///
    /// Returns an error if the packet cannot be decoded, the format is unsupported,
    /// or there's an I/O error reading from the audio source.
    #[allow(clippy::too_many_lines)]
    pub fn next_packet(&mut self) -> Result<Option<AudioBuffer<f32>>> {
        // Clear the sample buffer
        self.sample_buffer.clear();

        // Read the next packet from the format reader
        let packet = match self.format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Ok(None);
            }
            Err(e) => return Err(AppError::Audio(format!("Failed to read packet: {e}"))),
        };

        // Decode the packet
        let decoded = match self.decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(e) => return Err(AppError::Audio(format!("Failed to decode packet: {e}"))),
        };

        // Get the audio specification
        let spec = *decoded.spec();
        let channels = spec.channels.count();

        // Resize the sample buffer if needed
        let required_capacity = decoded.frames() * channels;
        if self.sample_buffer.capacity() < required_capacity {
            self.sample_buffer
                .reserve(required_capacity - self.sample_buffer.capacity());
        }

        // Convert samples to interleaved f32 format
        match decoded {
            AudioBufferRef::F32(buf) => {
                let channels = buf.spec().channels.count();
                for frame in 0..buf.frames() {
                    for ch in 0..channels {
                        let sample = buf.chan(ch)[frame];
                        self.sample_buffer.push(sample);
                    }
                }
            }
            AudioBufferRef::U8(buf) => {
                let channels = buf.spec().channels.count();
                for frame in 0..buf.frames() {
                    for ch in 0..channels {
                        let sample = buf.chan(ch)[frame];
                        self.sample_buffer.push((f32::from(sample) - 128.0) / 128.0);
                    }
                }
            }
            AudioBufferRef::U16(buf) => {
                let channels = buf.spec().channels.count();
                for frame in 0..buf.frames() {
                    for ch in 0..channels {
                        let sample = buf.chan(ch)[frame];
                        self.sample_buffer
                            .push((f32::from(sample) - 32768.0) / 32768.0);
                    }
                }
            }
            AudioBufferRef::U24(buf) => {
                let channels = buf.spec().channels.count();
                for frame in 0..buf.frames() {
                    for ch in 0..channels {
                        let sample = buf.chan(ch)[frame].inner();
                        #[allow(clippy::cast_precision_loss)]
                        self.sample_buffer.push((sample as f32 / 8_388_608.0) - 1.0);
                    }
                }
            }
            AudioBufferRef::U32(buf) => {
                let channels = buf.spec().channels.count();
                for frame in 0..buf.frames() {
                    for ch in 0..channels {
                        let sample = buf.chan(ch)[frame];
                        #[allow(clippy::cast_precision_loss)]
                        self.sample_buffer
                            .push((sample as f32 / 2_147_483_648.0) - 1.0);
                    }
                }
            }
            AudioBufferRef::S8(_) => {
                // S8 not supported, skip or handle as needed
            }
            AudioBufferRef::S16(buf) => {
                let channels = buf.spec().channels.count();
                for frame in 0..buf.frames() {
                    for ch in 0..channels {
                        let sample = buf.chan(ch)[frame];
                        self.sample_buffer.push(f32::from(sample) / 32768.0);
                    }
                }
            }
            AudioBufferRef::S24(buf) => {
                let channels = buf.spec().channels.count();
                for frame in 0..buf.frames() {
                    for ch in 0..channels {
                        let sample = buf.chan(ch)[frame].inner();
                        #[allow(clippy::cast_precision_loss)]
                        self.sample_buffer
                            .push((sample as f32 / 8_388_608.0).clamp(-1.0, 1.0));
                    }
                }
            }
            AudioBufferRef::S32(buf) => {
                let channels = buf.spec().channels.count();
                for frame in 0..buf.frames() {
                    for ch in 0..channels {
                        let sample = buf.chan(ch)[frame];
                        #[allow(clippy::cast_precision_loss)]
                        self.sample_buffer
                            .push((sample as f32 / 2_147_483_648.0).clamp(-1.0, 1.0));
                    }
                }
            }
            AudioBufferRef::F64(buf) => {
                let channels = buf.spec().channels.count();
                for frame in 0..buf.frames() {
                    for ch in 0..channels {
                        let sample = buf.chan(ch)[frame];
                        #[allow(clippy::cast_possible_truncation)]
                        self.sample_buffer.push((sample as f32).clamp(-1.0, 1.0));
                    }
                }
            }
        }

        // Create and return the audio buffer
        Ok(Some(AudioBuffer {
            data: self.sample_buffer.clone(),
            format: self.stream.sample_format,
            sample_rate: self.stream.sample_rate,
            channels: self.stream.channels,
        }))
    }
    /// Gets information about the audio stream
    #[must_use]
    pub const fn stream_info(&self) -> &AudioStream {
        &self.stream
    }

    /// Gets the sample rate of the audio
    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        self.stream.sample_rate
    }

    /// Gets the number of channels in the audio
    #[must_use]
    pub const fn channels(&self) -> u16 {
        self.stream.channels
    }

    /// Gets the sample format of the audio
    #[must_use]
    pub const fn sample_format(&self) -> SampleFormat {
        self.stream.sample_format
    }
    /// Gets the duration of the audio in seconds, if known
    #[must_use]
    pub const fn duration(&self) -> Option<f64> {
        self.stream.duration
    }

    /// Seeks to a specific position in the audio stream
    ///
    /// # Errors
    ///
    /// Returns an error if the seek operation fails, the time base is not available,
    /// or the seek position is invalid.
    pub fn seek(&mut self, timestamp: f64) -> Result<()> {
        // Get the time base for the track
        let time_base =
            self.track.codec_params.time_base.ok_or_else(|| {
                AppError::Audio("Cannot seek: no time base available".to_string())
            })?; // Calculate the target time in the track's time base
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let time = (timestamp * f64::from(time_base.denom) / f64::from(time_base.numer)) as u64;

        // Seek to the specified time
        self.format
            .seek(
                SeekMode::Coarse,
                SeekTo::Time {
                    time: time.into(),
                    track_id: Some(self.track.id),
                },
            )
            .map_err(|e| AppError::Audio(format!("Seek failed: {e}")))?;

        // Reset the decoder state
        self.decoder.reset();

        // Clear the sample buffer
        self.sample_buffer.clear();
        self.position = 0;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_unsupported_format() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test.txt");
        std::fs::write(&test_file, "not an audio file")
            .map_err(|e| {
                eprintln!("Failed to write test file: {e}");
            })
            .ok();

        let result = AudioDecoder::open(&test_file);
        assert!(matches!(result, Err(AppError::Audio(_))));

        // Clean up
        let _ = std::fs::remove_file(test_file);
    }

    #[test]
    fn test_audio_buffer_duration() {
        // Create a test buffer with 2 channels, 44100 Hz, 1 second of audio
        let sample_rate = 44100;
        let channels = 2;
        let duration_seconds = 1.0;
        let sample_count = {
            let total_samples_f64 = sample_rate as f64 * duration_seconds;
            if total_samples_f64.is_finite()
                && total_samples_f64 >= 0.0
                && total_samples_f64 <= usize::MAX as f64
            {
                let sample_count = total_samples_f64.round() as usize;
                sample_count.saturating_mul(u16::try_from(channels).unwrap_or(2) as usize)
            } else {
                1024 // fallback safe size
            }
        };

        let data = vec![0.0f32; sample_count];
        let buffer = AudioBuffer {
            data,
            format: SampleFormat::F32,
            sample_rate,
            channels: u16::try_from(channels).unwrap_or(2),
        };

        assert_eq!(buffer.duration(), duration_seconds);
    }
}
