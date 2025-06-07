//! Database module for ABOP
//!
//! This module provides database functionality for storing and retrieving
//! audiobook metadata and library information.

pub mod connection;
pub mod connection_adapter;
pub mod error;
pub mod health;
mod migrations;
mod queries;
pub mod repositories;
pub mod retry;
pub mod statistics;

use rusqlite::OptionalExtension;
use std::path::Path;
use std::sync::Arc;

pub use self::connection::{ConnectionConfig, EnhancedConnection};
pub use self::connection_adapter::ConnectionAdapter;
pub use self::error::{DatabaseError, DbResult};
pub use self::health::ConnectionHealth;
pub use self::migrations::{Migration, MigrationManager, MigrationResult};
pub use self::repositories::{
    AudiobookRepository, LibraryRepository, ProgressRepository, Repository, RepositoryManager,
};
pub use self::retry::{RetryExecutor, RetryPolicy};
pub use self::statistics::ConnectionStats;
use crate::{
    error::Result,
    models::{Audiobook, Library, Progress},
};

/// Database connection wrapper with enhanced connection management
#[derive(Clone)]
pub struct Database {
    enhanced_conn: Arc<EnhancedConnection>,
    repositories: RepositoryManager,
}

impl Database {
    /// Opens or creates a new database at the specified path
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database file cannot be created or opened
    /// - The enhanced connection cannot be established
    /// - Database initialization fails
    /// - Schema creation or migration fails
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        log::debug!("Opening database at: {}", path.as_ref().display());

        // Create enhanced connection with default configuration
        let enhanced_conn = Arc::new(EnhancedConnection::new(path.as_ref()));

        // Establish the connection first
        enhanced_conn
            .connect()
            .map_err(|e| crate::error::AppError::Other(e.to_string()))?;

        // Create repositories using the enhanced connection
        // The repositories will automatically use the enhanced connection for all operations
        let repositories = RepositoryManager::with_enhanced_connection(enhanced_conn.clone());

