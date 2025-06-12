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
    pub const fn new(extensions: Vec<String>) -> Self {
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

    /// Check if a file has one of the allowed extensions (case-insensitive)
    fn has_valid_extension(path: &Path, extensions: &[String]) -> bool {
        path.extension()
            .and_then(OsStr::to_str)
            .map(|ext| {
                // Convert both the file extension and allowed extensions to lowercase for comparison
                let ext_lower = ext.to_lowercase();
                extensions
                    .iter()
                    .any(|allowed| allowed.to_lowercase() == ext_lower)
            })
            .unwrap_or(false)
    }

    /// Find audio files synchronously (used internally)
    fn find_audio_files_sync(path: &Path, extensions: &[String]) -> crate::Result<Vec<PathBuf>> {
        debug!("🔍 Starting scan of path: {}", path.display());

        // Check if the path exists and is a directory
        if !path.exists() {
            let msg = format!("Path does not exist: {}", path.display());
            debug!("{}", msg);
            return Err(crate::error::AppError::Io(msg));
        }
        if !path.is_dir() {
            let msg = format!("Path is not a directory: {}", path.display());
            debug!("{}", msg);
            return Err(crate::error::AppError::Io(msg));
        }

        // Normalize extensions to lowercase for consistent comparison
        let extensions: Vec<String> = extensions.iter().map(|ext| ext.to_lowercase()).collect();

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
            .filter(|entry| {
                let path = entry.path();
                let is_match = Self::has_valid_extension(path, &extensions);
                if is_match {
                    debug!("🎵 Found audio file: {}", path.display());
                }
                is_match
            })
            .map(|entry| entry.into_path())
            .collect();

        debug!("🔍 Completed scan, found {} files", results.len());
        Ok(results)
    }
}

#[async_trait::async_trait]
impl FileDiscoverer for DefaultFileDiscoverer {
    async fn discover_files(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let path = path.to_path_buf();
        let extensions = self.extensions.clone();

        // Spawn blocking task for file system operations
        task::spawn_blocking(move || Self::find_audio_files_sync(&path, &extensions))
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
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_test_file(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        writeln!(file, "test content").unwrap();
        path
    }

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

    #[tokio::test]
    async fn test_case_insensitive_extension_matching() {
        let temp_dir = tempdir().unwrap();
        let extensions = vec!["mp3".to_string(), "FLAC".to_string()];

        // Create test files with different case variations
        let mp3_upper = create_test_file(temp_dir.path(), "UPPER.MP3");
        let flac_lower = create_test_file(temp_dir.path(), "lower.flac");
        let mixed_case = create_test_file(temp_dir.path(), "Mixed.CaSe.Mp3");

        let discoverer = DefaultFileDiscoverer::new(extensions);
        let files = discoverer.discover_files(temp_dir.path()).await.unwrap();

        assert_eq!(files.len(), 3);
        assert!(files.contains(&mp3_upper));
        assert!(files.contains(&flac_lower));
        assert!(files.contains(&mixed_case));
    }

    #[tokio::test]
    async fn test_ignores_files_without_extension() {
        let temp_dir = tempdir().unwrap();
        create_test_file(temp_dir.path(), "no_extension");

        let discoverer = DefaultFileDiscoverer::with_default_extensions();
        let files = discoverer.discover_files(temp_dir.path()).await.unwrap();

        assert!(files.is_empty());
    }

    #[tokio::test]
    async fn test_handles_nonexistent_directory() {
        let temp_dir = tempdir().unwrap();
        let non_existent = temp_dir.path().join("nonexistent");

        let discoverer = DefaultFileDiscoverer::with_default_extensions();
        let result = discoverer.discover_files(&non_existent).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handles_permission_denied() {
        // On Unix-like systems, we can test permission denied scenarios
        // On Windows, this test will be skipped
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let temp_dir = tempdir().unwrap();
            let protected_dir = temp_dir.path().join("protected");
            std::fs::create_dir(&protected_dir).unwrap();

            // Set directory to read-only
            let mut perms = std::fs::metadata(&protected_dir).unwrap().permissions();
            perms.set_mode(0o000); // No permissions
            std::fs::set_permissions(&protected_dir, perms).unwrap();

            let discoverer = DefaultFileDiscoverer::with_default_extensions();
            let result = discoverer.discover_files(&protected_dir).await;

            // Cleanup: restore permissions so tempdir can be deleted
            let mut perms = std::fs::metadata(&protected_dir).unwrap().permissions();
            perms.set_mode(0o755);
            let _ = std::fs::set_permissions(&protected_dir, perms);

            assert!(result.is_err());
        }
    }
}
