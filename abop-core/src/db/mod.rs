//! Database module for ABOP
//!
//! This module provides database functionality for storing and retrieving
//! audiobook metadata and library information.

pub mod connection;
pub mod connection_adapter;
pub mod datetime_serde;
pub mod error;
pub mod health;
mod migrations;
mod queries;
pub mod repositories;
pub mod retry;
pub mod statistics;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, params};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, instrument};

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

/// Database connection with connection pooling
#[derive(Debug, Clone)]
pub struct Database {
    /// Connection pool
    pool: Arc<r2d2::Pool<SqliteConnectionManager>>,
    /// Mutex for thread-safe access to shared state
    state: Arc<Mutex<DatabaseState>>,
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
    pub async fn new(config: PoolConfig) -> Result<Self> {
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

        Ok(Self {
            pool: Arc::new(pool),
            state: Arc::new(Mutex::new(DatabaseState::default())),
        })
    }

    /// Creates a new in-memory database for testing
    #[must_use]
    pub fn in_memory() -> Result<Self> {
        let config = PoolConfig {
            path: ":memory:".to_string(),
            ..Default::default()
        };

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { Self::new(config).await })
    }
    /// Initializes the database schema
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
    pub async fn add_library(&self, library: &Library) -> Result<String> {
        let conn = self.pool.get().map_err(|e| {
            AppError::Database(DatabaseError::ConnectionFailed(format!(
                "Failed to get connection from pool: {e}"
            )))
        })?;
        let id = uuid::Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO libraries (id, name, path) VALUES (?1, ?2, ?3)",
            params![id, library.name, library.path.to_string_lossy()],
        )
        .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        // Update cache
        self.state
            .lock()
            .await
            .library_cache
            .insert(library.path.to_string_lossy().to_string(), id.clone());

        Ok(id)
    }
    /// Gets a library ID from the database, using cache if available
    #[instrument(skip(self, path))]
    async fn get_library_id(&self, path: &Path) -> Result<String> {
        let path_str = path.to_string_lossy().to_string();

        // Check cache first
        if let Some(id) = self.state.lock().await.library_cache.get(&path_str) {
            return Ok(id.clone());
        }

        let conn = self.pool.get().map_err(|e| {
            AppError::Database(DatabaseError::ConnectionFailed(format!(
                "Failed to get connection from pool: {e}"
            )))
        })?;

        let id: String = conn
            .query_row(
                "SELECT id FROM libraries WHERE path = ?1",
                params![path_str],
                |row| row.get(0),
            )
            .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        // Update cache
        self.state
            .lock()
            .await
            .library_cache
            .insert(path_str, id.clone());

        Ok(id)
    }
    /// Adds an audiobook to the database
    #[instrument(skip(self, audiobook))]
    pub async fn add_audiobook(&self, audiobook: &Audiobook) -> Result<()> {
        let conn = self.pool.get().map_err(|e| {
            AppError::Database(DatabaseError::ConnectionFailed(format!(
                "Failed to get connection from pool: {e}"
            )))
        })?;
        let library_id = self
            .get_library_id(audiobook.path.parent().unwrap())
            .await?;

        conn.execute(
            "INSERT INTO audiobooks (id, library_id, path, title, author, narrator, description, duration_seconds, size_bytes, cover_art, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
             ON CONFLICT(library_id, path) DO UPDATE SET
                title = excluded.title,
                author = excluded.author,
                narrator = excluded.narrator,
                description = excluded.description,
                duration_seconds = excluded.duration_seconds,
                size_bytes = excluded.size_bytes,
                cover_art = excluded.cover_art,
                updated_at = excluded.updated_at",
            params![
                audiobook.id,
                library_id,
                audiobook.path.to_string_lossy(),
                audiobook.title,
                audiobook.author,
                audiobook.narrator,
                audiobook.description,
                audiobook.duration_seconds.map(|d| d as i64),
                audiobook.size_bytes.map(|s| s as i64),
                audiobook.cover_art,
                audiobook.created_at.to_rfc3339(),
                audiobook.updated_at.to_rfc3339(),
            ],
        )
        .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        Ok(())
    }
    /// Adds multiple audiobooks to the database in bulk
    #[instrument(skip(self, audiobooks))]
    pub async fn add_audiobooks_bulk(&self, audiobooks: &[Audiobook]) -> Result<()> {
        if audiobooks.is_empty() {
            return Ok(());
        }

        // Group audiobooks by library for efficient bulk insert
        let mut library_audiobooks = std::collections::HashMap::new();
        for audiobook in audiobooks {
            let library_path = audiobook.path.parent().unwrap();
            library_audiobooks
                .entry(library_path.to_path_buf())
                .or_insert_with(Vec::new)
                .push(audiobook);
        } // Process each library's audiobooks in bulk
        for (library_path, books) in library_audiobooks {
            let library_id = self.get_library_id(&library_path).await?;

            let mut conn = self.pool.get().map_err(|e| {
                AppError::Database(DatabaseError::ConnectionFailed(format!(
                    "Failed to get connection from pool: {e}"
                )))
            })?;
            let tx = conn
                .transaction()
                .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

            let mut stmt = tx.prepare(
                "INSERT INTO audiobooks (id, library_id, path, title, author, narrator, description, duration_seconds, size_bytes, cover_art, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
                 ON CONFLICT(library_id, path) DO UPDATE SET
                    title = excluded.title,
                    author = excluded.author,
                    narrator = excluded.narrator,
                    description = excluded.description,
                    duration_seconds = excluded.duration_seconds,
                    size_bytes = excluded.size_bytes,
                    cover_art = excluded.cover_art,
                    updated_at = excluded.updated_at",
            ).map_err(|e| AppError::Database(DatabaseError::from(e)))?;

            for audiobook in books {
                stmt.execute(params![
                    audiobook.id,
                    library_id,
                    audiobook.path.to_string_lossy(),
                    audiobook.title,
                    audiobook.author,
                    audiobook.narrator,
                    audiobook.description,
                    audiobook.duration_seconds.map(|d| d as i64),
                    audiobook.size_bytes.map(|s| s as i64),
                    audiobook.cover_art,
                    audiobook.created_at.to_rfc3339(),
                    audiobook.updated_at.to_rfc3339(),
                ])
                .map_err(|e| AppError::Database(DatabaseError::from(e)))?;
            }

            drop(stmt);
            tx.commit()
                .map_err(|e| AppError::Database(DatabaseError::from(e)))?;
        }

        Ok(())
    }
    /// Gets all audiobooks from a library
    #[instrument(skip(self, library_path))]
    pub async fn get_audiobooks(&self, library_path: &Path) -> Result<Vec<Audiobook>> {
        let conn = self.pool.get().map_err(|e| {
            AppError::Database(DatabaseError::ConnectionFailed(format!(
                "Failed to get connection from pool: {e}"
            )))
        })?;
        let library_id = self.get_library_id(library_path).await?;

        let mut stmt = conn.prepare(
            "SELECT id, library_id, path, title, author, narrator, description, duration_seconds, size_bytes, created_at, updated_at
             FROM audiobooks
             WHERE library_id = ?1
             ORDER BY title",
        ).map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        let audiobooks = stmt
            .query_map(params![library_id], |row| {
                Ok(Audiobook {
                    id: row.get::<_, String>(0)?,
                    library_id: row.get::<_, String>(1)?,
                    path: PathBuf::from(row.get::<_, String>(2)?),
                    title: row.get(3)?,
                    author: row.get(4)?,
                    narrator: row.get(5)?,
                    description: row.get(6)?,
                    duration_seconds: row.get::<_, Option<i64>>(7)?.map(|d| d as u64),
                    size_bytes: row.get::<_, Option<i64>>(8)?.map(|s| s as u64),
                    cover_art: None, // Cover art not selected for performance
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                        .map_err(|_| {
                            rusqlite::Error::InvalidColumnType(
                                9,
                                "created_at".to_string(),
                                rusqlite::types::Type::Text,
                            )
                        })?
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                        .map_err(|_| {
                            rusqlite::Error::InvalidColumnType(
                                10,
                                "updated_at".to_string(),
                                rusqlite::types::Type::Text,
                            )
                        })?
                        .with_timezone(&chrono::Utc),
                    selected: false,
                })
            })
            .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        let mut result = Vec::new();
        for audiobook in audiobooks {
            result.push(audiobook.map_err(|e| AppError::Database(DatabaseError::from(e)))?);
        }

        Ok(result)
    }
    /// Gets a single audiobook by path
    #[instrument(skip(self, path))]
    pub async fn get_audiobook(&self, path: &Path) -> Result<Option<Audiobook>> {
        let conn = self.pool.get().map_err(|e| {
            AppError::Database(DatabaseError::ConnectionFailed(format!(
                "Failed to get connection from pool: {e}"
            )))
        })?;

        let mut stmt = conn.prepare(
            "SELECT id, library_id, path, title, author, narrator, description, duration_seconds, size_bytes, created_at, updated_at
             FROM audiobooks
             WHERE path = ?1",
        ).map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        let audiobook = stmt.query_row(params![path.to_string_lossy()], |row| {
            Ok(Audiobook {
                id: row.get::<_, String>(0)?,
                library_id: row.get::<_, String>(1)?,
                path: PathBuf::from(row.get::<_, String>(2)?),
                title: row.get(3)?,
                author: row.get(4)?,
                narrator: row.get(5)?,
                description: row.get(6)?,
                duration_seconds: row.get::<_, Option<i64>>(7)?.map(|d| d as u64),
                size_bytes: row.get::<_, Option<i64>>(8)?.map(|s| s as u64),
                cover_art: None, // Cover art not selected for performance
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .map_err(|_| {
                        rusqlite::Error::InvalidColumnType(
                            9,
                            "created_at".to_string(),
                            rusqlite::types::Type::Text,
                        )
                    })?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .map_err(|_| {
                        rusqlite::Error::InvalidColumnType(
                            10,
                            "updated_at".to_string(),
                            rusqlite::types::Type::Text,
                        )
                    })?
                    .with_timezone(&chrono::Utc),
                selected: false,
            })
        });

        match audiobook {
            Ok(audiobook) => Ok(Some(audiobook)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(DatabaseError::from(e))),
        }
    }
    /// Deletes an audiobook from the database
    #[instrument(skip(self, path))]
    pub async fn delete_audiobook(&self, path: &Path) -> Result<()> {
        let conn = self.pool.get().map_err(|e| {
            AppError::Database(DatabaseError::ConnectionFailed(format!(
                "Failed to get connection from pool: {e}"
            )))
        })?;

        conn.execute(
            "DELETE FROM audiobooks WHERE path = ?1",
            params![path.to_string_lossy()],
        )
        .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

        Ok(())
    }

    /// Deletes all audiobooks from a library
    #[instrument(skip(self, library_path))]
    pub async fn delete_library_audiobooks(&self, library_path: &Path) -> Result<()> {
        let conn = self.pool.get().map_err(|e| {
            AppError::Database(DatabaseError::ConnectionFailed(format!(
                "Failed to get connection from pool: {e}"
            )))
        })?;
        let library_id = self.get_library_id(library_path).await?;

        conn.execute(
            "DELETE FROM audiobooks WHERE library_id = ?1",
            params![library_id],
        )
        .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

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
        let _conn = self.pool.get().expect("Failed to get connection from pool");
        AudiobookRepository::new(Arc::new(EnhancedConnection::with_config(
            ConnectionConfig::default(),
        )))
    }
    /// Get the library repository
    #[must_use]
    pub fn library_repository(&self) -> LibraryRepository {
        let _conn = self.pool.get().expect("Failed to get connection from pool");
        LibraryRepository::new(Arc::new(EnhancedConnection::with_config(
            ConnectionConfig::default(),
        )))
    }

    /// Get the progress repository
    #[must_use]
    pub fn progress_repository(&self) -> ProgressRepository {
        let _conn = self.pool.get().expect("Failed to get connection from pool");
        ProgressRepository::new(Arc::new(EnhancedConnection::with_config(
            ConnectionConfig::default(),
        )))
    }

    /// Opens a database at the specified path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = PoolConfig {
            path: path.as_ref().to_string_lossy().to_string(),
            ..Default::default()
        };

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { Self::new(config).await })
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

    /// Create a new library and add it to the database
    #[instrument(skip(self))]
    pub async fn add_library_with_path(&self, name: &str, path: PathBuf) -> Result<String> {
        let library = Library::new(name, path);
        self.add_library(&library).await
    }
}
