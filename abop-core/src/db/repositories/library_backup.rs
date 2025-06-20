//! Library repository for database operations
//!
//! This module handles all database operations related to libraries.

use rusqlite::{Connection, OptionalExtension};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

use super::super::error::{DatabaseError, DbResult};
use super::{EnhancedRepository, Repository};
use crate::models::Library;
use crate::db::EnhancedConnection;

/// Repository for library-related database operations
pub struct LibraryRepository {
    enhanced_connection: Arc<EnhancedConnection>,
}

impl LibraryRepository {
    /// Create a new library repository
    pub const fn new(enhanced_connection: Arc<EnhancedConnection>) -> Self {
        Self { enhanced_connection }
    }

    /// Add a new library to the database
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::DuplicateEntry`] if a library with the same name already exists.
    /// Returns [`DatabaseError::ValidationFailed`] if the library data fails validation.
    pub fn create<P: AsRef<Path> + Send + 'static>(&self, name: &str, path: P) -> DbResult<Library> {
        let id = Uuid::new_v4().to_string();
        let name = name.to_string();
        let path_str = path.as_ref().to_string_lossy().to_string();
        let path_buf = path.as_ref().to_path_buf();

        self.execute_query(move |conn| {
            // Check if library with same name already exists
            let existing_check = conn
                .query_row("SELECT id FROM libraries WHERE name = ?1", [&name], |row| {
                    row.get::<_, String>(0)
                })
                .optional();

            if let Ok(Some(_)) = existing_check {
                return Err(rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                    Some(format!("Library with name '{}' already exists", name)),
                ));
            }

            // Insert the new library
            conn.execute(
                "INSERT INTO libraries (id, name, path) VALUES (?1, ?2, ?3)",
                (&id, &name, &path_str),
            )?;

            Ok(Library {
                id,
                name,
                path: path_buf,
            })
        })
        .map_err(|e| {
            // Convert constraint violations to our specific error type
            match &e {
                DatabaseError::Sqlite(rusqlite::Error::SqliteFailure(err, Some(_msg)))
                    if err.code == rusqlite::ErrorCode::ConstraintViolation =>
                {
                    DatabaseError::duplicate_entry("Library", "name", &name)
                }
                _ => e,
            }
        })
    }

    /// Find a library by its ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_id(&self, id: &str) -> DbResult<Option<Library>> {
        let id = id.to_string();
        self.execute_query(move |conn| {
            conn.query_row(
                "SELECT id, name, path FROM libraries WHERE id = ?1",
                [id],
                |row| {
                    Ok(Library {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                    })
                },
            )
            .optional()
        })
    }

    /// Find a library by its name
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_name(&self, name: &str) -> DbResult<Option<Library>> {
        let name = name.to_string();
        self.execute_query(move |conn| {
            conn.query_row(
                "SELECT id, name, path FROM libraries WHERE name = ?1",
                [name],
                |row| {
                    Ok(Library {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                    })
                },
            )
            .optional()
        })
    }

    /// Get all libraries
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_all(&self) -> DbResult<Vec<Library>> {
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare("SELECT id, name, path FROM libraries ORDER BY name")?;
            let libraries = stmt
                .query_map([], |row| {
                    Ok(Library {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        path: PathBuf::from(row.get::<_, String>(2)?),
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(libraries)
        })
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
        self.execute_query(|conn| {
            let rows_affected = conn.execute(
                "UPDATE libraries SET name = ?1, path = ?2 WHERE id = ?3",
                (name, path_str.as_ref(), id),
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete a library
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    /// Returns [`DatabaseError::ConstraintViolation`] if the library cannot be deleted due to dependent audiobooks.
    pub fn delete(&self, id: &str) -> DbResult<bool> {
        self.execute_query(|conn| {
            // Check if library has audiobooks
            let audiobook_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM audiobooks WHERE library_id = ?1",
                [id],
                |row| row.get(0),
            )?;

            if audiobook_count > 0 {
                return Err(rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                    Some(format!(
                        "Cannot delete library: {audiobook_count} audiobooks depend on it"
                    )),
                ));
            }

            let rows_affected = conn.execute("DELETE FROM libraries WHERE id = ?1", [id])?;
            Ok(rows_affected > 0)
        })
        .map_err(|e| match &e {
            DatabaseError::Sqlite(rusqlite::Error::SqliteFailure(err, Some(msg)))
                if err.code == rusqlite::ErrorCode::ConstraintViolation =>
            {
                DatabaseError::ConstraintViolation {
                    message: msg.clone(),
                }
            }
            _ => e,
        })
    }

    /// Check if a library exists by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn exists(&self, id: &str) -> DbResult<bool> {
        self.execute_query(|conn| {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM libraries WHERE id = ?1",
                [id],
                |row| row.get(0),
            )?;
            Ok(count > 0)
        })
    }
}

impl Repository for LibraryRepository {
    fn connect(&self) -> &Arc<EnhancedConnection> {
        &self.enhanced_connection
    }
    
    #[deprecated(note = "use connect() instead")]
    fn get_connection(&self) -> &Arc<EnhancedConnection> {
        self.connect()
    }
}

impl EnhancedRepository for LibraryRepository {}
