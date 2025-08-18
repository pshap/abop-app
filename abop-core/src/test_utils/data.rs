//! Data factories for ABOP-Core tests

use crate::models::Audiobook;
use chrono::Utc;
use std::path::{Path, PathBuf};

/// Centralized factory for creating test data objects used across core tests
///
/// This factory provides low-level audiobook creation with flexible parameter handling.
/// Unlike the GUI TestDataFactory, this version:
/// - Accepts `Option<&Path>` for explicit path control and fallback handling
/// - Takes `library_id` as a separate parameter for database testing scenarios
/// - Focuses on core business logic testing (repository, database, audio processing)
///
/// Use this factory when:
/// - Testing database operations and repositories
/// - Testing audio processing with specific file paths
/// - Creating audiobooks with precise control over all optional fields
pub struct TestDataFactory;

impl TestDataFactory {
    /// Create an Audiobook with customizable metadata
    pub fn custom_audiobook(
        id: &str,
        library_id: &str,
        title: Option<&str>,
        author: Option<&str>,
        path: Option<&Path>,
        duration_seconds: Option<u64>,
        size_bytes: Option<u64>,
    ) -> Audiobook {
        let now = Utc::now();
        Audiobook {
            id: id.to_string(),
            library_id: library_id.to_string(),
            path: path
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("/test/path/audiobook.mp3")),
            title: title.map(|s| s.to_string()),
            author: author.map(|s| s.to_string()),
            narrator: None,
            description: None,
            duration_seconds,
            size_bytes,
            cover_art: None,
            created_at: now,
            updated_at: now,
            selected: false,
        }
    }

    /// Create a minimal Audiobook with id, library_id and path
    pub fn audiobook_with_path(
        id: &str,
        library_id: &str,
        path: &Path,
        title: &str,
        author: &str,
    ) -> Audiobook {
        Self::custom_audiobook(
            id,
            library_id,
            Some(title),
            Some(author),
            Some(path),
            Some(3600),
            Some(1024 * 1024),
        )
    }
}
