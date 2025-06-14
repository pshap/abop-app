//! Database row mapping utilities
//!
//! This module provides centralized, consistent row mapping functions for all database entities.

use super::error::{DatabaseError, DbResult};
use super::helpers::parse_datetime_from_row;
use crate::models::{Audiobook, Library, Progress};
use rusqlite::Row;
use std::path::PathBuf;

/// Helper macro to extract row fields with consistent error handling
macro_rules! get_field {
    ($row:expr, $idx:expr, $field_name:literal) => {
        $row.get($idx).map_err(|e| DatabaseError::ExecutionFailed {
            message: format!("Failed to get {}: {e}", $field_name),
        })?
    };
    ($row:expr, $idx:expr, $field_name:literal, optional) => {
        $row.get($idx).ok()
    };
}

/// Alias for backwards compatibility and clarity in row-based access
macro_rules! get_row_field {
    ($row:expr, $idx:expr, $field_name:literal) => {
        get_field!($row, $idx, $field_name)
    };
}

/// Alias for backwards compatibility and clarity in indexed access
macro_rules! get_indexed_field {
    ($row:expr, $idx:expr, $field_name:literal) => {
        get_field!($row, $idx, $field_name)
    };
}

/// Helper macro to extract path fields from rows
macro_rules! get_row_path {
    ($row:expr, $idx:expr, $field_name:literal) => {{
        let path_str: String = get_row_field!($row, $idx, $field_name);
        PathBuf::from(path_str)
    }};
}

/// Helper macro to extract indexed path fields from rows
macro_rules! get_indexed_path {
    ($row:expr, $idx:expr, $field_name:literal) => {{
        let path_str: String = get_indexed_field!($row, $idx, $field_name);
        PathBuf::from(path_str)
    }};
}

/// Centralized row mapping utilities
pub struct RowMappers;

impl RowMappers {
    /// Map a database row to an Audiobook entity using standard column layout
    ///
    /// This function expects columns to be in the exact order defined by AUDIOBOOK_COLUMNS.
    /// For queries with custom column ordering, use `audiobook_from_row_indexed` instead.
    ///
    /// # Arguments
    /// * `row` - The database row to map from
    ///
    /// # Column Order Expected
    /// The row must contain columns in this exact order:
    /// id, library_id, path, title, author, narrator, description,
    /// duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
    pub fn audiobook_from_row(row: &Row) -> DbResult<Audiobook> {
        Ok(Audiobook {
            id: get_field!(row, 0, "audiobook id"),
            library_id: get_field!(row, 1, "library_id"),
            path: get_row_path!(row, 2, "path"),
            title: get_field!(row, 3, "title", optional),
            author: get_field!(row, 4, "author", optional),
            narrator: get_field!(row, 5, "narrator", optional),
            description: get_field!(row, 6, "description", optional),
            duration_seconds: get_field!(row, 7, "duration_seconds", optional),
            size_bytes: get_field!(row, 8, "size_bytes", optional),
            cover_art: get_field!(row, 9, "cover_art", optional),
            created_at: parse_datetime_from_row(row, "created_at")?,
            updated_at: parse_datetime_from_row(row, "updated_at")?,
            selected: get_field!(row, 12, "selected", optional).unwrap_or(false),
        })
    }
    /// Map a database row to a Library entity
    pub fn library_from_row(row: &Row) -> DbResult<Library> {
        Ok(Library {
            id: get_row_field!(row, 0, "library id"),
            name: get_row_field!(row, 1, "library name"),
            path: get_row_path!(row, 2, "library path"),
        })
    }
    /// Map a database row to a Progress entity
    pub fn progress_from_row(row: &Row) -> DbResult<Progress> {
        use crate::db::datetime_serde::SqliteDateTime;

        let last_played: Option<SqliteDateTime> = get_field!(row, 4, "last_played", optional);
        let created_at: SqliteDateTime = get_field!(row, 5, "created_at");
        let updated_at: SqliteDateTime = get_field!(row, 6, "updated_at");

        Ok(Progress {
            id: get_field!(row, 0, "progress id"),
            audiobook_id: get_field!(row, 1, "audiobook_id"),
            position_seconds: get_field!(row, 2, "position_seconds"),
            completed: get_field!(row, 3, "completed"),
            last_played: last_played.map(|dt| dt.into()),
            created_at: created_at.into(),
            updated_at: updated_at.into(),
        })
    }
    /// Map a database row to an Audiobook using custom column indices
    ///
    /// Use this version when the column order differs from the standard layout
    /// or when optimizing performance-critical queries with custom projections.
    /// This is particularly useful for complex JOINs or SELECT statements where
    /// columns appear in a different order than the standard AUDIOBOOK_COLUMNS.
    ///
    /// # Arguments
    /// * `row` - The database row to map from
    /// * `indices` - Column indices specifying where each field appears in the row
    ///
    /// # Performance
    /// This function provides the same functionality as `audiobook_from_row` but
    /// allows for flexible column positioning, making it ideal for optimized queries.
    pub fn audiobook_from_row_indexed(
        row: &Row,
        indices: &AudiobookColumnIndices,
    ) -> DbResult<Audiobook> {
        Ok(Audiobook {
            id: get_indexed_field!(row, indices.id, "audiobook id"),
            library_id: get_indexed_field!(row, indices.library_id, "library_id"),
            path: get_indexed_path!(row, indices.path, "path"),
            title: get_field!(row, indices.title, "title", optional),
            author: get_field!(row, indices.author, "author", optional),
            narrator: get_field!(row, indices.narrator, "narrator", optional),
            description: get_field!(row, indices.description, "description", optional),
            duration_seconds: get_field!(
                row,
                indices.duration_seconds,
                "duration_seconds",
                optional
            ),
            size_bytes: get_field!(row, indices.size_bytes, "size_bytes", optional),
            cover_art: get_field!(row, indices.cover_art, "cover_art", optional),
            created_at: {
                let datetime_str: String =
                    get_indexed_field!(row, indices.created_at, "created_at");
                super::helpers::parse_datetime_string(&datetime_str)?
            },
            updated_at: {
                let datetime_str: String =
                    get_indexed_field!(row, indices.updated_at, "updated_at");
                super::helpers::parse_datetime_string(&datetime_str)?
            },
            selected: get_field!(row, indices.selected, "selected", optional).unwrap_or(false),
        })
    }
}

