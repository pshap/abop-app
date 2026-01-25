//! Database test utilities for creating test databases and common test data
//!
//! This module provides centralized utilities for database testing, eliminating
//! the massive code duplication found across repository test files.

use crate::db::{connection::EnhancedConnection, migrations::run_migrations};
use crate::models::{Audiobook, Library, Progress};
use chrono::Utc;
use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::{NamedTempFile, TempPath};

/// Centralized database test setup result
pub struct TestDatabase {
    /// The enhanced connection for repository operations
    pub connection: Arc<EnhancedConnection>,
    /// Temporary file path that will be cleaned up when dropped
    pub _temp_path: TempPath,
}

impl TestDatabase {
    /// Create a new test database with migrations applied
    ///
    /// # Returns
    /// A `TestDatabase` with a fresh database and all migrations applied
    ///
    /// # Panics
    /// Panics if database setup fails
    pub fn new() -> Self {
        let temp_file =
            NamedTempFile::new().expect("Failed to create temporary file for test database");
        let temp_path = temp_file.into_temp_path();

        // Run migrations first
        let mut conn = Connection::open(&temp_path)
            .expect("Failed to open database connection for migrations");
        run_migrations(&mut conn).expect("Failed to run database migrations during test setup");
        drop(conn); // Close the migration connection

        // Create enhanced connection
        let connection = Arc::new(EnhancedConnection::new(&temp_path));
        connection
            .connect()
            .expect("Failed to connect to test database");

        Self {
            connection,
            _temp_path: temp_path,
        }
    }

    /// Create a test database with sample libraries pre-populated
    ///
    /// # Returns
    /// A `TestDatabase` with sample library data
    pub fn with_sample_libraries() -> Self {
        let db = Self::new();

        // Insert sample libraries
        let libraries = [
            ("test-library-1", "Test Library 1", "/test/library/path1"),
            ("test-library-2", "Test Library 2", "/test/library/path2"),
            ("music-library", "Music Collection", "/home/user/Music"),
        ];

        for (id, name, path) in &libraries {
            db.insert_test_library(id, name, path);
        }

        db
    }

    /// Insert a test library directly into the database
    ///
    /// # Arguments
    /// * `id` - Library ID
    /// * `name` - Library name
    /// * `path` - Library path
    pub fn insert_test_library(&self, id: &str, name: &str, path: &str) {
        let conn = Connection::open(&self._temp_path)
            .expect("Failed to open connection for test data insertion");

        conn.execute(
            "INSERT OR IGNORE INTO libraries (id, name, path) VALUES (?, ?, ?)",
            params![id, name, path],
        )
        .expect("Failed to insert test library");
    }

    /// Insert a test audiobook directly into the database
    ///
    /// # Arguments
    /// * `audiobook` - The audiobook to insert
    pub fn insert_test_audiobook(&self, audiobook: &Audiobook) {
        let conn = Connection::open(&self._temp_path)
            .expect("Failed to open connection for test data insertion");

        conn.execute(
            "INSERT OR IGNORE INTO audiobooks (id, library_id, title, author, path, duration_seconds, size_bytes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                audiobook.id,
                audiobook.library_id,
                audiobook.title,
                audiobook.author,
                audiobook.path.to_string_lossy(),
                audiobook.duration_seconds,
                audiobook.size_bytes,
                audiobook.created_at.timestamp(),
                audiobook.updated_at.timestamp()
            ],
        ).expect("Failed to insert test audiobook");
    }

    /// Insert test progress directly into the database
    ///
    /// # Arguments
    /// * `progress` - The progress record to insert
    pub fn insert_test_progress(&self, progress: &Progress) {
        let conn = Connection::open(&self._temp_path)
            .expect("Failed to open connection for test data insertion");

        conn.execute(
            "INSERT OR REPLACE INTO progress (id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                progress.id,
                progress.audiobook_id,
                progress.position_seconds,
                progress.completed,
                progress.last_played.map(|dt| dt.timestamp()),
                progress.created_at.timestamp(),
                progress.updated_at.timestamp()
            ],
        ).expect("Failed to insert test progress");
    }
}

/// Factory for creating test data objects
pub struct TestDataFactory;

impl TestDataFactory {
    /// Create a test library with default values
    ///
    /// # Arguments
    /// * `name` - Optional library name (defaults to "Test Library")
    /// * `path` - Optional library path (defaults to "/test/path")
    ///
    /// # Returns
    /// A test `Library` instance
    pub fn library(name: Option<&str>, path: Option<&str>) -> Library {
        let name = name.unwrap_or("Test Library");
        let path = path.unwrap_or("/test/path");

        Library {
            id: format!("test-library-{}", uuid::Uuid::new_v4()),
            name: name.to_string(),
            path: PathBuf::from(path),
        }
    }

    /// Create a test audiobook with default values
    ///
    /// # Arguments
    /// * `library_id` - ID of the parent library
    /// * `title` - Optional title (defaults to "Test Audiobook")
    /// * `path` - Optional file path (defaults to "/test/audiobook.mp3")
    ///
    /// # Returns
    /// A test `Audiobook` instance
    pub fn audiobook(library_id: &str, title: Option<&str>, path: Option<&str>) -> Audiobook {
        let title = title.unwrap_or("Test Audiobook");
        let path = path.unwrap_or("/test/audiobook.mp3");
        let now = Utc::now();

        Audiobook {
            id: format!("test-audiobook-{}", uuid::Uuid::new_v4()),
            library_id: library_id.to_string(),
            path: PathBuf::from(path),
            title: Some(title.to_string()),
            author: Some("Test Author".to_string()),
            narrator: None,
            description: None,
            duration_seconds: Some(3600),  // 1 hour in seconds
            size_bytes: Some(1024 * 1024), // 1MB
            cover_art: None,
            created_at: now,
            updated_at: now,
            selected: false,
        }
    }

