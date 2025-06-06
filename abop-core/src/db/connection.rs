//! Enhanced database connection management
//!
//! This module provides enhanced connection management with health monitoring,
//! retry logic, and performance tracking for improved reliability and observability.

use crate::db::error::{DatabaseError, DbResult};
use crate::db::health::{HealthMonitor, ConnectionHealth};
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
#[derive(Clone, Debug)]
pub struct EnhancedConnection {
    /// Database connection (thread-safe)
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
    pub fn new<P: AsRef<Path>>(path: P) -> DbResult<Self> {
        let config = ConnectionConfig {
            path: path.as_ref().to_path_buf(),
            ..Default::default()
        };
        Ok(Self::with_config(config))
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

    /// Execute a closure with database connection, with retry logic
    ///
    /// # Errors
    ///
    /// Returns a database error if:
    /// - Failed to acquire connection lock
    /// - No active database connection
    /// - The operation closure returns an error
    /// - Connection retry attempts are exhausted
    /// - Database is in an unrecoverable state
    pub fn with_connection<F, R>(&self, operation: F) -> DbResult<R>
    where
        F: FnOnce(&Connection) -> DbResult<R> + Send + 'static,
        R: Send + 'static,
    {
        let start_time = Instant::now();
        let connection = Arc::clone(&self.connection);
        let stats_collector = self.stats_collector.clone();
        let operation = Arc::new(Mutex::new(Some(operation)));

        let result = self.retry_executor.execute(move || {
            let conn = connection.lock().map_err(|_| {
                DatabaseError::ConnectionFailed("Failed to acquire connection lock".to_string())
            })?;
            let conn = conn.as_ref().ok_or_else(|| {
                DatabaseError::ConnectionFailed("No active database connection".to_string())
            })?;

            let mut operation_guard = operation.lock().map_err(|_| {
                DatabaseError::ConnectionFailed("Failed to acquire operation lock".to_string())
            })?;
            let operation = operation_guard.take().ok_or_else(|| {
                DatabaseError::ConnectionFailed("Operation already executed".to_string())
            })?;

            operation(conn)
        })?;

        stats_collector.record_success(start_time.elapsed());
        Ok(result)
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
    pub fn connect(&self) -> DbResult<()> {
        log::debug!(
            "Establishing database connection to: {}",
            self.config.path.display()
        );

        self.health_monitor.set_connecting();

        let start_time = Instant::now();
        let result = self.try_connect();

        match result {
            Ok(()) => {
                self.health_monitor.set_healthy();
                self.stats_collector.record_connection();
                self.stats_collector.record_success(start_time.elapsed());
                log::info!("Database connection established successfully");
                Ok(())
            }
            Err(e) => {
                self.health_monitor.set_failed();
                self.stats_collector.record_failure(start_time.elapsed());
                log::error!("Failed to establish database connection: {e}");
                Err(e)
            }
        }
    }

    /// Attempt to establish connection with retry logic
    fn try_connect(&self) -> DbResult<()> {
        let config = self.config.clone();
        let connection = Arc::clone(&self.connection);
        let stats_collector = self.stats_collector.clone();

        self.retry_executor.execute(move || {
            match Self::establish_connection_internal(&config) {
                Ok(conn) => {
                    let mut connection = connection.lock().map_err(|_| {
                        DatabaseError::ConnectionFailed("Failed to acquire connection lock".to_string())
                    })?;
                    *connection = Some(conn);
                    Ok(())
                }
                Err(e) => {
                    stats_collector.record_reconnection_attempt();
                    Err(e)
                }
            }
        })
    }

    /// Internal method to establish a new database connection
    fn establish_connection_internal(config: &ConnectionConfig) -> DbResult<Connection> {
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE;
        let conn = Connection::open_with_flags(&config.path, flags).map_err(|e| {
            DatabaseError::ConnectionFailed(format!("Failed to open database: {e}"))
        })?;

        // Configure database settings
        Self::configure_connection_internal(&conn, config)?;

        Ok(conn)
    }

    /// Internal method to configure database connection settings
    fn configure_connection_internal(conn: &Connection, config: &ConnectionConfig) -> DbResult<()> {
        let mut pragmas = Vec::new();

        if config.enable_foreign_keys {
            pragmas.push("PRAGMA foreign_keys = ON;");
        }

        if config.enable_wal_mode {
            pragmas.push("PRAGMA journal_mode = WAL;");
            pragmas.push("PRAGMA synchronous = NORMAL;");
        }

        for pragma in pragmas {
            conn.execute_batch(pragma).map_err(|e| {
                DatabaseError::ConnectionFailed(format!("Failed to set pragma {}: {}", pragma, e))
            })?;
        }

        Ok(())
    }

    /// Get connection statistics
    #[must_use]
    pub fn stats(&self) -> ConnectionStats {
        self.stats_collector.get_stats()
    }

    /// Get connection health status
    #[must_use]
    pub fn health(&self) -> ConnectionHealth {
        self.health_monitor.status()
    }

    /// Perform a health check on the connection
    pub fn health_check(&self) -> DbResult<()> {
        self.with_connection(|conn| {
            conn.execute_batch("SELECT 1").map_err(|e| {
                DatabaseError::ConnectionFailed(format!("Health check failed: {}", e))
            })
        })
    }

    /// Close the database connection
    pub fn close(&self) -> DbResult<()> {
        let mut conn = self.connection.lock().map_err(|_| {
            DatabaseError::ConnectionFailed("Failed to acquire connection lock".to_string())
        })?;
        if let Some(connection) = conn.take() {
            connection.close().map_err(|(_, e)| {
                DatabaseError::ConnectionFailed(format!("Failed to close connection: {:?}", e))
            })?;
        }
        Ok(())
    }

    /// Get the current retry policy
    #[must_use]
    pub fn get_retry_policy(&self) -> RetryPolicy {
        self.config.retry_policy.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    fn setup_test_db() -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, "").unwrap();
        temp_file
    }

    #[test]
    fn test_connection_creation() {
        let temp_file = setup_test_db();
        let conn = EnhancedConnection::new(temp_file.path()).unwrap();
        assert_eq!(conn.health(), ConnectionHealth::Connecting);
    }

    #[test]
    fn test_connection_establishment() {
        let temp_file = setup_test_db();
        let conn = EnhancedConnection::new(temp_file.path()).unwrap();
        conn.connect().unwrap();
        assert_eq!(conn.health(), ConnectionHealth::Healthy);
    }

    #[test]
    fn test_connection_operations() {
        let temp_file = setup_test_db();
        let conn = EnhancedConnection::new(temp_file.path()).unwrap();
        conn.connect().unwrap();

        conn.with_connection(|db| {
            db.execute_batch("CREATE TABLE test (id INTEGER PRIMARY KEY)").map_err(|e| {
                DatabaseError::ConnectionFailed(format!("Failed to create table: {}", e))
            })
        })
        .unwrap();

        let stats = conn.stats();
        assert!(stats.successful_operations > 0);
    }

    #[test]
    fn test_connection_health_check() {
        let temp_file = setup_test_db();
        let conn = EnhancedConnection::new(temp_file.path()).unwrap();
        conn.connect().unwrap();
        conn.health_check().unwrap();
        assert_eq!(conn.health(), ConnectionHealth::Healthy);
    }

    #[test]
    fn test_connection_close() {
        let temp_file = setup_test_db();
        let conn = EnhancedConnection::new(temp_file.path()).unwrap();
        conn.connect().unwrap();
        conn.close().unwrap();
        assert_eq!(conn.health(), ConnectionHealth::Failed);
    }
}
