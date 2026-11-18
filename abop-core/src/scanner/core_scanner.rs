//! Core scanning functionality separated from orchestration concerns
//!
//! This module provides pure file discovery and metadata extraction functionality
//! without database operations, progress reporting, or task management.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use tracing::warn;
use walkdir::WalkDir;

use crate::{
    audio::AudioMetadata,
    error::Result,
    models::Audiobook,
    scanner::{
        config::ScannerConfig,
        error::ScanResult,
        performance::{OperationType, PerformanceMonitor},
    },
};

/// Core scanner responsible for file discovery and metadata extraction
#[derive(Clone)]
pub struct CoreScanner {
    /// Configuration for scanning operations
    config: ScannerConfig,
}

impl CoreScanner {
    /// Creates a new core scanner with default configuration
    #[must_use]
    pub fn new() -> Self {
        let config = ScannerConfig::default();
        Self { config }
    }

    /// Creates a new core scanner with custom configuration
    #[must_use]
    pub const fn with_config(config: ScannerConfig) -> Self {
        Self { config }
    }

    /// Discovers all audio files in a directory (synchronous)
    pub fn discover_files(&self, path: &Path) -> ScanResult<Vec<PathBuf>> {
        log::debug!("Discovering audio files in {}", path.display());
        let extensions = &self.config.extensions;

        // Verify the path exists before scanning
        if !path.exists() {
            warn!(
                "Error reading directory: The system cannot find the path specified: {}",
                path.display()
            );
            return Ok(Vec::new()); // Return empty list instead of failing
        }
        // An unreadable library folder is an error, not an empty library: callers
        // would otherwise treat every known book as deleted
        std::fs::read_dir(path)?;

        Ok(Self::find_audio_files_in_path(
            path,
            extensions,
            self.config.recursive,
        ))
    }

    /// Extracts metadata from an audio file (synchronous)
    pub fn extract_metadata(&self, library_id: &str, path: &Path) -> Result<Audiobook> {
        Self::extract_audiobook_metadata(library_id, path)
    }

    /// Extracts metadata with performance monitoring (synchronous)
    pub fn extract_metadata_with_monitoring(
        &self,
        library_id: &str,
        path: &Path,
        monitor: Option<&PerformanceMonitor>,
    ) -> Result<Audiobook> {
        let start_time = std::time::Instant::now();
        let _timer = monitor.map(|m| {
            m.start_operation(
                path.to_string_lossy().as_ref(),
                OperationType::MetadataExtraction,
            )
        });

        let result = Self::extract_audiobook_metadata(library_id, path);

        // Record the operation result
        if let Some(monitor) = monitor {
            let duration = start_time.elapsed();
            monitor.record_file_processed(duration, result.is_ok());
        }

        result
    }

