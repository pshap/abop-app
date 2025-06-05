//! Audio metadata extraction

use std::path::Path;

use symphonia::core::probe::Hint;

use super::{AudioStream, SampleFormat};
use crate::error::{AppError, Result};

/// Represents the metadata for an audio file
#[derive(Debug, Clone, Default)]
pub struct AudioMetadata {
    /// The title of the audio
    pub title: Option<String>,
    /// The artist or author
    pub artist: Option<String>,
    /// The album name
    pub album: Option<String>,
    /// The track number
    pub track: Option<u32>,
    /// The total duration in seconds
    pub duration_seconds: Option<f64>,
    /// The audio stream information
    pub stream: Option<AudioStream>,
    /// The cover art image data, if available
    pub cover_art: Option<Vec<u8>>,
    /// The narrator of the audiobook, if available
    pub narrator: Option<String>,
    /// The genre of the audio
    pub genre: Option<String>,
    /// The year of release
    pub year: Option<i32>,
    /// The publisher
    pub publisher: Option<String>,
    /// The language of the audio
    pub language: Option<String>,
    /// The description or synopsis
    pub description: Option<String>,
}

impl AudioMetadata {
    /// Creates a new [`AudioMetadata`] with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Extracts metadata from a file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened, the format is not supported,
    /// or there's an error reading the metadata.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let mut meta = Self::new();

        // Try to read metadata using symphonia
        let file = std::fs::File::open(path).map_err(AppError::Io)?;
        let mss = symphonia::core::io::MediaSourceStream::new(
            Box::new(file),
            symphonia::core::io::MediaSourceStreamOptions::default(),
        );

        // Create a hint to help the format registry guess the format
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        // Set up format options
        let format_opts = symphonia::core::formats::FormatOptions::default();
        let metadata_opts = symphonia::core::meta::MetadataOptions::default();

        // Probe the media format
        let mut probed = match symphonia::default::get_probe().format(
            &hint,
            mss,
            &format_opts,
            &metadata_opts,
        ) {
            Ok(probed) => probed,
            Err(e) => {
                return Err(AppError::Audio(format!(
                    "Failed to probe audio format: {e}"
                )));
            }
        };

        // Process the default track if available
        if let Some(track) = probed.format.default_track() {
            meta = process_track_metadata(track, meta);
        }

        // Extract metadata tags from the format's metadata
        let metadata = probed.format.metadata();
        meta = process_metadata_tags(&metadata, meta);

        Ok(meta)
    }

    /// Creates a new [`AudioMetadata`] with the given title
    #[must_use]
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Creates a new [`AudioMetadata`] with the given artist
    #[must_use]
    pub fn with_artist<S: Into<String>>(mut self, artist: S) -> Self {
        self.artist = Some(artist.into());
        self
    }

    /// Creates a new [`AudioMetadata`] with the given album
    #[must_use]
    pub fn with_album<S: Into<String>>(mut self, album: S) -> Self {
        self.album = Some(album.into());
        self
    }

    /// Creates a new [`AudioMetadata`] with the given track number
    #[must_use]
    pub const fn with_track(mut self, track: u32) -> Self {
        self.track = Some(track);
        self
    }

    /// Creates a new [`AudioMetadata`] with the given duration in seconds
    #[must_use]
    pub const fn with_duration_seconds(mut self, duration: f64) -> Self {
        self.duration_seconds = Some(duration);
        self
    }

    /// Creates a new [`AudioMetadata`] with the given cover art
    #[must_use]
    pub fn with_cover_art(mut self, cover_art: Vec<u8>) -> Self {
        self.cover_art = Some(cover_art);
        self
    }

    /// Creates a new [`AudioMetadata`] with the given narrator
    #[must_use]
    pub fn with_narrator<S: Into<String>>(mut self, narrator: S) -> Self {
        self.narrator = Some(narrator.into());
        self
    }

    /// Creates a new [`AudioMetadata`] with the given genre
    #[must_use]
    pub fn with_genre<S: Into<String>>(mut self, genre: S) -> Self {
        self.genre = Some(genre.into());
        self
    }

    /// Creates a new [`AudioMetadata`] with the given year
    #[must_use]
    pub const fn with_year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    /// Creates a new [`AudioMetadata`] with the given publisher
    #[must_use]
    pub fn with_publisher<S: Into<String>>(mut self, publisher: S) -> Self {
        self.publisher = Some(publisher.into());
        self
    }

    /// Creates a new [`AudioMetadata`] with the given language
    #[must_use]
    pub fn with_language<S: Into<String>>(mut self, language: S) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Creates a new [`AudioMetadata`] with the given description
    #[must_use]
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Processes track metadata and updates the [`AudioMetadata`]
///
/// Extracts audio stream information including sample rate, channels, format,
/// and duration from the track's codec parameters.
fn process_track_metadata(
    track: &symphonia::core::formats::Track,
    mut meta: AudioMetadata,
) -> AudioMetadata {
    // Get audio stream information
    if let Some(sample_rate) = track.codec_params.sample_rate {
        // Get the sample format from the codec parameters
        let sample_format = match track.codec_params.sample_format {
            Some(symphonia::core::sample::SampleFormat::U8) => SampleFormat::U8,
            Some(symphonia::core::sample::SampleFormat::U16) => SampleFormat::U16,
            Some(symphonia::core::sample::SampleFormat::U24) => SampleFormat::U24,
            Some(symphonia::core::sample::SampleFormat::U32) => SampleFormat::U32,
            Some(symphonia::core::sample::SampleFormat::S16) => SampleFormat::S16,
            Some(symphonia::core::sample::SampleFormat::S24) => SampleFormat::S24,
            Some(symphonia::core::sample::SampleFormat::S32) => SampleFormat::S32,
            Some(symphonia::core::sample::SampleFormat::F64) => SampleFormat::F64,
            // Default to F32 for unknown/unsupported formats
            _ => SampleFormat::F32,
        };

        // Get number of channels (default to 2 if not specified)
        let channels = track
            .codec_params
            .channels
            .map_or(2, |c| u16::try_from(c.count()).unwrap_or(2));

        // Calculate duration if possible
        let duration = track.codec_params.n_frames.map(|frames| {
            // Use checked arithmetic to prevent overflow
            if frames == 0 || sample_rate == 0 {
                0.0
            } else {
                // For very large frame counts, use f64 throughout
                // clippy: allow precision loss for u64->f64 conversion, as this is for display/approximation only
                #[allow(clippy::cast_precision_loss)]
                let frames_f64 = frames as f64;
                let sample_rate_f64 = f64::from(sample_rate);
                frames_f64 / sample_rate_f64
            }
        });

        meta.stream = Some(AudioStream {
            sample_rate,
            channels,
            sample_format,
            duration,
        });

        meta.duration_seconds = duration;
    }
    meta
}

