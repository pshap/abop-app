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
/// # DEPRECATED
/// 
/// **This type will be removed in version 0.2.0**
/// 
/// This type alias uses unsafe type erasure and creates security vulnerabilities.
/// All code using this type should migrate to the safe alternatives:
/// - Use `TypedRowCallback<T>` for type-safe callbacks
/// - Use specific repository methods with concrete return types
/// - Use the `Repository` pattern with strongly-typed methods
/// 
/// # Safety
///
/// The callback returns `Box<dyn Any + Send>` which requires careful type casting
/// by the caller. This is an unsafe pattern that should be avoided.
///
/// # Migration Path
/// 
/// Replace usage with `TypedRowCallback<T>` or direct repository methods.
#[deprecated(
    since = "0.1.0",
    note = "Use TypedRowCallback<T> or typed repository methods instead. Will be removed in 0.2.0"
)]
type RowCallback =
    Box<dyn FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<Box<dyn Any + Send>> + Send>;

/// Type alias for typed row processing callback to reduce complexity
///
/// This callback provides type safety while allowing dynamic query construction.
/// Unlike `RowCallback`, this returns a concrete type `T` known at compile time.
type TypedRowCallback<T> = Box<dyn FnOnce(&rusqlite::Row) -> Result<T, rusqlite::Error> + Send>;

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
        // Use efficient parameter conversion with early error handling
        let query = query.to_string();
        let owned_params = convert_params_efficiently(params)?;

        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(&query)?;
            let param_refs: Vec<&dyn rusqlite::ToSql> = owned_params
                .iter()
                .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
                .collect();
            stmt.execute(rusqlite::params_from_iter(param_refs))
        })
    }
    /// Execute a query that returns a single row (dynamic version)
    ///
    /// **⚠️ DEPRECATED**: This method uses unsafe type erasure and should be avoided.
    /// Use the `SafeDynRepository` trait and its `query_row_safe` method instead for type safety.
    ///
    /// # ⚠️ CRITICAL SECURITY WARNING ⚠️
    ///
    /// This method performs type erasure and returns `Box<dyn Any + Send>`, creating significant security risks:
    ///
    /// ## Security Vulnerabilities:
    /// - **Runtime panics**: Mismatched types during downcasting cause immediate program crashes
    /// - **Type confusion attacks**: Malicious code could exploit type mismatches for memory corruption
    /// - **Memory safety violations**: Incorrect downcasting can lead to undefined behavior
    /// - **No compile-time safety**: Type errors only surface at runtime in production
    ///
    /// ## Attack Vectors:
    /// - Malicious SQL injection could return unexpected data types
    /// - Race conditions in concurrent access could cause type confusion
    /// - External dependencies changing return types could break type assumptions
    ///
    /// ## Recommended Migration:
    /// ```ignore
    /// // Instead of this unsafe pattern:
    /// let result = repo.query_row_dyn(sql, params, callback)?;
    /// let value: MyType = *result.downcast::<MyType>().unwrap(); // UNSAFE!
    ///
    /// // Use this type-safe approach:
    /// let value: MyType = repo.query_row_safe(sql, params, |row| {
    ///     Ok(MyType::from(row))
    /// })?; // SAFE!
    /// ```
    ///    /// **Use `SafeDynRepository::query_row_safe` instead for type safety.**
    ///
    /// # Safety Requirements
    ///
    /// - Query must return exactly one row (enforced by rusqlite)
    /// - Callback can only be called once per method invocation
    /// - Caller must ensure correct type casting to prevent security issues
    /// - **CRITICAL**: Never use this method with untrusted input or in security-sensitive contexts
    #[deprecated(
        since = "0.1.0",
        note = "Use SafeDynRepository::query_row_safe for type safety"
    )]
    fn query_row_dyn(
        &self,
        query: &str,
        params: &[&(dyn rusqlite::ToSql + Sync)],
        callback: RowCallback,
    ) -> DbResult<Box<dyn Any + Send>> {
        // Provide a default implementation that discourages usage
        // and logs security warnings when this unsafe method is called
        log::error!(
            "SECURITY WARNING: Deprecated unsafe method query_row_dyn called! \
             This method uses type erasure and creates security vulnerabilities. \
             Query: '{query}', Caller should migrate to SafeDynRepository::query_row_safe"
        );

        // Implement the unsafe functionality but with additional safety checks
        self.query_row_dyn_unsafe_impl(query, params, callback)
    }

    /// Internal unsafe implementation - DO NOT CALL DIRECTLY
    ///
    /// This method exists only to maintain backward compatibility for existing code
    /// that hasn't been migrated yet. It will be removed in a future version.
    ///
    /// # Safety
    ///
    /// This method is inherently unsafe due to type erasure. Only use through
    /// the deprecated `query_row_dyn` method, which provides additional logging
    /// and safety warnings.
    #[doc(hidden)]
    fn query_row_dyn_unsafe_impl(
        &self,
        query: &str,
        params: &[&(dyn rusqlite::ToSql + Sync)],
        callback: RowCallback,
    ) -> DbResult<Box<dyn Any + Send>>;
}