    /// Create a test progress record with default values
    ///
    /// # Arguments
    /// * `audiobook_id` - ID of the audiobook
    /// * `position_seconds` - Optional playback position in seconds (defaults to 1800 - 30 minutes)
    ///
    /// # Returns
    /// A test `Progress` instance
    pub fn progress(audiobook_id: &str, position_seconds: Option<u64>) -> Progress {
        let position_seconds = position_seconds.unwrap_or(1800); // 30 minutes
        let now = Utc::now();

        Progress {
            id: format!("test-progress-{}", uuid::Uuid::new_v4()),
            audiobook_id: audiobook_id.to_string(),
            position_seconds,
            completed: false,
            last_played: Some(now),
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a test audiobook with associated progress
    ///
    /// # Arguments
    /// * `library_id` - ID of the parent library
    /// * `position_seconds` - Optional playback position in seconds
    ///
    /// # Returns
    /// A tuple of (Audiobook, Progress)
    pub fn audiobook_with_progress(
        library_id: &str,
        position_seconds: Option<u64>,
    ) -> (Audiobook, Progress) {
        let audiobook = Self::audiobook(library_id, None, None);
        let progress = Self::progress(&audiobook.id, position_seconds);

        (audiobook, progress)
    }
}

/// Common test assertions for database operations
pub struct TestAssertions;

impl TestAssertions {
    /// Assert that a library matches expected values
    ///
    /// # Arguments
    /// * `actual` - The actual library
    /// * `expected_name` - Expected name
    /// * `expected_path` - Expected path
    pub fn assert_library_matches(actual: &Library, expected_name: &str, expected_path: &str) {
        assert_eq!(actual.name, expected_name);
        assert_eq!(actual.path, PathBuf::from(expected_path));
        assert!(!actual.id.is_empty(), "Library ID should not be empty");
    }

    /// Assert that an audiobook matches expected values
    ///
    /// # Arguments
    /// * `actual` - The actual audiobook
    /// * `expected_library_id` - Expected library ID
    /// * `expected_title` - Expected title
    /// * `expected_path` - Expected file path
    pub fn assert_audiobook_matches(
        actual: &Audiobook,
        expected_library_id: &str,
        expected_title: &str,
        expected_path: &str,
    ) {
        assert_eq!(actual.library_id, expected_library_id);
        assert_eq!(actual.title, Some(expected_title.to_string()));
        assert_eq!(actual.path, PathBuf::from(expected_path));
        assert!(!actual.id.is_empty(), "Audiobook ID should not be empty");
    }

    /// Assert that progress matches expected values
    ///
    /// # Arguments
    /// * `actual` - The actual progress
    /// * `expected_audiobook_id` - Expected audiobook ID
    /// * `expected_position_seconds` - Expected position in seconds
    pub fn assert_progress_matches(
        actual: &Progress,
        expected_audiobook_id: &str,
        expected_position_seconds: u64,
    ) {
        assert_eq!(actual.audiobook_id, expected_audiobook_id);
        assert_eq!(actual.position_seconds, expected_position_seconds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::repositories::LibraryRepository;

    #[test]
    fn test_database_setup() {
        let db = TestDatabase::new();
        // The connection was established during creation
        assert!(db.connection.connect().is_ok());
    }

    #[test]
    fn test_sample_libraries() {
        let db = TestDatabase::with_sample_libraries();

        let repo = LibraryRepository::new(db.connection.clone());
        let libraries = repo.find_all().expect("Failed to list libraries");

        // Should have the 3 sample libraries
        assert_eq!(libraries.len(), 3);

        // Check one of them
        let test_lib = libraries
            .iter()
            .find(|lib| lib.name == "Test Library 1")
            .expect("Should find Test Library 1");
        assert_eq!(test_lib.path, PathBuf::from("/test/library/path1"));
    }

    #[test]
    fn test_data_factory() {
        let library = TestDataFactory::library(Some("My Library"), Some("/my/path"));
        assert_eq!(library.name, "My Library");
        assert_eq!(library.path, PathBuf::from("/my/path"));

        let audiobook = TestDataFactory::audiobook("lib-123", Some("My Book"), Some("/book.mp3"));
        assert_eq!(audiobook.library_id, "lib-123");
        assert_eq!(audiobook.title, Some("My Book".to_string()));
        assert_eq!(audiobook.path, PathBuf::from("/book.mp3"));
    }

    #[test]
    fn test_audiobook_with_progress() {
        let (audiobook, progress) = TestDataFactory::audiobook_with_progress("lib-123", Some(2400));

        assert_eq!(audiobook.library_id, "lib-123");
        assert_eq!(progress.audiobook_id, audiobook.id);
        assert_eq!(progress.position_seconds, 2400);
    }

    #[test]
    fn test_assertions() {
        let library = TestDataFactory::library(Some("Test"), Some("/test"));
        TestAssertions::assert_library_matches(&library, "Test", "/test");

        let audiobook = TestDataFactory::audiobook("lib-1", Some("Book"), Some("/book.mp3"));
        TestAssertions::assert_audiobook_matches(&audiobook, "lib-1", "Book", "/book.mp3");

        let progress = TestDataFactory::progress("book-1", Some(123));
        TestAssertions::assert_progress_matches(&progress, "book-1", 123);
    }
}
