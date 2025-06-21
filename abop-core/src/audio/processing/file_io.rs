//! File I/O operations for audio processing
//!
//! This module provides functionality for reading, processing, and writing audio files.
//! It handles the conversion between different audio formats and applies processing
//! pipelines to audio data.
//!
//! # Examples
//! ```
//! use abop_core::audio::processing::file_io::{AudioFileProcessor, FileProcessingOptions};
//! use abop_core::audio::processing::pipeline::AudioProcessingPipeline;
//! use abop_core::audio::SampleFormat;
//! use std::path::PathBuf;
//!
//! // Create a processing pipeline
//! let pipeline = AudioProcessingPipeline::default();
//!
//! // Configure processing options
//! let options = FileProcessingOptions {
//!     output_format: SampleFormat::F32,
//!     preserve_metadata: true,
//!     output_directory: Some(PathBuf::from("processed")),
//!     naming_pattern: "{filename}_processed".to_string(),
//! };
//!
//! // Create a processor
//! let processor = AudioFileProcessor::new(pipeline, options);
//! ```

use super::pipeline::AudioProcessingPipeline;
use crate::audio::{AudioBuffer, SampleFormat};
use crate::error::Result;
use std::path::{Path, PathBuf};

/// Options for file processing, such as output format and naming pattern.
///
/// This struct configures how audio files are processed and saved, including
/// the output format, metadata handling, and file naming conventions.
///
/// # Examples
/// ```
/// use abop_core::audio::processing::file_io::FileProcessingOptions;
/// use abop_core::audio::SampleFormat;
/// use std::path::PathBuf;
///
/// let options = FileProcessingOptions {
///     output_format: SampleFormat::F32,
///     preserve_metadata: true,
///     output_directory: Some(PathBuf::from("output")),
///     naming_pattern: "{filename}_normalized".to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct FileProcessingOptions {
    /// Output audio sample format (e.g., F32, S16)
    ///
    /// This determines the precision and format of the processed audio samples.
    /// F32 provides the highest quality but uses more storage space.
    pub output_format: SampleFormat,

    /// Whether to preserve audio metadata (tags, cover art, etc.)
    ///
    /// When true, metadata from the source file is copied to the output file.
    /// When false, the output file will have no metadata.
    pub preserve_metadata: bool,

    /// Optional output directory for processed files
    ///
    /// If None, processed files are saved in the same directory as the source file.
    /// If Some(path), all processed files are saved in the specified directory.
    pub output_directory: Option<PathBuf>,

    /// Naming pattern for output files
    ///
    /// Uses a simple template system where {filename} is replaced with the
    /// original filename (without extension). For example, "book.mp3" with
    /// pattern "{filename}_processed" becomes "book_processed.mp3".
    pub naming_pattern: String,
}

impl Default for FileProcessingOptions {
    fn default() -> Self {
        Self {
            output_format: SampleFormat::F32,
            preserve_metadata: true,
            output_directory: None,
            naming_pattern: "{filename}_processed".to_string(),
        }
    }
}

/// Processor for loading, processing, and saving audio files using a pipeline.
#[derive(Debug, Clone)]
pub struct AudioFileProcessor {
    /// The audio processing pipeline to use
    pub pipeline: AudioProcessingPipeline,
    /// File processing options
    pub options: FileProcessingOptions,
}

impl AudioFileProcessor {
    /// Create a new `AudioFileProcessor` with the given pipeline and options.
    #[must_use]
    pub const fn new(pipeline: AudioProcessingPipeline, options: FileProcessingOptions) -> Self {
        Self { pipeline, options }
    }

