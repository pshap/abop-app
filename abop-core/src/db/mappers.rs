//! Database row mapping utilities
//!
//! This module provides centralized, consistent row mapping functions for all database entities.

use super::error::{DatabaseError, DbResult};
use super::helpers::parse_datetime_from_row;
use crate::models::{Audiobook, Library, Progress};
use rusqlite::Row;
use std::path::PathBuf;

/// Centralized row mapping utilities
pub struct RowMappers;

impl RowMappers {
    /// Map a database row to an Audiobook entity
    pub fn audiobook_from_row(row: &Row) -> DbResult<Audiobook> {
        Ok(Audiobook {
            id: row.get(0).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get audiobook id: {e}"),
            })?,
            library_id: row.get(1).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get library_id: {e}"),
            })?,
            path: {
                let path_str: String = row.get(2).map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to get path: {e}"),
                })?;
                PathBuf::from(path_str)
            },
            title: row.get(3).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get title: {e}"),
            })?,
            author: row.get(4).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get author: {e}"),
            })?,
            narrator: row.get(5).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get narrator: {e}"),
            })?,
            description: row.get(6).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get description: {e}"),
            })?,
            duration_seconds: row.get(7).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get duration_seconds: {e}"),
            })?,
            size_bytes: row.get(8).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get size_bytes: {e}"),
            })?,
            cover_art: row.get(9).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get cover_art: {e}"),
            })?,
            created_at: parse_datetime_from_row(row, "created_at")?,
            updated_at: parse_datetime_from_row(row, "updated_at")?,
            selected: row.get(12).unwrap_or(false), // Default to false if not present
        })
    }
    /// Map a database row to a Library entity
    pub fn library_from_row(row: &Row) -> DbResult<Library> {
        Ok(Library {
            id: row.get(0).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get library id: {e}"),
            })?,
            name: row.get(1).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get library name: {e}"),
            })?,
            path: {
                let path_str: String = row.get(2).map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to get library path: {e}"),
                })?;
                PathBuf::from(path_str)
            },
        })
    }

    /// Map a database row to a Progress entity
    pub fn progress_from_row(row: &Row) -> DbResult<Progress> {
        use crate::db::datetime_serde::SqliteDateTime;

        let last_played: Option<SqliteDateTime> =
            row.get(4).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get last_played: {e}"),
            })?;
        let created_at: SqliteDateTime =
            row.get(5).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get created_at: {e}"),
            })?;
        let updated_at: SqliteDateTime =
            row.get(6).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get updated_at: {e}"),
            })?;

        Ok(Progress {
            id: row.get(0).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get progress id: {e}"),
            })?,
            audiobook_id: row.get(1).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get audiobook_id: {e}"),
            })?,
            position_seconds: row.get(2).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get position_seconds: {e}"),
            })?,
            completed: row.get(3).map_err(|e| DatabaseError::ExecutionFailed {
                message: format!("Failed to get completed: {e}"),
            })?,
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
            id: row
                .get(indices.id)
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to get audiobook id: {e}"),
                })?,
            library_id: row.get(indices.library_id).map_err(|e| {
                DatabaseError::ExecutionFailed {
                    message: format!("Failed to get library_id: {e}"),
                }
            })?,
            path: {
                let path_str: String =
                    row.get(indices.path)
                        .map_err(|e| DatabaseError::ExecutionFailed {
                            message: format!("Failed to get path: {e}"),
                        })?;
                PathBuf::from(path_str)
            },
            title: row
                .get(indices.title)
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to get title: {e}"),
                })?,
            author: row
                .get(indices.author)
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to get author: {e}"),
                })?,
            narrator: row
                .get(indices.narrator)
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to get narrator: {e}"),
                })?,
            description: row.get(indices.description).map_err(|e| {
                DatabaseError::ExecutionFailed {
                    message: format!("Failed to get description: {e}"),
                }
            })?,
            duration_seconds: row.get(indices.duration_seconds).map_err(|e| {
                DatabaseError::ExecutionFailed {
                    message: format!("Failed to get duration_seconds: {e}"),
                }
            })?,
            size_bytes: row.get(indices.size_bytes).map_err(|e| {
                DatabaseError::ExecutionFailed {
                    message: format!("Failed to get size_bytes: {e}"),
                }
            })?,
            cover_art: row
                .get(indices.cover_art)
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to get cover_art: {e}"),
                })?,
            created_at: {
                let datetime_str: String =
                    row.get(indices.created_at)
                        .map_err(|e| DatabaseError::ExecutionFailed {
                            message: format!("Failed to get created_at: {e}"),
                        })?;
                super::helpers::parse_datetime_string(&datetime_str)?
            },
            updated_at: {
                let datetime_str: String =
                    row.get(indices.updated_at)
                        .map_err(|e| DatabaseError::ExecutionFailed {
                            message: format!("Failed to get updated_at: {e}"),
                        })?;
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
            Some(where_clause) => format!("{} WHERE {}", base, where_clause),
            None => base,
        }
    }

    /// Generate a standard library SELECT query with optional WHERE clause
    pub fn library_select(where_clause: Option<&str>) -> String {
        let base = format!("SELECT {} FROM libraries", Self::LIBRARY_COLUMNS);
        match where_clause {
            Some(where_clause) => format!("{} WHERE {}", base, where_clause),
            None => base,
        }
    }

    /// Generate a standard progress SELECT query with optional WHERE clause
    pub fn progress_select(where_clause: Option<&str>) -> String {
        let base = format!("SELECT {} FROM progress", Self::PROGRESS_COLUMNS);
        match where_clause {
            Some(where_clause) => format!("{} WHERE {}", base, where_clause),
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
