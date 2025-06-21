//! Audiobook repository for database operations
//!
//! This module handles all database operations related to audiobooks.

use rusqlite::OptionalExtension;
use std::path::PathBuf;
use std::sync::Arc;

use super::super::error::{DatabaseError, DbResult};
use super::{EnhancedRepository, Repository, RepositoryBase};
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

    /// Add or update an audiobook in the database
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::ValidationFailed`] if the audiobook data fails validation.
    pub fn upsert(&self, audiobook: &Audiobook) -> Result<()> {
        let audiobook = audiobook.clone();
        self.execute_query(move |conn| {
            let _rows_affected = conn.execute(
                "INSERT INTO audiobooks (
                    id, library_id, path, title, author, narrator, description,
                    duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                ON CONFLICT(id) DO UPDATE SET
                    library_id = excluded.library_id,
                    path = excluded.path,
                    title = excluded.title,
                    author = excluded.author,
                    narrator = excluded.narrator,
                    description = excluded.description,
                    duration_seconds = excluded.duration_seconds,
                    size_bytes = excluded.size_bytes,
                    cover_art = excluded.cover_art,
                    updated_at = excluded.updated_at,
                    selected = excluded.selected",
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

    /// Find an audiobook by its ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if the ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_id(&self, id: &str) -> Result<Option<Audiobook>> {
        if id.is_empty() {
            return Err(crate::AppError::Database(DatabaseError::validation_failed(
                "audiobook_id",
                "Audiobook ID cannot be empty",
            )));
        }
        let id = id.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description,
                 duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                 FROM audiobooks WHERE id = ?1",
            )?;
            let audiobook = stmt
                .query_row([&id], |row| {
                    let created_at: SqliteDateTime = row.get(10)?;
                    let updated_at: SqliteDateTime = row.get(11)?;

                    Ok(Audiobook {
                        id: row.get(0)?,
                        library_id: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                        title: row.get(3)?,
                        author: row.get(4)?,
                        narrator: row.get(5)?,
                        description: row.get(6)?,
                        duration_seconds: row.get(7)?,
                        size_bytes: row.get(8)?,
                        cover_art: row.get(9)?,
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
                        selected: row.get(12)?,
                    })
                })
                .optional()?;
            Ok(audiobook)
        })
        .map_err(AppError::from)
    }

    /// Find audiobooks by library ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if the library ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_library(&self, library_id: &str) -> Result<Vec<Audiobook>> {
        if library_id.is_empty() {
            return Err(crate::AppError::Database(DatabaseError::validation_failed(
                "library_id",
                "Library ID cannot be empty",
            )));
        }
        let library_id = library_id.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description,
                        duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                 FROM audiobooks WHERE library_id = ?1",
            )?;
            let audiobooks = stmt
                .query_map([&library_id], |row| {
                    let created_at: SqliteDateTime = row.get(10)?;
                    let updated_at: SqliteDateTime = row.get(11)?;

                    Ok(Audiobook {
                        id: row.get(0)?,
                        library_id: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                        title: row.get(3)?,
                        author: row.get(4)?,
                        narrator: row.get(5)?,
                        description: row.get(6)?,
                        duration_seconds: row.get(7)?,
                        size_bytes: row.get(8)?,
                        cover_art: row.get(9)?,
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
                        selected: row.get(12)?,
                    })
                })?
                .collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?;
            Ok(audiobooks)
        })
        .map_err(AppError::from)
    }

    /// Find audiobooks by library with pagination support
    ///
    /// # Arguments
    ///
    /// * `library_id` - The library ID to search in
    /// * `limit` - Maximum number of audiobooks to return (None for no limit)
    /// * `offset` - Number of audiobooks to skip (for pagination)
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_library_paginated(
        &self,
        library_id: &str,
        limit: Option<usize>,
        offset: usize,
    ) -> Result<Vec<Audiobook>> {
        let library_id = library_id.to_string();
        self.execute_query(move |conn| {
            let audiobooks = if let Some(limit_value) = limit {
                let mut stmt = conn.prepare(
                    "SELECT id, library_id, path, title, author, narrator, description,
                            duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                     FROM audiobooks WHERE library_id = ?1 
                     ORDER BY title ASC 
                     LIMIT ?2 OFFSET ?3"
                )?;
                let rows = stmt.query_map(rusqlite::params![library_id, limit_value as i64, offset as i64], |row| {
                    let created_at: SqliteDateTime = row.get(10)?;
                    let updated_at: SqliteDateTime = row.get(11)?;

                    Ok(Audiobook {
                        id: row.get(0)?,
                        library_id: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                        title: row.get(3)?,
                        author: row.get(4)?,
                        narrator: row.get(5)?,
                        description: row.get(6)?,
                        duration_seconds: row.get(7)?,
                        size_bytes: row.get(8)?,
                        cover_art: row.get(9)?,
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
                        selected: row.get(12)?,
                    })
                })?;
                rows.collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?
            } else {
                let mut stmt = conn.prepare(
                    "SELECT id, library_id, path, title, author, narrator, description,
                            duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                     FROM audiobooks WHERE library_id = ?1 
                     ORDER BY title ASC 
                     OFFSET ?2"
                )?;
                let rows = stmt.query_map(rusqlite::params![library_id, offset as i64], |row| {
                    let created_at: SqliteDateTime = row.get(10)?;
                    let updated_at: SqliteDateTime = row.get(11)?;

                    Ok(Audiobook {
                        id: row.get(0)?,
                        library_id: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                        title: row.get(3)?,
                        author: row.get(4)?,
                        narrator: row.get(5)?,
                        description: row.get(6)?,
                        duration_seconds: row.get(7)?,
                        size_bytes: row.get(8)?,
                        cover_art: row.get(9)?,
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
                        selected: row.get(12)?,
                    })
                })?;
                rows.collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?
            };
            Ok(audiobooks)
        })
        .map_err(AppError::from)
    }

    /// Count total audiobooks in a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if the library ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn count_by_library(&self, library_id: &str) -> Result<usize> {
        if library_id.is_empty() {
            return Err(crate::AppError::Database(DatabaseError::validation_failed(
                "library_id",
                "Library ID cannot be empty",
            )));
        }
        let library_id = library_id.to_string();
        self.execute_query(move |conn| {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM audiobooks WHERE library_id = ?1",
                [&library_id],
                |row| row.get(0),
            )?;
            Ok(count as usize)
        })
        .map_err(AppError::from)
    }

    /// Find audiobooks by author
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if the author string is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_author(&self, author: &str) -> Result<Vec<Audiobook>> {
        if author.is_empty() {
            return Err(AppError::Database(DatabaseError::validation_failed(
                "author",
                "Author cannot be empty",
            )));
        }
        let author = author.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description,
                        duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                 FROM audiobooks WHERE author LIKE ?1",
            )?;
            let audiobooks = stmt
                .query_map([&format!("%{author}%")], |row| {
                    let created_at: SqliteDateTime = row.get(10)?;
                    let updated_at: SqliteDateTime = row.get(11)?;

                    Ok(Audiobook {
                        id: row.get(0)?,
                        library_id: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                        title: row.get(3)?,
                        author: row.get(4)?,
                        narrator: row.get(5)?,
                        description: row.get(6)?,
                        duration_seconds: row.get(7)?,
                        size_bytes: row.get(8)?,
                        cover_art: row.get(9)?,
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
                        selected: row.get(12)?,
                    })
                })?
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(audiobooks)
        })
        .map_err(AppError::from)
    }

    /// Find an audiobook by its file path
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_path(&self, path: &str) -> Result<Option<Audiobook>> {
        if path.is_empty() {
            return Err(crate::AppError::Database(DatabaseError::validation_failed(
                "path",
                "Audiobook path cannot be empty",
            )));
        }
        let path = path.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description,
                        duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                 FROM audiobooks WHERE path = ?1",
            )?;
            let audiobook = stmt
                .query_row([&path], |row| {
                    let created_at: SqliteDateTime = row.get(10)?;
                    let updated_at: SqliteDateTime = row.get(11)?;

                    Ok(Audiobook {
                        id: row.get(0)?,
                        library_id: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                        title: row.get(3)?,
                        author: row.get(4)?,
                        narrator: row.get(5)?,
                        description: row.get(6)?,
                        duration_seconds: row.get(7)?,
                        size_bytes: row.get(8)?,
                        cover_art: row.get(9)?,
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
                        selected: row.get(12)?,
                    })
                })
                .optional()?;
            Ok(audiobook)
        })
        .map_err(AppError::from)
    }

    /// Get all audiobooks
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_all(&self) -> Result<Vec<Audiobook>> {
        self.execute_query(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description,
                        duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                 FROM audiobooks",
            )?;
            let audiobooks = stmt
                .query_map([], |row| {
                    let created_at: SqliteDateTime = row.get(10)?;
                    let updated_at: SqliteDateTime = row.get(11)?;

                    Ok(Audiobook {
                        id: row.get(0)?,
                        library_id: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                        title: row.get(3)?,
                        author: row.get(4)?,
                        narrator: row.get(5)?,
                        description: row.get(6)?,
                        duration_seconds: row.get(7)?,
                        size_bytes: row.get(8)?,
                        cover_art: row.get(9)?,
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
                        selected: row.get(12)?,
                    })
                })?
                .collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?;
            Ok(audiobooks)
        })
        .map_err(AppError::from)
    }

    /// Update an existing audiobook in the database
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    /// Returns [`DatabaseError::RecordNotFound`] if the audiobook doesn't exist.
    pub fn update(&self, audiobook: &Audiobook) -> DbResult<bool> {
        // Clone the data we need to avoid lifetime issues
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

        self.execute_query(move |conn| {
            let rows_affected = conn.execute(
                "UPDATE audiobooks SET
                    library_id = ?2,
                    path = ?3,
                    title = ?4,
                    author = ?5,
                    narrator = ?6,
                    description = ?7,
                    duration_seconds = ?8,
                    size_bytes = ?9,
                    cover_art = ?10,
                    updated_at = ?11,
                    selected = ?12
                WHERE id = ?1",
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
    /// Returns [`DatabaseError::ValidationFailed`] if the ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete(&self, id: &str) -> DbResult<bool> {
        if id.is_empty() {
            return Err(DatabaseError::validation_failed(
                "id",
                "Audiobook ID cannot be empty",
            ));
        }
        let id = id.to_string();
        self.execute_query(move |conn| {
            let rows_affected = conn.execute("DELETE FROM audiobooks WHERE id = ?1", [&id])?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete all audiobooks in a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete_by_library(&self, library_id: &str) -> DbResult<usize> {
        let library_id = library_id.to_string();
        self.execute_query(move |conn| {
            let rows_affected = conn.execute(
                "DELETE FROM audiobooks WHERE library_id = ?1",
                [&library_id],
            )?;
            Ok(rows_affected)
        })
    }

    /// Check if an audiobook exists
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if the ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn exists(&self, id: &str) -> DbResult<bool> {
        if id.is_empty() {
            return Err(DatabaseError::validation_failed(
                "id",
                "Audiobook ID cannot be empty",
            ));
        }
        let id = id.to_string();
        self.execute_query(move |conn| {
            let exists: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM audiobooks WHERE id = ?1)",
                [&id],
                |row| row.get(0),
            )?;
            Ok(exists)
        })
    }
}

impl RepositoryBase for AudiobookRepository {
    fn connect(&self) -> &Arc<EnhancedConnection> {
        &self.enhanced_connection
    }
}

impl EnhancedRepository for AudiobookRepository {}

#[cfg(test)]
mod tests;
