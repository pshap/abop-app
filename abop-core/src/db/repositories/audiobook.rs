//! Audiobook repository for database operations
//!
//! This module handles all database operations related to audiobooks.

use rusqlite::{Connection, OptionalExtension, Row};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::super::error::DbResult;
use super::{EnhancedRepository, Repository};
use crate::models::audiobook::Audiobook;
use crate::db::error::DatabaseError;

/// Repository for audiobook-related database operations
#[derive(Debug)]
pub struct AudiobookRepository {
    conn: Arc<Mutex<Connection>>,
}

impl Repository for AudiobookRepository {
    fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }
}

impl AudiobookRepository {
    /// Create a new audiobook repository
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Add or update an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::ValidationFailed`] if the audiobook data fails validation.
    pub fn add(&self, audiobook: &Audiobook) -> DbResult<()> {
        if audiobook.id.is_empty() {
            return Err(DatabaseError::validation_failed("id", "Audiobook ID cannot be empty"));
        }
        if audiobook.library_id.is_empty() {
            return Err(DatabaseError::validation_failed("library_id", "Library ID cannot be empty"));
        }

        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        
        tx.execute(
            "INSERT INTO audiobooks (
                id, library_id, path, title, author, narrator, description,
                duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            (
                &audiobook.id,
                &audiobook.library_id,
                &audiobook.path.to_string_lossy(),
                &audiobook.title,
                &audiobook.author,
                &audiobook.narrator,
                &audiobook.description,
                &audiobook.duration_seconds,
                &audiobook.size_bytes,
                &audiobook.cover_art,
                &audiobook.created_at,
                &audiobook.updated_at,
                &audiobook.selected,
            ),
        )?;
        
        tx.commit()?;
        Ok(())
    }

    /// Find an audiobook by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_id(&self, id: &str) -> DbResult<Option<Audiobook>> {
        let mut conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT * FROM audiobooks WHERE id = ?1",
            [id],
            |row| self.map_row(row),
        )
        .optional()
        .map_err(|e| DatabaseError::internal(&format!("Failed to get audiobook: {}", e)))
    }

    /// Find audiobooks by library ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_library(&self, library_id: &str) -> DbResult<Vec<Audiobook>> {
        let mut conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM audiobooks WHERE library_id = ?1")?;
        let audiobooks = stmt
            .query_map([library_id], |row| self.map_row(row))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(audiobooks)
    }

    /// Find audiobooks by author
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_author(&self, author: &str) -> DbResult<Vec<Audiobook>> {
        let mut conn = self.conn.lock().unwrap();
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
    }

    /// Find an audiobook by its file path
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_path(&self, path: &std::path::Path) -> DbResult<Option<Audiobook>> {
        let path_str = path.to_string_lossy();
        let mut conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, library_id, path, title, author, narrator, description, 
                    duration_seconds, size_bytes, cover_art, created_at, updated_at 
             FROM audiobooks WHERE path = ?1",
        )?;
        let audiobook = stmt.query_row([path_str.as_ref()], |row| {
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
        })?;
        Ok(Some(audiobook))
    }

    /// Get all audiobooks
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_all(&self) -> DbResult<Vec<Audiobook>> {
        let mut conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM audiobooks ORDER BY title")?;
        let audiobooks = stmt
            .query_map([], |row| self.map_row(row))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DatabaseError::internal(&format!("Failed to collect results: {}", e)))?;
        Ok(audiobooks)
    }

    /// Update an audiobook's metadata
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::validation_failed`] if the audiobook data fails validation.
    pub fn update(&self, audiobook: &Audiobook) -> DbResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        
        tx.execute(
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
            (
                &audiobook.id,
                &audiobook.library_id,
                &audiobook.path.to_string_lossy(),
                &audiobook.title,
                &audiobook.author,
                &audiobook.narrator,
                &audiobook.description,
                &audiobook.duration_seconds,
                &audiobook.size_bytes,
                &audiobook.cover_art,
                &audiobook.updated_at,
                &audiobook.selected,
            ),
        )?;
        
        tx.commit()?;
        Ok(())
    }

    /// Delete an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete(&self, id: &str) -> DbResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        let mut stmt = tx.prepare("DELETE FROM audiobooks WHERE id = ?1")?;
        stmt.execute([id])?;
        drop(stmt); // Ensure stmt is dropped before tx.commit
        tx.commit()?;
        Ok(())
    }

    /// Delete all audiobooks in a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete_by_library(&self, library_id: &str) -> DbResult<usize> {
        let mut conn = self.conn.lock().unwrap();
        let count = conn
            .prepare("DELETE FROM audiobooks WHERE library_id = ?1")?
            .execute([library_id])?;
        Ok(count)
    }

    /// Count audiobooks in a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn count_by_library(&self, library_id: &str) -> DbResult<usize> {
        let mut conn = self.conn.lock().unwrap();
        let count: usize = conn
            .query_row(
                "SELECT COUNT(*) FROM audiobooks WHERE library_id = ?1",
                [library_id],
                |row| row.get(0),
            )?;
        Ok(count)
    }

    /// Check if an audiobook exists by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn exists(&self, id: &str) -> DbResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        let count: usize = conn
            .query_row(
                "SELECT COUNT(*) FROM audiobooks WHERE id = ?1",
                [id],
                |row| row.get(0),
            )?;
        Ok(count > 0)
    }

    /// Add or update multiple audiobooks in a single transaction
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::validation_failed`] if any audiobook data fails validation.
    pub fn batch_add(&self, audiobooks: &[Audiobook]) -> DbResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        
        for audiobook in audiobooks {
            tx.execute(
                "INSERT INTO audiobooks (
                    id, library_id, path, title, author, narrator, description,
                    duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                (
                    &audiobook.id,
                    &audiobook.library_id,
                    &audiobook.path.to_string_lossy(),
                    &audiobook.title,
                    &audiobook.author,
                    &audiobook.narrator,
                    &audiobook.description,
                    &audiobook.duration_seconds,
                    &audiobook.size_bytes,
                    &audiobook.cover_art,
                    &audiobook.created_at,
                    &audiobook.updated_at,
                    &audiobook.selected,
                ),
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }

    /// Delete multiple audiobooks in a single transaction
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn batch_delete(&self, ids: &[String]) -> DbResult<usize> {
        if ids.is_empty() {
            return Ok(0);
        }

        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        let mut count = 0;

        for id in ids {
            let mut stmt = tx.prepare("DELETE FROM audiobooks WHERE id = ?1")?;
            count += stmt.execute([id])?;
        }
        // No need to drop stmt explicitly here as it is scoped in the loop
        tx.commit()?;
        Ok(count)
    }

    /// Find multiple audiobooks by their IDs
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_ids(&self, ids: &[String]) -> DbResult<Vec<Audiobook>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT * FROM audiobooks WHERE id IN ({}) ORDER BY title",
            placeholders
        );

        let mut conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&query)?;
        let audiobooks = stmt
            .query_map(rusqlite::params_from_iter(ids.iter()), |row| self.map_row(row))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DatabaseError::internal(&format!("Failed to collect results: {}", e)))?;
        Ok(audiobooks)
    }

    /// Get an audiobook by ID
    pub fn get(&self, id: &str) -> DbResult<Option<Audiobook>> {
        self.find_by_id(id)
    }

    /// Get all audiobooks in a library
    pub fn get_by_library(&self, library_id: &str) -> DbResult<Vec<Audiobook>> {
        self.find_by_library(library_id)
    }

    /// Add or update an audiobook (upsert)
    pub fn upsert(&self, audiobook: &Audiobook) -> DbResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO audiobooks (
                id, library_id, path, title, author, narrator, description,
                duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            (
                &audiobook.id,
                &audiobook.library_id,
                &audiobook.path.to_string_lossy(),
                &audiobook.title,
                &audiobook.author,
                &audiobook.narrator,
                &audiobook.description,
                &audiobook.duration_seconds,
                &audiobook.size_bytes,
                &audiobook.cover_art,
                &audiobook.created_at,
                &audiobook.updated_at,
                &audiobook.selected,
            ),
        )?;
        Ok(())
    }

    fn map_row(&self, row: &Row) -> Result<Audiobook, rusqlite::Error> {
        Ok(Audiobook {
            id: row.get("id")?,
            library_id: row.get("library_id")?,
            path: PathBuf::from(row.get::<_, String>("path")?),
            title: row.get("title")?,
            author: row.get("author")?,
            narrator: row.get("narrator")?,
            description: row.get("description")?,
            duration_seconds: row.get("duration_seconds")?,
            size_bytes: row.get("size_bytes")?,
            cover_art: row.get("cover_art")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
            selected: row.get("selected")?,
        })
    }
}

impl EnhancedRepository for AudiobookRepository {}

impl Clone for AudiobookRepository {
    fn clone(&self) -> Self {
        Self {
            conn: self.conn.clone(),
        }
    }
}
