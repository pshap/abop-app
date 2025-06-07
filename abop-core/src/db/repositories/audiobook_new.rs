//! Audiobook repository for database operations
//!
//! This module handles all database operations related to audiobooks.

use rusqlite::OptionalExtension;
use std::path::PathBuf;
use std::sync::Arc;

use super::super::error::DbResult;
use super::{EnhancedRepository, Repository};
use crate::db::{
    EnhancedConnection,
    datetime_serde::{datetime_from_sql, datetime_to_sql, optional_datetime_to_sql_output, SqliteDateTime},
};
use crate::error::{AppError, Result};
use crate::models::Audiobook;
use rusqlite::types::ToSql;

/// Repository for audiobook-related database operations
pub struct AudiobookRepository {
    enhanced_connection: Arc<EnhancedConnection>,
}

impl AudiobookRepository {
    /// Create a new audiobook repository
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
                    SqliteDateTime::from(audiobook.created_at).to_sql()?,
                    SqliteDateTime::from(audiobook.updated_at).to_sql()?,
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
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_id(&self, id: &str) -> Result<Option<Audiobook>> {
        let id = id.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description,
                 duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                 FROM audiobooks WHERE id = ?1",
            )?;
            let audiobook = stmt
                .query_row([&id], |row| {
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
                        created_at: row.get::<_, SqliteDateTime>()?.into(),
                        updated_at: row.get::<_, SqliteDateTime>()?.into(),
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
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_library(&self, library_id: &str) -> Result<Vec<Audiobook>> {
        let library_id = library_id.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description,
                        duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                 FROM audiobooks WHERE library_id = ?1",
            )?;
            let audiobooks = stmt
                .query_map([&library_id], |row| {
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
                        created_at: row.get::<_, SqliteDateTime>()?.into(),
                        updated_at: row.get::<_, SqliteDateTime>()?.into(),
                        selected: row.get(12)?,
                    })
                })?
                .collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?;
            Ok(audiobooks)
        })
        .map_err(AppError::from)
    }

    /// Find audiobooks by author
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_author(&self, author: &str) -> Result<Vec<Audiobook>> {
        let author = author.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description,
                        duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                 FROM audiobooks WHERE author LIKE ?1",
            )?;
            let audiobooks = stmt
                .query_map([&format!("%{author}%")], |row| {
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
                        created_at: row.get::<_, SqliteDateTime>()?.into(),
                        updated_at: row.get::<_, SqliteDateTime>()?.into(),
                        selected: row.get(12)?,
                    })
                })?
                .collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?;
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
        let path = path.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description,
                        duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                 FROM audiobooks WHERE path = ?1",
            )?;
            let audiobook = stmt
                .query_row([&path], |row| {
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
                        created_at: row.get::<_, SqliteDateTime>()?.into(),
                        updated_at: row.get::<_, SqliteDateTime>()?.into(),
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
                        created_at: row.get::<_, SqliteDateTime>()?.into(),
                        updated_at: row.get::<_, SqliteDateTime>()?.into(),
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
        let updated_at = SqliteDateTime::from().to_sql()?;
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
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete(&self, id: &str) -> DbResult<bool> {
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

    /// Count audiobooks in a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn count_by_library(&self, library_id: &str) -> DbResult<usize> {
        let library_id = library_id.to_string();
        self.execute_query(move |conn| {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM audiobooks WHERE library_id = ?1",
                [&library_id],
                |row| row.get(0),
            )?;
            Ok(count as usize)
        })
    }

    /// Check if an audiobook exists
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn exists(&self, id: &str) -> DbResult<bool> {
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

impl Repository for AudiobookRepository {
    fn get_connection(&self) -> &Arc<EnhancedConnection> {
        &self.enhanced_connection
    }
}

impl EnhancedRepository for AudiobookRepository {}

