//! Audiobook data model and operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::error::Result;

/// Common fallback text constants for better performance
mod fallbacks {
    pub const UNKNOWN_TITLE: &str = "Unknown Title";
    pub const UNKNOWN_AUTHOR: &str = "Unknown Author";
    pub const UNKNOWN: &str = "Unknown";
}

/// Represents an audiobook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audiobook {
    /// Unique identifier for the audiobook
    pub id: String,
    /// ID of the library this audiobook belongs to
    pub library_id: String,
    /// Filesystem path to the audiobook file
    pub path: PathBuf,
    /// Title of the audiobook
    pub title: Option<String>,
    /// Author of the audiobook
    pub author: Option<String>,
    /// Narrator of the audiobook
    pub narrator: Option<String>,
    /// Description or synopsis
    pub description: Option<String>,
    /// Duration in seconds
    pub duration_seconds: Option<u64>,
    /// File size in bytes
    pub size_bytes: Option<u64>,
    /// Cover art image data (JPEG/PNG)
    pub cover_art: Option<Vec<u8>>,
    /// When the audiobook was added to the library
    pub created_at: DateTime<Utc>,
    /// When the audiobook was last updated
    pub updated_at: DateTime<Utc>,
    /// Whether this audiobook is selected in the UI
    pub selected: bool,
}

impl Audiobook {
    /// Creates a new audiobook with default values
    #[must_use]
    pub fn new<P: AsRef<Path>>(library_id: &str, path: P) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            library_id: library_id.to_string(),
            path: path.as_ref().to_path_buf(),
            title: None,
            author: None,
            narrator: None,
            description: None,
            duration_seconds: None,
            size_bytes: None,
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        }
    }

    /// Creates a new audiobook from audio metadata
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file path is invalid
    /// - The metadata is invalid
    pub fn from_metadata<P: AsRef<Path>>(metadata: crate::audio::AudioMetadata, path: P) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let mut audiobook = Self::new("", &path_buf);
        
        audiobook.title = metadata.title;
        audiobook.author = metadata.artist;
        audiobook.narrator = metadata.narrator;
        audiobook.description = metadata.description;
        audiobook.duration_seconds = metadata.duration_seconds.map(|d| d.round() as u64);
        audiobook.cover_art = metadata.cover_art;
        
        if let Ok(metadata) = std::fs::metadata(&path_buf) {
            audiobook.size_bytes = Some(metadata.len());
        }
        
        Ok(audiobook)
    }

    /// Gets the file name of the audiobook
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name()?.to_str()
    }

    /// Gets the file extension of the audiobook
    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        self.path.extension()?.to_str()
    }
    /// Gets the display title, falling back to filename if no title is set
    #[must_use]
    pub fn display_title(&self) -> String {
        self.title
            .clone()
            .or_else(|| self.file_name().map(String::from))
            .unwrap_or_else(|| fallbacks::UNKNOWN_TITLE.to_string())
    }
    /// Gets the display author, with fallback text
    #[must_use]
    pub fn display_author(&self) -> &str {
        self.author.as_deref().unwrap_or(fallbacks::UNKNOWN_AUTHOR)
    }

    /// Updates the last modified timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
    /// Gets the duration formatted as HH:MM:SS
    #[must_use]
    pub fn formatted_duration(&self) -> String {
        self.duration_seconds.map_or_else(
            || fallbacks::UNKNOWN.to_string(),
            |seconds| {
                crate::utils::time::format_seconds(
                    seconds,
                    crate::utils::time::TimeFormat::AlwaysHours,
                )
            },
        )
    }
    /// Gets the file size formatted as human-readable string
    pub fn formatted_size(&self) -> String {
        self.size_bytes.map_or_else(
            || fallbacks::UNKNOWN.to_string(),
            crate::utils::size::format_bytes,
        )
    }

    /// Sets the library ID for this audiobook
    pub fn with_library_id(mut self, library_id: &str) -> Self {
        self.library_id = library_id.to_string();
        self
    }

    /// Sets the title for this audiobook
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the author for this audiobook
    pub fn with_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    /// Sets the narrator for this audiobook
    pub fn with_narrator(mut self, narrator: &str) -> Self {
        self.narrator = Some(narrator.to_string());
        self
    }

    /// Sets the description for this audiobook
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the duration in seconds for this audiobook
    pub fn with_duration(mut self, duration_seconds: u64) -> Self {
        self.duration_seconds = Some(duration_seconds);
        self
    }

    /// Sets the file size in bytes for this audiobook
    pub fn with_size(mut self, size_bytes: u64) -> Self {
        self.size_bytes = Some(size_bytes);
        self
    }

    /// Sets the cover art data for this audiobook
    pub fn with_cover_art(mut self, cover_art: Vec<u8>) -> Self {
        self.cover_art = Some(cover_art);
        self
    }

    /// Marks this audiobook as selected
    pub fn select(&mut self) {
        self.selected = true;
    }

    /// Marks this audiobook as not selected
    pub fn deselect(&mut self) {
        self.selected = false;
    }

    /// Returns whether this audiobook is currently selected
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Updates the metadata of this audiobook with new information
    pub fn update_metadata(&mut self, metadata: crate::audio::AudioMetadata) {
        self.title = metadata.title;
        self.author = metadata.artist;
        self.narrator = metadata.narrator;
        self.description = metadata.description;
        self.duration_seconds = metadata.duration_seconds.map(|d| d.round() as u64);
        self.cover_art = metadata.cover_art;
        self.updated_at = Utc::now();
    }
}