/// Type-safe repository trait for dynamic queries
///
/// This trait provides type-safe alternatives to the unsafe methods in `DynRepository`.
/// Unlike `DynRepository`, this trait is not object-safe due to generic methods,
/// but provides compile-time type safety for dynamic query construction.
pub trait SafeDynRepository: RepositoryBase {
    /// Execute a query that returns a single row with type safety
    ///
    /// **Type Safety**: This method provides a type-safe alternative to `query_row_dyn`
    /// by using generics to ensure compile-time type checking while still allowing
    /// dynamic query construction.
    ///    /// **Preferred Usage**: Use this method instead of `query_row_dyn` for new code
    /// that requires dynamic queries with type safety.
    fn query_row_safe<T: Send + 'static>(
        &self,
        query: &str,
        params: &[&(dyn rusqlite::ToSql + Sync)],
        callback: TypedRowCallback<T>,
    ) -> DbResult<T> {
        let query = query.to_string();
        let owned_params = convert_params_efficiently(params)?;

        // Use Cell for single-use callback ownership transfer (same pattern as query_row_dyn)
        use std::cell::Cell;
        let callback_option = Cell::new(Some(callback));

        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(&query)?;
            let param_refs: Vec<&dyn rusqlite::ToSql> = owned_params
                .iter()
                .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
                .collect();

            stmt.query_row(rusqlite::params_from_iter(param_refs), |row| {
                let callback = callback_option.take().ok_or_else(|| {
                    rusqlite::Error::SqliteFailure(
                        rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                        Some("Callback consumed - query_row_safe single-use only".to_string()),
                    )
                })?;
                callback(row)
            })
        })
    }

    /// Execute a query dyn with migration to safe alternative
    ///
    /// This method provides a migration path from the unsafe `query_row_dyn` to the safe
    /// `query_row_safe` method. It accepts a generic return type and provides compile-time
    /// type safety while maintaining the dynamic query capability.
    ///
    /// # Type Safety
    ///
    /// Unlike `query_row_dyn`, this method:
    /// - Provides compile-time type checking
    /// - Eliminates runtime type casting errors
    /// - Prevents type confusion vulnerabilities
    /// - Maintains performance with zero-cost abstractions
    ///
    /// # Migration Example
    ///
    /// ```ignore
    /// // Old unsafe pattern:
    /// let result = repo.query_row_dyn(sql, params, |row| {
    ///     Ok(Box::new(MyStruct::from(row)) as Box<dyn Any + Send>)
    /// })?;
    /// let my_struct = *result.downcast::<MyStruct>().unwrap(); // UNSAFE!
    ///    /// // New safe pattern:
    /// let my_struct: MyStruct = repo.query_row_safe_migration(sql, params, |row| {
    ///     MyStruct::from(row)
    /// })?; // SAFE!
    /// ```
    fn query_row_safe_migration<T: Send + 'static>(
        &self,
        query: &str,
        params: &[&(dyn rusqlite::ToSql + Sync)],
        callback: impl FnOnce(&rusqlite::Row) -> Result<T, rusqlite::Error> + Send + 'static,
    ) -> DbResult<T> {
        let query = query.to_string();
        let owned_params = convert_params_efficiently(params)?;

        // Use Cell for single-use callback ownership transfer
        use std::cell::Cell;
        let callback_option = Cell::new(Some(callback));

        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(&query)?;
            let param_refs: Vec<&dyn rusqlite::ToSql> = owned_params
                .iter()
                .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
                .collect();

            stmt.query_row(rusqlite::params_from_iter(param_refs), |row| {
                let callback = callback_option.take().ok_or_else(|| {
                    rusqlite::Error::SqliteFailure(
                        rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                        Some(
                            "Callback consumed - query_row_safe_migration single-use only"
                                .to_string(),
                        ),
                    )
                })?;
                callback(row)
            })
        })
    }
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
                    // Attempt rollback with improved logging for transaction context
                    if let Err(rollback_err) = tx.rollback() {
                        log::error!(
                            "Repository transaction rollback failed during error recovery. \
                             Transaction state: failed, Rollback state: failed, \
                             Context: standard_repository_transaction, Operation: rollback_on_error, \
                             Rollback error: {rollback_err}, Original error: {e}"
                        );
                        // Still return the original error as primary concern
                        Err(e)
                    } else {
                        log::debug!(
                            "Repository transaction rolled back successfully. \
                             Transaction state: rolled_back, Rollback state: success, \
                             Context: standard_repository_transaction, Operation: rollback_on_error, \
                             Original error: {e}"
                        );
                        Err(e)
                    }
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
        // Use efficient parameter conversion with early error handling
        let query = query.to_string();
        let owned_params = convert_params_efficiently(params)?;

        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(&query)?;
            let param_refs: Vec<&dyn rusqlite::ToSql> = owned_params
                .iter()
                .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
                .collect();
            stmt.execute(rusqlite::params_from_iter(param_refs))
        })
    }
    fn query_row_dyn_unsafe_impl(
        &self,
        query: &str,
        params: &[&(dyn rusqlite::ToSql + Sync)],
        callback: RowCallback,
    ) -> DbResult<Box<dyn Any + Send>> {
        // Use the same efficient parameter conversion as execute_query_dyn
        let query = query.to_string();
        let owned_params = convert_params_efficiently(params)?;

        // Use Cell for single-use callback ownership transfer
        use std::cell::Cell;
        let callback_option = Cell::new(Some(callback));

        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(&query)?;
            let param_refs: Vec<&dyn rusqlite::ToSql> = owned_params
                .iter()
                .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
                .collect();

            // Execute query expecting exactly one row
            stmt.query_row(rusqlite::params_from_iter(param_refs), |row| {
                let callback = callback_option.take().ok_or_else(|| {
                    rusqlite::Error::SqliteFailure(
                        rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                        Some("Callback consumed - query_row_dyn single-use only".to_string()),
                    )
                })?;
                callback(row)
            })
        })
    }
}

