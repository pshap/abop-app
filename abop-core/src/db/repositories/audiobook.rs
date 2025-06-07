//! Audiobook repository for database operations
//!
//! This module handles all database operations related to audiobooks.

use rusqlite::{Connection, OptionalExtension};
use std::path::PathBuf;
use std::sync::Arc;

use super::super::error::{DbResult, DatabaseError};
use super::{EnhancedRepository, Repository};
use crate::models::Audiobook;
use crate::db::EnhancedConnection;

/// Repository for audiobook-related database operations
pub struct AudiobookRepository {
    enhanced_connection: Arc<EnhancedConnection>,
}

impl AudiobookRepository {
    /// Create a new audiobook repository
    pub const fn new(enhanced_connection: Arc<EnhancedConnection>) -> Self {
        Self { enhanced_connection }
    }
    /// Add or update an audiobook in the database
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::ValidationFailed`] if the audiobook data fails validation.
    pub fn upsert(&self, audiobook: &Audiobook) -> DbResult<()> {
        let audiobook = audiobook.clone();
        self.execute_query(move |conn| {
            conn.execute(
                "INSERT OR REPLACE INTO audiobooks (
                    id, library_id, path, title, author, narrator, 
                    description, duration_seconds, size_bytes, cover_art,
                    created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                (
                    &audiobook.id,
                    &audiobook.library_id,
                    &audiobook.path.to_string_lossy(),
                    &audiobook.title,
                    &audiobook.author,
                    &audiobook.narrator,
                    &audiobook.description,
                    audiobook.duration_seconds,
                    audiobook.size_bytes,
                    &audiobook.cover_art,
                    &audiobook.created_at,
                    &audiobook.updated_at,
                ),
            )?;
            Ok(())
        })
    }

    /// Find an audiobook by its ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_id(&self, id: &str) -> DbResult<Option<Audiobook>> {
        let id = id.to_string();
        self.execute_query(move |conn| {
            conn.query_row(
                "SELECT id, library_id, path, title, author, narrator, description, 
                        duration_seconds, size_bytes, cover_art, created_at, updated_at 
                 FROM audiobooks WHERE id = ?1",
                [id],
                |row| {
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
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                        selected: false,
                    })
                },
            )
            .optional()
        })
    }

    /// Find audiobooks by library ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_library(&self, library_id: &str) -> DbResult<Vec<Audiobook>> {
        let library_id = library_id.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description, 
                        duration_seconds, size_bytes, cover_art, created_at, updated_at 
                 FROM audiobooks WHERE library_id = ?1 ORDER BY title",
            )?;

            let audiobooks = stmt
                .query_map([library_id], |row| {
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
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                        selected: false,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(audiobooks)
        })
    }

    /// Find audiobooks by author
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_author(&self, author: &str) -> DbResult<Vec<Audiobook>> {
        let author = author.to_string();
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description, 
                        duration_seconds, size_bytes, cover_art, created_at, updated_at 
                 FROM audiobooks WHERE author LIKE ?1 ORDER BY title",
            )?;

            let pattern = format!("%{author}%");
            let audiobooks = stmt
                .query_map([&pattern], |row| {
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
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                        selected: false,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(audiobooks)
        })
    }

    /// Find an audiobook by its file path
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_path(&self, path: &std::path::Path) -> DbResult<Option<Audiobook>> {
        let path_str = path.to_string_lossy().to_string();
        self.execute_query(move |conn| {
            conn.query_row(
                "SELECT id, library_id, path, title, author, narrator, description, 
                        duration_seconds, size_bytes, cover_art, created_at, updated_at 
                 FROM audiobooks WHERE path = ?1",
                [path_str],
                |row| {
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
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                        selected: false,
                    })
                },
            )
            .optional()
        })
    }

    /// Get all audiobooks
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_all(&self) -> DbResult<Vec<Audiobook>> {
        self.execute_query(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, library_id, path, title, author, narrator, description, 
                        duration_seconds, size_bytes, cover_art, created_at, updated_at 
                 FROM audiobooks ORDER BY title",
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
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                        selected: false,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(audiobooks)
        })
    }

    /// Update an audiobook's metadata
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::ValidationFailed`] if the audiobook data fails validation.
    pub fn update(&self, audiobook: &Audiobook) -> DbResult<bool> {
        let audiobook = audiobook.clone();
        self.execute_query(move |conn| {
            let rows_affected = conn.execute(
                "UPDATE audiobooks SET 
                    library_id = ?1, path = ?2, title = ?3, author = ?4, narrator = ?5, 
                    description = ?6, duration_seconds = ?7, size_bytes = ?8, cover_art = ?9,
                    updated_at = ?10
                 WHERE id = ?11",
                (
                    &audiobook.library_id,
                    &audiobook.path.to_string_lossy(),
                    &audiobook.title,
                    &audiobook.author,
                    &audiobook.narrator,
                    &audiobook.description,
                    audiobook.duration_seconds,
                    audiobook.size_bytes,
                    &audiobook.cover_art,
                    &audiobook.updated_at,
                    &audiobook.id,
                ),
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete(&self, id: &str) -> DbResult<bool> {
        let id = id.to_string();
        self.execute_query(move |conn| {
            let rows_affected = conn.execute("DELETE FROM audiobooks WHERE id = ?1", [id])?;
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
            let rows_affected =
                conn.execute("DELETE FROM audiobooks WHERE library_id = ?1", [library_id])?;
            Ok(rows_affected)
        })
    }

    /// Count audiobooks in a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn count_by_library(&self, library_id: &str) -> DbResult<usize> {
        let library_id = library_id.to_string();
        self.execute_query(move |conn| {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM audiobooks WHERE library_id = ?1",
                [library_id],
                |row| row.get(0),
            )?;
            Ok(count)
        })
        .map(|count| count as usize)
    }

    /// Check if an audiobook exists by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn exists(&self, id: &str) -> DbResult<bool> {
        let id = id.to_string();
        self.execute_query(move |conn| {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM audiobooks WHERE id = ?1",
                [id],
                |row| row.get(0),
            )?;
            Ok(count > 0)
        })
    }
}

impl Repository for AudiobookRepository {
    fn execute_query_enhanced<F, R>(&self, f: F) -> DbResult<R>
    where
        F: FnOnce(&Connection) -> Result<R, rusqlite::Error> + Send + 'static,
        R: Send + 'static,
    {
        // We need to work around the fact that with_connection expects Fn but we have FnOnce
        // We'll use a thread-safe approach by wrapping the operation
        let operation = std::sync::Arc::new(std::sync::Mutex::new(Some(f)));
        self.enhanced_connection.with_connection(move |conn| {
            let mut op_guard = operation.lock().map_err(|_| {
                DatabaseError::ConnectionFailed("Failed to acquire operation lock".to_string())
            })?;
            let op = op_guard.take().ok_or_else(|| {
                DatabaseError::ConnectionFailed("Operation already consumed".to_string())
            })?;
            drop(op_guard); // Release the lock before calling the operation
            op(conn).map_err(DatabaseError::from)
        })
    }
}

impl EnhancedRepository for AudiobookRepository {
    fn get_enhanced_connection(&self) -> Option<&Arc<EnhancedConnection>> {
        Some(&self.enhanced_connection)
    }
}
