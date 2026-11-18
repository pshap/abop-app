//! CRUD operations for audiobook repository

use std::sync::Arc;

use super::queries::*;
use super::validation_operations::*;
use crate::db::error::DbResult;
use crate::db::repositories::{EnhancedRepository, Repository, RepositoryBase};
use crate::db::{
    EnhancedConnection,
    datetime_serde::{SqliteDateTime, datetime_to_sql},
};
use crate::error::{AppError, Result};
use crate::models::Audiobook;

/// Repository for audiobook-related database operations
pub struct AudiobookRepository {
    enhanced_connection: Arc<EnhancedConnection>,
}

impl AudiobookRepository {
    /// Create a new audiobook repository
    #[must_use]
    pub const fn new(enhanced_connection: Arc<EnhancedConnection>) -> Self {
        Self {
            enhanced_connection,
        }
    }

    /// Add or update an audiobook in database
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::ValidationFailed`] if audiobook data fails validation.
    pub fn upsert(&self, audiobook: &Audiobook) -> Result<()> {
        let audiobook = audiobook.clone();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let _rows_affected = conn.execute(
                UPSERT_AUDIOBOOK,
                rusqlite::params![
                    audiobook.id,
                    audiobook.library_id,
                    audiobook.path.to_string_lossy(),
                    audiobook.title,
                    audiobook.author,
                    audiobook.narrator,
                    audiobook.description,
                    audiobook.duration_seconds,
                    audiobook.size_bytes,
                    audiobook.cover_art,
                    SqliteDateTime::from(audiobook.created_at),
                    SqliteDateTime::from(audiobook.updated_at),
                    audiobook.selected,
                ],
            )?;
            Ok(())
        })
        .map_err(Into::into)
    }

    /// Update an existing audiobook in database
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL execution fails.
    /// Returns [`DatabaseError::RecordNotFound`] if audiobook doesn't exist.
    pub fn update(&self, audiobook: &Audiobook) -> DbResult<bool> {
        // Clone data we need to avoid lifetime issues
        let id = audiobook.id.clone();
        let library_id = audiobook.library_id.clone();
        let path = audiobook.path.to_string_lossy().to_string();
        let title = audiobook.title.clone();
        let author = audiobook.author.clone();
        let narrator = audiobook.narrator.clone();
        let description = audiobook.description.clone();
        let duration_seconds = audiobook.duration_seconds;
        let size_bytes = audiobook.size_bytes;
        let cover_art = audiobook.cover_art.clone();
        let updated_at = datetime_to_sql(&audiobook.updated_at);
        let selected = audiobook.selected;

        self.execute_query(move |conn: &rusqlite::Connection| {
            let rows_affected = conn.execute(
                UPDATE_AUDIOBOOK,
                rusqlite::params![
                    id,
                    library_id,
                    path,
                    title,
                    author,
                    narrator,
                    description,
                    duration_seconds,
                    size_bytes,
                    cover_art,
                    updated_at,
                    selected,
                ],
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete an audiobook by its ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn delete(&self, id: &str) -> DbResult<bool> {
        validate_id(id, "id")?;
        let id = id.to_string();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let rows_affected = conn.execute(DELETE_BY_ID, [&id])?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete all audiobooks in a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn delete_by_library(&self, library_id: &str) -> DbResult<usize> {
        let library_id = library_id.to_string();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let rows_affected = conn.execute(DELETE_BY_LIBRARY_ID, [&library_id])?;
            Ok(rows_affected)
        })
    }

    /// Check if an audiobook exists
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn exists(&self, id: &str) -> DbResult<bool> {
        validate_id(id, "id")?;
        let id = id.to_string();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let exists: bool =
                conn.query_row(EXISTS_BY_ID, [&id], |row: &rusqlite::Row| row.get(0))?;
            Ok(exists)
        })
    }

    /// Count total audiobooks in a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if library ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn count_by_library(&self, library_id: &str) -> Result<usize> {
        validate_library_id(library_id)?;
        let library_id = library_id.to_string();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let count: i64 =
                conn.query_row(COUNT_BY_LIBRARY_ID, [&library_id], |row: &rusqlite::Row| {
                    row.get(0)
                })?;
            Ok(count as usize)
        })
        .map_err(AppError::from)
    }
}

impl RepositoryBase for AudiobookRepository {
    fn connect(&self) -> &Arc<EnhancedConnection> {
        &self.enhanced_connection
    }
}

impl EnhancedRepository for AudiobookRepository {}
