//! Streaming conversions that work on files of any length
//!
//! Unlike [`super::file_io`], which loads a whole file into memory (WAV only),
//! these functions decode any format Symphonia supports packet by packet, so
//! memory use stays small even for multi-hour audiobooks.

use std::fs;
use std::path::{Path, PathBuf};

use hound::{SampleFormat as HoundSampleFormat, WavSpec, WavWriter};

use crate::audio::decoder::AudioDecoder;
use crate::error::{AppError, Result};

/// Summary of a finished conversion
#[derive(Debug, Clone, PartialEq)]
pub struct ConversionSummary {
    /// The file that was written
    pub output: PathBuf,
    /// Number of mono samples written
    pub frames_written: u64,
    /// Sample rate of the output
    pub sample_rate: u32,
}

/// Estimate the size in bytes of a 16-bit mono WAV of the given duration
#[must_use]
pub fn estimate_mono_wav_bytes(duration_seconds: u64, sample_rate: u32) -> u64 {
    const WAV_HEADER_BYTES: u64 = 44;
    duration_seconds
        .saturating_mul(u64::from(sample_rate))
        .saturating_mul(2)
        .saturating_add(WAV_HEADER_BYTES)
}

/// Decode `input` and write a 16-bit mono WAV to `output`
///
/// Channels are mixed down by averaging. The output is written to a temporary
/// `.part` file and renamed when complete, so a failed or interrupted
/// conversion never leaves a truncated file at `output`. `progress` is called
/// with values from 0.0 to 1.0 when the input duration is known.
///
/// # Errors
///
/// Returns an error if `output` already exists, the input cannot be decoded,
/// or the output cannot be written.
pub fn convert_to_mono_wav(
    input: &Path,
    output: &Path,
    mut progress: impl FnMut(f32),
) -> Result<ConversionSummary> {
    if output.exists() {
        return Err(AppError::Io(format!(
            "Output file already exists: {}",
            output.display()
        )));
    }

    let mut decoder = AudioDecoder::open(input)?;
    let total_seconds = decoder.duration().filter(|d| *d > 0.0);

    let mut part_name = output.as_os_str().to_owned();
    part_name.push(".part");
    let part_path = PathBuf::from(part_name);

    let result = write_mono(&mut decoder, &part_path, total_seconds, &mut progress);
    match result {
        Ok((frames_written, sample_rate)) => {
            fs::rename(&part_path, output)?;
            progress(1.0);
            Ok(ConversionSummary {
                output: output.to_path_buf(),
                frames_written,
                sample_rate,
            })
        }
        Err(error) => {
            let _ = fs::remove_file(&part_path);
            Err(error)
        }
    }
}

/// Decode everything from `decoder` into a mono WAV at `path`
fn write_mono(
    decoder: &mut AudioDecoder,
    path: &Path,
    total_seconds: Option<f64>,
    progress: &mut impl FnMut(f32),
) -> Result<(u64, u32)> {
    let wav_error = |e: hound::Error| AppError::WavFile(e.to_string());
    let mut writer: Option<WavWriter<std::io::BufWriter<fs::File>>> = None;
    let mut sample_rate = 0;
    let mut frames_written: u64 = 0;
    let mut last_reported = 0.0_f32;

    while let Some(buffer) = decoder.next_packet()? {
        let channels = usize::from(buffer.channels.max(1));
        if writer.is_none() {
            sample_rate = buffer.sample_rate;
            let spec = WavSpec {
                channels: 1,
                sample_rate,
                bits_per_sample: 16,
                sample_format: HoundSampleFormat::Int,
            };
            writer = Some(WavWriter::create(path, spec).map_err(wav_error)?);
        }
        let Some(out) = writer.as_mut() else {
            unreachable!("writer is created above");
        };

        for frame in buffer.data.chunks_exact(channels) {
            out.write_sample(mix_to_i16(frame)).map_err(wav_error)?;
        }
        frames_written += (buffer.data.len() / channels) as u64;

        if let Some(total) = total_seconds {
            #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
            let fraction =
                ((frames_written as f64 / f64::from(sample_rate)) / total).min(1.0) as f32;
            // Report in 1% steps to keep callbacks cheap
            if fraction - last_reported >= 0.01 {
                last_reported = fraction;
                progress(fraction);
            }
        }
    }

    let writer = writer.ok_or_else(|| AppError::Audio("Input contains no audio".to_string()))?;
    writer.finalize().map_err(wav_error)?;
    Ok((frames_written, sample_rate))
}

