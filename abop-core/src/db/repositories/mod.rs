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

/// Repository manager that provides access to all repositories
pub struct RepositoryManager {
    connection: Arc<Mutex<Connection>>,
    enhanced_connection: Option<Arc<EnhancedConnection>>,
    audiobook_repo: AudiobookRepository,
    library_repo: LibraryRepository,
    progress_repo: ProgressRepository,
}

impl RepositoryManager {
    /// Create a new repository manager with the given database connection
    #[must_use]
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self {
            audiobook_repo: AudiobookRepository::new(connection.clone()),
            library_repo: LibraryRepository::new(connection.clone()),
            progress_repo: ProgressRepository::new(connection.clone()),
            connection,
            enhanced_connection: None,
        }
    }

    /// Create a new repository manager with enhanced connection support
    pub fn with_enhanced_connection(
        connection: Arc<Mutex<Connection>>,
        enhanced_connection: Arc<EnhancedConnection>,
    ) -> Self {
        Self {
            audiobook_repo: AudiobookRepository::new(connection.clone()),
            library_repo: LibraryRepository::new(connection.clone()),
            progress_repo: ProgressRepository::new(connection.clone()),
            connection,
            enhanced_connection: Some(enhanced_connection),
        }
    }
    /// Get the audiobook repository
    #[must_use]
    pub const fn audiobooks(&self) -> &AudiobookRepository {
        &self.audiobook_repo
    }

    /// Get the library repository
    #[must_use]
    pub const fn libraries(&self) -> &LibraryRepository {
        &self.library_repo
    }

    /// Get the progress repository
    #[must_use]
    pub const fn progress(&self) -> &ProgressRepository {
        &self.progress_repo
    }

    /// Get access to the raw connection for complex operations
    #[must_use]
    pub const fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.connection
    }

    /// Get access to the enhanced connection if available
    #[must_use]
    pub const fn enhanced_connection(&self) -> Option<&Arc<EnhancedConnection>> {
        self.enhanced_connection.as_ref()
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

impl Clone for RepositoryManager {
    fn clone(&self) -> Self {
        Self {
            audiobook_repo: AudiobookRepository::new(self.connection.clone()),
            library_repo: LibraryRepository::new(self.connection.clone()),
            progress_repo: ProgressRepository::new(self.connection.clone()),
            connection: self.connection.clone(),
            enhanced_connection: self.enhanced_connection.clone(),
        }
    }
}
