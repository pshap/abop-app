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
use std::any::Any;
use std::sync::Arc;

/// Type alias for row processing callback to reduce complexity
/// 
/// # Safety
/// 
/// The callback returns `Box<dyn Any + Send>` which requires careful type casting
/// by the caller. This is an unsafe pattern that should be avoided in favor of
/// typed alternatives when possible. Consider using specific repository methods
/// with concrete return types instead of this dynamic callback approach.
type RowCallback =
    Box<dyn FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<Box<dyn Any + Send>> + Send>;

/// Base repository trait with non-generic methods
pub trait RepositoryBase: Send + Sync + 'static {
    /// Get the enhanced connection
    fn connect(&self) -> &Arc<EnhancedConnection>;

    /// Get a dyn-compatible reference to this repository
    fn as_dyn(&self) -> &dyn DynRepository
    where
        Self: Sized,
    {
        self
    }
}

/// Repository operations with generic methods
/// This trait is not object-safe due to generic methods
pub trait Repository: RepositoryBase {
    /// Execute a query with proper error handling
    fn execute_query<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&Connection) -> std::result::Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static;

    /// Execute a query with enhanced connection
    fn execute_query_enhanced<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&EnhancedConnection) -> std::result::Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static;

    /// Execute a transaction with enhanced connection features
    fn execute_transaction<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&rusqlite::Transaction) -> std::result::Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static;
}

/// Object-safe repository trait for dynamic dispatch
pub trait DynRepository: RepositoryBase + Send + Sync + 'static {
    /// Execute a query with proper error handling (dynamic version)
    fn execute_query_dyn(
        &self,
        query: &str,
        params: &[&(dyn rusqlite::ToSql + Sync)],
    ) -> DbResult<usize> {
        // Clone the query string to own it
        let query = query.to_string();

        // Convert parameters to a vector of owned values
        let params: Vec<rusqlite::types::Value> = params
            .iter()
            .filter_map(|p| match p.to_sql().ok()? {
                rusqlite::types::ToSqlOutput::Borrowed(v) => match v {
                    rusqlite::types::ValueRef::Null => Some(rusqlite::types::Value::Null),
                    rusqlite::types::ValueRef::Integer(i) => {
                        Some(rusqlite::types::Value::Integer(i))
                    }
                    rusqlite::types::ValueRef::Real(f) => Some(rusqlite::types::Value::Real(f)),
                    rusqlite::types::ValueRef::Text(s) => Some(rusqlite::types::Value::Text(
                        String::from_utf8_lossy(s).into_owned(),
                    )),
                    rusqlite::types::ValueRef::Blob(b) => {
                        Some(rusqlite::types::Value::Blob(b.to_vec()))
                    }
                },
                rusqlite::types::ToSqlOutput::Owned(v) => Some(v),
                _ => None,
            })
            .collect();

        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(&query)?;
            // Convert the parameters to references for the query
            let param_refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
            stmt.execute(rusqlite::params_from_iter(param_refs))
        })
    }

    /// Execute a query that returns a single row (dynamic version)
    fn query_row_dyn(
        &self,
        query: &str,
        params: &[&(dyn rusqlite::ToSql + Sync)],
        callback: RowCallback,
    ) -> DbResult<Box<dyn Any + Send>>;
}

// Default implementation for Repository methods
impl<T: RepositoryBase + ?Sized> Repository for T {
    fn execute_query<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&Connection) -> std::result::Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static,
    {
        self.connect()
            .with_connection(move |conn| f(conn).map_err(DatabaseError::from))
    }

    fn execute_query_enhanced<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&EnhancedConnection) -> std::result::Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static,
    {
        let config = self.connect().config().clone();
        self.connect().with_connection(move |_| {
            let enhanced = EnhancedConnection::with_config(config.clone());
            f(&enhanced).map_err(DatabaseError::from)
        })
    }

    fn execute_transaction<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&rusqlite::Transaction) -> std::result::Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static,
    {
        self.connect().with_connection_mut(move |conn| {
            let tx = conn.transaction().map_err(DatabaseError::from)?;
            let result = f(&tx).map_err(DatabaseError::from);
            match result {
                Ok(r) => {
                    tx.commit().map_err(DatabaseError::from)?;
                    Ok(r)
                }
                Err(e) => {
                    let _ = tx.rollback(); // Ignore rollback errors
                    Err(e)
                }
            }
        })
    }
}

