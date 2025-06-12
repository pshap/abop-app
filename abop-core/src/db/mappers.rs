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
    /// Map a database row to an Audiobook entity
    pub fn audiobook_from_row(row: &Row) -> DbResult<Audiobook> {
        Ok(Audiobook {
            id: get_row_field!(row, 0, "audiobook id"),
            library_id: get_row_field!(row, 1, "library_id"),
            path: get_row_path!(row, 2, "path"),
            title: get_row_field!(row, 3, "title"),
            author: get_row_field!(row, 4, "author"),
            narrator: get_row_field!(row, 5, "narrator"),
            description: get_row_field!(row, 6, "description"),
            duration_seconds: get_row_field!(row, 7, "duration_seconds"),
            size_bytes: get_row_field!(row, 8, "size_bytes"),
            cover_art: get_row_field!(row, 9, "cover_art"),
            created_at: parse_datetime_from_row(row, "created_at")?,
            updated_at: parse_datetime_from_row(row, "updated_at")?,
            selected: row.get(12).unwrap_or(false), // Default to false if not present
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

        let last_played: Option<SqliteDateTime> = get_row_field!(row, 4, "last_played");
        let created_at: SqliteDateTime = get_row_field!(row, 5, "created_at");
        let updated_at: SqliteDateTime = get_row_field!(row, 6, "updated_at");

        Ok(Progress {
            id: get_row_field!(row, 0, "progress id"),
            audiobook_id: get_row_field!(row, 1, "audiobook_id"),
            position_seconds: get_row_field!(row, 2, "position_seconds"),
            completed: get_row_field!(row, 3, "completed"),
            last_played: last_played.map(|dt| dt.into()),
            created_at: created_at.into(),
            updated_at: updated_at.into(),
        })
    }
    /// Map a database row to an Audiobook with specific column indices for optimized queries
    pub fn audiobook_from_row_indexed(
        row: &Row,
        indices: &AudiobookColumnIndices,
    ) -> DbResult<Audiobook> {
        Ok(Audiobook {
            id: get_indexed_field!(row, indices.id, "audiobook id"),
            library_id: get_indexed_field!(row, indices.library_id, "library_id"),
            path: get_indexed_path!(row, indices.path, "path"),
            title: get_indexed_field!(row, indices.title, "title"),
            author: get_indexed_field!(row, indices.author, "author"),
            narrator: get_indexed_field!(row, indices.narrator, "narrator"),
            description: get_indexed_field!(row, indices.description, "description"),
            duration_seconds: get_indexed_field!(row, indices.duration_seconds, "duration_seconds"),
            size_bytes: get_indexed_field!(row, indices.size_bytes, "size_bytes"),
            cover_art: get_indexed_field!(row, indices.cover_art, "cover_art"),
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
            selected: row.get(indices.selected).unwrap_or(false),
        })
    }
}

/// Column indices for optimized audiobook queries
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

    /// Generate a standard audiobook SELECT query with optional WHERE clause
    pub fn audiobook_select(where_clause: Option<&str>) -> String {
        let base = format!("SELECT {} FROM audiobooks", Self::AUDIOBOOK_COLUMNS);
        match where_clause {
            Some(where_clause) => format!("{base} WHERE {where_clause}"),
            None => base,
        }
    }

    /// Generate a standard library SELECT query with optional WHERE clause
    pub fn library_select(where_clause: Option<&str>) -> String {
        let base = format!("SELECT {} FROM libraries", Self::LIBRARY_COLUMNS);
        match where_clause {
            Some(where_clause) => format!("{base} WHERE {where_clause}"),
            None => base,
        }
    }

    /// Generate a standard progress SELECT query with optional WHERE clause
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
