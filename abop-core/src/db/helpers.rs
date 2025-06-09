//! Database helper functions for common operations
//!
//! This module provides helper functions to reduce code duplication and
//! improve consistency across database operations.

use super::error::{DatabaseError, DbResult};
use super::operations::DatabaseOperations;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Row};
use std::sync::Arc;
use tracing::debug;

/// Helper trait for database connection pool operations
pub trait PoolHelper {
    /// Get a connection from the pool with proper error handling
    fn get_connection(&self) -> DbResult<r2d2::PooledConnection<SqliteConnectionManager>>;
}

impl PoolHelper for Arc<Pool<SqliteConnectionManager>> {
    fn get_connection(&self) -> DbResult<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.get().map_err(|e| {
            DatabaseError::ConnectionFailed(format!("Failed to get connection from pool: {e}"))
        })
    }
}

/// Helper for parsing datetime fields from database rows
pub fn parse_datetime_from_row(
    row: &Row,
    column_name: &str,
) -> DbResult<chrono::DateTime<chrono::Utc>> {
    let datetime_str: String = row.get(column_name).map_err(|e| {
        DatabaseError::execution_failed(&format!("Failed to get {column_name} column: {e}"))
    })?;

    // Try parsing as RFC3339 first (ISO 8601)
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
        return Ok(dt.with_timezone(&chrono::Utc));
    }

    // Fallback to other common formats
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%.f",
    ];

    for format in &formats {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, format) {
            return Ok(dt.and_utc());
        }
    }

    Err(DatabaseError::ExecutionFailed {
        message: format!("Failed to parse datetime: {datetime_str}"),
    })
}

/// Helper for parsing optional datetime fields
pub fn parse_optional_datetime_from_row(
    row: &Row,
    column_name: &str,
) -> DbResult<Option<chrono::DateTime<chrono::Utc>>> {
    match row.get::<_, Option<String>>(column_name) {
        Ok(Some(datetime_str)) => parse_datetime_string(&datetime_str).map(Some),
        Ok(None) => Ok(None),
        Err(e) => Err(DatabaseError::execution_failed(&format!(
            "Failed to get optional {column_name} column: {e}"
        ))),
    }
}

/// Enhanced utility functions for database operations using new infrastructure
pub struct DatabaseHelpers;

impl DatabaseHelpers {
    /// Create a new DatabaseOperations instance from a connection pool
    pub fn operations_from_pool(pool: Arc<Pool<SqliteConnectionManager>>) -> DatabaseOperations {
        DatabaseOperations::new(pool)
    }
    /// Execute a simple query and return the count of affected rows
    pub fn execute_simple_query(
        ops: &DatabaseOperations,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
    ) -> DbResult<usize> {
        let sql = sql.to_string();
        let params: Vec<String> = params
            .iter()
            .map(|p| format!("{:?}", p.to_sql().unwrap()))
            .collect();

        ops.execute(move |conn| {
            conn.execute(&sql, rusqlite::params_from_iter(params.iter()))
                .map_err(|e| {
                    DatabaseError::execution_failed(&format!("Failed to execute SQL: {e}"))
                })
        })
    }
    /// Check if a table exists in the database
    pub fn table_exists(ops: &DatabaseOperations, table_name: &str) -> DbResult<bool> {
        let table_name = table_name.to_string();
        ops.execute(move |conn| {
            let mut stmt = conn
                .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?")
                .map_err(|e| {
                    DatabaseError::execution_failed(&format!("Failed to prepare statement: {e}"))
                })?;

            let count: i64 = stmt
                .query_row([&table_name], |row| row.get(0))
                .map_err(|e| {
                    DatabaseError::execution_failed(&format!("Failed to execute query: {e}"))
                })?;

            Ok(count > 0)
        })
    }
    /// Get the database version from user_version pragma
    pub fn get_db_version(ops: &DatabaseOperations) -> DbResult<i32> {
        ops.execute(|conn| {
            let version: i32 = conn
                .query_row("PRAGMA user_version", [], |row| row.get(0))
                .map_err(|e| {
                    DatabaseError::execution_failed(&format!("Failed to get version: {e}"))
                })?;

            Ok(version)
        })
    }
    /// Set the database version using user_version pragma
    pub fn set_db_version(ops: &DatabaseOperations, version: i32) -> DbResult<()> {
        ops.execute(move |conn| {
            conn.execute(&format!("PRAGMA user_version = {version}"), [])
                .map_err(|e| {
                    DatabaseError::execution_failed(&format!("Failed to set version: {e}"))
                })?;
            Ok(())
        })
    }
}

/// Simplified datetime parsing that works with actual row data
pub fn parse_datetime_string(datetime_str: &str) -> DbResult<chrono::DateTime<chrono::Utc>> {
    // Try parsing as RFC3339 first (ISO 8601)
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(datetime_str) {
        return Ok(dt.with_timezone(&chrono::Utc));
    }

    // Fallback to other common formats
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%.f",
    ];

    for format in &formats {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(datetime_str, format) {
            return Ok(dt.and_utc());
        }
    }

    Err(DatabaseError::ExecutionFailed {
        message: format!("Failed to parse datetime: {datetime_str}"),
    })
}

/// Helper for executing bulk insert operations with transaction
pub fn execute_bulk_insert<F>(conn: &mut Connection, operation: F) -> DbResult<()>
where
    F: FnOnce(&rusqlite::Transaction) -> DbResult<()>,
{
    let tx = conn.transaction().map_err(|e| {
        DatabaseError::execution_failed(&format!("Failed to start transaction: {e}"))
    })?;

    operation(&tx)?;

    tx.commit().map_err(|e| {
        DatabaseError::execution_failed(&format!("Failed to commit transaction: {e}"))
    })?;

    debug!("Bulk insert operation completed successfully");
    Ok(())
}

/// Helper for executing operations with proper connection acquisition
pub fn with_connection<T, F>(pool: &Arc<Pool<SqliteConnectionManager>>, operation: F) -> DbResult<T>
where
    F: FnOnce(&Connection) -> DbResult<T>,
{
    let conn = pool.get_connection()?;
    operation(&conn)
}

/// Helper for executing mutable operations with proper connection acquisition
pub fn with_connection_mut<T, F>(
    pool: &Arc<Pool<SqliteConnectionManager>>,
    operation: F,
) -> DbResult<T>
where
    F: FnOnce(&mut Connection) -> DbResult<T>,
{
    let mut conn = pool.get_connection()?;
    operation(&mut conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike; // Import the trait for date methods

    #[test]
    fn test_datetime_parsing() {
        // Test RFC3339 format
        let rfc3339_str = "2023-12-01T15:30:45Z";
        let parsed = parse_datetime_string(rfc3339_str).unwrap();
        assert_eq!(parsed.year(), 2023);
        assert_eq!(parsed.month(), 12);

        // Test alternative format
        let alt_str = "2023-12-01 15:30:45";
        let parsed_alt = parse_datetime_string(alt_str).unwrap();
        assert_eq!(parsed_alt.year(), 2023);
        assert_eq!(parsed_alt.month(), 12);
    }

    #[test]
    fn test_invalid_datetime() {
        let invalid_str = "not-a-date";
        assert!(parse_datetime_string(invalid_str).is_err());
    }
}