// Implement DynRepository for all repository types
impl<T: RepositoryBase + ?Sized> DynRepository for T {
    fn execute_query_dyn(
        &self,
        query: &str,
        params: &[&(dyn rusqlite::ToSql + Sync)],
    ) -> DbResult<usize> {
        // Clone the query string to own it
        let query = query.to_string();

        // Convert parameters to a vector of owned values
        let params: Vec<rusqlite::types::Value> = params
            .iter()
            .filter_map(|p| {
                match p.to_sql().ok()? {
                    rusqlite::types::ToSqlOutput::Borrowed(v) => match v {
                        rusqlite::types::ValueRef::Null => Some(rusqlite::types::Value::Null),
                        rusqlite::types::ValueRef::Integer(i) => {
                            Some(rusqlite::types::Value::Integer(i))
                        }
                        rusqlite::types::ValueRef::Real(f) => Some(rusqlite::types::Value::Real(f)),
                        rusqlite::types::ValueRef::Text(s) => Some(rusqlite::types::Value::Text(
                            String::from_utf8_lossy(s).into_owned(),
                        )),
                        rusqlite::types::ValueRef::Blob(b) => {
                            Some(rusqlite::types::Value::Blob(b.to_vec()))
                        }
                    },
                    rusqlite::types::ToSqlOutput::Owned(v) => Some(v),
                    // Handle any future variants of ToSqlOutput
                    _ => None,
                }
            })
            .collect();

        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(&query)?;
            // Convert the parameters back to references for the query
            let param_refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
            stmt.execute(rusqlite::params_from_iter(param_refs))
        })
    }

    fn query_row_dyn(
        &self,
        query: &str,
        params: &[&(dyn rusqlite::ToSql + Sync)],
        callback: RowCallback,
    ) -> DbResult<Box<dyn Any + Send>> {
        // Clone the query string to own it
        let query = query.to_string();

        // Convert parameters to a vector of owned values
        let params: Vec<rusqlite::types::Value> = params
            .iter()
            .filter_map(|p| match p.to_sql().ok()? {
                rusqlite::types::ToSqlOutput::Borrowed(v) => match v {
                    rusqlite::types::ValueRef::Null => Some(rusqlite::types::Value::Null),
                    rusqlite::types::ValueRef::Integer(i) => {
                        Some(rusqlite::types::Value::Integer(i))
                    }
                    rusqlite::types::ValueRef::Real(f) => Some(rusqlite::types::Value::Real(f)),
                    rusqlite::types::ValueRef::Text(s) => Some(rusqlite::types::Value::Text(
                        String::from_utf8_lossy(s).into_owned(),
                    )),
                    rusqlite::types::ValueRef::Blob(b) => {
                        Some(rusqlite::types::Value::Blob(b.to_vec()))
                    }
                },
                rusqlite::types::ToSqlOutput::Owned(v) => Some(v),
                _ => None,
            })
            .collect();

        // Use Cell to allow interior mutability for single-use callback
        use std::cell::Cell;
        let callback_option = Cell::new(Some(callback));

        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(&query)?;

            // Convert the parameters to references for the query
            let param_refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();

            // Execute the query and call callback
            stmt.query_row(rusqlite::params_from_iter(param_refs), |row| {
                // Take the callback from the Cell (only works once)
                let callback = callback_option.take().ok_or_else(|| {
                    rusqlite::Error::SqliteFailure(
                        rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                        Some("Callback already consumed".to_string())
                    )
                })?;
                
                callback(row)
            })
        })
    }
}

/// Enhanced repository trait for repositories that can leverage enhanced connection features
pub trait EnhancedRepository: Repository {
    /// Get access to enhanced connection
    fn get_enhanced_connection(&self) -> &Arc<EnhancedConnection> {
        self.connect()
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
    #[must_use]
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
        F: Fn(&Connection) -> rusqlite::Result<R> + Send + 'static,
        R: Send + 'static,
    {
        self.enhanced_connection
            .with_connection(move |conn| f(conn).map_err(DatabaseError::from))
    }

    /// Execute a transaction with enhanced connection features
    pub fn with_enhanced_transaction<F, R>(&self, f: F) -> DbResult<R>
    where
        F: Fn(&rusqlite::Transaction) -> DbResult<R> + Send + 'static,
        R: Send + 'static,
    {
        self.enhanced_connection.with_connection_mut(move |conn| {
            let tx = conn.transaction().map_err(DatabaseError::from)?;
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
        F: Fn(&Connection) -> rusqlite::Result<R> + Send + 'static,
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
