//! Legacy path-based CRUD operations.
//!
//! These methods predate the repository pattern and are retained for
//! backward compatibility. Prefer repository usage for new code.
//!
//! Deprecated in v0.2.0: scheduled for removal after migrating all callers.

use std::path::Path;
use tracing::instrument;

use crate::{
    error::{AppError, Result},
    models::{Audiobook, Library},
};

use super::facade::Database;
use super::{
    error::DatabaseError,
    mappers::{RowMappers, SqlQueries},
};

impl Database {
    /// Adds a library to the database (direct insert)
    #[instrument(skip(self, library))]
    pub fn add_library(&self, library: &Library) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let library_name = library.name.clone();
        let library_path = library.path.to_string_lossy().to_string();
        let id_clone = id.clone();
        let library_path_for_cache = library_path.clone();

        self.operations
            .execute(move |conn| {
                conn.execute(
                    "INSERT INTO libraries (id, name, path) VALUES (?1, ?2, ?3)",
                    [
                        &id_clone as &dyn rusqlite::ToSql,
                        &library_name,
                        &library_path,
                    ],
                )
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to insert library: {e}"),
                })?;
                Ok(())
            })
            .map_err(AppError::Database)?;

        // Update cache
        self.state
            .lock()
            .unwrap()
            .library_cache
            .insert(library_path_for_cache, id.clone());

        Ok(id)
    }

    /// Gets a library ID from the database, using cache if available (internal helper)
    #[instrument(skip(self, path))]
    pub(crate) fn get_library_id(&self, path: &Path) -> Result<String> {
        let path_str = path.to_string_lossy().to_string();

        // Check cache first
        if let Some(id) = self.state.lock().unwrap().library_cache.get(&path_str) {
            return Ok(id.clone());
        }

        // Not in cache, query the database
        let path_str_for_query = path_str.clone();
        let id = self
            .operations
            .execute(move |conn| {
                use rusqlite::OptionalExtension;
                let mut stmt = conn.prepare("SELECT id FROM libraries WHERE path = ?")?;
                let id: Option<String> = stmt
                    .query_row([&path_str_for_query], |row| row.get(0))
                    .optional()
                    .map_err(|e| DatabaseError::ExecutionFailed {
                        message: format!("Failed to query library ID: {e}"),
                    })?;
                Ok(id)
            })
            .map_err(AppError::Database)?;

        if let Some(id) = id {
            // Update cache
            self.state
                .lock()
                .unwrap()
                .library_cache
                .insert(path_str, id.clone());
            Ok(id)
        } else {
            Err(AppError::Database(DatabaseError::NotFound {
                entity: "Library".to_string(),
                id: path_str,
            }))
        }
    }

    /// Adds an audiobook to the database (direct insert)
    #[instrument(skip(self, audiobook))]
    pub fn add_audiobook(&self, audiobook: &Audiobook) -> Result<()> {
        let library_id = self.get_library_id(audiobook.path.parent().unwrap())?;

        let audiobook_clone = audiobook.clone();
        let library_id_clone = library_id.clone();
        self.operations
            .execute(move |conn| {
                let mut stmt = conn.prepare(
                    "INSERT OR REPLACE INTO audiobooks 
                (id, library_id, path, title, author, narrator, description, 
                 duration_seconds, size_bytes, cover_art, created_at, updated_at) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                )?;

                stmt.execute(rusqlite::params![
                    audiobook_clone.id,
                    library_id_clone,
                    audiobook_clone.path.to_string_lossy(),
                    audiobook_clone.title,
                    audiobook_clone.author,
                    audiobook_clone.narrator,
                    audiobook_clone.description,
                    audiobook_clone.duration_seconds.map(|d| d as i64),
                    audiobook_clone.size_bytes.map(|s| s as i64),
                    audiobook_clone.cover_art,
                    audiobook_clone.created_at.to_rfc3339(),
                    audiobook_clone.updated_at.to_rfc3339(),
                ])
                .map_err(|e| DatabaseError::ExecutionFailed {
                    message: format!("Failed to insert audiobook: {e}"),
                })?;

                Ok(())
            })
            .map_err(AppError::Database)
    }

    /// Adds multiple audiobooks to the database in bulk
    #[instrument(skip(self, audiobooks))]
    pub fn add_audiobooks_bulk(&self, audiobooks: &[Audiobook]) -> Result<()> {
        if audiobooks.is_empty() {
            return Ok(());
        }

        // Group audiobooks by library_id (which is already stored in each audiobook)
        let mut library_groups: std::collections::HashMap<String, Vec<Audiobook>> =
            std::collections::HashMap::new();
        for audiobook in audiobooks {
            library_groups
                .entry(audiobook.library_id.clone())
                .or_default()
                .push(audiobook.clone());
        }

        // Process each library's audiobooks
        for (library_id, audiobooks_for_library) in library_groups {
            self.operations.execute_transaction(move |tx| {
                let mut stmt = tx.prepare(
                    "INSERT OR REPLACE INTO audiobooks 
                    (id, library_id, path, title, author, narrator, description, 
                     duration_seconds, size_bytes, cover_art, created_at, updated_at) 
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                )?;

                for audiobook in &audiobooks_for_library {
                    stmt.execute(rusqlite::params![
                        audiobook.id,
                        &library_id,
                        audiobook.path.to_string_lossy(),
                        &audiobook.title,
                        &audiobook.author,
                        &audiobook.narrator,
                        &audiobook.description,
                        audiobook.duration_seconds.map(|d| d as i64),
                        audiobook.size_bytes.map(|s| s as i64),
                        &audiobook.cover_art,
                        audiobook.created_at.to_rfc3339(),
                        audiobook.updated_at.to_rfc3339(),
                    ])
                    .map_err(|e| DatabaseError::ExecutionFailed {
                        message: format!("Failed to insert audiobook: {e}"),
                    })?;
                }

                Ok(())
            })?;
        }

        Ok(())
    }

    /// Gets all audiobooks from a library (path-based)
    #[instrument(skip(self, library_path))]
    pub fn get_audiobooks(&self, library_path: &Path) -> Result<Vec<Audiobook>> {
        let library_id = self.get_library_id(library_path)?;

        self.operations
            .execute_query(move |conn| {
                let mut stmt = conn
                    .prepare(&SqlQueries::audiobook_select(Some(
                        "library_id = ?1 ORDER BY title",
                    )))
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!(
                            "Failed to prepare statement: {e}"
                        ))
                    })?;

                let rows = stmt
                    .query_map([&library_id], |row| {
                        match RowMappers::audiobook_from_row(row) {
                            Ok(audiobook) => Ok(audiobook),
                            Err(_) => Err(rusqlite::Error::InvalidColumnType(
                                0,
                                "mapping failed".to_string(),
                                rusqlite::types::Type::Null,
                            )),
                        }
                    })
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!("Failed to execute query: {e}"))
                    })?;

                let mut audiobooks = Vec::new();
                for row in rows {
                    match row {
                        Ok(audiobook) => audiobooks.push(audiobook),
                        Err(e) => {
                            return Err(DatabaseError::execution_failed(&format!(
                                "Failed to process row: {e}"
                            )));
                        }
                    }
                }

                Ok(audiobooks)
            })
            .map_err(AppError::Database)
    }

    /// Gets a single audiobook by path (path-based)
    #[instrument(skip(self, path))]
    pub fn get_audiobook(&self, path: &Path) -> Result<Option<Audiobook>> {
        let path_str = path.to_string_lossy().to_string();

        self.operations
            .execute_query(move |conn| {
                let mut stmt = conn
                    .prepare(&SqlQueries::audiobook_select(Some("path = ?1")))
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!(
                            "Failed to prepare statement: {e}"
                        ))
                    })?;

                let mut rows = stmt
                    .query_map([&path_str], |row| {
                        match RowMappers::audiobook_from_row(row) {
                            Ok(audiobook) => Ok(audiobook),
                            Err(_) => Err(rusqlite::Error::InvalidColumnType(
                                0,
                                "mapping failed".to_string(),
                                rusqlite::types::Type::Null,
                            )),
                        }
                    })
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!("Failed to execute query: {e}"))
                    })?;

                match rows.next() {
                    Some(Ok(audiobook)) => Ok(Some(audiobook)),
                    Some(Err(e)) => Err(DatabaseError::execution_failed(&format!(
                        "Failed to process row: {e}"
                    ))),
                    None => Ok(None),
                }
            })
            .map_err(AppError::Database)
    }

    /// Deletes an audiobook from the database (path-based)
    #[instrument(skip(self, path))]
    pub fn delete_audiobook(&self, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy().to_string();

        self.operations
            .execute(move |conn| {
                conn.execute("DELETE FROM audiobooks WHERE path = ?1", [&path_str])
                    .map_err(|e| {
                        DatabaseError::execution_failed(&format!("Failed to delete audiobook: {e}"))
                    })?;

                Ok(())
            })
            .map_err(AppError::Database)?;

        Ok(())
    }

    /// Deletes all audiobooks from a library (path-based)
    #[instrument(skip(self, library_path))]
    pub fn delete_library_audiobooks(&self, library_path: &Path) -> Result<()> {
        let library_id = self.get_library_id(library_path)?;

        self.operations
            .execute(move |conn| {
                conn.execute(
                    "DELETE FROM audiobooks WHERE library_id = ?1",
                    [&library_id],
                )
                .map_err(|e| {
                    DatabaseError::execution_failed(&format!(
                        "Failed to delete library audiobooks: {e}"
                    ))
                })?;

                Ok(())
            })
            .map_err(AppError::Database)?;

        Ok(())
    }
}
