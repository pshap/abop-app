//! Repository pattern implementation for database operations
//!
//! This module provides a structured approach to database operations
//! using the repository pattern for better organization and testability.

pub mod audiobook;
pub mod library;
pub mod progress;

pub use audiobook::AudiobookRepository;
pub use library::LibraryRepository;
pub use progress::ProgressRepository;

use super::connection::EnhancedConnection;
use super::error::{DatabaseError, DbResult};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

/// Base repository trait for common functionality
pub trait Repository {
    /// Get a reference to the database connection
    fn connection(&self) -> &Arc<Mutex<Connection>>;

    /// Execute a query with proper error handling
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database connection cannot be acquired
    /// - The query execution fails
    fn execute_query<F, R>(&self, f: F) -> DbResult<R>
    where
        F: FnOnce(&Connection) -> Result<R, rusqlite::Error>,
    {
        let conn = self.connection().lock().unwrap();
        f(&conn).map_err(DatabaseError::from)
    }
}

/// Enhanced repository trait for repositories that can leverage enhanced connection features
pub trait EnhancedRepository: Repository {
    /// Get access to enhanced connection if available through the repository manager
    fn get_enhanced_connection(&self) -> Option<&Arc<EnhancedConnection>> {
        None // Default implementation returns None
    }
}

/// Manager for all repositories
#[derive(Debug, Clone)]
pub struct RepositoryManager {
    /// Audiobook repository
    audiobook: AudiobookRepository,
    /// Library repository
    library: LibraryRepository,
    /// Progress repository
    progress: ProgressRepository,
    /// Shared connection
    connection: Arc<Mutex<Connection>>,
}

impl RepositoryManager {
    /// Create a new repository manager
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self {
            audiobook: AudiobookRepository::new(connection.clone()),
            library: LibraryRepository::new(connection.clone()),
            progress: ProgressRepository::new(connection.clone()),
            connection,
        }
    }

    /// Get the audiobook repository
    pub fn audiobook(&self) -> &AudiobookRepository {
        &self.audiobook
    }

    /// Get the library repository
    pub fn library(&self) -> &LibraryRepository {
        &self.library
    }

    /// Get the progress repository
    pub fn progress(&self) -> &ProgressRepository {
        &self.progress
    }

    /// Get the shared connection
    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.connection
    }

    /// Execute a transaction across multiple repositories
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Transaction creation fails
    /// - The transaction function fails
    /// - Transaction commit fails
    ///
    /// # Panics
    ///
    /// Panics if the internal connection mutex is poisoned.
    #[allow(clippy::significant_drop_tightening)]
    pub fn with_transaction<F, R>(&self, f: F) -> DbResult<R>
    where
        F: FnOnce(&rusqlite::Transaction) -> DbResult<R>,
    {
        let mut conn_lock = self.connection.lock().unwrap();
        let tx = conn_lock.transaction().map_err(DatabaseError::from)?;

        match f(&tx) {
            Ok(result) => {
                tx.commit().map_err(DatabaseError::from)?;
                Ok(result)
            }
            Err(e) => {
                let _ = tx.rollback(); // Ignore rollback errors
                Err(e)
            }
        }
    }

    /// Execute a transaction with enhanced connection features if available
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Transaction creation fails
    /// - The transaction function fails
    /// - Transaction commit fails
    pub fn with_enhanced_transaction<F, R>(&self, f: F) -> DbResult<R>
    where
        F: FnOnce(&rusqlite::Transaction) -> DbResult<R>,
    {
        // For now, enhanced transactions use the same approach as regular transactions
        // The enhanced connection monitoring is applied at a higher level
        self.with_transaction(f)
    }
}

