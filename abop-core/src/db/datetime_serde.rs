//! `DateTime` serialization helpers for `SQLite`
//!
//! This module provides custom serialization and deserialization functions
//! for `chrono::DateTime<Utc>` to work with rusqlite's `ToSql` and `FromSql` traits.

use chrono::{DateTime, Utc};
use rusqlite::{
    Error as SqliteError, Result as SqliteResult,
    types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
};

/// Wrapper for `DateTime<Utc>` that implements rusqlite traits
#[derive(Debug, Clone, Copy)]
pub struct SqliteDateTime(pub DateTime<Utc>);

impl From<DateTime<Utc>> for SqliteDateTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }
}

impl From<SqliteDateTime> for DateTime<Utc> {
    fn from(dt: SqliteDateTime) -> Self {
        dt.0
    }
}

impl ToSql for SqliteDateTime {
    fn to_sql(&self) -> SqliteResult<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.to_rfc3339()))
    }
}

impl FromSql for SqliteDateTime {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;
        DateTime::parse_from_rfc3339(s)
            .map_err(|e| FromSqlError::Other(Box::new(e)))
            .map(|dt| Self(dt.with_timezone(&Utc)))
    }
}

/// Custom error types for `DateTime` serialization operations
#[derive(Debug, thiserror::Error)]
pub enum DateTimeError {
    /// `SQLite` database operation error
    #[error("SQLite error: {0}")]
    Sqlite(#[from] SqliteError),
    /// `DateTime` parsing error when converting from string
    #[error("DateTime parse error: {0}")]
    Parse(#[from] chrono::ParseError),
}

impl From<DateTimeError> for SqliteError {
    fn from(err: DateTimeError) -> Self {
        match err {
            DateTimeError::Sqlite(e) => e,
            DateTimeError::Parse(e) => {
                Self::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
            }
        }
    }
}

/// Helper function to convert `DateTime<Utc>` to SQL string
#[must_use]
pub fn datetime_to_sql(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Helper function to parse SQL string to `DateTime<Utc>`
///
/// # Errors
///
/// Returns [`DateTimeError::Parse`] if the input string is not a valid RFC3339 datetime format.
pub fn datetime_from_sql(s: &str) -> Result<DateTime<Utc>, DateTimeError> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(DateTimeError::Parse)
}

/// Convert DateTime<Utc> to ToSqlOutput
#[must_use]
pub fn datetime_to_sql_output(dt: &DateTime<Utc>) -> ToSqlOutput<'_> {
    ToSqlOutput::from(dt.to_rfc3339())
}

/// Convert Option<DateTime<Utc>> to ToSqlOutput
#[must_use]
pub fn optional_datetime_to_sql_output(dt: &Option<DateTime<Utc>>) -> ToSqlOutput<'_> {
    dt.as_ref().map_or_else(
        || ToSqlOutput::from(rusqlite::types::Null),
        |dt| ToSqlOutput::from(dt.to_rfc3339()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_datetime_serialization() {
        let dt = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let sqlite_dt = SqliteDateTime::from(dt);

        let sql_output = sqlite_dt.to_sql().unwrap();
        assert!(matches!(sql_output, ToSqlOutput::Owned(_)));
    }

    #[test]
    fn test_datetime_roundtrip() {
        let original = Utc::now();
        let serialized = datetime_to_sql(&original);
        let deserialized = datetime_from_sql(&serialized).unwrap();

        // Allow for slight precision differences
        let diff = (original - deserialized).num_milliseconds().abs();
        assert!(diff < 1000); // Less than 1 second difference
    }
}