    /// Internal helper to find audio files in a path (synchronous)
    fn find_audio_files_in_path(
        path: &Path,
        extensions: &[String],
        recursive: bool,
    ) -> Vec<PathBuf> {
        // Double-check that the path exists and is accessible
        if !path.exists() {
            warn!(
                "Error reading directory: Path does not exist: {}",
                path.display()
            );
            return Vec::new();
        }

        let mut walker = WalkDir::new(path).follow_links(true);
        if !recursive {
            // Depth 0 is the folder itself, depth 1 its direct children
            walker = walker.max_depth(1);
        }
        let results: Vec<PathBuf> = walker
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(entry) => {
                    let entry_path = entry.path();
                    log::trace!("Examining {}", entry_path.display());
                    Some(entry)
                }
                Err(e) => {
                    warn!(
                        "Error reading directory entry: IO error for operation on {}: {}",
                        path.display(),
                        e
                    );
                    None
                }
            })
            .filter(|e| e.file_type().is_file())
            .filter_map(|entry| {
                let path = entry.path();
                if let Some(ext) = path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(str::to_lowercase)
                {
                    if extensions.contains(&ext) {
                        log::trace!("Found audio file: {}", path.display());
                        Some(entry.into_path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        log::debug!(
            "CORE SCANNER: Completed scan, found {} files",
            results.len()
        );
        results
    }

    /// Internal helper to extract audiobook metadata (pure function)
    fn extract_audiobook_metadata(library_id: &str, path: &Path) -> Result<Audiobook> {
        // Try to extract metadata, but fall back to basic file info if it fails
        let metadata = match AudioMetadata::from_file(path) {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                // Check if this is a common expected error that shouldn't be treated as fatal
                let error_msg = e.to_string();
                if error_msg.contains("end of stream")
                    || error_msg.contains("Failed to probe audio format")
                    || error_msg.contains("unsupported format")
                {
                    warn!(
                        "Unable to extract metadata from {}, using filename fallback: {}",
                        path.display(),
                        e
                    );
                    None // Use fallback instead of failing
                } else {
                    // For other errors, still fail as these might indicate real issues
                    warn!("Error reading metadata for {}: {}", path.display(), e);
                    return Err(e);
                }
            }
        };

        let mut audiobook = Audiobook::new(library_id, path);

        // Extract title - prefer metadata, fall back to filename
        audiobook.title = Some(
            metadata
                .as_ref()
                .and_then(|m| m.title.clone())
                .unwrap_or_else(|| {
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(crate::models::audiobook::fallbacks::UNKNOWN_TITLE)
                        .to_string()
                }),
        );

        // Set other metadata fields if available
        if let Some(ref meta) = metadata {
            audiobook.author = meta.artist.clone();
            audiobook.narrator = meta.narrator.clone();
            audiobook.duration_seconds = meta.duration_seconds.map(|d| {
                if d.is_nan() || d < 0.0 {
                    0
                } else {
                    // Safely convert f64 to u64 with explicit bounds checking
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    let duration = d.round().clamp(0.0, f64::from(u32::MAX)) as u64;
                    duration
                }
            });

            if let Some(cover_art) = &meta.cover_art {
                audiobook.cover_art = Some(cover_art.clone());
            }
        }

        // Set file size for metadata completeness
        if let Ok(meta) = std::fs::metadata(path) {
            audiobook.size_bytes = Some(meta.len());
        }

        Ok(audiobook)
    }
}

impl Default for CoreScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_discover_files() {
        let temp_dir = tempdir().unwrap();
        let test_files = ["test.mp3", "test.m4b", "test.txt", "test.flac"];

        // Create test files
        for file in &test_files {
            let path = temp_dir.path().join(file);
            let mut f = File::create(&path).unwrap();
            write!(f, "test content").unwrap();
        }

        let scanner = CoreScanner::new();
        let files = scanner.discover_files(temp_dir.path()).unwrap();

        // Should find audio files but not txt
        assert!(files.len() >= 3);
        assert!(files.iter().any(|p| p.file_name().unwrap() == "test.mp3"));
        assert!(files.iter().any(|p| p.file_name().unwrap() == "test.m4b"));
        assert!(files.iter().any(|p| p.file_name().unwrap() == "test.flac"));
        assert!(!files.iter().any(|p| p.file_name().unwrap() == "test.txt"));
    }

    #[test]
    fn test_discover_files_respects_recursive_setting() {
        let temp_dir = tempdir().unwrap();
        std::fs::create_dir_all(temp_dir.path().join("author/series")).unwrap();
        for path in ["top.mp3", "author/nested.m4b", "author/series/deep.mp3"] {
            File::create(temp_dir.path().join(path)).unwrap();
        }
        let names = |recursive: bool| {
            let scanner = CoreScanner::with_config(ScannerConfig {
                recursive,
                ..ScannerConfig::default()
            });
            let mut names: Vec<_> = scanner
                .discover_files(temp_dir.path())
                .unwrap()
                .iter()
                .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
                .collect();
            names.sort();
            names
        };

        assert_eq!(names(true), ["deep.mp3", "nested.m4b", "top.mp3"]);
        assert_eq!(names(false), ["top.mp3"]);
    }
}