/// Processes metadata tags and updates the [`AudioMetadata`]
///
/// Extracts standard metadata tags such as title, artist, album, track number,
/// genre, date, and cover art from the format's metadata reader.
fn process_metadata_tags(
    metadata: &symphonia::core::meta::Metadata,
    mut meta: AudioMetadata,
) -> AudioMetadata {
    if let Some(reader) = metadata.current() {
        for tag in reader.tags() {
            if let Some(std_key) = tag.std_key {
                match std_key {
                    symphonia::core::meta::StandardTagKey::TrackTitle => {
                        meta.title = Some(tag.value.to_string());
                    }
                    symphonia::core::meta::StandardTagKey::Artist => {
                        meta.artist = Some(tag.value.to_string());
                    }
                    symphonia::core::meta::StandardTagKey::Album => {
                        meta.album = Some(tag.value.to_string());
                    }
                    symphonia::core::meta::StandardTagKey::TrackNumber => {
                        if let Ok(track) = tag.value.to_string().parse::<u32>() {
                            meta.track = Some(track);
                        }
                    }
                    symphonia::core::meta::StandardTagKey::Genre => {
                        meta.genre = Some(tag.value.to_string());
                    }
                    symphonia::core::meta::StandardTagKey::Date => {
                        if let Ok(year) = tag.value.to_string().parse::<i32>() {
                            meta.year = Some(year);
                        }
                    }
                    // Ignore composer and comment fields for now
                    symphonia::core::meta::StandardTagKey::Composer
                    | symphonia::core::meta::StandardTagKey::Comment => {}
                    _ => {}
                }
            }
        }
    }

    // Extract cover art if available (first visual in the metadata)
    if let Some(reader) = metadata.current()
        && let Some(visual) = reader.visuals().iter().next()
    {
        meta.cover_art = Some(visual.data.to_vec());
    }

    meta
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::metadata::*;

    #[test]
    fn test_metadata_builder() {
        let meta = AudioMetadata::new()
            .with_title(TEST_TITLE)
            .with_artist(TEST_ARTIST)
            .with_album(TEST_ALBUM)
            .with_track(1)
            .with_duration_seconds(180.0)
            .with_narrator(TEST_NARRATOR)
            .with_genre(TEST_GENRE)
            .with_year(2023)
            .with_publisher(TEST_PUBLISHER)
            .with_language(TEST_LANGUAGE)
            .with_description(TEST_DESCRIPTION);

        assert_eq!(meta.title, Some(TEST_TITLE.to_string()));
        assert_eq!(meta.artist, Some(TEST_ARTIST.to_string()));
        assert_eq!(meta.album, Some(TEST_ALBUM.to_string()));
        assert_eq!(meta.track, Some(1));
        assert_eq!(meta.duration_seconds, Some(180.0));
        assert_eq!(meta.narrator, Some(TEST_NARRATOR.to_string()));
        assert_eq!(meta.genre, Some(TEST_GENRE.to_string()));
        assert_eq!(meta.year, Some(2023));
        assert_eq!(meta.publisher, Some(TEST_PUBLISHER.to_string()));
        assert_eq!(meta.language, Some(TEST_LANGUAGE.to_string()));
        assert_eq!(meta.description, Some(TEST_DESCRIPTION.to_string()));
    }

    #[test]
    fn test_metadata_from_nonexistent_file() {
        let result = AudioMetadata::from_file("/nonexistent/file.mp3");
        assert!(result.is_err()); // Should fail with Io error
    }

    #[test]
    fn test_metadata_with_cover_art() {
        let cover_art = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG magic number
        let meta = AudioMetadata::new().with_cover_art(cover_art.clone());

        assert_eq!(meta.cover_art, Some(cover_art));
    }
}
