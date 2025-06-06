//! Database module for ABOP
//!
//! This module provides database functionality for the ABOP application,
//! including connection management, migrations, and repositories.

pub mod connection;
pub mod migrations;
pub mod repositories;
pub mod retry;
pub mod statistics;
pub mod error;
pub mod health;

use std::sync::Arc;
use crate::db::error::{DatabaseError, DbResult};
use crate::db::repositories::{AudiobookRepository, LibraryRepository, ProgressRepository};
use crate::db::statistics::ConnectionStats;
use std::sync::Mutex;
use crate::models::{Library, Audiobook, Progress};
use crate::error::AppError;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Path to the database file
    pub path: String,
    /// Whether to use enhanced features
    pub enhanced: bool,
}

/// Database connection manager
#[derive(Debug)]
pub struct Database {
    /// Shared database connection
    connection: Arc<Mutex<rusqlite::Connection>>,
    /// Library repository
    library: LibraryRepository,
    /// Audiobook repository
    audiobook: AudiobookRepository,
    /// Progress repository
    progress: ProgressRepository,
    /// Connection statistics
    stats: ConnectionStats,
}

impl Database {
    /// Create a new database connection
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to establish database connection.
    pub fn new(config: DatabaseConfig) -> DbResult<Self> {
        let connection = Arc::new(Mutex::new(rusqlite::Connection::open(&config.path)?));
        
        Ok(Self {
            library: LibraryRepository::new(Arc::clone(&connection)),
            audiobook: AudiobookRepository::new(Arc::clone(&connection)),
            progress: ProgressRepository::new(Arc::clone(&connection)),
            connection,
            stats: ConnectionStats::default(),
        })
    }

    /// Get connection statistics
    #[must_use]
    pub fn stats(&self) -> &ConnectionStats {
        &self.stats
    }

    /// Add a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn add_library(&self, library: &Library) -> DbResult<()> {
        self.library.upsert(library)
    }

    /// Get a library by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_library(&self, id: &str) -> DbResult<Option<Library>> {
        self.library.find_by_id(id)
    }

    /// Get all libraries
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_libraries(&self) -> DbResult<Vec<Library>> {
        self.library.find_all()
    }

    /// Add an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn add_audiobook(&self, audiobook: &Audiobook) -> DbResult<()> {
        self.audiobook.upsert(audiobook)
    }

    /// Get an audiobook by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_audiobook(&self, id: &str) -> DbResult<Option<Audiobook>> {
        self.audiobook.find_by_id(id)
    }

    /// Get audiobooks by library ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_audiobooks_by_library(&self, library_id: &str) -> DbResult<Vec<Audiobook>> {
        self.audiobook.find_by_library(library_id)
    }

    /// Get all audiobooks
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_audiobooks(&self) -> DbResult<Vec<Audiobook>> {
        self.audiobook.find_all()
    }

    /// Add progress record
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn add_progress(&self, progress: &Progress) -> DbResult<()> {
        self.progress.upsert(progress)
    }

    /// Get progress by audiobook ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_progress(&self, audiobook_id: &str) -> DbResult<Option<Progress>> {
        self.progress.find_by_audiobook(audiobook_id)
    }

    /// Get all progress records
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_all_progress(&self) -> DbResult<Vec<Progress>> {
        self.progress.find_all()
    }

    /// Batch add multiple audiobooks
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn batch_add_audiobooks(&self, audiobooks: &[Audiobook]) -> DbResult<()> {
        self.audiobook.batch_add(audiobooks)
    }

    /// Batch add multiple progress records
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn batch_add_progress(&self, progress_records: &[Progress]) -> DbResult<()> {
        self.progress.batch_add(progress_records)
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            connection: Arc::clone(&self.connection),
            library: self.library.clone(),
            audiobook: self.audiobook.clone(),
            progress: self.progress.clone(),
            stats: self.stats.clone(),
        }
    }
}

impl From<AppError> for DatabaseError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Database(e) => DatabaseError::Sqlite(e),
            other => DatabaseError::ExecutionFailed {
                message: other.to_string(),
            },
        }
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(err: std::io::Error) -> Self {
        DatabaseError::ExecutionFailed {
            message: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::NamedTempFile;
    use uuid::Uuid;
    use std::fs;

    fn create_test_db() -> (Database, NamedTempFile) {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, "").unwrap();
        let db = Database::new(DatabaseConfig {
            path: path.to_string_lossy().to_string(),
            enhanced: true,
        }).unwrap();
        (db, temp_file)
    }

    #[test]
    fn test_add_and_get_library() {
        let (db, _temp) = create_test_db();

        // Add a library
        let library = Library {
            id: Uuid::new_v4().to_string(),
            name: "Test Library".to_string(),
            path: Path::new("/test/path").to_path_buf(),
        };
        db.add_library(&library).unwrap();

        // Get the library by ID
        let retrieved = db.get_library(&library.id).unwrap().unwrap();
        assert_eq!(retrieved.name, "Test Library");
        assert_eq!(retrieved.path, Path::new("/test/path"));

        // Get all libraries
        let libraries = db.get_libraries().unwrap();
        assert_eq!(libraries.len(), 1);
        assert_eq!(libraries[0].name, "Test Library");
    }

    #[test]
    fn test_add_and_get_audiobook() {
        let (db, _temp) = create_test_db();

        // Add a library
        let library = Library {
            id: Uuid::new_v4().to_string(),
            name: "Test Library".to_string(),
            path: Path::new("/test/path").to_path_buf(),
        };
        db.add_library(&library).unwrap();

        // Create an audiobook
        let audiobook = Audiobook {
            id: Uuid::new_v4().to_string(),
            library_id: library.id.clone(),
            path: Path::new("/test/path/book.mp3").to_path_buf(),
            title: Some("Test Book".to_string()),
            author: Some("Test Author".to_string()),
            narrator: Some("Test Narrator".to_string()),
            description: Some("Test Description".to_string()),
            duration_seconds: Some(3600),
            size_bytes: Some(1024 * 1024),
            cover_art: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            selected: false,
        };

        // Add the audiobook
        db.add_audiobook(&audiobook).unwrap();

        // Get the audiobook by ID
        let retrieved = db.get_audiobook(&audiobook.id).unwrap().unwrap();
        assert_eq!(retrieved.title, audiobook.title);
        assert_eq!(retrieved.author, audiobook.author);

        // Get audiobooks in library
        let audiobooks = db.get_audiobooks_by_library(&library.id).unwrap();
        assert_eq!(audiobooks.len(), 1);
        assert_eq!(audiobooks[0].id, audiobook.id);
    }
}
