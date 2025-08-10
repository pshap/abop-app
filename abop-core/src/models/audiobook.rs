//! Audiobook data model and operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Common fallback text constants for better performance
pub mod fallbacks {
    /// Default title to use when an audiobook has no title metadata.
    pub const UNKNOWN_TITLE: &str = "Unknown Title";
    /// Default author to use when an audiobook has no author/artist metadata.
    pub const UNKNOWN_AUTHOR: &str = "Unknown Author";
    /// Generic unknown placeholder used for miscellaneous fields (e.g., duration/size).
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
        let now = Utc::now();
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
            created_at: now,
            updated_at: now,
            selected: false,
        }
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
        self.size_bytes
            .map_or_else(|| fallbacks::UNKNOWN.to_string(), |b| {
                crate::utils::casting::format_file_size_exact(b)
            })
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
