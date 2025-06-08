//! Core scanning functionality separated from orchestration concerns
//!
//! This module provides pure file discovery and metadata extraction functionality
//! without database operations, progress reporting, or task management.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::sync::Semaphore;
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
    /// Semaphore to limit concurrent operations
    semaphore: Arc<Semaphore>,
}

impl CoreScanner {
    /// Creates a new core scanner with default configuration
    #[must_use]
    pub fn new() -> Self {
        let config = ScannerConfig::default();
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_tasks));
        
        Self {
            config,
            semaphore,
        }
    }

    /// Creates a new core scanner with custom configuration
    #[must_use]
    pub fn with_config(config: ScannerConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_tasks));
        
        Self {
            config,
            semaphore,
        }
    }

    /// Discovers all audio files in a directory
    pub async fn discover_files(&self, path: &Path) -> ScanResult<Vec<PathBuf>> {
        let path = path.to_path_buf();
        let extensions = self.config.extensions.clone();

        tokio::task::spawn_blocking(move || {
            Self::find_audio_files_in_path(&path, &extensions)
        })
        .await
        .map_err(|e| {
            warn!("Failed to discover audio files: {}", e);
            crate::scanner::error::ScanError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("File discovery failed: {}", e),
            ))
        })
    }

    /// Extracts metadata from an audio file
    pub async fn extract_metadata(
        &self,
        library_id: &str,
        path: &Path,
    ) -> Result<Audiobook> {        // Acquire semaphore permit to limit concurrent operations
        let _permit = self.semaphore.acquire().await
            .map_err(|_| crate::error::AppError::Threading("Semaphore acquisition failed".to_string()))?;

        Self::extract_audiobook_metadata(library_id, path)
    }

    /// Extracts metadata with performance monitoring
    pub async fn extract_metadata_with_monitoring(
        &self,
        library_id: &str,
        path: &Path,
        monitor: Option<&PerformanceMonitor>,    ) -> Result<Audiobook> {
        // Acquire semaphore permit to limit concurrent operations
        let _permit = self.semaphore.acquire().await
            .map_err(|_| crate::error::AppError::Threading("Semaphore acquisition failed".to_string()))?;

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
    fn find_audio_files_in_path(path: &Path, extensions: &[String]) -> Vec<PathBuf> {
        log::debug!("🔍 CORE SCANNER: Starting scan of path: {}", path.display());

        let results: Vec<PathBuf> = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Ok(entry) => {
                    let entry_path = entry.path();
                    log::debug!("🔍 CORE SCANNER: Examining entry: {}", entry_path.display());
                    Some(entry)
                }
                Err(e) => {
                    log::warn!("Error reading directory entry: {}", e);
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
                        log::debug!("🎵 CORE SCANNER: Found audio file: {}", path.display());
                        Some(entry.into_path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        log::debug!("🔍 CORE SCANNER: Completed scan, found {} files", results.len());
        results
    }

    /// Internal helper to extract audiobook metadata (pure function)
    fn extract_audiobook_metadata(library_id: &str, path: &Path) -> Result<Audiobook> {
        let metadata = AudioMetadata::from_file(path).map_err(|e| {
            warn!("Error reading metadata for {}: {}", path.display(), e);
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

    #[tokio::test]
    async fn test_discover_files() {
        let temp_dir = tempdir().unwrap();
        let test_files = ["test.mp3", "test.m4b", "test.txt", "test.flac"];

        // Create test files
        for file in &test_files {
            let path = temp_dir.path().join(file);
            let mut f = File::create(&path).unwrap();
            write!(f, "test content").unwrap();
        }

        let scanner = CoreScanner::new();
        let files = scanner.discover_files(temp_dir.path()).await.unwrap();
        
        // Should find audio files but not txt
        assert!(files.len() >= 3);
        assert!(files.iter().any(|p| p.file_name().unwrap() == "test.mp3"));
        assert!(files.iter().any(|p| p.file_name().unwrap() == "test.m4b"));
        assert!(files.iter().any(|p| p.file_name().unwrap() == "test.flac"));
        assert!(!files.iter().any(|p| p.file_name().unwrap() == "test.txt"));
    }
}
