//! Convenience repository wrapper methods for `Database`.
//!
//! These are thin delegations to repository APIs kept for ergonomics and
//! backward compatibility. Prefer using repositories directly in new code.

use std::path::Path;
use tracing::{debug, instrument};

use crate::{
    error::{AppError, Result},
    models::{Audiobook, Library},
};

use super::LibraryRepository;
use super::facade::Database;

impl Database {
    /// Gets all libraries from the database
    #[instrument(skip(self))]
    pub fn get_libraries(&self) -> Result<Vec<Library>> {
        let repo = self.library_repository();
        repo.find_all().map_err(AppError::from)
    }

    /// Gets audiobooks in a specific library by library ID
    #[instrument(skip(self, library_id))]
    pub fn get_audiobooks_in_library(&self, library_id: &str) -> Result<Vec<Audiobook>> {
        let repo = self.audiobook_repository();
        repo.find_by_library(library_id)
    }

    /// Gets audiobooks in a specific library with pagination support
    #[instrument(skip(self, library_id), fields(limit = ?limit, offset = offset))]
    pub fn get_audiobooks_in_library_paginated(
        &self,
        library_id: &str,
        limit: Option<usize>,
        offset: usize,
    ) -> Result<Vec<Audiobook>> {
        // Validate library exists before querying audiobooks
        let library_repo = self.library_repository();
        library_repo
            .find_by_id(library_id)?
            .ok_or_else(|| AppError::Other(format!("Library with ID '{library_id}' not found")))?;

        let repo = self.audiobook_repository();
        repo.find_by_library_paginated(library_id, limit, offset)
    }

    /// Counts total audiobooks in a specific library
    #[instrument(skip(self, library_id))]
    pub fn count_audiobooks_in_library(&self, library_id: &str) -> Result<usize> {
        // Validate library exists before counting
        let library_repo = self.library_repository();
        library_repo
            .find_by_id(library_id)?
            .ok_or_else(|| AppError::Other(format!("Library with ID '{library_id}' not found")))?;

        let repo = self.audiobook_repository();
        repo.count_by_library(library_id)
    }

    /// Gets all audiobooks across all libraries
    ///
    /// This method performs a single optimized query to retrieve all audiobooks
    /// rather than making separate queries for each library (avoiding N+1 problem).
    #[instrument(skip(self))]
    pub fn get_all_audiobooks(&self) -> Result<Vec<Audiobook>> {
        Ok(self.operations.execute_query(|conn| {
            use super::error::DatabaseError;
            use crate::db::mappers::{RowMappers, SqlQueries};

            let mut stmt = conn
                .prepare(&SqlQueries::audiobook_select_with_order(
                    None,
                    Some("title"),
                ))
                .map_err(|e| {
                    DatabaseError::execution_failed(&format!("Failed to prepare statement: {e}"))
                })?;

            let rows = stmt
                .query_map([], |row| {
                    RowMappers::audiobook_from_row(row).map_err(|e| {
                        log::error!("Failed to map audiobook row: {}", e);
                        rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Text,
                            Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                format!("Audiobook row mapping failed: {e}"),
                            )),
                        )
                    })
                })
                .map_err(|e| {
                    DatabaseError::execution_failed(&format!("Failed to execute query: {e}"))
                })?;

            let mut audiobooks = Vec::new();
            for row_result in rows {
                let audiobook = row_result.map_err(|e| {
                    DatabaseError::execution_failed(&format!("Failed to map row: {e}"))
                })?;
                audiobooks.push(audiobook);
            }

            Ok(audiobooks)
        })?)
    }

    /// Gets a library repository for more complex operations
    #[must_use]
    pub fn libraries(&self) -> LibraryRepository {
        self.library_repository()
    }

    /// Finds a library by its path
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails
    pub fn find_library_by_path<P: AsRef<Path>>(&self, path: P) -> Result<Option<Library>> {
        let repo = self.library_repository();
        repo.find_by_path(path).map_err(AppError::from)
    }

    /// Create a new library and add it to the database if it doesn't already exist
    ///
    /// If a library with the same path already exists, returns the ID of the existing library.
    /// Otherwise, creates a new library and returns its ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails
    #[instrument(skip(self))]
    pub fn add_library_with_path(&self, name: &str, path: std::path::PathBuf) -> Result<String> {
        // First check if a library with this path already exists
        if let Some(existing) = self.find_library_by_path(&path)? {
            debug!("Library already exists at path: {}", path.display());
            return Ok(existing.id);
        }

        // If not, create a new library via repository
        let repo = self.library_repository();
        let created = repo.create(name, path.clone()).map_err(AppError::from)?;

        // Update in-memory cache to keep subsequent lookups fast
        if let Ok(mut state) = self.state.lock() {
            state.library_cache.insert(
                created.path.to_string_lossy().to_string(),
                created.id.clone(),
            );
        }

        Ok(created.id)
    }
}
