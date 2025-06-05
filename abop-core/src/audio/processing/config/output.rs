use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::audio::processing::error::Result;
use crate::audio::processing::traits::Validatable;

/// Common filename suffixes for different output configurations
mod suffixes {
    pub const MASTERED: &str = "_mastered";
    pub const ARCHIVE: &str = "_archive";
    pub const WEB: &str = "_web";
    pub const PODCAST: &str = "_podcast";
    pub const PROCESSED: &str = "_processed";
}

/// Configuration for output formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Output audio format (optional)
    pub format: Option<AudioFormat>,
    /// Bit depth for output audio
    pub bit_depth: BitDepth,
    /// Output directory for processed files (optional)
    pub output_dir: Option<PathBuf>,
    /// Whether to overwrite existing files
    pub overwrite: bool,
    /// Suffix to append to output filenames
    pub filename_suffix: String,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: Some(AudioFormat::Wav),
            bit_depth: BitDepth::Sixteen,
            output_dir: None,
            overwrite: false,
            filename_suffix: suffixes::PROCESSED.to_string(),
        }
    }
}

impl Validatable for OutputConfig {
    fn validate(&self) -> Result<()> {
        // Use the validation utilities for consistent error messages
        use super::validation;

        // Filename suffix cannot be empty
        validation::non_empty_string(&self.filename_suffix, "Filename suffix")?;

        // Output directory must exist if specified
        if let Some(ref output_dir) = self.output_dir {
            validation::directory_exists(output_dir, "Output directory")?;
        }

        Ok(())
    }
}

/// Supported audio formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AudioFormat {
    /// WAV format
    Wav,
    /// FLAC format
    Flac,
    /// MP3 format
    Mp3,
    /// OGG format
    Ogg,
}

impl AudioFormat {
    /// Returns the standard file extension for the audio format.
    ///
    /// # Returns
    /// A string slice representing the file extension (e.g., "wav", "mp3").
    ///
    /// # Examples
    /// ```
    /// use abop_core::audio::processing::config::AudioFormat;
    ///
    /// let ext = AudioFormat::Mp3.extension();
    /// assert_eq!(ext, "mp3");
    /// ```
    #[must_use]
    pub const fn extension(&self) -> &'static str {
        match self {
            Self::Wav => "wav",
            Self::Flac => "flac",
            Self::Mp3 => "mp3",
            Self::Ogg => "ogg",
        }
    }
}

/// Supported bit depths
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BitDepth {
    /// 16-bit output
    Sixteen,
    /// 24-bit output
    TwentyFour,
    /// 32-bit output
    ThirtyTwo,
}

/// Builder for `OutputConfig`
#[derive(Debug, Default)]
pub struct OutputConfigBuilder {
    format: Option<AudioFormat>,
    bit_depth: Option<BitDepth>,
    output_dir: Option<PathBuf>,
    overwrite: Option<bool>,
    filename_suffix: Option<String>,
}

impl OutputConfigBuilder {
    /// Create a new builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the output format
    #[must_use]
    pub const fn with_format(mut self, format: AudioFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Set the bit depth
    #[must_use]
    pub const fn with_bit_depth(mut self, bit_depth: BitDepth) -> Self {
        self.bit_depth = Some(bit_depth);
        self
    }

    /// Set the output directory
    #[must_use]
    pub fn with_output_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.output_dir = Some(dir.into());
        self
    }

    /// Enable or disable overwriting existing files
    #[must_use]
    pub const fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    /// Set the filename suffix
    #[must_use]
    pub fn with_filename_suffix<S: Into<String>>(mut self, suffix: S) -> Self {
        self.filename_suffix = Some(suffix.into());
        self
    }

    /// Build the `OutputConfig`
    #[must_use]
    pub fn build(self) -> OutputConfig {
        OutputConfig {
            format: self.format,
            bit_depth: self.bit_depth.unwrap_or(BitDepth::Sixteen),
            output_dir: self.output_dir,
            overwrite: self.overwrite.unwrap_or(false),
            filename_suffix: self
                .filename_suffix
                .unwrap_or_else(|| suffixes::PROCESSED.to_string()),
        }
    }

    /// Build and validate the `OutputConfig`
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn build_validated(self) -> Result<OutputConfig> {
        let config = self.build();
        config.validate()?;
        Ok(config)
    }

    /// Configure for podcast distribution (MP3 format)
    #[must_use]
    pub fn for_podcast(mut self) -> Self {
        self.format = Some(AudioFormat::Mp3);
        self.bit_depth = Some(BitDepth::Sixteen);
        self.filename_suffix = Some(suffixes::PODCAST.to_string());
        self.overwrite = Some(false);
        self
    }

    /// Configure for high-quality music (FLAC format)
    #[must_use]
    pub fn for_music_distribution(mut self) -> Self {
        self.format = Some(AudioFormat::Flac);
        self.bit_depth = Some(BitDepth::TwentyFour);
        self.filename_suffix = Some(suffixes::MASTERED.to_string());
        self.overwrite = Some(false);
        self
    }

    /// Configure for archival purposes (WAV format, 24-bit)
    #[must_use]
    pub fn for_archival(mut self) -> Self {
        self.format = Some(AudioFormat::Wav);
        self.bit_depth = Some(BitDepth::TwentyFour);
        self.filename_suffix = Some(suffixes::ARCHIVE.to_string());
        self.overwrite = Some(false);
        self
    }

    /// Configure for web streaming (OGG format)
    #[must_use]
    pub fn for_web_streaming(mut self) -> Self {
        self.format = Some(AudioFormat::Ogg);
        self.bit_depth = Some(BitDepth::Sixteen);
        self.filename_suffix = Some(suffixes::WEB.to_string());
        self.overwrite = Some(false);
        self
    }
}

impl OutputConfig {
    /// Create a new builder for `OutputConfig`
    #[must_use]
    pub fn builder() -> OutputConfigBuilder {
        OutputConfigBuilder::new()
    }
}
