//! Search operations for audiobook repository

use super::crud_operations::AudiobookRepository;
use super::queries::*;
use super::validation_operations::*;
use crate::db::datetime_serde::SqliteDateTime;
use crate::db::repositories::Repository;
use crate::error::{AppError, Result};
use crate::models::Audiobook;
use rusqlite::OptionalExtension;
use std::path::PathBuf;

impl AudiobookRepository {
    /// Find an audiobook by its ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn find_by_id(&self, id: &str) -> Result<Option<Audiobook>> {
        validate_audiobook_id(id)?;
        let id = id.to_string();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let mut stmt =
                conn.prepare(&format!("{} {}", SELECT_ALL_COLUMNS, where_clauses::BY_ID))?;
            let audiobook = stmt.query_row([&id], map_row_to_audiobook).optional()?;
            Ok(audiobook)
        })
        .map_err(AppError::from)
    }

    /// Find audiobooks by library ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if library ID is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn find_by_library(&self, library_id: &str) -> Result<Vec<Audiobook>> {
        validate_library_id(library_id)?;
        let library_id = library_id.to_string();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let mut stmt = conn.prepare(&format!(
                "{} {}",
                SELECT_ALL_COLUMNS,
                where_clauses::BY_LIBRARY_ID
            ))?;
            let audiobooks = stmt
                .query_map([&library_id], map_row_to_audiobook)?
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
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn find_by_library_paginated(
        &self,
        library_id: &str,
        limit: Option<usize>,
        offset: usize,
    ) -> Result<Vec<Audiobook>> {
        let library_id = library_id.to_string();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let audiobooks = if let Some(limit_value) = limit {
                let mut stmt = conn.prepare(&format!(
                    "{} {} {}",
                    SELECT_ALL_COLUMNS,
                    where_clauses::BY_LIBRARY_ID,
                    order_by::BY_TITLE_ASC_LIMIT_OFFSET
                ))?;
                let rows = stmt.query_map(
                    rusqlite::params![library_id, limit_value as i64, offset as i64],
                    map_row_to_audiobook,
                )?;
                rows.collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?
            } else {
                let mut stmt = conn.prepare(&format!(
                    "{} {} {}",
                    SELECT_ALL_COLUMNS,
                    where_clauses::BY_LIBRARY_ID,
                    order_by::BY_TITLE_ASC_OFFSET
                ))?;
                let rows = stmt.query_map(rusqlite::params![library_id, offset as i64], |row| {
                    map_row_to_audiobook(row)
                })?;
                rows.collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?
            };
            Ok(audiobooks)
        })
        .map_err(AppError::from)
    }

    /// Find audiobooks by author
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ValidationFailed`] if author string is empty.
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn find_by_author(&self, author: &str) -> Result<Vec<Audiobook>> {
        validate_author(author)?;
        let author = author.to_string();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let mut stmt = conn.prepare(&format!(
                "{} {}",
                SELECT_ALL_COLUMNS,
                where_clauses::BY_AUTHOR_LIKE
            ))?;
            let audiobooks = stmt
                .query_map([&format!("%{author}%")], map_row_to_audiobook)?
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
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn find_by_path(&self, path: &str) -> Result<Option<Audiobook>> {
        validate_path(path)?;
        let path = path.to_string();
        self.execute_query(move |conn: &rusqlite::Connection| {
            let mut stmt = conn.prepare(&format!(
                "{} {}",
                SELECT_ALL_COLUMNS,
                where_clauses::BY_PATH
            ))?;
            let audiobook = stmt.query_row([&path], map_row_to_audiobook).optional()?;
            Ok(audiobook)
        })
        .map_err(AppError::from)
    }

    /// Get all audiobooks
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if SQL query execution fails.
    pub fn find_all(&self) -> Result<Vec<Audiobook>> {
        self.execute_query(|conn: &rusqlite::Connection| {
            let mut stmt = conn.prepare(SELECT_ALL_COLUMNS)?;
            let audiobooks = stmt
                .query_map([], map_row_to_audiobook)?
                .collect::<std::result::Result<Vec<_>, rusqlite::Error>>()?;
            Ok(audiobooks)
        })
        .map_err(AppError::from)
    }
}

/// Map SQLite row to Audiobook struct
fn map_row_to_audiobook(row: &rusqlite::Row) -> std::result::Result<Audiobook, rusqlite::Error> {
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
}