/// Helper function to convert a ValueRef to an owned ToSql value
///
/// **Purpose**: Converts borrowed SQLite values (ValueRef) to owned values that can be
/// safely moved across thread boundaries (Send) for use in dynamic database operations.
/// This is essential for the dynamic repository pattern where parameters need to be
/// converted from various types into a uniform trait object representation.
///
/// **Usage**: Called internally by `execute_query_dyn` and `query_row_dyn` to handle
/// parameter conversion in a consistent manner across all repository implementations.
fn convert_value_ref_to_owned(v: rusqlite::types::ValueRef) -> Box<dyn rusqlite::ToSql + Send> {
    match v {
        rusqlite::types::ValueRef::Null => Box::new(None::<String>),
        rusqlite::types::ValueRef::Integer(i) => Box::new(i),
        rusqlite::types::ValueRef::Real(f) => Box::new(f),
        rusqlite::types::ValueRef::Text(s) => Box::new(String::from_utf8_lossy(s).into_owned()),
        rusqlite::types::ValueRef::Blob(b) => Box::new(b.to_vec()),
    }
}

/// Helper function to convert a Value to an owned ToSql value
///
/// **Purpose**: Converts owned SQLite values (Value) to boxed ToSql trait objects
/// that can be safely used in dynamic database operations across thread boundaries.
/// This complements `convert_value_ref_to_owned` by handling already-owned values.
///
/// **Usage**: Called internally by `execute_query_dyn` and `query_row_dyn` when
/// parameter conversion yields owned values rather than borrowed references.
fn convert_value_to_owned(v: rusqlite::types::Value) -> Box<dyn rusqlite::ToSql + Send> {
    match v {
        rusqlite::types::Value::Null => Box::new(None::<String>),
        rusqlite::types::Value::Integer(i) => Box::new(i),
        rusqlite::types::Value::Real(f) => Box::new(f),
        rusqlite::types::Value::Text(s) => Box::new(s),
        rusqlite::types::Value::Blob(b) => Box::new(b),
    }
}

