//! Progress repository for database operations
//!
//! This module handles all database operations related to audiobook progress tracking.

use rusqlite::{Connection, OptionalExtension};
use std::sync::{Arc, Mutex};

use crate::db::error::DbResult;
use crate::models::progress::Progress;
use crate::db::repositories::Repository;

/// Repository for progress-related database operations
#[derive(Debug)]
pub struct ProgressRepository {
    connection: Arc<Mutex<Connection>>,
}

impl Repository for ProgressRepository {
    fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.connection
    }
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

    /// Update position for an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn update_position(&self, audiobook_id: &str, position_seconds: f64) -> DbResult<bool> {
        self.execute_query(|conn| {
            let rows_affected = conn.execute(
                "UPDATE progress SET position_seconds = ?1, last_played = CURRENT_TIMESTAMP
                 WHERE audiobook_id = ?2",
                (position_seconds, audiobook_id),
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Mark an audiobook as completed or not completed
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn mark_completed(&self, audiobook_id: &str, completed: bool) -> DbResult<bool> {
        self.execute_query(|conn| {
            let rows_affected = conn.execute(
                "UPDATE progress SET completed = ?1, updated_at = CURRENT_TIMESTAMP
                 WHERE audiobook_id = ?2",
                (completed, audiobook_id),
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete progress by audiobook ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn delete_by_audiobook(&self, audiobook_id: &str) -> DbResult<bool> {
        self.execute_query(|conn| {
            let rows_affected = conn.execute(
                "DELETE FROM progress WHERE audiobook_id = ?1",
                [audiobook_id],
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete progress by ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn delete(&self, id: &str) -> DbResult<bool> {
        self.execute_query(|conn| {
            let rows_affected = conn.execute(
                "DELETE FROM progress WHERE id = ?1",
                [id],
            )?;
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
            let mut stmt = conn.prepare(
                "SELECT
                    COUNT(DISTINCT audiobook_id) as total_books,
                    SUM(CASE WHEN completed = 1 THEN 1 ELSE 0 END) as completed_books,
                    SUM(CASE WHEN completed = 0 AND position_seconds > 0 THEN 1 ELSE 0 END) as in_progress_books,
                    SUM(position_seconds) as total_listening_time
                 FROM progress"
            )?;

            let stats = stmt.query_row([], |row| {
                Ok(ProgressStatistics {
                    total_books: row.get::<_, i64>(0)? as usize,
                    completed_books: row.get::<_, i64>(1)? as usize,
                    in_progress_books: row.get::<_, i64>(2)? as usize,
                    total_listening_time_seconds: row.get::<_, f64>(3)?,
                })
            })?;

            Ok(stats)
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

    /// Batch add or update progress records
    pub fn batch_add(&self, progress_records: &[Progress]) -> DbResult<()> {
        let mut conn = self.connection.lock().unwrap();
        let tx = conn.transaction()?;
        for progress in progress_records {
            tx.execute(
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
        }
        tx.commit()?;
        Ok(())
    }
}

impl Clone for ProgressRepository {
    fn clone(&self) -> Self {
        Self {
            connection: Arc::clone(&self.connection),
        }
    }
}

/// Progress statistics
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
