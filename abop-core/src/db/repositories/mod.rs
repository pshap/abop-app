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
use std::sync::Arc;

/// Base repository trait for common functionality
pub trait Repository {
    /// Execute a query with proper error handling
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database connection cannot be acquired
    /// - The query execution fails
    fn execute_query<F, R>(&self, f: F) -> DbResult<R>
    where
        F: FnOnce(&Connection) -> Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static,
    {
        // Default implementation uses enhanced connection through repository manager
        // Individual repositories should override this to use their manager's enhanced connection
        self.execute_query_enhanced(f)
    }

    /// Execute a query with enhanced connection
    /// This method must be implemented by repositories to use the enhanced connection
    fn execute_query_enhanced<F, R>(&self, f: F) -> DbResult<R>
    where
        F: FnOnce(&Connection) -> Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static;
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
    enhanced_connection: Arc<EnhancedConnection>,
    audiobook_repo: AudiobookRepository,
    library_repo: LibraryRepository,
    progress_repo: ProgressRepository,
}

impl RepositoryManager {
    /// Create a new repository manager with enhanced connection support
    /// This is the preferred method for creating repositories that use the enhanced connection
    pub fn with_enhanced_connection(enhanced_connection: Arc<EnhancedConnection>) -> Self {
        Self {
            audiobook_repo: AudiobookRepository::new(enhanced_connection.clone()),
            library_repo: LibraryRepository::new(enhanced_connection.clone()),
            progress_repo: ProgressRepository::new(enhanced_connection.clone()),
            enhanced_connection,
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

    /// Get access to the enhanced connection
    #[must_use]
    pub const fn enhanced_connection(&self) -> &Arc<EnhancedConnection> {
        &self.enhanced_connection
    }

    /// Execute a query using the enhanced connection
    pub fn execute_repository_query<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&Connection) -> Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static,
    {
        self.enhanced_connection.with_connection(move |conn| f(conn).map_err(DatabaseError::from))
    }

    /// Execute a transaction with enhanced connection features
    pub fn with_enhanced_transaction<F, R>(&self, f: F) -> DbResult<R>
    where
        F: FnOnce(&rusqlite::Transaction) -> DbResult<R> + Send + 'static,
        R: Send + 'static,
    {
        // Use the same approach as the repository implementations
        let operation = std::sync::Arc::new(std::sync::Mutex::new(Some(f)));
        self.enhanced_connection.with_connection_mut(move |conn| {
            let tx = conn.transaction().map_err(DatabaseError::from)?;
            let mut op_guard = operation.lock().map_err(|_| {
                DatabaseError::ConnectionFailed("Failed to acquire operation lock".to_string())
            })?;
            let op = op_guard.take().ok_or_else(|| {
                DatabaseError::ConnectionFailed("Operation already consumed".to_string())
            })?;
            match op(&tx) {
                Ok(result) => {
                    tx.commit().map_err(DatabaseError::from)?;
                    Ok(result)
                }
                Err(e) => {
                    let _ = tx.rollback(); // Ignore rollback errors
                    Err(e)
                }
            }
        })
    }
}

impl Clone for RepositoryManager {
    fn clone(&self) -> Self {
        Self {
            audiobook_repo: AudiobookRepository::new(self.enhanced_connection.clone()),
            library_repo: LibraryRepository::new(self.enhanced_connection.clone()),
            progress_repo: ProgressRepository::new(self.enhanced_connection.clone()),
            enhanced_connection: self.enhanced_connection.clone(),
        }
    }
}

/// Enhanced repository wrapper that provides access to repository manager's enhanced connection
pub struct EnhancedRepositoryWrapper<'a, T: Repository> {
    repo: &'a T,
    manager: &'a RepositoryManager,
}

impl<'a, T: Repository> EnhancedRepositoryWrapper<'a, T> {
    /// Execute a query using the enhanced connection if available, otherwise fall back to the repository's own connection
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