/// Efficient parameter conversion function to eliminate duplication between dynamic methods
///
/// **Purpose**: Converts an array of SQL parameters to owned values suitable for dynamic
/// database operations. This centralizes parameter conversion logic and provides efficient
/// error handling with early bailout on conversion failures.
///
/// **Performance**: Pre-allocates result vector and uses direct conversion for efficiency,
/// avoiding iterator overhead and providing fast-fail behavior on invalid parameters.
///
/// **Architecture**: This function is decomposed into smaller helper functions for better
/// maintainability and readability. Each helper function handles a specific aspect of the
/// conversion process with detailed documentation.
fn convert_params_efficiently(
    params: &[&(dyn rusqlite::ToSql + Sync)],
) -> Result<Vec<Box<dyn rusqlite::ToSql + Send>>, DatabaseError> {
    // Pre-allocate vector with exact capacity to avoid reallocations
    let mut result = Vec::with_capacity(params.len());

    for p in params {
        let converted_param = convert_single_param(p)?;
        result.push(converted_param);
    }

    Ok(result)
}

/// Converts a single SQL parameter to an owned value
///
/// **Purpose**: Handles the conversion of individual SQL parameters, providing clear
/// error messages and consistent behavior across all parameter types.
///
/// **Error Handling**: Returns early on conversion failures with contextual error information
/// to help with debugging parameter-related issues. No longer performs silent failure
/// conversions that could mask underlying problems.
///
/// **Safety**: Ensures all returned values implement both `ToSql` and `Send` traits
/// for safe use in concurrent database operations.
fn convert_single_param(
    param: &(dyn rusqlite::ToSql + Sync),
) -> Result<Box<dyn rusqlite::ToSql + Send>, DatabaseError> {
    match param.to_sql() {
        Ok(output) => convert_sql_output_to_owned(output),
        Err(e) => Err(DatabaseError::parameter_conversion_failed(&format!(
            "Failed to convert SQL parameter: {e}"
        ))),
    }
}

/// Converts SQLite ToSqlOutput to owned boxed values
///
/// **Purpose**: Handles the conversion of SQLite's output types to owned values that can
/// be safely moved across thread boundaries. This function centralizes the logic for
/// handling both borrowed and owned SQL values.
///
/// **Design**: Uses pattern matching to handle all possible SQLite output types,
/// ensuring comprehensive coverage and preventing runtime panics from unhandled cases.
///
/// **Performance**: Directly converts values without intermediate allocations where possible,
/// and provides fallback handling for unexpected or future SQLite output types.
///
/// **Error Handling**: Unlike silent failure patterns, this function properly logs
/// unexpected variants and provides detailed diagnostic information for debugging.
fn convert_sql_output_to_owned(
    output: rusqlite::types::ToSqlOutput<'_>,
) -> Result<Box<dyn rusqlite::ToSql + Send>, DatabaseError> {
    match output {
        rusqlite::types::ToSqlOutput::Borrowed(value_ref) => {
            Ok(convert_value_ref_to_owned(value_ref))
        }
        rusqlite::types::ToSqlOutput::Owned(value) => Ok(convert_value_to_owned(value)),
        // Handle any other potential variants with comprehensive error reporting
        _ => {
            // Log detailed diagnostic information for debugging and monitoring
            log::error!(
                "Database parameter conversion failed: Encountered unexpected ToSqlOutput variant. \
                 Context: convert_sql_output_to_owned, Variant: {:?}, \
                 Action: Failing conversion instead of silent NULL conversion. \
                 This may indicate a SQLite version compatibility issue or corrupted parameter data.",
                std::mem::discriminant(&output)
            );

            // Instead of silently converting to NULL (which masks errors),
            // return a proper error that can be handled by the caller
            Err(DatabaseError::parameter_conversion_failed(
                "Encountered unexpected SQLite ToSqlOutput variant. \
                 This may indicate a version compatibility issue or data corruption. \
                 Check logs for detailed diagnostic information.",
            ))
        }
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
                    // Attempt to rollback, but preserve the original error if rollback fails
                    if let Err(rollback_err) = tx.rollback() {
                        log::error!(
                            "Database transaction rollback failed during error recovery. \
                             Transaction state: failed, Rollback state: failed, \
                             Context: enhanced_repository_transaction, Operation: rollback_on_error, \
                             Rollback error: {rollback_err}, Original error: {e}"
                        );
                        // Return a compound error that includes both the original and rollback errors
                        Err(DatabaseError::transaction_failed(&format!(
                            "Transaction failed: {e}. Rollback also failed: {rollback_err}", 
                        )))
                    } else {
                        log::warn!(
                            "Database transaction rolled back successfully after error. \
                             Transaction state: rolled_back, Rollback state: success, \
                             Context: enhanced_repository_transaction, Operation: rollback_on_error, \
                             Original error: {e}"
                        );
                        Err(e)
                    }
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

// Implement SafeDynRepository for all repository types
impl<T: RepositoryBase + ?Sized> SafeDynRepository for T {}
