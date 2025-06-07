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
use super::connection_adapter::ConnectionAdapter;
use super::error::{DatabaseError, DbResult};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

/// Base repository trait for common functionality
pub trait Repository {
    /// Get a reference to the database connection
    fn connection(&self) -> &Arc<Mutex<Connection>>;

    /// Get a reference to the connection adapter if available
    fn connection_adapter(&self) -> Option<&ConnectionAdapter> {
        None
    }
    
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
        // Default implementation uses direct connection
        // Individual repositories can override this to use enhanced connection
        let conn = self.connection().lock().unwrap();
        f(&conn).map_err(DatabaseError::from)
    }
    
    /// Execute a query with enhanced connection if available
    /// This method should be overridden by repositories that have access to enhanced connection
    fn execute_query_enhanced<F, R>(&self, f: F) -> DbResult<R>
    where
        F: FnOnce(&Connection) -> Result<R, rusqlite::Error>,
    {
        // Default implementation falls back to regular execute_query
        self.execute_query(f)
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
    connection_adapter: Option<ConnectionAdapter>,
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
            connection_adapter: None,
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
            connection_adapter: None,
            enhanced_connection: Some(enhanced_connection),
        }
    }

    /// Create a new repository manager with connection adapter
    pub fn with_connection_adapter(
        connection: Arc<Mutex<Connection>>,
        connection_adapter: ConnectionAdapter,
    ) -> Self {
        Self {
            audiobook_repo: AudiobookRepository::new(connection.clone()),
            library_repo: LibraryRepository::new(connection.clone()),
            progress_repo: ProgressRepository::new(connection.clone()),
            connection,
            connection_adapter: Some(connection_adapter),
            enhanced_connection: None,
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

    /// Get access to the connection adapter if available
    #[must_use]
    pub const fn connection_adapter(&self) -> Option<&ConnectionAdapter> {
        self.connection_adapter.as_ref()
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
    }    /// Execute a query using the appropriate connection method
    /// This method handles the logic of choosing between adapter and direct connection
    pub fn execute_repository_query<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&Connection) -> Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static,
    {
        // If we have an enhanced connection, use it instead of the dummy connection
        if let Some(enhanced_conn) = &self.enhanced_connection {
            // Use the enhanced connection's with_connection method which handles retry logic
            enhanced_conn.with_connection(move |conn| {
                f(conn).map_err(DatabaseError::from)
            })
        } else if let Some(_adapter) = &self.connection_adapter {
            // Connection adapter path - for now fall back to direct connection
            let conn = self.connection.lock().unwrap();
            f(&conn).map_err(DatabaseError::from)
        } else {
            // Direct connection path
            let conn = self.connection.lock().unwrap();
            f(&conn).map_err(DatabaseError::from)
        }
    }    /// Get an enhanced wrapper for the audiobook repository
    #[must_use]
    pub fn audiobooks_enhanced(&self) -> EnhancedRepositoryWrapper<'_, AudiobookRepository> {
        EnhancedRepositoryWrapper {
            repo: &self.audiobook_repo,
            manager: self,
        }
    }

    /// Get an enhanced wrapper for the library repository
    #[must_use]
    pub fn libraries_enhanced(&self) -> EnhancedRepositoryWrapper<'_, LibraryRepository> {
        EnhancedRepositoryWrapper {
            repo: &self.library_repo,
            manager: self,
        }
    }

    /// Get an enhanced wrapper for the progress repository
    #[must_use]
    pub fn progress_enhanced(&self) -> EnhancedRepositoryWrapper<'_, ProgressRepository> {
        EnhancedRepositoryWrapper {
            repo: &self.progress_repo,
            manager: self,
        }
    }
}

impl Clone for RepositoryManager {
    fn clone(&self) -> Self {
        Self {
            audiobook_repo: AudiobookRepository::new(self.connection.clone()),
            library_repo: LibraryRepository::new(self.connection.clone()),
            progress_repo: ProgressRepository::new(self.connection.clone()),
            connection: self.connection.clone(),
            connection_adapter: self.connection_adapter.clone(),
            enhanced_connection: self.enhanced_connection.clone(),
        }
    }
}

/// Enhanced repository wrapper that provides access to repository manager's enhanced connection
pub struct EnhancedRepositoryWrapper<'a, T: Repository> {
    repo: &'a T,
    manager: &'a RepositoryManager,
}

impl<'a, T: Repository> EnhancedRepositoryWrapper<'a, T> {    /// Execute a query using the enhanced connection if available, otherwise fall back to the repository's own connection
    pub fn execute_query<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&Connection) -> Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static,
    {
        // Use manager's enhanced connection if available
        self.manager.execute_repository_query(f)
    }
}

impl<'a, T: Repository> std::ops::Deref for EnhancedRepositoryWrapper<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.repo
    }
}
