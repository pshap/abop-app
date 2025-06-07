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

use rusqlite::{Connection, OptionalExtension};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub use self::connection::{ConnectionConfig, EnhancedConnection};
pub use self::connection_adapter::ConnectionAdapter;
pub use self::error::{DatabaseError, DbResult};
pub use self::health::ConnectionHealth;
pub use self::migrations::{Migration, MigrationManager, MigrationResult};
pub use self::repositories::{
    AudiobookRepository, LibraryRepository, ProgressRepository, RepositoryManager,
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

        // Create repositories using a shared dummy connection
        // The repositories will be configured to use the enhanced connection through the manager
        let dummy_conn = Connection::open_in_memory()?;
        let conn_arc = Arc::new(Mutex::new(dummy_conn));
        let repositories =
            RepositoryManager::with_enhanced_connection(conn_arc, enhanced_conn.clone());

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

        // For initialization, we need to temporarily use the basic connection
        // since migrations require mutable access
        let conn = self.repositories.connection().lock().map_err(|_| {
            DatabaseError::ConnectionFailed(
                "Failed to acquire connection lock during initialization".into(),
            )
        })?;

        conn.execute_batch(
            "PRAGMA foreign_keys = ON;\n\
             PRAGMA journal_mode = WAL;\n\
             PRAGMA synchronous = NORMAL;",
        )
        .map_err(|e| DatabaseError::ExecutionFailed {
            message: format!("Failed to configure database pragmas: {e}"),
        })?;

        log::debug!("Database settings configured");

        // Run migrations in a transaction
        log::debug!("Running migrations...");
        let mut conn = self.repositories.connection().lock().map_err(|_| {
            DatabaseError::ConnectionFailed(
                "Failed to acquire connection lock for migrations".into(),
            )
        })?;

        migrations::run_migrations(&mut conn).map_err(|e| {
            log::error!("Failed to run migrations: {e}");
            DatabaseError::MigrationFailed {
                version: 0, // We don't know which version failed at this point
                message: format!("Migration failed: {e}"),
            }
        })?;

        log::debug!("Migrations completed successfully");
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
    pub fn add_library<P: AsRef<Path>>(&self, name: &str, path: P) -> Result<Library> {
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
            .audiobooks_enhanced()
            .execute_query(move |conn| {
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
            .audiobooks_enhanced()
            .execute_query(move |conn| {
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
            .audiobooks_enhanced()
            .execute_query(move |conn| {
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
        let conn = self.repositories.connection().lock().map_err(|_| {
            DatabaseError::ConnectionFailed(
                "Failed to acquire connection lock for version check".into(),
            )
        })?;

        manager
            .current_version(&conn)
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
        let mut conn = self.repositories.connection().lock().map_err(|_| {
            DatabaseError::ConnectionFailed(
                "Failed to acquire connection lock for migration".into(),
            )
        })?;

        manager
            .migrate_up(&mut conn)
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
        let mut conn = self.repositories.connection().lock().map_err(|_| {
            DatabaseError::ConnectionFailed("Failed to acquire connection lock for rollback".into())
        })?;

        manager
            .migrate_down(&mut conn, target_version)
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
    ///
    /// # Panics
    ///
    /// Panics if the connection lock is poisoned (another thread panicked while holding the lock)
    #[allow(clippy::significant_drop_tightening)]
    pub fn pending_migrations(&self) -> Result<Vec<(u32, String)>> {
        let manager = migrations::MigrationManager::new();
        let conn_lock = self.repositories.connection().lock().unwrap();
        let pending = manager
            .pending_migrations(&conn_lock)
            .map_err(|e: DatabaseError| crate::error::AppError::from(e))?;
        Ok(pending
            .into_iter()
            .map(|m| (m.version, m.description.to_string()))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::NamedTempFile;
    use uuid::Uuid;

    fn create_test_db() -> Result<(Database, NamedTempFile)> {
        let temp_file = NamedTempFile::new()?;
        // Database::open already calls init() internally
        let db = Database::open(temp_file.path())?;
        Ok((db, temp_file))
    }

    #[test]
    fn test_add_and_get_library() -> Result<()> {
        let (db, _temp) = create_test_db()?;

        // Add a library
        let library = db.add_library("Test Library", Path::new("/test/path"))?;

        // Get the library by ID
        let retrieved = db.get_library(&library.id)?.unwrap();
        assert_eq!(retrieved.name, "Test Library");
        assert_eq!(retrieved.path, Path::new("/test/path"));

        // Get all libraries
        let libraries = db.get_libraries()?;
        assert_eq!(libraries.len(), 1);
        assert_eq!(libraries[0].name, "Test Library");
        Ok(())
    }

    #[test]
    fn test_connection_stats() -> Result<()> {
        let (db, _temp) = create_test_db()?;

        // Get initial stats
        let stats = db.connection_stats()?;
        assert_eq!(stats.successful_operations, 1); // Initial connection
        assert_eq!(stats.failed_operations, 0);

        // Perform some operations
        db.get_libraries()?;
        db.get_libraries()?;

        // Check updated stats
        let stats = db.connection_stats()?;
        assert_eq!(stats.successful_operations, 3); // Initial + 2 operations
        assert_eq!(stats.failed_operations, 0);
        assert!(stats.avg_operation_duration_ms >= 0.0);
        Ok(())
    }

    #[test]
    fn test_add_and_get_audiobook() -> Result<()> {
        let (db, _temp) = create_test_db()?;

        // Add a library
        let library = db.add_library("Test Library", Path::new("/test/path"))?;

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
        db.add_audiobook(&audiobook)?;

        // Get the audiobook by ID
        let retrieved = db.get_audiobook(&audiobook.id)?.unwrap();
        assert_eq!(retrieved.title, audiobook.title);
        assert_eq!(retrieved.author, audiobook.author);

        // Get audiobooks in library
        let audiobooks = db.get_audiobooks_in_library(&library.id)?;
        assert_eq!(audiobooks.len(), 1);
        assert_eq!(audiobooks[0].id, audiobook.id);
        Ok(())
    }
}
