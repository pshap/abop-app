//! Database-specific error types for ABOP
//!
//! This module provides structured error handling for database operations,
//! offering better context and recovery options than generic error types.

use thiserror::Error;

/// Database-specific error types
#[derive(Error, Debug, Clone)]
pub enum DatabaseError {
    /// Requested resource was not found.
    #[error("Resource not found: {entity} with id {id} not found")]
    NotFound {
        /// The type of entity that was not found
        entity: String,
        /// The ID that was not found
        id: String,
    },

    /// Connection to the database failed.
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),

    /// Raw `SQLite` error from the underlying driver.
    #[error("SQLite error: {0}")]
    Sqlite(String),

    /// Database transaction failed.
    #[error("Database transaction failed: {message}")]
    TransactionFailed {
        /// Error message describing the transaction failure.
        message: String,
    },

    /// Database migration failed.
    #[error("Database migration failed: version {version} - {message}")]
    MigrationFailed {
        /// Schema version where the migration failed.
        version: u32,
        /// Error message describing the migration failure.
        message: String,
    },

    /// Record not found in the database.
    #[error("Record not found: {entity} with id '{id}'")]
    RecordNotFound {
        /// Entity type (e.g., table name) for the missing record.
        entity: String,
        /// Identifier of the missing record.
        id: String,
    },

    /// Database constraint violation (e.g., foreign key, unique).
    #[error("Database constraint violation: {message}")]
    ConstraintViolation {
        /// Error message describing the violated constraint.
        message: String,
    },

    /// Data validation failed for a field.
    #[error("Data validation failed: {field} - {message}")]
    ValidationFailed {
        /// Name of the field that failed validation.
        field: String,
        /// Error message describing the validation failure.
        message: String,
    },

    /// Duplicate entry detected in the database.
    #[error("Duplicate entry: {entity} with {field} '{value}' already exists")]
    DuplicateEntry {
        /// Entity type for the duplicate entry.
        entity: String,
        /// Field name that is duplicated.
        field: String,
        /// Value that caused the duplication.
        value: String,
    },

    /// Database schema version mismatch.
    #[error("Database schema mismatch: expected version {expected}, found {actual}")]
    SchemaMismatch {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version found in the database.
        actual: u32,
    },

    /// Database lock timeout occurred.
    #[error("Database lock timeout after {timeout_ms}ms")]
    LockTimeout {
        /// Timeout duration in milliseconds.
        timeout_ms: u64,
    },

    /// Query preparation failed.
    #[error("Query preparation failed: {query} - {message}")]
    QueryPreparationFailed {
        /// The SQL query that failed to prepare.
        query: String,
        /// Error message describing the preparation failure.
        message: String,
    },
    /// Database execution failed.
    #[error("Database execution failed: {message}")]
    ExecutionFailed {
        /// Error message describing the execution failure.
        message: String,
    },

    /// Query execution failed.
    #[error("Query failed: {0}")]
    Query(String),
    /// I/O error occurred.
    #[error("I/O error: {0}")]
    Io(String),
}

/// Convenient Result type for database operations
pub type DbResult<T> = Result<T, DatabaseError>;

// Helper functions for common error scenarios
impl DatabaseError {
    /// Create a `RecordNotFound` error for a specific entity
    #[must_use]
    pub fn record_not_found(entity: &str, id: &str) -> Self {
        Self::RecordNotFound {
            entity: entity.to_string(),
            id: id.to_string(),
        }
    }

    /// Create a `ValidationFailed` error for a specific field
    #[must_use]
    pub fn validation_failed(field: &str, message: &str) -> Self {
        Self::ValidationFailed {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a `DuplicateEntry` error
    #[must_use]
    pub fn duplicate_entry(entity: &str, field: &str, value: &str) -> Self {
        Self::DuplicateEntry {
            entity: entity.to_string(),
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a `TransactionFailed` error
    #[must_use]
    pub fn transaction_failed(message: &str) -> Self {
        Self::TransactionFailed {
            message: message.to_string(),
        }
    }

    /// Create a `MigrationFailed` error
    #[must_use]
    pub fn migration_failed(version: u32, message: &str) -> Self {
        Self::MigrationFailed {
            version,
            message: message.to_string(),
        }
    }

    /// Create an `ExecutionFailed` error
    #[must_use]
    pub fn execution_failed(message: &str) -> Self {
        Self::ExecutionFailed {
            message: message.to_string(),
        }
    }
}

// Integration with existing AppError
impl From<DatabaseError> for crate::error::AppError {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::Sqlite(e) => Self::Database(DatabaseError::Sqlite(e)),
            other => Self::Other(other.to_string()),
        }
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Sqlite(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation_helpers() {
        let error = DatabaseError::record_not_found("Audiobook", "123");
        assert_eq!(
            error.to_string(),
            "Record not found: Audiobook with id '123'"
        );

        let error = DatabaseError::validation_failed("title", "cannot be empty");
        assert_eq!(
            error.to_string(),
            "Data validation failed: title - cannot be empty"
        );

        let error = DatabaseError::duplicate_entry("Library", "name", "My Library");
        assert_eq!(
            error.to_string(),
            "Duplicate entry: Library with name 'My Library' already exists"
        );
    }

    #[test]
    fn test_conversion_to_app_error() {
        let db_error = DatabaseError::record_not_found("Test", "456");
        let app_error: crate::error::AppError = db_error.into();

        // Should convert to AppError::Other for now
        assert!(matches!(app_error, crate::error::AppError::Other(_)));
    }
}