        let db = Self {
            enhanced_conn,
            repositories,
        };
        db.init()?;
        log::debug!("Database initialization complete");
        Ok(db)
    }

    /// Initializes the database schema
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Database pragmas cannot be set
    /// - Schema migrations fail to run
    /// - Database connection is unavailable
    /// - Connection lock acquisition fails
    fn init(&self) -> Result<()> {
        log::debug!("Initializing database schema...");

        // Configure database settings using enhanced connection
        self.enhanced_conn.with_connection(|conn| {
            conn.execute_batch(
                "PRAGMA foreign_keys = ON;\n\
                 PRAGMA journal_mode = WAL;\n\
                 PRAGMA synchronous = NORMAL;",
            )
            .map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to configure database pragmas: {e}"),
            })?;

            log::debug!("Database settings configured");
            Ok(())
        })?;

        // Run migrations using enhanced connection with mutable access
        log::debug!("Running migrations...");
        self.enhanced_conn.with_connection_mut(|conn| {
            migrations::run_migrations(conn).map_err(|e| {
                log::error!("Failed to run migrations: {e}");
                DatabaseError::MigrationFailed {
                    version: 0, // We don't know which version failed at this point
                    message: format!("Migration failed: {e}"),
                }
            })?;

            log::debug!("Migrations completed successfully");
            Ok(())
        })?;

        log::debug!("Database initialization complete");
        Ok(())
    }

    /// Get access to the repository manager
    #[must_use]
    pub const fn repositories(&self) -> &RepositoryManager {
        &self.repositories
    }

    /// Get access to the library repository
    #[must_use]
    pub const fn libraries(&self) -> &LibraryRepository {
        self.repositories.libraries()
    }

    /// Get access to the audiobook repository
    #[must_use]
    pub const fn audiobooks(&self) -> &AudiobookRepository {
        self.repositories.audiobooks()
    }

    /// Get access to the progress repository
    #[must_use]
    pub const fn progress(&self) -> &ProgressRepository {
        self.repositories.progress()
    }

    /// Get the current connection health
    #[must_use]
    pub fn connection_health(&self) -> ConnectionHealth {
        self.enhanced_conn.health()
    }

    /// Get connection statistics
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to acquire statistics lock
    /// - Failed to read connection timestamp
    /// - Database connection is unavailable
    pub fn connection_stats(&self) -> Result<ConnectionStats> {
        self.enhanced_conn
            .stats()
            .map_err(|e| crate::error::AppError::Other(e.to_string()))
    }

    /// Force a connection health check
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The health check operation fails
    /// - The database connection is unavailable
    /// - Failed to record health check statistics
    pub fn check_connection_health(&self) -> DbResult<ConnectionHealth> {
        self.enhanced_conn
            .health_check()
            .map(|()| self.enhanced_conn.health())
    }

    /// Execute a database operation with enhanced connection features
    /// This provides retry logic and health monitoring
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database operation fails
    /// - Connection retry attempts are exhausted
    /// - The database connection is unavailable
    pub fn execute_with_enhanced_connection<F, R>(&self, f: F) -> crate::error::Result<R>
    where
        F: Fn(&rusqlite::Connection) -> crate::db::error::DbResult<R> + Send + Sync + 'static,
        R: Send + 'static,
    {
        self.enhanced_conn
            .with_connection(f)
            .map_err(|db_err| crate::error::AppError::Other(db_err.to_string()))
    }

    /// Adds a new library to the database
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database operation fails
    /// - A library with the same name already exists
    /// - The path is invalid or inaccessible
    pub fn add_library<P: AsRef<Path> + Send + 'static>(
        &self,
        name: &str,
        path: P,
    ) -> Result<Library> {
        self.libraries()
            .create(name, path)
            .map_err(std::convert::Into::into)
    }

    /// Gets a library by ID
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database query fails
    /// - The database connection is unavailable
    pub fn get_library(&self, id: &str) -> Result<Option<Library>> {
        self.libraries()
            .find_by_id(id)
            .map_err(std::convert::Into::into)
    }

    /// Gets all libraries
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database query fails
    /// - The database connection is unavailable
    pub fn get_libraries(&self) -> Result<Vec<Library>> {
        self.libraries()
            .find_all()
            .map_err(std::convert::Into::into)
    }

    /// Adds an audiobook to the database
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database operation fails
    /// - The audiobook data is invalid
    /// - The associated library does not exist
    pub fn add_audiobook(&self, audiobook: &Audiobook) -> Result<()> {
        let audiobook_clone = audiobook.clone();
        // Use enhanced repository wrapper to leverage enhanced connection
        self.repositories
            .audiobooks()
            .execute_query_enhanced(move |conn| {
                conn.execute(
                    "INSERT OR REPLACE INTO audiobooks (
                        id, library_id, path, title, author, narrator, 
                        description, duration_seconds, size_bytes, cover_art,
                        created_at, updated_at
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                    (
                        &audiobook_clone.id,
                        &audiobook_clone.library_id,
                        &audiobook_clone.path.to_string_lossy(),
                        &audiobook_clone.title,
                        &audiobook_clone.author,
                        &audiobook_clone.narrator,
                        &audiobook_clone.description,
                        audiobook_clone.duration_seconds,
                        audiobook_clone.size_bytes,
                        &audiobook_clone.cover_art,
                        &audiobook_clone.created_at,
                        &audiobook_clone.updated_at,
                    ),
                )?;
                Ok(())
            })
            .map_err(std::convert::Into::into)
    }

    /// Gets an audiobook by ID
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database query fails
    /// - The database connection is unavailable
    pub fn get_audiobook(&self, id: &str) -> Result<Option<Audiobook>> {
        use std::path::PathBuf;
        let id_clone = id.to_string();

        // Use enhanced repository wrapper to leverage enhanced connection
        self.repositories
            .audiobooks()
            .execute_query_enhanced(move |conn| {
                conn.query_row(
                    "SELECT id, library_id, path, title, author, narrator, description, 
                            duration_seconds, size_bytes, cover_art, created_at, updated_at 
                     FROM audiobooks WHERE id = ?1",
                    [&id_clone],
                    |row| {
                        Ok(Audiobook {
                            id: row.get(0)?,
                            library_id: row.get(1)?,
                            path: PathBuf::from(row.get::<_, String>(2)?),
                            title: row.get(3)?,
                            author: row.get(4)?,
                            narrator: row.get(5)?,
                            description: row.get(6)?,
                            duration_seconds: row.get(7)?,
                            size_bytes: row.get(8)?,
                            cover_art: row.get(9)?,
                            created_at: row.get(10)?,
                            updated_at: row.get(11)?,
                            selected: false,
                        })
                    },
                )
                .optional()
            })
            .map_err(std::convert::Into::into)
    }

    /// Gets all audiobooks in a library
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database query fails
    /// - The database connection is unavailable
    /// - The library ID does not exist
    pub fn get_audiobooks_in_library(&self, library_id: &str) -> Result<Vec<Audiobook>> {
        use std::path::PathBuf;
        let library_id_clone = library_id.to_string();

        // Use enhanced repository wrapper to leverage enhanced connection
        self.repositories
            .audiobooks()
            .execute_query_enhanced(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT id, library_id, path, title, author, narrator, description, 
                            duration_seconds, size_bytes, cover_art, created_at, updated_at 
                     FROM audiobooks WHERE library_id = ?1 ORDER BY title",
                )?;

                let rows = stmt.query_map([&library_id_clone], |row| {
                    Ok(Audiobook {
                        id: row.get(0)?,
                        library_id: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                        title: row.get(3)?,
                        author: row.get(4)?,
                        narrator: row.get(5)?,
                        description: row.get(6)?,
                        duration_seconds: row.get(7)?,
                        size_bytes: row.get(8)?,
                        cover_art: row.get(9)?,
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                        selected: false,
                    })
                })?;

                let audiobooks: Vec<Audiobook> =
                    rows.collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?;

                Ok(audiobooks)
            })
            .map_err(std::convert::Into::into)
    }

    /// Saves the playback progress of an audiobook
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database operation fails
    /// - The progress data is invalid
    /// - The associated audiobook does not exist
    pub fn save_progress(&self, progress: &Progress) -> Result<()> {
        self.progress()
            .upsert(progress)
            .map_err(std::convert::Into::into)
    }

    /// Gets the playback progress for an audiobook
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database query fails
    /// - The database connection is unavailable
    pub fn get_progress(&self, audiobook_id: &str) -> Result<Option<Progress>> {
        self.progress()
            .find_by_audiobook(audiobook_id)
            .map_err(std::convert::Into::into)
    }

    /// Get the current migration version of the database
    ///
    /// # Returns
    ///
    /// The current migration version number
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The migration version query fails
    /// - The database connection is unavailable
    /// - Connection lock acquisition fails
    pub fn migration_version(&self) -> Result<u32> {
        let manager = migrations::MigrationManager::new();
        self.enhanced_conn
            .with_connection(move |conn| manager.current_version(conn))
            .map_err(std::convert::Into::into)
    }

    /// Apply all pending migrations
    ///
    /// This will run all migrations that haven't been applied yet,
    /// bringing the database schema up to the latest version.
    ///
    /// # Returns
    ///
    /// A vector of migration results showing which migrations were applied
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Migration operations fail
    /// - The database connection is unavailable
    /// - Connection lock acquisition fails
    /// - Migration files are corrupted or missing
    pub fn migrate_up(&self) -> Result<Vec<migrations::MigrationResult>> {
        let manager = migrations::MigrationManager::new();
        self.enhanced_conn
            .with_connection_mut(move |conn| manager.migrate_up(conn))
            .map_err(std::convert::Into::into)
    }

    /// Rollback database migrations down to the specified target version
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Migration rollback operations fail
    /// - The target version is invalid
    /// - The database connection is unavailable
    /// - Connection lock acquisition fails
    pub fn migrate_down(&self, target_version: u32) -> Result<Vec<migrations::MigrationResult>> {
        let manager = migrations::MigrationManager::new();
        self.enhanced_conn
            .with_connection_mut(move |conn| manager.migrate_down(conn, target_version))
            .map_err(std::convert::Into::into)
    }

    /// Get list of pending migrations
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The migration query fails
    /// - The database connection is unavailable
    /// - Migration metadata is corrupted
    pub fn pending_migrations(&self) -> Result<Vec<(u32, String)>> {
        let manager = migrations::MigrationManager::new();
        self.enhanced_conn
            .with_connection(move |conn| {
                let pending = manager.pending_migrations(conn)?;
                Ok(pending
                    .into_iter()
                    .map(|m| (m.version, m.description.to_string()))
                    .collect())
            })
            .map_err(std::convert::Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Audiobook, Progress};
    use std::path::Path;
    use tempfile::NamedTempFile;
    use uuid::Uuid;

    #[test]
    fn test_initialization_uses_correct_connection() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db = Database::open(temp_file.path())?;
        db.init()?;

        // Verify pragmas are set on actual file
        let file_conn = rusqlite::Connection::open(temp_file.path())?;
        let journal_mode: String =
            file_conn.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
        assert_eq!(journal_mode, "wal");

        let foreign_keys: i64 = file_conn.query_row("PRAGMA foreign_keys", [], |row| row.get(0))?;
        assert_eq!(foreign_keys, 1);

        Ok(())
    }

    #[test]
    fn test_migrations_run_on_actual_database() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let db = Database::open(temp_file.path())?;
        db.init()?;

        // Verify schema exists in actual file
        let file_conn = rusqlite::Connection::open(temp_file.path())?;
        let table_count: i64 = file_conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
            [],
            |row| row.get(0),
        )?;
        assert!(table_count > 0);

        // Verify specific tables exist
        let tables = ["libraries", "audiobooks", "progress"];
        for table in tables {
            let exists: i64 = file_conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                [table],
                |row| row.get(0),
            )?;
            assert_eq!(exists, 1, "Table {} should exist", table);
        }

        Ok(())
    }

    #[test]
    fn test_single_connection_consistency() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_path_buf();
        let db = Database::open(&temp_path)?;
        db.init()?;

        // Create a library
        let library = db.add_library("Test Library", temp_path.clone())?;

        // Verify library exists through all repository paths
        assert!(db.libraries().exists(&library.id)?);
        assert!(db.audiobooks().count_by_library(&library.id)? == 0);

        // Add an audiobook
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
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            selected: false,
        };
        db.add_audiobook(&audiobook)?;

        // Verify audiobook exists through all repository paths
        assert!(db.audiobooks().exists(&audiobook.id)?);
        assert_eq!(db.audiobooks().count_by_library(&library.id)?, 1);

        // Add progress
        let now = chrono::Utc::now();
        let progress = Progress {
            id: Uuid::new_v4().to_string(),
            audiobook_id: audiobook.id.clone(),
            position_seconds: 1800,
            completed: false,
            last_played: Some(now),
            created_at: now,
            updated_at: now,
        };
        db.save_progress(&progress)?;

        // Verify progress exists through all repository paths
        let found_progress = db.get_progress(&audiobook.id)?;
        assert!(found_progress.is_some());
        assert_eq!(found_progress.unwrap().position_seconds, 1800);

        Ok(())
    }

    #[test]
    fn test_enhanced_connection_retry() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_path_buf();
        let db = Database::open(&temp_path)?;
        db.init()?;

        // Create a library
        let library = db.add_library("Test Library", temp_path.clone())?;

        // Simulate connection issues by temporarily making the file read-only
        let metadata = std::fs::metadata(&temp_path)?;
        let mut perms = metadata.permissions();
        perms.set_readonly(true);
        std::fs::set_permissions(&temp_path, perms)?;

        // Try to access the database - should fail gracefully
        let result = db.libraries().exists(&library.id);
        assert!(result.is_err());

        // Make the file writable again
        let mut perms = std::fs::metadata(&temp_path)?.permissions();
        perms.set_readonly(false);
        std::fs::set_permissions(&temp_path, perms)?;

        // Should work again after making writable
        assert!(db.libraries().exists(&library.id)?);

        Ok(())
    }
}
