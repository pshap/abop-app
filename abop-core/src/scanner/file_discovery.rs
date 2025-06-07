//! File discovery functional    /// Create a new file discoverer with specific extensions
    //! File discovery for the scanner
//!
//! This module provides the core file discovery functionality, abstracted behind
//! a trait to allow for different discovery strategies.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tokio::task;
use tracing::{debug, warn};
use walkdir::WalkDir;

use super::constants::SUPPORTED_AUDIO_EXTENSIONS;
use crate::error::Result;

/// Trait for discovering audio files in a directory
#[async_trait::async_trait]
pub trait FileDiscoverer: Send + Sync {
    /// Find all audio files in the given directory
    async fn discover_files(&self, path: &Path) -> Result<Vec<PathBuf>>;
}

/// Default implementation of FileDiscoverer using walkdir
#[derive(Debug, Clone)]
pub struct DefaultFileDiscoverer {
    /// File extensions to include in discovery
    extensions: Vec<String>,
}

impl DefaultFileDiscoverer {
    /// Create a new file discoverer with the given extensions
    #[allow(dead_code)]
    #[must_use]
    pub fn new(extensions: Vec<String>) -> Self {
        Self { extensions }
    }

    /// Create a new file discoverer with default audio extensions
    #[must_use]
    pub fn with_default_extensions() -> Self {
        Self {
            extensions: SUPPORTED_AUDIO_EXTENSIONS
                .iter()
                .map(ToString::to_string)
                .collect(),
        }
    }

    /// Find audio files synchronously (used internally)
    fn find_audio_files_sync(path: &Path, extensions: &[String]) -> Vec<PathBuf> {
        debug!("🔍 Starting scan of path: {}", path.display());

        let results: Vec<PathBuf> = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(entry) => {
                    let entry_path = entry.path();
                    debug!("🔍 Examining entry: {}", entry_path.display());
                    Some(entry)
                }
                Err(e) => {
                    warn!("Error reading directory entry: {e}");
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
                        debug!("🎵 Found audio file: {}", path.display());
                        Some(entry.into_path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        debug!("🔍 Completed scan, found {} files", results.len());
        results
    }
}

#[async_trait::async_trait]
impl FileDiscoverer for DefaultFileDiscoverer {
    async fn discover_files(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let path = path.to_path_buf();
        let extensions = self.extensions.clone();

        // Spawn blocking task for file system operations
        task::spawn_blocking(move || {
            Ok::<_, crate::error::AppError>(Self::find_audio_files_sync(&path, &extensions))
        })
        .await
        .map_err(|e| crate::error::AppError::TaskJoin(e.to_string()))?
    }
}

impl Default for DefaultFileDiscoverer {
    fn default() -> Self {
        Self::with_default_extensions()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_discover_files() {
        // Create a temporary directory with test files
        let temp_dir = tempdir().unwrap();
        let test_files = ["test.mp3", "test.m4b", "test.txt", "test.flac"];

        // Create the files
        for file in &test_files {
            let path = temp_dir.path().join(file);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            let mut f = File::create(&path).unwrap();
            write!(f, "test content").unwrap();
        }

        // Create discoverer and find files
        let discoverer = DefaultFileDiscoverer::default();
        let audio_files = discoverer.discover_files(temp_dir.path()).await.unwrap();

        // Should find 3 audio files (excluding the .txt file)
        assert_eq!(audio_files.len(), 3);
        assert!(audio_files.iter().any(|p| p.ends_with("test.mp3")));
        assert!(audio_files.iter().any(|p| p.ends_with("test.m4b")));
        assert!(audio_files.iter().any(|p| p.ends_with("test.flac")));
        assert!(!audio_files.iter().any(|p| p.ends_with("test.txt")));
    }
}