/// Column indices for optimized audiobook queries with custom column ordering
///
/// This struct defines where each audiobook field appears in a database row
/// when columns are not in the standard order. Use with `audiobook_from_row_indexed`
/// for performance-critical queries, JOINs, or custom SELECT statements.
///
/// # Example Usage
/// ```ignore
/// let indices = AudiobookColumnIndices {
///     id: 0,
///     title: 1,
///     author: 2,
///     // ... other fields as they appear in your custom query
/// };
/// let audiobook = RowMappers::audiobook_from_row_indexed(&row, &indices)?;
/// ```
#[derive(Debug, Clone)]
pub struct AudiobookColumnIndices {
    /// Column index for audiobook ID
    pub id: usize,
    /// Column index for library ID
    pub library_id: usize,
    /// Column index for file path
    pub path: usize,
    /// Column index for audiobook title
    pub title: usize,
    /// Column index for audiobook author
    pub author: usize,
    /// Column index for audiobook narrator
    pub narrator: usize,
    /// Column index for audiobook description
    pub description: usize,
    /// Column index for duration in seconds
    pub duration_seconds: usize,
    /// Column index for file size in bytes
    pub size_bytes: usize,
    /// Column index for cover art data
    pub cover_art: usize,
    /// Column index for creation timestamp
    pub created_at: usize,
    /// Column index for last update timestamp
    pub updated_at: usize,
    /// Column index for selection state
    pub selected: usize,
}

impl AudiobookColumnIndices {
    /// Standard column indices for full audiobook queries
    #[must_use]
    pub const fn standard() -> Self {
        Self {
            id: 0,
            library_id: 1,
            path: 2,
            title: 3,
            author: 4,
            narrator: 5,
            description: 6,
            duration_seconds: 7,
            size_bytes: 8,
            cover_art: 9,
            created_at: 10,
            updated_at: 11,
            selected: 12,
        }
    }
}

/// Common SQL query fragments
pub struct SqlQueries;

impl SqlQueries {
    /// Standard audiobook SELECT columns
    pub const AUDIOBOOK_COLUMNS: &'static str =
        "id, library_id, path, title, author, narrator, description, 
         duration_seconds, size_bytes, cover_art, created_at, updated_at, selected";

    /// Standard library SELECT columns
    pub const LIBRARY_COLUMNS: &'static str = "id, name, path, created_at";

    /// Standard progress SELECT columns
    pub const PROGRESS_COLUMNS: &'static str =
        "id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at";

    /// Base audiobook SELECT query without WHERE clause
    const BASE_AUDIOBOOK_QUERY: &'static str = "SELECT id, library_id, path, title, author, narrator, description, duration_seconds, size_bytes, cover_art, created_at, updated_at, selected FROM audiobooks";

    /// Base library SELECT query without WHERE clause  
    const BASE_LIBRARY_QUERY: &'static str = "SELECT id, name, path, created_at FROM libraries";

    /// Generate a standard audiobook SELECT query with optional WHERE clause
    #[must_use]
    pub fn audiobook_select(where_clause: Option<&str>) -> String {
        match where_clause {
            Some(clause) => format!("{} WHERE {}", Self::BASE_AUDIOBOOK_QUERY, clause),
            None => Self::BASE_AUDIOBOOK_QUERY.to_string(),
        }
    }

    /// Generate a standard library SELECT query with optional WHERE clause
    #[must_use]
    pub fn library_select(where_clause: Option<&str>) -> String {
        match where_clause {
            Some(clause) => format!("{} WHERE {}", Self::BASE_LIBRARY_QUERY, clause),
            None => Self::BASE_LIBRARY_QUERY.to_string(),
        }
    }

    /// Generate a standard progress SELECT query with optional WHERE clause
    #[must_use]
    pub fn progress_select(where_clause: Option<&str>) -> String {
        let base = format!("SELECT {} FROM progress", Self::PROGRESS_COLUMNS);
        match where_clause {
            Some(where_clause) => format!("{base} WHERE {where_clause}"),
            None => base,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_query_generation() {
        let audiobook_query = SqlQueries::audiobook_select(Some("library_id = ?1"));
        assert!(audiobook_query.contains("SELECT"));
        assert!(audiobook_query.contains("FROM audiobooks"));
        assert!(audiobook_query.contains("WHERE library_id = ?1"));

        let library_query = SqlQueries::library_select(None);
        assert!(library_query.contains("SELECT"));
        assert!(library_query.contains("FROM libraries"));
        assert!(!library_query.contains("WHERE"));
    }

    #[test]
    fn test_column_indices() {
        let indices = AudiobookColumnIndices::standard();
        assert_eq!(indices.id, 0);
        assert_eq!(indices.library_id, 1);
        assert_eq!(indices.selected, 12);
    }
}
