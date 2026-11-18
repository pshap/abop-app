//! A rodio [`Source`] backed by our Symphonia [`AudioDecoder`]
//!
//! rodio 0.20's built-in decoder wraps the input in a media source that reports
//! no byte length. Symphonia's MP4 reader needs the length to locate the index
//! (`moov` atom) when it sits at the end of the file, which is common for M4B
//! audiobooks, so rodio fails (and panics) on them. [`AudioDecoder`] opens the
//! `File` directly, so all formats Symphonia supports play correctly.

use std::path::Path;
use std::time::Duration;

use rodio::Source;

use super::decoder::AudioDecoder;
use crate::error::Result;

/// Give up after this many consecutive undecodable packets
const MAX_CONSECUTIVE_DECODE_ERRORS: usize = 16;

/// Streams interleaved `f32` samples from a file for playback
pub struct DecoderSource {
    decoder: AudioDecoder,
    buffer: Vec<f32>,
    position: usize,
    channels: u16,
    sample_rate: u32,
    total_duration: Option<Duration>,
}

impl DecoderSource {
    /// Open a file for playback
    ///
    /// Decodes the first packet up front so format errors surface here rather
    /// than as silence.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or contains no decodable audio.
    pub fn open(path: &Path) -> Result<Self> {
        let mut decoder = AudioDecoder::open(path)?;
        let total_duration = decoder
            .duration()
            .filter(|d| d.is_finite() && *d > 0.0)
            .map(Duration::from_secs_f64);
        let first = next_buffer(&mut decoder).ok_or_else(|| {
            crate::error::AppError::Audio(format!("No playable audio in {}", path.display()))
        })?;

        Ok(Self {
            channels: first.channels.max(1),
            sample_rate: first.sample_rate,
            buffer: first.data,
            position: 0,
            decoder,
            total_duration,
        })
    }
}

/// Decode the next non-empty packet, skipping occasional corrupt packets
fn next_buffer(decoder: &mut AudioDecoder) -> Option<super::AudioBuffer<f32>> {
    let mut errors = 0;
    loop {
        match decoder.next_packet() {
            Ok(Some(buffer)) if buffer.data.is_empty() => {}
            Ok(Some(buffer)) => return Some(buffer),
            Ok(None) => return None,
            Err(error) => {
                errors += 1;
                if errors >= MAX_CONSECUTIVE_DECODE_ERRORS {
                    log::warn!("Stopping playback after repeated decode errors: {error}");
                    return None;
                }
            }
        }
    }
}

impl Iterator for DecoderSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.position >= self.buffer.len() {
            let buffer = next_buffer(&mut self.decoder)?;
            self.buffer = buffer.data;
            self.position = 0;
        }
        let sample = self.buffer[self.position];
        self.position += 1;
        Some(sample)
    }
}

impl Source for DecoderSource {
    fn current_frame_len(&self) -> Option<usize> {
        // Samples left in the current packet; format can only change between packets
        Some(self.buffer.len() - self.position).filter(|n| *n > 0)
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        self.total_duration
    }

    fn try_seek(&mut self, pos: Duration) -> std::result::Result<(), rodio::source::SeekError> {
        self.decoder
            .seek(pos.as_secs_f64())
            .map_err(|e| rodio::source::SeekError::Other(Box::new(e)))?;
        // Drop the rest of the old packet; the next sample comes from the new position
        self.buffer.clear();
        self.position = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    #[test]
    fn streams_m4b_and_mp3() {
        for (name, rate, channels) in [("tagged.m4b", 22_050, 2), ("tagged.mp3", 8_000, 1)] {
            let source = DecoderSource::open(&fixture(name)).unwrap();
            assert_eq!(source.sample_rate(), rate, "{name}");
            assert_eq!(source.channels(), channels, "{name}");
            let duration = source.total_duration().expect("duration is known");
            assert!(
                (0.8..1.5).contains(&duration.as_secs_f64()),
                "{name}: {duration:?}"
            );

            let samples = crate::utils::casting::samples_to_f64(source.count());
            let seconds = samples / f64::from(rate * u32::from(channels));
            assert!((0.8..1.5).contains(&seconds), "{name}: decoded {seconds}s");
        }
    }

    #[test]
    fn seeking_skips_ahead() {
        let path = fixture("tagged.m4b");
        let full = DecoderSource::open(&path).unwrap().count();
        let mut source = DecoderSource::open(&path).unwrap();
        source.try_seek(Duration::from_millis(500)).unwrap();
        let rest = source.count();
        assert!(
            rest < full,
            "seeking should skip samples ({rest} vs {full})"
        );
        assert!(rest > 0, "audio after the seek point should remain");
    }

    #[test]
    fn rejects_non_audio_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fake.mp3");
        std::fs::write(&path, b"not audio").unwrap();
        assert!(DecoderSource::open(&path).is_err());
    }
}
