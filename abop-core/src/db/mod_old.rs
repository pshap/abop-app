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
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rusqlite::{Connection, params, Result as SqliteResult};
use tracing::{debug, error, instrument, warn};

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
    models::{Audiobook, Library, Progress},
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
    pool: Pool,
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
        let mut pool_config = Config::new(Path::new(&config.path));
        pool_config.max_connections = config.max_connections;
        pool_config.create_if_missing = config.create_if_missing;

        let pool = pool_config
            .create_pool(Runtime::Tokio1)
            .map_err(|e| AppError::Database(e.into()))?;

        // Initialize database schema
        let conn = pool.get().await.map_err(|e| AppError::Database(e.into()))?;
        Self::init_schema(&conn).await?;

        Ok(Self {
            pool,
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
    async fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS libraries (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
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
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (library_id) REFERENCES libraries(id),
                UNIQUE(library_id, path)
            );
            CREATE INDEX IF NOT EXISTS idx_audiobooks_library_id ON audiobooks(library_id);
            CREATE INDEX IF NOT EXISTS idx_audiobooks_path ON audiobooks(path);
            CREATE INDEX IF NOT EXISTS idx_audiobooks_title ON audiobooks(title);
            CREATE INDEX IF NOT EXISTS idx_audiobooks_author ON audiobooks(author);",
        )
        .map_err(|e| AppError::Database(e))?;

        Ok(())
    }

    /// Adds a library to the database
    #[instrument(skip(self, library))]
    pub async fn add_library(&self, library: &Library) -> Result<String> {
        let conn = self.pool.get().await.map_err(|e| AppError::Database(e.into()))?;
        let id = uuid::Uuid::new_v4().to_string();
        
        conn.execute(
            "INSERT INTO libraries (id, name, path) VALUES (?1, ?2, ?3)",
            params![id, library.name, library.path.to_string_lossy()],
        )
        .map_err(|e| AppError::Database(e))?;
        
        // Update cache
        self.state.lock().await.library_cache.insert(library.path.to_string_lossy().to_string(), id.clone());
        
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

        let conn = self.pool.get().await.map_err(|e| AppError::Database(e.into()))?;
        
        let id: String = conn.query_row(
            "SELECT id FROM libraries WHERE path = ?1",
            params![path_str],
            |row| row.get(0),
        )
        .map_err(|e| AppError::Database(e))?;

        // Update cache
        self.state.lock().await.library_cache.insert(path_str, id.clone());
        
        Ok(id)
    }

    /// Adds an audiobook to the database
    #[instrument(skip(self, audiobook))]
    pub async fn add_audiobook(&self, audiobook: &Audiobook) -> Result<()> {
        let conn = self.pool.get().await.map_err(|e| AppError::Database(e.into()))?;
        let library_id = self.get_library_id(&audiobook.path.parent().unwrap()).await?;

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
        .map_err(|e| AppError::Database(e))?;

        Ok(())
    }

    /// Adds multiple audiobooks to the database in bulk
    #[instrument(skip(self, audiobooks))]
    pub async fn add_audiobooks_bulk(&self, audiobooks: &[Audiobook]) -> Result<()> {
        if audiobooks.is_empty() {
            return Ok(());
        }

        let conn = self.pool.get().await.map_err(|e| AppError::Database(e.into()))?;
        
        // Group audiobooks by library for efficient bulk insert
        let mut library_audiobooks = std::collections::HashMap::new();
        for audiobook in audiobooks {
            let library_path = audiobook.path.parent().unwrap();
            library_audiobooks
                .entry(library_path.to_path_buf())
                .or_insert_with(Vec::new)
                .push(audiobook);
        }

        // Process each library's audiobooks in bulk
        for (library_path, books) in library_audiobooks {
            let library_id = self.get_library_id(&library_path).await?;
            
            let tx = conn.transaction()?;
            
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
            )?;

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
                ])?;
            }

            tx.commit()?;
        }

        Ok(())
    }

    /// Gets all audiobooks from a library
    #[instrument(skip(self, library_path))]
    pub async fn get_audiobooks(&self, library_path: &Path) -> Result<Vec<Audiobook>> {
        let conn = self.pool.get().await.map_err(|e| AppError::Database(e.into()))?;
        let library_id = self.get_library_id(library_path).await?;

        let mut stmt = conn.prepare(
            "SELECT id, library_id, path, title, author, narrator, description, duration_seconds, size_bytes, created_at, updated_at
             FROM audiobooks
             WHERE library_id = ?1
             ORDER BY title",
        )?;

        let audiobooks = stmt.query_map(params![library_id], |row| {
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
                    .map_err(|_| rusqlite::Error::InvalidColumnType(9, "created_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(10, "updated_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&chrono::Utc),
                selected: false,
            })
        })?;

        let mut result = Vec::new();
        for audiobook in audiobooks {
            result.push(audiobook.map_err(|e| AppError::Database(e))?);
        }

        Ok(result)
    }

    /// Gets a single audiobook by path
    #[instrument(skip(self, path))]
    pub async fn get_audiobook(&self, path: &Path) -> Result<Option<Audiobook>> {
        let conn = self.pool.get().await.map_err(|e| AppError::Database(e.into()))?;
        
        let mut stmt = conn.prepare(
            "SELECT id, library_id, path, title, author, narrator, description, duration_seconds, size_bytes, created_at, updated_at
             FROM audiobooks
             WHERE path = ?1",
        )?;

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
                    .map_err(|_| rusqlite::Error::InvalidColumnType(9, "created_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(10, "updated_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&chrono::Utc),
                selected: false,
            })
        });

        match audiobook {
            Ok(audiobook) => Ok(Some(audiobook)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Deletes an audiobook from the database
    #[instrument(skip(self, path))]
    pub async fn delete_audiobook(&self, path: &Path) -> Result<()> {
        let conn = self.pool.get().await.map_err(|e| AppError::Database(e.into()))?;
        
        conn.execute(
            "DELETE FROM audiobooks WHERE path = ?1",
            params![path.to_string_lossy()],
        )
        .map_err(|e| AppError::Database(e))?;

        Ok(())
    }

    /// Deletes all audiobooks from a library
    #[instrument(skip(self, library_path))]
    pub async fn delete_library_audiobooks(&self, library_path: &Path) -> Result<()> {
        let conn = self.pool.get().await.map_err(|e| AppError::Database(e.into()))?;
        let library_id = self.get_library_id(library_path).await?;
        
        conn.execute(
            "DELETE FROM audiobooks WHERE library_id = ?1",
            params![library_id],
        )
        .map_err(|e| AppError::Database(e))?;

        Ok(())
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

        // Create a library first
        let library = db.add_library("Test Library", temp_path.clone())?;
        assert!(db.libraries().exists(&library.id)?);

        // Test that database operations work correctly with the enhanced connection
        // This verifies that the retry logic is properly integrated and the connection
        // can handle normal operations successfully
        
        // Test a read operation
        let retrieved_library = db.get_library(&library.id)?;
        assert!(retrieved_library.is_some());
        assert_eq!(retrieved_library.unwrap().name, "Test Library");

        // Test connection health monitoring
        let health = db.connection_health();
        assert_eq!(health, crate::db::ConnectionHealth::Healthy);

        // Test connection statistics are being tracked
        let stats = db.connection_stats()?;
        assert!(stats.successful_operations > 0);

        Ok(())
    }
}
