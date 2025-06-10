//! Database module for ABOP
//!
//! This module provides database functionality for storing and retrieving
//! audiobook metadata and library information.

pub mod connection;
pub mod connection_adapter;
pub mod datetime_serde;
pub mod error;
pub mod health;
pub mod helpers;
pub mod mappers;
mod migrations;
pub mod operations;
mod queries;
pub mod repositories;
pub mod retry;
pub mod statistics;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, OptionalExtension};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tracing::{debug, info, instrument};

pub use self::connection::{ConnectionConfig, EnhancedConnection};
pub use self::connection_adapter::ConnectionAdapter;
pub use self::error::{DatabaseError, DbResult};
pub use self::health::ConnectionHealth;
pub use self::helpers::{
    DatabaseHelpers, PoolHelper, execute_bulk_insert, parse_datetime_string, with_connection,
    with_connection_mut,
};
pub use self::mappers::{AudiobookColumnIndices, RowMappers, SqlQueries};
pub use self::migrations::{Migration, MigrationManager, MigrationResult};
pub use self::operations::DatabaseOperations;
pub use self::repositories::{
    AudiobookRepository, LibraryRepository, ProgressRepository, Repository, RepositoryManager,
};
pub use self::retry::{RetryExecutor, RetryPolicy};
pub use self::statistics::ConnectionStats;
use crate::{
    error::{AppError, Result},
    models::{Audiobook, Library},
};

/// Database connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of connections in the pool
    pub max_connections: usize,
    /// Path to the database file
    pub path: String,
    /// Whether to create the database if it doesn't exist
    pub create_if_missing: bool,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 4,
            path: ":memory:".to_string(),
            create_if_missing: true,
        }
    }
}

/// Database connection with connection pooling and enhanced operations
#[derive(Debug, Clone)]
pub struct Database {
    /// Connection pool
    pool: Arc<r2d2::Pool<SqliteConnectionManager>>,
    /// High-level database operations
    operations: DatabaseOperations,
    /// Mutex for thread-safe access to shared state
    state: Arc<Mutex<DatabaseState>>,
    /// Database file path for repository creation
    db_path: PathBuf,
}

/// Shared state for database operations
#[derive(Debug, Default)]
struct DatabaseState {
    /// Cache of library IDs
    library_cache: std::collections::HashMap<String, String>,
}

impl Database {
    /// Creates a new database connection with the specified configuration
    #[instrument(skip(config))]
    pub fn new(config: PoolConfig) -> Result<Self> {
        let manager = SqliteConnectionManager::file(&config.path);
        let pool = Pool::builder()
            .max_size(config.max_connections as u32)
            .build(manager)
            .map_err(|e| {
                AppError::Database(DatabaseError::ConnectionFailed(format!(
                    "Failed to create connection pool: {e}"
                )))
            })?; // Initialize database schema using the pool connection
        let mut conn = pool.get().map_err(|e| {
            AppError::Database(DatabaseError::ConnectionFailed(format!(
                "Failed to get connection from pool: {e}"
            )))
        })?;

        // Set up pragmas on the actual connection
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = 1000;
             PRAGMA temp_store = memory;",
        )
        .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        // Run migrations on the actual database connection
        migrations::run_migrations(&mut conn)
            .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        debug!(
            "Database initialized successfully with {} max connections",
            config.max_connections
        );

        let pool = Arc::new(pool);
        let operations = DatabaseOperations::new(pool.clone());

