//! Enhanced database connection management
//!
//! This module provides enhanced connection management with health monitoring,
//! retry logic, and performance tracking for improved reliability and observability.

use crate::db::error::{DatabaseError, DbResult};
use crate::db::health::HealthMonitor;
use crate::db::retry::{RetryExecutor, RetryPolicy};
use crate::db::statistics::{ConnectionStats, StatisticsCollector};
use rusqlite::{Connection, OpenFlags};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Configuration for enhanced connection management
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Database file path
    pub path: PathBuf,
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,
    /// Enable WAL mode for better concurrency
    pub enable_wal_mode: bool,
    /// Enable foreign key constraints
    pub enable_foreign_keys: bool,
    /// Retry policy for connection attempts
    pub retry_policy: RetryPolicy,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("database.db"),
            connection_timeout_seconds: 30,
            health_check_interval_seconds: 60,
            enable_wal_mode: true,
            enable_foreign_keys: true,
            retry_policy: RetryPolicy::default(),
        }
    }
}

/// Enhanced database connection manager
pub struct EnhancedConnection {
    /// Database connection
    connection: Arc<Mutex<Option<Connection>>>,
    /// Connection configuration
    config: ConnectionConfig,
    /// Statistics collector
    stats_collector: StatisticsCollector,
    /// Health monitor
    health_monitor: Arc<HealthMonitor>,
    /// Retry executor
    retry_executor: RetryExecutor,
}