    /// Process a single audio file and save to an auto-generated output path.
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Io`] if the input file cannot be read or the output file
    /// cannot be written, or [`AppError::Audio`] if audio processing fails.
    pub fn process_file<P: AsRef<Path>>(&mut self, input_path: P) -> Result<PathBuf> {
        let input_path = input_path.as_ref();
        // Load audio file
        let mut buffer = Self::load_audio_file(input_path)?;
        // Process through pipeline
        self.pipeline.process_buffer(&mut buffer)?;
        // Save processed file
        let output_path = self.generate_output_path(input_path);
        Self::save_audio_file(&buffer, &output_path)?;
        Ok(output_path)
    }

    /// Process a single audio file and write to the specified output path.
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Io`] if the input file cannot be read or the output file
    /// cannot be written, or [`AppError::Audio`] if audio processing fails.
    pub fn process_file_with_output<P: AsRef<Path>, Q: AsRef<Path>>(
        &mut self,
        input_path: P,
        output_path: Q,
    ) -> Result<()> {
        let input_path = input_path.as_ref();
        let output_path = output_path.as_ref();
        // Load audio file
        let mut buffer = Self::load_audio_file(input_path)?;
        // Process through pipeline
        self.pipeline.process_buffer(&mut buffer)?;
        // Save processed file to the explicit output path
        Self::save_audio_file(&buffer, output_path)?;
        Ok(())
    }

    /// Load an audio file into a buffer, converting to f32 samples.
    fn load_audio_file(path: &Path) -> Result<AudioBuffer<f32>> {
        use hound::{SampleFormat as HoundSampleFormat, WavReader}; // Open the WAV file
        let reader = WavReader::open(path).map_err(|e| {
            crate::error::AppError::Io(format!(
                "Failed to open audio file '{}': {}",
                path.display(),
                e
            ))
        })?;

        let spec = reader.spec();
        let mut data = Vec::with_capacity(reader.len() as usize);

        // Handle different sample formats
        match (spec.sample_format, spec.bits_per_sample) {
            (HoundSampleFormat::Int, 8) => {
                // Convert u8 to f32: map [0, 255] to [-1.0, 1.0]
                for sample in reader.into_samples::<i8>() {
                    let sample = sample.map_err(|e| {
                        crate::error::AppError::Audio(format!("Error reading sample: {e}"))
                    })?;
                    data.push(sample as f32 / 128.0);
                }
            }
            (HoundSampleFormat::Int, 16) => {
                // Convert i16 to f32: map [-32768, 32767] to [-1.0, 1.0]
                for sample in reader.into_samples::<i16>() {
                    let sample = sample.map_err(|e| {
                        crate::error::AppError::Audio(format!("Error reading sample: {e}"))
                    })?;
                    data.push(sample as f32 / 32768.0);
                }
            }
            (HoundSampleFormat::Int, 24) => {
                // Convert i24 to f32: map [-2^23, 2^23-1] to [-1.0, 1.0]
                for sample in reader.into_samples::<i32>() {
                    let sample = sample.map_err(|e| {
                        crate::error::AppError::Audio(format!("Error reading sample: {e}"))
                    })?;
                    data.push((sample as f32) / (1u32 << 23) as f32);
                }
            }
            (HoundSampleFormat::Int, 32) => {
                // Convert i32 to f32: map [-2^31, 2^31-1] to [-1.0, 1.0]
                for sample in reader.into_samples::<i32>() {
                    let sample = sample.map_err(|e| {
                        crate::error::AppError::Audio(format!("Error reading sample: {e}"))
                    })?;
                    data.push((sample as f32) / (1u32 << 31) as f32);
                }
            }
            (HoundSampleFormat::Float, 32) => {
                // Directly use f32 samples
                for sample in reader.into_samples::<f32>() {
                    let sample = sample.map_err(|e| {
                        crate::error::AppError::Audio(format!("Error reading sample: {e}"))
                    })?;
                    data.push(sample);
                }
            }
            _ => {
                return Err(crate::error::AppError::Audio(format!(
                    "Unsupported WAV format: {:?} {}-bit (only 8, 16, 24, or 32-bit integer/float supported)",
                    spec.sample_format, spec.bits_per_sample
                )));
            }
        }

        Ok(AudioBuffer {
            data,
            format: SampleFormat::F32,
            sample_rate: spec.sample_rate,
            channels: spec.channels,
        })
    }

    /// Save an audio buffer to a file.
    fn save_audio_file(buffer: &AudioBuffer<f32>, path: &Path) -> Result<()> {
        use hound::{SampleFormat as HoundSampleFormat, WavSpec, WavWriter};

        // Create the WAV file with the appropriate spec
        let spec = WavSpec {
            channels: buffer.channels,
            sample_rate: buffer.sample_rate,
            bits_per_sample: 32,
            sample_format: HoundSampleFormat::Float,
        }; // Create the writer and write all samples
        let mut writer = WavWriter::create(path, spec).map_err(|e| {
            crate::error::AppError::Io(format!(
                "Failed to create output file '{}': {}",
                path.display(),
                e
            ))
        })?; // Write all samples
        for &sample in &buffer.data {
            writer
                .write_sample(sample)
                .map_err(|e| crate::error::AppError::Io(format!("Error writing sample: {e}")))?;
        }

        // Finalize the writer (this is a no-op in newer hound versions but kept for compatibility)
        drop(writer);

        Ok(())
    }

    /// Generate the output path for a processed file based on options and input path.
    fn generate_output_path(&self, input_path: &Path) -> PathBuf {
        let output_dir = self
            .options
            .output_directory
            .as_deref()
            .unwrap_or_else(|| input_path.parent().unwrap_or_else(|| Path::new(".")));
        let input_stem = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("processed");
        // Use the naming pattern from options, replacing {filename} with the stem
        #[allow(clippy::literal_string_with_formatting_args)]
        let filename = self
            .options
            .naming_pattern
            .replace("{filename}", input_stem);
        let extension = input_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("wav");
        let output_filename = format!("{filename}.{extension}");
        // Return directly instead of let binding
        output_dir.join(output_filename)
    }
}
