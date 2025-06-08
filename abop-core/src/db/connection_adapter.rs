//! Connection adapter for bridging EnhancedConnection with Repository pattern
//!
//! This module provides adapters that allow repositories to use the enhanced connection
//! infrastructure while maintaining the existing Repository trait interface.

use super::connection::EnhancedConnection;
use super::error::{DatabaseError, DbResult};
use rusqlite::Connection;
use std::sync::Arc;

/// A connection adapter that delegates operations to an EnhancedConnection
///
/// This adapter implements the `Arc<Mutex<Connection>>` interface expected by
/// repositories while actually delegating all operations to an EnhancedConnection.
/// This avoids creating multiple SQLite connections to the same database file.
pub struct ConnectionAdapter {
    enhanced_conn: Arc<EnhancedConnection>,
}

impl ConnectionAdapter {
    /// Create a new connection adapter
    pub const fn new(enhanced_conn: Arc<EnhancedConnection>) -> Self {
        Self { enhanced_conn }
    }

    /// Execute an operation with the underlying enhanced connection
    ///
    /// This method provides the same interface as locking an `Arc<Mutex<Connection>>`
    /// but delegates to the enhanced connection's `with_connection` method.
    pub fn execute<F, R>(&self, operation: F) -> DbResult<R>
    where
        F: FnOnce(&Connection) -> Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static,
    {
        // We need to work around the fact that with_connection expects Fn but we have FnOnce
        // We'll use a thread-safe approach by wrapping the operation
        let operation = std::sync::Arc::new(std::sync::Mutex::new(Some(operation)));
        self.enhanced_conn.with_connection(move |conn| {
            let mut op_guard = operation.lock().map_err(|_| {
                DatabaseError::ConnectionFailed("Failed to acquire operation lock".to_string())
            })?;
            let op = op_guard.take().ok_or_else(|| {
                DatabaseError::ConnectionFailed("Operation already consumed".to_string())
            })?;
            drop(op_guard); // Release the lock before calling the operation
            op(conn).map_err(DatabaseError::from)
        })
    }

    /// Get a reference to the enhanced connection
    pub const fn enhanced_connection(&self) -> &Arc<EnhancedConnection> {
        &self.enhanced_conn
    }
}

impl Clone for ConnectionAdapter {
    fn clone(&self) -> Self {
        Self {
            enhanced_conn: self.enhanced_conn.clone(),
        }
    }
}