        Ok(Self {
            pool,
            operations,
            state: Arc::new(Mutex::new(DatabaseState::default())),
            db_path: PathBuf::from(&config.path),
        })
    }

    /// Creates a new in-memory database for testing
    #[must_use = "Database should be used or an error handled"]
    pub fn in_memory() -> Result<Self> {
        let config = PoolConfig {
            path: ":memory:".to_string(),
            max_connections: 1,
            ..Default::default()
        };

        Self::new(config)
    }
    /// Initializes the database schema
    #[allow(dead_code)]
    #[instrument(skip(conn))]
    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS libraries (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS audiobooks (
                id TEXT PRIMARY KEY,
                library_id TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                title TEXT,
                author TEXT,
                narrator TEXT,
                description TEXT,
                duration_seconds INTEGER,
                size_bytes INTEGER,
                cover_art BLOB,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (library_id) REFERENCES libraries(id),
                UNIQUE(library_id, path)
            );
            CREATE INDEX IF NOT EXISTS idx_audiobooks_library_id ON audiobooks(library_id);
            CREATE INDEX IF NOT EXISTS idx_audiobooks_path ON audiobooks(path);
            CREATE INDEX IF NOT EXISTS idx_audiobooks_title ON audiobooks(title);
            CREATE INDEX IF NOT EXISTS idx_audiobooks_author ON audiobooks(author);",
        )
        .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        Ok(())
    }
    /// Adds a library to the database
    #[instrument(skip(self, library))]
    pub fn add_library(&self, library: &Library) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let library_name = library.name.clone();
        let library_path = library.path.to_string_lossy().to_string();
        let id_clone = id.clone();
        let library_path_for_cache = library_path.clone();

        self.operations
            .execute(move |conn| {
                conn.execute(
                    "INSERT INTO libraries (id, name, path) VALUES (?1, ?2, ?3)",
                    [
                        &id_clone as &dyn rusqlite::ToSql,
                        &library_name,
                        &library_path,
                    ],
                )
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to insert library: {e}"),
                })?;
                Ok(())
            })
            .map_err(AppError::Database)?;

        // Update cache
        self.state
            .lock()
            .unwrap()
            .library_cache
            .insert(library_path_for_cache, id.clone());

        Ok(id)
    }
    /// Gets a library ID from the database, using cache if available
    #[instrument(skip(self, path))]
    fn get_library_id(&self, path: &Path) -> Result<String> {
        let path_str = path.to_string_lossy().to_string();

        // Check cache first
        if let Some(id) = self.state.lock().unwrap().library_cache.get(&path_str) {
            return Ok(id.clone());
        }

        // Not in cache, query the database
        let path_str_for_query = path_str.clone();
        let id = self
            .operations
            .execute(move |conn| {
                let mut stmt = conn.prepare("SELECT id FROM libraries WHERE path = ?")?;
                let id: Option<String> = stmt
                    .query_row([&path_str_for_query], |row| row.get(0))
                    .optional()
                    .map_err(|e| DatabaseError::ExecutionFailed {
                        message: format!("Failed to query library ID: {e}"),
                    })?;
                Ok(id)
            })
            .map_err(AppError::Database)?;

        if let Some(id) = id {
            // Update cache
            self.state
                .lock()
                .unwrap()
                .library_cache
                .insert(path_str, id.clone());
            Ok(id)
        } else {
            Err(AppError::Database(DatabaseError::NotFound {
                entity: "Library".to_string(),
                id: path_str,
            }))
        }
    }
    /// Adds an audiobook to the database
    #[instrument(skip(self, audiobook))]
    pub fn add_audiobook(&self, audiobook: &Audiobook) -> Result<()> {
        let library_id = self.get_library_id(audiobook.path.parent().unwrap())?;

        let audiobook_clone = audiobook.clone();
        let library_id_clone = library_id.clone();
        self.operations
            .execute(move |conn| {
                let mut stmt = conn.prepare(
                    "INSERT OR REPLACE INTO audiobooks 
                (id, library_id, path, title, author, narrator, description, 
                 duration_seconds, size_bytes, cover_art, created_at, updated_at) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                )?;

                stmt.execute(rusqlite::params![
                    audiobook_clone.id,
                    library_id_clone,
                    audiobook_clone.path.to_string_lossy(),
                    audiobook_clone.title,
                    audiobook_clone.author,
                    audiobook_clone.narrator,
                    audiobook_clone.description,
                    audiobook_clone.duration_seconds.map(|d| d as i64),
                    audiobook_clone.size_bytes.map(|s| s as i64),
                    audiobook_clone.cover_art,
                    audiobook_clone.created_at.to_rfc3339(),
                    audiobook_clone.updated_at.to_rfc3339(),
                ])
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to insert audiobook: {e}"),
                })?;

                Ok(())
            })
            .map_err(AppError::Database)
    }
    /// Adds multiple audiobooks to the database in bulk
    #[instrument(skip(self, audiobooks))]
    pub fn add_audiobooks_bulk(&self, audiobooks: &[Audiobook]) -> Result<()> {
        if audiobooks.is_empty() {
            return Ok(());
        }

        // Group audiobooks by library_id (which is already stored in each audiobook)
        let mut library_groups: std::collections::HashMap<String, Vec<Audiobook>> =
            std::collections::HashMap::new();
        for audiobook in audiobooks {
            library_groups
                .entry(audiobook.library_id.clone())
                .or_default()
                .push(audiobook.clone());
        }

        // Process each library's audiobooks
        for (library_id, audiobooks_for_library) in library_groups {

            self.operations.execute_transaction(move |tx| {
                let mut stmt = tx.prepare(
                    "INSERT OR REPLACE INTO audiobooks 
                    (id, library_id, path, title, author, narrator, description, 
                     duration_seconds, size_bytes, cover_art, created_at, updated_at) 
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                )?;

                for audiobook in &audiobooks_for_library {
                    stmt.execute(rusqlite::params![
                        audiobook.id,
                        &library_id,
                        audiobook.path.to_string_lossy(),
                        &audiobook.title,
                        &audiobook.author,
                        &audiobook.narrator,
                        &audiobook.description,
                        audiobook.duration_seconds.map(|d| d as i64),
                        audiobook.size_bytes.map(|s| s as i64),
                        &audiobook.cover_art,
                        audiobook.created_at.to_rfc3339(),
                        audiobook.updated_at.to_rfc3339(),
                    ])
                    .map_err(|e| DatabaseError::ExecutionFailed {
                        message: format!("Failed to insert audiobook: {e}"),
                    })?;
                }

                Ok(())
            })?;
        }

        Ok(())
    }
    /// Gets all audiobooks from a library
    #[instrument(skip(self, library_path))]
    pub fn get_audiobooks(&self, library_path: &Path) -> Result<Vec<Audiobook>> {
        let library_id = self.get_library_id(library_path)?;

        self.operations
            .execute_query(move |conn| {
                let mut stmt = conn
                    .prepare(&SqlQueries::audiobook_select(Some(
                        "library_id = ?1 ORDER BY title",
                    )))
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!(
                            "Failed to prepare statement: {e}"
                        ))
                    })?;

                let rows = stmt
                    .query_map([&library_id], |row| {
                        match RowMappers::audiobook_from_row(row) {
                            Ok(audiobook) => Ok(audiobook),
                            Err(_) => Err(rusqlite::Error::InvalidColumnType(
                                0,
                                "mapping failed".to_string(),
                                rusqlite::types::Type::Null,
                            )),
                        }
                    })
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!("Failed to execute query: {e}"))
                    })?;

                let mut audiobooks = Vec::new();
                for row in rows {
                    match row {
                        Ok(audiobook) => audiobooks.push(audiobook),
                        Err(e) => {
                            return Err(DatabaseError::execution_failed(&format!(
                                "Failed to process row: {e}"
                            )));
                        }
                    }
                }

                Ok(audiobooks)
            })
            .map_err(AppError::Database)
    }
    /// Gets a single audiobook by path
    #[instrument(skip(self, path))]
    pub fn get_audiobook(&self, path: &Path) -> Result<Option<Audiobook>> {
        let path_str = path.to_string_lossy().to_string();

        self.operations
            .execute_query(move |conn| {
                let mut stmt = conn
                    .prepare(&SqlQueries::audiobook_select(Some("path = ?1")))
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!(
                            "Failed to prepare statement: {e}"
                        ))
                    })?;

                let mut rows = stmt
                    .query_map([&path_str], |row| {
                        match RowMappers::audiobook_from_row(row) {
                            Ok(audiobook) => Ok(audiobook),
                            Err(_) => Err(rusqlite::Error::InvalidColumnType(
                                0,
                                "mapping failed".to_string(),
                                rusqlite::types::Type::Null,
                            )),
                        }
                    })
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!("Failed to execute query: {e}"))
                    })?;

                match rows.next() {
                    Some(Ok(audiobook)) => Ok(Some(audiobook)),
                    Some(Err(e)) => Err(DatabaseError::execution_failed(&format!(
                        "Failed to process row: {e}"
                    ))),
                    None => Ok(None),
                }
            })
            .map_err(AppError::Database)
    }
    /// Deletes an audiobook from the database
    #[instrument(skip(self, path))]
    pub fn delete_audiobook(&self, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy().to_string();

        self.operations
            .execute(move |conn| {
                conn.execute("DELETE FROM audiobooks WHERE path = ?1", [&path_str])
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!("Failed to delete audiobook: {e}"))
                    })?;

                Ok(())
            })
            .map_err(AppError::Database)?;

        Ok(())
    }

    /// Deletes all audiobooks from a library
    #[instrument(skip(self, library_path))]
    pub fn delete_library_audiobooks(&self, library_path: &Path) -> Result<()> {
        let library_id = self.get_library_id(library_path)?;

        self.operations
            .execute(move |conn| {
                conn.execute(
                    "DELETE FROM audiobooks WHERE library_id = ?1",
                    [&library_id],
                )
                .map_err(|e| {
                    DatabaseError::execution_failed(&format!(
                        "Failed to delete library audiobooks: {e}"
                    ))
                })?;

                Ok(())
            })
            .map_err(AppError::Database)?;

        Ok(())
    }
    /// Gets a connection from the database connection pool
    ///
    /// # Errors
    ///
    /// Returns an error if unable to acquire a connection from the pool
    pub fn connect(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.pool
            .get()
            .map_err(|e| AppError::Database(DatabaseError::ConnectionFailed(e.to_string())))
    }
    /// Get the audiobook repository
    #[must_use]
    pub fn audiobook_repository(&self) -> AudiobookRepository {
        let config = ConnectionConfig {
            path: self.db_path.clone(),
            ..Default::default()
        };
        AudiobookRepository::new(Arc::new(EnhancedConnection::with_config(config)))
    }

    /// Get the library repository
    #[must_use]
    pub fn library_repository(&self) -> LibraryRepository {
        let config = ConnectionConfig {
            path: self.db_path.clone(),
            ..Default::default()
        };
        LibraryRepository::new(Arc::new(EnhancedConnection::with_config(config)))
    }

    /// Get the progress repository
    #[must_use]
    pub fn progress_repository(&self) -> ProgressRepository {
        let config = ConnectionConfig {
            path: self.db_path.clone(),
            ..Default::default()
        };
        ProgressRepository::new(Arc::new(EnhancedConnection::with_config(config)))
    }

    /// Opens a database at the specified path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = PoolConfig {
            path: path.as_ref().to_string_lossy().to_string(),
            ..Default::default()
        };

        Self::new(config)
    }

    /// Opens the centralized application database
    /// 
    /// This creates a single database file in the app's data directory,
    /// avoiding the need for separate databases per library.
    pub fn open_app_database() -> Result<Self> {
        let db_path = Self::get_app_database_path()?;
        
        // Ensure the parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::Config(format!("Failed to create database directory: {e}")))?;
        }
        
        info!("Using centralized database at: {}", db_path.display());
        Self::open(&db_path)
    }

    /// Gets the path to the centralized application database
    pub fn get_app_database_path() -> Result<PathBuf> {
        let mut path = dirs::data_dir()
            .ok_or_else(|| AppError::Config("Could not find data directory".to_string()))?;
        path.push("abop-iced");
        path.push("database.db");
        Ok(path)
    }

    /// Gets all libraries from the database
    #[instrument(skip(self))]
    pub fn get_libraries(&self) -> Result<Vec<Library>> {
        let repo = self.library_repository();
        repo.find_all().map_err(AppError::from)
    }

    /// Gets audiobooks in a specific library by library ID
    #[instrument(skip(self, library_id))]
    pub fn get_audiobooks_in_library(&self, library_id: &str) -> Result<Vec<Audiobook>> {
        let repo = self.audiobook_repository();
        repo.find_by_library(library_id)
    }

    /// Gets a library repository for more complex operations
    #[must_use]
    pub fn libraries(&self) -> LibraryRepository {
        self.library_repository()
    }

    /// Finds a library by its path
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails
    pub fn find_library_by_path<P: AsRef<Path>>(&self, path: P) -> Result<Option<Library>> {
        let repo = self.library_repository();
        repo.find_by_path(path).map_err(AppError::from)
    }

    /// Create a new library and add it to the database if it doesn't already exist
    ///
    /// If a library with the same path already exists, returns the ID of the existing library.
    /// Otherwise, creates a new library and returns its ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails
    #[instrument(skip(self))]
    pub fn add_library_with_path(&self, name: &str, path: PathBuf) -> Result<String> {
        // First check if a library with this path already exists
        if let Some(existing) = self.find_library_by_path(&path)? {
            debug!("Library already exists at path: {}", path.display());
            return Ok(existing.id);
        }

        // If not, create a new library
        let library = Library::new(name, path);
        self.add_library(&library)
    }
}
