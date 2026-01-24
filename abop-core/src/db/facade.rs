//! Core Database facade: struct, constructors, connection, repositories, and state.
//!
//! This thin facade centralizes pooling and repository factories. Higher-level
//! conveniences live in `convenience`, legacy path-based methods in `legacy_crud`,
//! app-specific utilities in `app_database`, and bootstrap SQL in `schema`.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tracing::{debug, instrument};

use crate::error::{AppError, Result};

use super::{
    connection::{ConnectionConfig, EnhancedConnection},
    operations::DatabaseOperations,
    repositories::{AudiobookRepository, LibraryRepository, ProgressRepository},
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
    pub(crate) pool: Arc<r2d2::Pool<SqliteConnectionManager>>,
    /// High-level database operations
    pub(crate) operations: DatabaseOperations,
    /// Mutex for thread-safe access to shared state
    pub(crate) state: Arc<Mutex<DatabaseState>>,
    /// Database file path for repository creation
    pub(crate) db_path: PathBuf,
}

/// Shared state for database operations
#[derive(Debug, Default)]
pub(crate) struct DatabaseState {
    /// Cache of library IDs
    pub(crate) library_cache: std::collections::HashMap<String, String>,
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
                AppError::Database(super::error::DatabaseError::ConnectionFailed(format!(
                    "Failed to create connection pool: {e}"
                )))
            })?;

        let mut conn = pool.get().map_err(|e| {
            AppError::Database(super::error::DatabaseError::ConnectionFailed(format!(
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
        .map_err(|e| AppError::Database(super::error::DatabaseError::from(e)))?;

        // Run migrations on the actual database connection
        super::migrations::run_migrations(&mut conn).map_err(AppError::Database)?;

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

    /// Gets a connection from the database connection pool
    ///
    /// # Errors
    ///
    /// Returns an error if unable to acquire a connection from the pool
    pub fn connect(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.pool.get().map_err(|e| {
            AppError::Database(super::error::DatabaseError::ConnectionFailed(e.to_string()))
        })
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
}