impl EnhancedConnection {
    /// Create a new enhanced connection with default configuration
    #[must_use]
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let config = ConnectionConfig {
            path: path.as_ref().to_path_buf(),
            ..Default::default()
        };
        Self::with_config(config)
    }

    /// Create a new enhanced connection with custom configuration
    #[must_use]
    pub fn with_config(config: ConnectionConfig) -> Self {
        let retry_executor = RetryExecutor::new(config.retry_policy.clone());

        Self {
            connection: Arc::new(Mutex::new(None)),
            config,
            stats_collector: StatisticsCollector::new(),
            health_monitor: Arc::new(HealthMonitor::new()),
            retry_executor,
        }
    }

    /// Establish or re-establish database connection
    ///
    /// # Errors
    ///
    /// Returns a database error if:
    /// - Failed to establish connection to the database
    /// - Database file is corrupted or inaccessible
    /// - Insufficient permissions to access the database file
    /// - Database configuration is invalid
    /// - Failed to record connection statistics
    pub fn connect(&self) -> DbResult<()> {
        log::debug!(
            "Establishing database connection to: {}",
            self.config.path.display()
        );

        if let Err(e) = self.health_monitor.set_connecting() {
            log::warn!("Failed to set health status to connecting: {}", e);
        }

        let start_time = Instant::now();
        let result = self.try_connect();

        match result {
            Ok(()) => {
                self.health_monitor.set_healthy();
                self.stats_collector.record_connection().map_err(|e| {
                    DatabaseError::ConnectionFailed(format!("Failed to record connection: {e}"))
                })?;
                self.stats_collector
                    .record_success(start_time.elapsed())
                    .map_err(|e| {
                        DatabaseError::ConnectionFailed(format!("Failed to record success: {e}"))
                    })?;
                log::info!("Database connection established successfully");
                Ok(())
            }
            Err(e) => {
                self.health_monitor.set_failed();
                self.stats_collector
                    .record_failure(start_time.elapsed())
                    .map_err(|stats_err| {
                        DatabaseError::ConnectionFailed(format!(
                            "Failed to record failure: {stats_err}"
                        ))
                    })?;
                log::error!("Failed to establish database connection: {e}");
                Err(e)
            }
        }
    }

    /// Attempt to establish connection with retry logic
    fn try_connect(&self) -> DbResult<()> {
        self.retry_executor
            .execute(|| match self.establish_connection() {
                Ok(conn) => self.connection.lock().map_or_else(
                    |_| {
                        Err(DatabaseError::ConnectionFailed(
                            "Failed to acquire connection lock".to_string(),
                        ))
                    },
                    |mut connection| {
                        *connection = Some(conn);
                        Ok(())
                    },
                ),
                Err(e) => {
                    let _ = self.stats_collector.record_reconnection_attempt();
                    Err(e)
                }
            })
    }

    /// Establish a new database connection
    fn establish_connection(&self) -> DbResult<Connection> {
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE;
        let conn = Connection::open_with_flags(&self.config.path, flags).map_err(|e| {
            DatabaseError::ConnectionFailed(format!("Failed to open database: {e}"))
        })?;

        // Configure database settings
        self.configure_connection(&conn)?;

        Ok(conn)
    }

    /// Configure database connection settings
    fn configure_connection(&self, conn: &Connection) -> DbResult<()> {
        let mut pragmas = Vec::new();

        if self.config.enable_foreign_keys {
            pragmas.push("PRAGMA foreign_keys = ON;");
        }

        if self.config.enable_wal_mode {
            pragmas.push("PRAGMA journal_mode = WAL;");
            pragmas.push("PRAGMA synchronous = NORMAL;");
        }

        // Set connection timeout
        let timeout_ms = self.config.connection_timeout_seconds * 1000;
        let timeout_pragma = format!("PRAGMA busy_timeout = {timeout_ms};");
        pragmas.push(&timeout_pragma);

        let pragma_sql = pragmas.join("\n");
        conn.execute_batch(&pragma_sql)
            .map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to configure database with pragmas '{pragma_sql}': {e}"),
            })?;

        Ok(())
    }

    /// Execute a closure with database connection, with retry logic
    ///
    /// # Errors
    ///
    /// Returns a database error if:
    /// - Failed to establish database connection
    /// - The operation closure returns an error
    /// - Connection retry attempts are exhausted
    /// - Database is in an unrecoverable state
    /// - Failed to record operation statistics
    pub fn with_connection<F, R>(&self, operation: F) -> DbResult<R>
    where
        F: Fn(&Connection) -> DbResult<R> + Send + 'static,
        R: Send + 'static,
    {
        let start_time = Instant::now();

        let result = self.retry_executor.execute(|| {
            let conn_guard = self.connection.lock().map_err(|_| {
                DatabaseError::ConnectionFailed("Failed to acquire connection lock".to_string())
            })?;

            if let Some(conn) = conn_guard.as_ref() {
                operation(conn)
            } else {
                drop(conn_guard);
                self.connect()?;
                let new_guard = self.connection.lock().map_err(|_| {
                    DatabaseError::ConnectionFailed(
                        "Failed to acquire connection lock after establishment".to_string(),
                    )
                })?;
                new_guard.as_ref().map_or_else(
                    || {
                        Err(DatabaseError::ConnectionFailed(
                            "Connection not available after establishment".to_string(),
                        ))
                    },
                    &operation,
                )
            }
        });

        match &result {
            Ok(_) => self
                .stats_collector
                .record_success(start_time.elapsed())
                .map_err(|e| {
                    DatabaseError::ConnectionFailed(format!("Failed to record success: {e}"))
                })?,
            Err(_) => self
                .stats_collector
                .record_failure(start_time.elapsed())
                .map_err(|e| {
                    DatabaseError::ConnectionFailed(format!("Failed to record failure: {e}"))
                })?,
        }

        result
    }

    /// Execute a closure with mutable database connection, with retry logic
    ///
    /// # Errors
    ///
    /// Returns a database error if:
    /// - Failed to establish database connection
    /// - The operation closure returns an error
    /// - Connection retry attempts are exhausted
    /// - Database is in an unrecoverable state
    /// - Failed to record operation statistics
    pub fn with_connection_mut<F, R>(&self, operation: F) -> DbResult<R>
    where
        F: Fn(&mut Connection) -> DbResult<R> + Send + 'static,
        R: Send + 'static,
    {
        let start_time = Instant::now();

        let result = self.retry_executor.execute(|| {
            let mut conn_guard = self.connection.lock().map_err(|_| {
                DatabaseError::ConnectionFailed("Failed to acquire connection lock".to_string())
            })?;

            if let Some(conn) = conn_guard.as_mut() {
                operation(conn)
            } else {
                drop(conn_guard);
                self.connect()?;
                let mut new_guard = self.connection.lock().map_err(|_| {
                    DatabaseError::ConnectionFailed(
                        "Failed to acquire connection lock after establishment".to_string(),
                    )
                })?;
                new_guard.as_mut().map_or_else(
                    || {
                        Err(DatabaseError::ConnectionFailed(
                            "Connection not available after establishment".to_string(),
                        ))
                    },
                    &operation,
                )
            }
        });

        match &result {
            Ok(_) => self
                .stats_collector
                .record_success(start_time.elapsed())
                .map_err(|e| {
                    DatabaseError::ConnectionFailed(format!("Failed to record success: {e}"))
                })?,
            Err(_) => self
                .stats_collector
                .record_failure(start_time.elapsed())
                .map_err(|e| {
                    DatabaseError::ConnectionFailed(format!("Failed to record failure: {e}"))
                })?,
        }

        result
    }

    /// Get connection statistics
    ///
    /// # Errors
    ///
    /// Returns a database error if:
    /// - Failed to acquire statistics lock
    /// - Failed to read connection timestamp
    pub fn stats(&self) -> DbResult<ConnectionStats> {
        self.stats_collector
            .get_stats()
            .map_err(|e| DatabaseError::ConnectionFailed(format!("Failed to get statistics: {e}")))
    }

    /// Get the current health status
    #[must_use]
    pub fn health(&self) -> crate::db::health::ConnectionHealth {
        self.health_monitor.status()
    }

    /// Perform a health check
    ///
    /// # Errors
    ///
    /// Returns a database error if:
    /// - Failed to establish database connection
    /// - Health check query execution fails
    /// - Database is not responding correctly
    pub fn health_check(&self) -> DbResult<()> {
        // Simple health check - try to execute a simple query
        self.with_connection(|conn| {
            conn.execute_batch("SELECT 1")
                .map_err(|e| DatabaseError::ConnectionFailed(format!("Health check failed: {e}")))
        })
    }

    /// Close database connection
    ///
    /// # Errors
    ///
    /// Returns a database error if:
    /// - Failed to properly close the database connection
    /// - Cleanup operations fail during shutdown
    pub fn close(&self) -> DbResult<()> {
        log::debug!("Closing database connection");

        let conn_opt = {
            if let Ok(mut conn_guard) = self.connection.lock() {
                conn_guard.take()
            } else {
                return Err(DatabaseError::ConnectionFailed(
                    "Failed to acquire connection lock".to_string(),
                ));
            }
        };

        if let Some(conn) = conn_opt
            && let Err(e) = conn.close()
        {
            log::error!("Error closing database connection: {e:?}");
            return Err(DatabaseError::ConnectionFailed(format!(
                "Failed to close connection: {e:?}"
            )));
        }

        self.health_monitor.set_failed();
        log::info!("Database connection closed successfully");
        Ok(())
    }

    /// Get a reference to the connection configuration
    #[must_use]
    pub const fn config(&self) -> &ConnectionConfig {
        &self.config
    }
}