impl Default for Audiobook {
    fn default() -> Self {
        Self::new("", "")
    }
}

#[cfg(test)]
mod tests {
    use super::fallbacks::{UNKNOWN, UNKNOWN_AUTHOR};
    use super::*;
    use crate::test_constants::*;
    use std::path::Path;

    #[test]
    fn test_audiobook_creation() {
        let audiobook = Audiobook::new(library::TEST_ID, audiobook::TEST_PATH);
        assert_eq!(audiobook.library_id, library::TEST_ID);
        assert_eq!(audiobook.path, Path::new(audiobook::TEST_PATH));
        assert!(!audiobook.id.is_empty());
        assert!(audiobook.title.is_none());
        assert!(audiobook.author.is_none());
    }

    #[test]
    fn test_audiobook_file_name() {
        let mut audiobook = Audiobook::new(library::TEST_ID, audiobook::TEST_PATH);
        assert_eq!(audiobook.file_name(), Some(audiobook::TEST_FILENAME));
        assert_eq!(audiobook.extension(), Some(audiobook::TEST_EXTENSION));

        audiobook.path = Path::new(audiobook::TEST_PATH_NO_EXT).to_path_buf();
        assert_eq!(audiobook.file_name(), Some(audiobook::TEST_FILENAME_NO_EXT));
        assert_eq!(audiobook.extension(), Some(audiobook::TEST_EXTENSION_ALT));
    }

    #[test]
    fn test_display_methods() {
        let mut audiobook = Audiobook::new(library::TEST_ID, audiobook::TEST_PATH);

        // Test display title fallback
        assert_eq!(audiobook.display_title(), audiobook::TEST_FILENAME);
        audiobook.title = Some(audiobook::TEST_TITLE_BOOK.to_string());
        assert_eq!(audiobook.display_title(), audiobook::TEST_TITLE_BOOK);

        // Test display author fallback
        assert_eq!(audiobook.display_author(), UNKNOWN_AUTHOR);
        audiobook.author = Some(audiobook::TEST_AUTHOR_JANE.to_string());
        assert_eq!(audiobook.display_author(), audiobook::TEST_AUTHOR_JANE);
    }

    #[test]
    fn test_duration_formatting() {
        let mut audiobook = Audiobook::new(library::TEST_ID, audiobook::TEST_PATH);

        assert_eq!(audiobook.formatted_duration(), UNKNOWN);

        audiobook.duration_seconds = Some(3661); // 1:01:01
        assert_eq!(audiobook.formatted_duration(), "01:01:01");

        audiobook.duration_seconds = Some(150); // 2:30
        assert_eq!(audiobook.formatted_duration(), "00:02:30");
    }

    #[test]
    fn test_size_formatting() {
        let mut audiobook = Audiobook::new(library::TEST_ID, audiobook::TEST_PATH);

        assert_eq!(audiobook.formatted_size(), UNKNOWN);

        audiobook.size_bytes = Some(1024);
        assert_eq!(audiobook.formatted_size(), "1 KB");

        audiobook.size_bytes = Some(1_048_576);
        assert_eq!(audiobook.formatted_size(), "1 MB");

        audiobook.size_bytes = Some(500);
        assert_eq!(audiobook.formatted_size(), "500 B");
    }

    #[test]
    fn test_touch() {
        let mut audiobook = Audiobook::new(library::TEST_ID, audiobook::TEST_PATH);
        let original_time = audiobook.updated_at;

        // Small delay to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(1));
        audiobook.touch();

        assert!(audiobook.updated_at > original_time);
    }
}
