//! Database operations abstraction layer
//!
//! This module provides a high-level abstraction for database operations,
//! consolidating connection management, transaction handling, and common patterns.

use super::error::{DatabaseError, DbResult};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Transaction};
use std::sync::Arc;
use tracing::{debug, instrument};

/// High-level database operations manager
#[derive(Clone, Debug)]
pub struct DatabaseOperations {
    /// Connection pool for basic operations
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl DatabaseOperations {
    /// Create a new database operations manager
    #[must_use] pub const fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }

    /// Execute a query using the connection pool
    #[instrument(skip(self, operation))]
    pub fn execute_query<T, F>(&self, operation: F) -> DbResult<T>
    where
        F: FnOnce(&Connection) -> DbResult<T>,
    {
        let conn = self.pool.get().map_err(|e| {
            DatabaseError::ConnectionFailed(format!("Failed to get connection from pool: {e}"))
        })?;

        operation(&conn)
    }

    /// Execute a synchronous operation with the connection pool
    #[instrument(skip(self, operation))]
    pub fn execute<T, F>(&self, operation: F) -> DbResult<T>
    where
        F: FnOnce(&Connection) -> DbResult<T> + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        let result = std::thread::spawn(move || {
            let conn = pool.get().map_err(|e| {
                DatabaseError::ConnectionFailed(format!("Failed to get connection from pool: {e}"))
            })?;
            operation(&conn)
        })
        .join()
        .map_err(|_| DatabaseError::execution_failed("Thread panicked during operation"))??;

        Ok(result)
    }

    /// Execute a query with a mutable connection
    #[instrument(skip(self, operation))]
    pub fn execute_query_mut<T, F>(&self, operation: F) -> DbResult<T>
    where
        F: FnOnce(&mut Connection) -> DbResult<T>,
    {
        let mut conn = self.pool.get().map_err(|e| {
            DatabaseError::ConnectionFailed(format!("Failed to get connection from pool: {e}"))
        })?;

        operation(&mut conn)
    }

    /// Execute operation within a transaction
    #[instrument(skip(self, operation))]
    pub fn execute_transaction<T, F>(&self, operation: F) -> DbResult<T>
    where
        F: FnOnce(&Transaction) -> DbResult<T>,
    {
        self.execute_query_mut(|conn| {
            let tx = conn
                .transaction()
                .map_err(|e| DatabaseError::TransactionFailed {
                    message: format!("Failed to start transaction: {e}"),
                })?;

            let result = operation(&tx)?;

            tx.commit().map_err(|e| DatabaseError::TransactionFailed {
                message: format!("Failed to commit transaction: {e}"),
            })?;

            debug!("Transaction completed successfully");
            Ok(result)
        })
    }

    /// Execute a bulk operation with proper connection management
    #[instrument(skip(self, operation))]
    pub fn execute_bulk_operation<T, F>(&self, operation: F) -> DbResult<T>
    where
        F: FnOnce(&mut Connection) -> DbResult<T>,
    {
        self.execute_query_mut(operation)
    }

    /// Execute a bulk operation with automatic batching
    #[instrument(skip(self, operations))]
    pub fn execute_bulk<T, F>(&self, operations: Vec<F>) -> DbResult<Vec<T>>
    where
        F: FnOnce(&Transaction) -> DbResult<T>,
    {
        if operations.is_empty() {
            return Ok(Vec::new());
        }

        self.execute_transaction(|tx| {
            let mut results = Vec::with_capacity(operations.len());
            for operation in operations {
                results.push(operation(tx)?);
            }
            Ok(results)
        })
    }
    /// Get access to the connection pool for direct access if needed
    #[must_use] pub const fn pool(&self) -> &Arc<Pool<SqliteConnectionManager>> {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_database_operations_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let manager = SqliteConnectionManager::file(temp_file.path());
        let pool = Arc::new(Pool::builder().max_size(1).build(manager).unwrap());

        let ops = DatabaseOperations::new(pool);

        // Test basic query execution
        let result = ops.execute_query(|conn| {
            conn.execute("CREATE TABLE test (id INTEGER)", [])
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Test query failed: {e}"),
                })?;
            Ok(42)
        });

        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_transaction_execution() {
        let temp_file = NamedTempFile::new().unwrap();
        let manager = SqliteConnectionManager::file(temp_file.path());
        let pool = Arc::new(Pool::builder().max_size(1).build(manager).unwrap());

        let ops = DatabaseOperations::new(pool);

        let result = ops.execute_transaction(|tx| {
            tx.execute("CREATE TABLE test (id INTEGER)", [])
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Transaction test failed: {e}"),
                })?;
            Ok("success")
        });

        assert_eq!(result.unwrap(), "success");
    }
}