/// Average one interleaved frame and convert it to a 16-bit sample
fn mix_to_i16(frame: &[f32]) -> i16 {
    #[allow(clippy::cast_precision_loss)]
    let mean = frame.iter().sum::<f32>() / frame.len() as f32;
    #[allow(clippy::cast_possible_truncation)]
    let sample = (mean.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
    sample
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/tagged.mp3");

    #[test]
    fn mixes_channels_by_averaging() {
        assert_eq!(mix_to_i16(&[0.0, 0.0]), 0);
        assert_eq!(mix_to_i16(&[1.0, -1.0]), 0);
        assert_eq!(mix_to_i16(&[1.0, 1.0]), i16::MAX);
        assert_eq!(
            mix_to_i16(&[2.0]),
            i16::MAX,
            "out-of-range input is clamped"
        );
        assert_eq!(mix_to_i16(&[-0.5, -0.5]), -16384);
    }

    #[test]
    fn estimates_wav_size() {
        assert_eq!(estimate_mono_wav_bytes(0, 44_100), 44);
        assert_eq!(estimate_mono_wav_bytes(10, 8_000), 160_044);
    }

    #[test]
    fn converts_mp3_to_mono_wav_with_progress() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("out.wav");
        let mut last_progress = 0.0;

        let summary =
            convert_to_mono_wav(Path::new(FIXTURE), &output, |p| last_progress = p).unwrap();

        assert_eq!(summary.output, output);
        assert!((last_progress - 1.0).abs() < f32::EPSILON);
        let reader = hound::WavReader::open(&output).unwrap();
        let spec = reader.spec();
        assert_eq!(spec.channels, 1);
        assert_eq!(spec.sample_rate, summary.sample_rate);
        // The fixture is about one second long
        let seconds = f64::from(reader.duration()) / f64::from(spec.sample_rate);
        assert!(
            (0.8..1.5).contains(&seconds),
            "unexpected duration {seconds}"
        );
        assert!(!dir.path().join("out.wav.part").exists());
    }

    #[test]
    fn converts_stereo_m4b_to_mono_wav() {
        let input = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/tagged.m4b");
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("book.wav");
        let summary = convert_to_mono_wav(Path::new(input), &output, |_| {}).unwrap();
        assert_eq!(summary.sample_rate, 22_050);
        let reader = hound::WavReader::open(&output).unwrap();
        assert_eq!(reader.spec().channels, 1);
        assert!(
            reader.duration() > 15_000,
            "expected about one second of audio"
        );
    }

    #[test]
    fn refuses_to_overwrite_and_cleans_up_on_failure() {
        let dir = tempfile::tempdir().unwrap();
        let existing = dir.path().join("exists.wav");
        fs::write(&existing, b"keep me").unwrap();
        assert!(convert_to_mono_wav(Path::new(FIXTURE), &existing, |_| {}).is_err());
        assert_eq!(fs::read(&existing).unwrap(), b"keep me");

        let bad_input = dir.path().join("not-audio.mp3");
        fs::write(&bad_input, b"definitely not audio").unwrap();
        let output = dir.path().join("bad.wav");
        assert!(convert_to_mono_wav(&bad_input, &output, |_| {}).is_err());
        assert!(!output.exists());
        assert!(!dir.path().join("bad.wav.part").exists());
    }
}
