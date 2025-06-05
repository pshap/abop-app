//! Audio file scanner module for ABOP
//!
//! This module provides functionality to scan directories for audio files,
//! extract metadata, and update the database with the found files.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use tracing::{Level, debug, error, info, instrument, span, warn};
use walkdir::WalkDir;

use crate::{
    audio::AudioMetadata,
    db::Database,
    error::Result,
    models::{Audiobook, Library},
};

/// Supported audio file extensions for scanning
pub const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["mp3", "m4a", "m4b", "flac", "ogg", "wav", "aac"];

/// Scans a library directory for audio files and updates the database
pub struct LibraryScanner {
    /// The database connection
    db: Database,
    /// The library being scanned
    library: Library,
    /// File extensions to include in the scan (using static strings for efficiency)
    extensions: &'static [&'static str],
}

impl LibraryScanner {
    /// Creates a new `LibraryScanner` for the given library
    #[must_use]
    pub const fn new(db: Database, library: Library) -> Self {
        Self {
            db,
            library,
            extensions: SUPPORTED_AUDIO_EXTENSIONS,
        }
    }

    /// Extracts audiobook metadata from a file path (no DB access, safe for parallel)
    ///
    /// # Errors
    /// Returns an error if metadata cannot be read from the file
    pub fn extract_audiobook_metadata(library_id: &str, path: &Path) -> Result<Audiobook> {
        let metadata = AudioMetadata::from_file(path).map_err(|e| {
            log::warn!("Error reading metadata for {}: {}", path.display(), e);
            e
        })?;

        let mut audiobook = Audiobook::new(library_id, path);
        audiobook.title = Some(metadata.title.unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown Title")
                .to_string()
        }));
        audiobook.author = metadata.artist;
        audiobook.narrator = metadata.narrator;
        audiobook.duration_seconds = metadata.duration_seconds.map(|d| {
            if d.is_nan() || d < 0.0 {
                0
            } else {
                // Safely convert f64 to u64 with explicit bounds checking
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let duration = d.round().clamp(0.0, f64::from(u32::MAX)) as u64;
                duration
            }
        });
        if let Some(cover_art) = metadata.cover_art {
            audiobook.cover_art = Some(cover_art);
        }

        Ok(audiobook)
    }

    /// Scans the library directory for audio files and updates the database
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Audio file discovery fails
    /// - Metadata extraction fails
    /// - Database operations fail
    /// - File system access is denied
    #[instrument(name = "scan_library", skip(self), fields(library_name = %self.library.name, library_path = %self.library.path.display()))]
    pub fn scan(&self) -> Result<ScanResult> {
        info!("Starting scan of library: {}", self.library.name);

        // Collect all audio files in the library directory
        let audio_files = self.find_audio_files();
        info!("Found {} audio files", audio_files.len());

        let library_id = self.library.id.clone();
        // Extract metadata in parallel, no DB access
        let _span = span!(Level::DEBUG, "extract_metadata_parallel").entered();
        let audiobooks: Vec<_> = audio_files
            .par_iter()
            .filter_map(
                |path| match Self::extract_audiobook_metadata(&library_id, path) {
                    Ok(book) => {
                        debug!("Successfully extracted metadata for: {}", path.display());
                        Some(book)
                    }
                    Err(e) => {
                        warn!("Error extracting metadata from {}: {}", path.display(), e);
                        None
                    }
                },
            )
            .collect();
        drop(_span);

        // Write to DB serially
        let _db_span = span!(Level::DEBUG, "database_writes").entered();
        let mut result = ScanResult::new();
        for audiobook in audiobooks {
            match self.db.add_audiobook(&audiobook) {
                Ok(()) => {
                    result.processed_count += 1;
                    debug!("Successfully saved audiobook: {}", audiobook.path.display());
                    result.audiobooks.push(audiobook);
                }
                Err(e) => {
                    error!("Error saving {}: {}", audiobook.path.display(), e);
                    result.error_count += 1;
                }
            }
        }

        log::info!(
            "Scan completed. Processed: {}, Errors: {}",
            result.processed_count,
            result.error_count
        );

        Ok(result)
    }

    /// Finds all audio files in the library directory
    pub fn find_audio_files(&self) -> Vec<PathBuf> {
        Self::find_audio_files_in_path(&self.library.path, self.extensions)
    }

    /// Finds all audio files in a given directory path
    pub fn find_audio_files_in_path(path: &Path, extensions: &[&str]) -> Vec<PathBuf> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry),
                Err(e) => {
                    log::warn!("Error reading directory entry: {e}");
                    None
                }
            })
            .filter(|e| e.file_type().is_file())
            .filter_map(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(str::to_lowercase)
                    .and_then(|ext| {
                        if extensions.contains(&&*ext) {
                            Some(entry.into_path())
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}

/// Represents the result of a library scan
pub struct ScanResult {
    /// Number of files successfully processed
    pub processed_count: usize,
    /// Number of files that had errors
    pub error_count: usize,
    /// List of processed audiobooks
    pub audiobooks: Vec<Audiobook>,
}

impl ScanResult {
    /// Creates a new empty scan result
    #[must_use]
    pub const fn new() -> Self {
        Self {
            processed_count: 0,
            error_count: 0,
            audiobooks: Vec::new(),
        }
    }
}

impl Default for ScanResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_find_audio_files() {
        // Create a temporary directory with some test files
        let temp_dir = tempdir().unwrap();
        let test_files = [
            file::TEST_MP3,
            file::TEST_M4B,
            file::TEST_TXT,
            file::TEST_FLAC,
        ];

        // Create the files
        for file in &test_files {
            let path = temp_dir.path().join(file);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            let mut f = File::create(&path).unwrap();
            write!(f, "{}", file::TEST_CONTENT).unwrap();
        }

        // Create a test library
        let db = Database::open(file::MEMORY_DB).unwrap();
        let library = Library::new(library::TEST_NAME, temp_dir.path());

        // Create a scanner and find audio files
        let scanner = LibraryScanner::new(db, library);
        let audio_files = scanner.find_audio_files();

        // Should find 3 audio files (excluding the .txt file)
        assert_eq!(audio_files.len(), 3);
        assert!(audio_files.iter().any(|p| p.ends_with(file::TEST_MP3)));
        assert!(audio_files.iter().any(|p| p.ends_with(file::TEST_M4B)));
        assert!(audio_files.iter().any(|p| p.ends_with(file::TEST_FLAC)));
        assert!(!audio_files.iter().any(|p| p.ends_with(file::TEST_TXT)));
    }
}