impl Clone for EnhancedConnection {
    fn clone(&self) -> Self {
        Self {
            connection: self.connection.clone(),
            config: self.config.clone(),
            stats_collector: self.stats_collector.clone(),
            health_monitor: self.health_monitor.clone(),
            retry_executor: RetryExecutor::new(self.config.retry_policy.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::ConnectionHealth;
    use tempfile::NamedTempFile;

    #[test]
    fn test_enhanced_connection_creation() -> DbResult<()> {
        let temp_file = NamedTempFile::new()?;
        let conn = EnhancedConnection::new(temp_file.path());

        let stats = conn.stats()?;
        assert_eq!(stats.successful_operations, 0);
        assert_eq!(stats.failed_operations, 0);
        Ok(())
    }

    #[test]
    fn test_connection_establishment() -> DbResult<()> {
        let temp_file = NamedTempFile::new()?;
        let conn = EnhancedConnection::new(temp_file.path());

        conn.connect()?;
        assert_eq!(conn.health_monitor.status(), ConnectionHealth::Healthy);
        Ok(())
    }

    #[test]
    fn test_operation_with_connection() -> DbResult<()> {
        let temp_file = NamedTempFile::new()?;
        let conn = EnhancedConnection::new(temp_file.path());

        conn.connect()?;

        let result = conn.with_connection(|db_conn| {
            db_conn
                .execute("CREATE TABLE test (id INTEGER PRIMARY KEY)", [])
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("CREATE TABLE test failed: {e}"),
                })?;
            Ok(42)
        })?;

        assert_eq!(result, 42);
        let stats = conn.stats()?;
        assert!(stats.successful_operations > 0);
        Ok(())
    }

    #[test]
    fn test_connection_stats_tracking() -> DbResult<()> {
        let temp_file = NamedTempFile::new()?;
        let conn = EnhancedConnection::new(temp_file.path());

        conn.connect()?;
        // Perform some operations
        for _ in 0..3 {
            conn.with_connection(|db_conn| {
                db_conn
                    .prepare("SELECT 1")
                    .and_then(|mut stmt| stmt.query_row([], |_| Ok(1)))
                    .map_err(|e| DatabaseError::ExecutionFailed {
                        message: format!("SELECT 1 failed: {e}"),
                    })?;
                Ok(())
            })?;
        }

        let stats = conn.stats()?;
        // Note: The actual count may vary depending on implementation details
        // We'll check for at least the expected operations
        assert!(
            stats.successful_operations >= 4,
            "Expected at least 4 successful operations (3 SELECT + 1 connect), got {}",
            stats.successful_operations
        );
        assert_eq!(stats.failed_operations, 0);
        assert!(stats.avg_operation_duration_ms >= 0.0);
        Ok(())
    }

    #[test]
    fn test_connection_close() -> DbResult<()> {
        let temp_file = NamedTempFile::new()?;
        let conn = EnhancedConnection::new(temp_file.path());

        conn.connect()?;
        assert_eq!(conn.health_monitor.status(), ConnectionHealth::Healthy);

        conn.close()?;
        assert_eq!(conn.health_monitor.status(), ConnectionHealth::Failed);
        Ok(())
    }
}
