//! Library repository for database operations
//!
//! This module handles all database operations related to libraries.

use std::path::Path;
use std::sync::{Arc, Mutex};

use super::super::error::{DatabaseError, DbResult};
use super::{EnhancedRepository, Repository};
use crate::models::library::Library;
use rusqlite::OptionalExtension;

/// Repository for library-related database operations
#[derive(Debug)]
pub struct LibraryRepository {
    connection: Arc<Mutex<rusqlite::Connection>>,
}

impl Repository for LibraryRepository {
    fn connection(&self) -> &Arc<Mutex<rusqlite::Connection>> {
        &self.connection
    }
}

impl LibraryRepository {
    /// Create a new library repository
    pub const fn new(connection: Arc<Mutex<rusqlite::Connection>>) -> Self {
        Self { connection }
    }

    /// Add or update a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::DuplicateEntry`] if a library with the same name already exists.
    pub fn upsert(&self, library: &Library) -> DbResult<()> {
        let mut conn = self.connection.lock().unwrap();
        // Check if a library with the same name exists
        let exists = conn.query_row(
            "SELECT 1 FROM libraries WHERE name = ?1 AND id != ?2",
            (&library.name, &library.id),
            |_| Ok(true),
        ).optional()?.is_some();

        if exists {
            return Err(DatabaseError::duplicate_entry("Library", "name", &library.name));
        }

        conn.execute(
            "INSERT OR REPLACE INTO libraries (id, name, path)
             VALUES (?1, ?2, ?3)",
            (
                &library.id,
                &library.name,
                &library.path.to_string_lossy(),
            ),
        )?;
        Ok(())
    }

    /// Find a library by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_id(&self, id: &str) -> DbResult<Option<Library>> {
        let mut conn = self.connection.lock().unwrap();
        Ok(conn.query_row(
            "SELECT id, name, path FROM libraries WHERE id = ?1",
            [id],
            |row| {
                Ok(Library {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: std::path::PathBuf::from(row.get::<_, String>(2)?),
                })
            },
        ).optional()?)
    }

    /// Find a library by name
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_name(&self, name: &str) -> DbResult<Option<Library>> {
        let mut conn = self.connection.lock().unwrap();
        Ok(conn.query_row(
            "SELECT id, name, path FROM libraries WHERE name = ?1",
            [name],
            |row| {
                Ok(Library {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: std::path::PathBuf::from(row.get::<_, String>(2)?),
                })
            },
        ).optional()?)
    }

    /// Get all libraries
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_all(&self) -> DbResult<Vec<Library>> {
        let mut conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, path FROM libraries ORDER BY name",
        )?;

        let libraries = stmt
            .query_map([], |row| {
                Ok(Library {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: std::path::PathBuf::from(row.get::<_, String>(2)?),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(libraries)
    }

    /// Update a library's information
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::ValidationFailed`] if the library data fails validation.
    pub fn update(&self, id: &str, name: &str, path: &Path) -> DbResult<bool> {
        let path_str = path.to_string_lossy();
        let mut conn = self.connection.lock().unwrap();
        let rows_affected = conn.execute(
            "UPDATE libraries SET name = ?1, path = ?2 WHERE id = ?3",
            (name, path_str.as_ref(), id),
        )?;
        Ok(rows_affected > 0)
    }

    /// Delete a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn delete(&self, id: &str) -> DbResult<bool> {
        let mut conn = self.connection.lock().unwrap();
        let tx = conn.transaction()?;
        let mut stmt = tx.prepare("DELETE FROM libraries WHERE id = ?1")?;
        let count = stmt.execute([id])?;
        drop(stmt);
        tx.commit()?;
        Ok(count > 0)
    }

    /// Check if a library exists by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn exists(&self, id: &str) -> DbResult<bool> {
        let mut conn = self.connection.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM libraries WHERE id = ?1",
            [id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Add a new library to the database
    pub fn add(&self, library: &Library) -> DbResult<()> {
        let mut conn = self.connection.lock().unwrap();
        // Check if library with same name already exists
        let existing_check = conn
            .query_row("SELECT id FROM libraries WHERE name = ?1", [&library.name], |row| {
                row.get::<_, String>(0)
            })
            .optional();

        if let Ok(Some(_)) = existing_check {
            return Err(DatabaseError::duplicate_entry("Library", "name", &library.name));
        }

        let tx = conn.transaction()?;
        let mut stmt = tx.prepare(
            "INSERT INTO libraries (id, name, path) VALUES (?1, ?2, ?3)"
        )?;

        stmt.execute((
            &library.id,
            &library.name,
            library.path.to_str().unwrap(),
        ))?;
        drop(stmt);
        tx.commit()?;
        Ok(())
    }

    /// Get a library by ID
    pub fn get(&self, id: &str) -> DbResult<Option<Library>> {
        self.find_by_id(id)
    }

    /// Get all libraries
    pub fn get_all(&self) -> DbResult<Vec<Library>> {
        self.find_all()
    }
}

impl Clone for LibraryRepository {
    fn clone(&self) -> Self {
        Self {
            connection: Arc::clone(&self.connection),
        }
    }
}

impl EnhancedRepository for LibraryRepository {}
