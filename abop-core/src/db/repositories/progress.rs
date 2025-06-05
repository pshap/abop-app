//! Progress repository for database operations
//!
//! This module handles all database operations related to audiobook progress tracking.

use rusqlite::{Connection, OptionalExtension};
use std::sync::{Arc, Mutex};

use super::super::error::DbResult;
use super::{EnhancedRepository, Repository};
use crate::models::Progress;

/// Repository for progress-related database operations
pub struct ProgressRepository {
    connection: Arc<Mutex<Connection>>,
}

impl ProgressRepository {
    /// Create a new progress repository
    pub const fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    /// Save or update progress for an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::ValidationFailed`] if the progress data fails validation.
    pub fn upsert(&self, progress: &Progress) -> DbResult<()> {
        self.execute_query(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO progress (
                    id, audiobook_id, position_seconds, completed, last_played, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP)",
                (
                    &progress.id,
                    &progress.audiobook_id,
                    progress.position_seconds,
                    progress.completed,
                    &progress.last_played,
                ),
            )?;
            Ok(())
        })
    }

    /// Find progress by audiobook ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_audiobook(&self, audiobook_id: &str) -> DbResult<Option<Progress>> {
        self.execute_query(|conn| {
            conn.query_row(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress WHERE audiobook_id = ?1",
                [audiobook_id],
                |row| {
                    Ok(Progress {
                        id: row.get(0)?,
                        audiobook_id: row.get(1)?,
                        position_seconds: row.get(2)?,
                        completed: row.get(3)?,
                        last_played: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                },
            ).optional()
        })
    }

    /// Find progress by progress ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_id(&self, id: &str) -> DbResult<Option<Progress>> {
        self.execute_query(|conn| {
            conn.query_row(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress WHERE id = ?1",
                [id],
                |row| {
                    Ok(Progress {
                        id: row.get(0)?,
                        audiobook_id: row.get(1)?,
                        position_seconds: row.get(2)?,
                        completed: row.get(3)?,
                        last_played: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                },
            ).optional()
        })
    }

    /// Get all progress records
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_all(&self) -> DbResult<Vec<Progress>> {
        self.execute_query(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress ORDER BY last_played DESC"
            )?;

            let progress_list = stmt
                .query_map([], |row| {
                    Ok(Progress {
                        id: row.get(0)?,
                        audiobook_id: row.get(1)?,
                        position_seconds: row.get(2)?,
                        completed: row.get(3)?,
                        last_played: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(progress_list)
        })
    }

    /// Get recently played audiobooks (within last N days)
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_recently_played(&self, days: i32) -> DbResult<Vec<Progress>> {
        self.execute_query(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress
                 WHERE last_played >= datetime('now', '-' || ?1 || ' days')
                 ORDER BY last_played DESC"
            )?;

            let progress_list = stmt
                .query_map([days], |row| {
                    Ok(Progress {
                        id: row.get(0)?,
                        audiobook_id: row.get(1)?,
                        position_seconds: row.get(2)?,
                        completed: row.get(3)?,
                        last_played: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(progress_list)
        })
    }

    /// Get completed audiobooks
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_completed(&self) -> DbResult<Vec<Progress>> {
        self.execute_query(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress
                 WHERE completed = 1
                 ORDER BY updated_at DESC"
            )?;

            let progress_list = stmt
                .query_map([], |row| {
                    Ok(Progress {
                        id: row.get(0)?,
                        audiobook_id: row.get(1)?,
                        position_seconds: row.get(2)?,
                        completed: row.get(3)?,
                        last_played: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(progress_list)
        })
    }

    /// Get in-progress audiobooks (not completed, has progress)
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_in_progress(&self) -> DbResult<Vec<Progress>> {
        self.execute_query(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress
                 WHERE completed = 0 AND position_seconds > 0
                 ORDER BY last_played DESC"
            )?;

            let progress_list = stmt
                .query_map([], |row| {
                    Ok(Progress {
                        id: row.get(0)?,
                        audiobook_id: row.get(1)?,
                        position_seconds: row.get(2)?,
                        completed: row.get(3)?,
                        last_played: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(progress_list)
        })
    }

    /// Update the position for an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn update_position(&self, audiobook_id: &str, position_seconds: f64) -> DbResult<bool> {
        self.execute_query(|conn| {
            let rows_affected = conn.execute(
                "UPDATE progress SET
                    position_seconds = ?1,
                    last_played = CURRENT_TIMESTAMP,
                    updated_at = CURRENT_TIMESTAMP
                 WHERE audiobook_id = ?2",
                (position_seconds, audiobook_id),
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Mark an audiobook as completed
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn mark_completed(&self, audiobook_id: &str, completed: bool) -> DbResult<bool> {
        self.execute_query(|conn| {
            let rows_affected = conn.execute(
                "UPDATE progress SET
                    completed = ?1,
                    last_played = CURRENT_TIMESTAMP,
                    updated_at = CURRENT_TIMESTAMP
                 WHERE audiobook_id = ?2",
                (completed, audiobook_id),
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete progress for an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete_by_audiobook(&self, audiobook_id: &str) -> DbResult<bool> {
        self.execute_query(|conn| {
            let rows_affected = conn.execute(
                "DELETE FROM progress WHERE audiobook_id = ?1",
                [audiobook_id],
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete progress record by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete(&self, id: &str) -> DbResult<bool> {
        self.execute_query(|conn| {
            let rows_affected = conn.execute("DELETE FROM progress WHERE id = ?1", [id])?;
            Ok(rows_affected > 0)
        })
    }

    /// Get progress statistics
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_statistics(&self) -> DbResult<ProgressStatistics> {
        self.execute_query(|conn| {
            let total_books: i64 =
                conn.query_row("SELECT COUNT(*) FROM progress", [], |row| row.get(0))?;

            let completed_books: i64 = conn.query_row(
                "SELECT COUNT(*) FROM progress WHERE completed = 1",
                [],
                |row| row.get(0),
            )?;

            let in_progress_books: i64 = conn.query_row(
                "SELECT COUNT(*) FROM progress WHERE completed = 0 AND position_seconds > 0",
                [],
                |row| row.get(0),
            )?;

            let total_listening_time: f64 = conn.query_row(
                "SELECT COALESCE(SUM(position_seconds), 0) FROM progress",
                [],
                |row| row.get(0),
            )?;

            Ok(ProgressStatistics {
                total_books: crate::utils::casting::safe_db_count_to_usize(total_books),
                completed_books: crate::utils::casting::safe_db_count_to_usize(completed_books),
                in_progress_books: crate::utils::casting::safe_db_count_to_usize(in_progress_books),
                total_listening_time_seconds: total_listening_time,
            })
        })
    }

    /// Check if progress exists for an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn exists_for_audiobook(&self, audiobook_id: &str) -> DbResult<bool> {
        self.execute_query(|conn| {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM progress WHERE audiobook_id = ?1",
                [audiobook_id],
                |row| row.get(0),
            )?;
            Ok(count > 0)
        })
    }
}

impl Repository for ProgressRepository {
    fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.connection
    }
}

impl EnhancedRepository for ProgressRepository {}

/// Statistics about progress across all audiobooks
#[derive(Debug, Clone)]
pub struct ProgressStatistics {
    /// Total number of books with progress records
    pub total_books: usize,
    /// Number of completed books
    pub completed_books: usize,
    /// Number of books currently in progress
    pub in_progress_books: usize,
    /// Total listening time in seconds
    pub total_listening_time_seconds: f64,
}
