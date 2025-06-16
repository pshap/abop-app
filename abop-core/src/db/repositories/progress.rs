//! Progress repository for database operations
//!
//! This module handles all database operations related to audiobook progress tracking.

use rusqlite::{OptionalExtension, params};
use std::sync::Arc;

use super::super::error::DbResult;
use super::{EnhancedRepository, Repository};
use crate::db::EnhancedConnection;
use crate::db::datetime_serde::SqliteDateTime;
use crate::models::Progress;

/// Repository for progress-related database operations
pub struct ProgressRepository {
    enhanced_connection: Arc<EnhancedConnection>,
}

impl ProgressRepository {
    /// Create a new progress repository
    #[must_use]
    pub const fn new(enhanced_connection: Arc<EnhancedConnection>) -> Self {
        Self {
            enhanced_connection,
        }
    }    /// Save or update progress for an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails due to constraint violations or invalid data.
    /// Returns [`DatabaseError::ValidationFailed`] if the progress data fails validation.
    pub fn upsert(&self, progress: &Progress) -> DbResult<()> {
        let progress = progress.clone();
        self.execute_query(move |conn| {
            let last_played_sql = progress.last_played.map(SqliteDateTime::from);
            let created_at_sql = SqliteDateTime::from(progress.created_at);
            let updated_at_sql = SqliteDateTime::from(progress.updated_at);
            conn.execute(
                "INSERT OR REPLACE INTO progress (
                    id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    &progress.id,
                    &progress.audiobook_id,
                    progress.position_seconds,
                    progress.completed,
                    last_played_sql,
                    created_at_sql,
                    updated_at_sql,
                ],
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
        let audiobook_id = audiobook_id.to_string();
        self.execute_query(move |conn| {
            let audiobook_id = audiobook_id.clone();
            conn.query_row(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress WHERE audiobook_id = ?1",
                [audiobook_id],
                |row| {
                    let last_played: Option<SqliteDateTime> = row.get(4)?;
                    let created_at: SqliteDateTime = row.get(5)?;
                    let updated_at: SqliteDateTime = row.get(6)?;
                    Ok(Progress {
                        id: row.get(0)?,
                        audiobook_id: row.get(1)?,
                        position_seconds: row.get(2)?,
                        completed: row.get(3)?,
                        last_played: last_played.map(|dt| dt.into()),
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
                    })
                },
            ).optional()
        })
    }

    /// Find progress by its ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn find_by_id(&self, id: &str) -> DbResult<Option<Progress>> {
        let id = id.to_string();
        self.execute_query(move |conn| {
            let id = id.clone();
            conn.query_row(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress WHERE id = ?1",
                [id],
                |row| {
                    let last_played: Option<SqliteDateTime> = row.get(4)?;
                    let created_at: SqliteDateTime = row.get(5)?;
                    let updated_at: SqliteDateTime = row.get(6)?;
                    Ok(Progress {
                        id: row.get(0)?,
                        audiobook_id: row.get(1)?,
                        position_seconds: row.get(2)?,
                        completed: row.get(3)?,
                        last_played: last_played.map(|dt| dt.into()),
                        created_at: created_at.into(),
                        updated_at: updated_at.into(),
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
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress ORDER BY updated_at DESC"
            )?;
            let progress_list = stmt.query_map([], |row| {
                let last_played: Option<SqliteDateTime> = row.get(4)?;
                let created_at: SqliteDateTime = row.get(5)?;
                let updated_at: SqliteDateTime = row.get(6)?;
                Ok(Progress {
                    id: row.get(0)?,
                    audiobook_id: row.get(1)?,
                    position_seconds: row.get(2)?,
                    completed: row.get(3)?,
                    last_played: last_played.map(|dt| dt.into()),
                    created_at: created_at.into(),
                    updated_at: updated_at.into(),
                })
            })?.collect::<Result<Vec<_>, _>>()?;
            Ok(progress_list)
        })
    }

    /// Get recently played audiobooks (within specified days)
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_recently_played(&self, days: i32) -> DbResult<Vec<Progress>> {
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress 
                 WHERE last_played IS NOT NULL 
                   AND last_played >= datetime('now', '-' || ?1 || ' days')
                 ORDER BY last_played DESC"
            )?;
            let progress_list = stmt.query_map([days], |row| {
                let last_played: Option<SqliteDateTime> = row.get(4)?;
                let created_at: SqliteDateTime = row.get(5)?;
                let updated_at: SqliteDateTime = row.get(6)?;
                Ok(Progress {
                    id: row.get(0)?,
                    audiobook_id: row.get(1)?,
                    position_seconds: row.get(2)?,
                    completed: row.get(3)?,
                    last_played: last_played.map(|dt| dt.into()),
                    created_at: created_at.into(),
                    updated_at: updated_at.into(),
                })
            })?.collect::<Result<Vec<_>, _>>()?;
            Ok(progress_list)
        })
    }

    /// Get completed audiobooks
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_completed(&self) -> DbResult<Vec<Progress>> {
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress WHERE completed = 1 ORDER BY updated_at DESC"
            )?;
            let progress_list = stmt.query_map([], |row| {
                let last_played: Option<SqliteDateTime> = row.get(4)?;
                let created_at: SqliteDateTime = row.get(5)?;
                let updated_at: SqliteDateTime = row.get(6)?;
                Ok(Progress {
                    id: row.get(0)?,
                    audiobook_id: row.get(1)?,
                    position_seconds: row.get(2)?,
                    completed: row.get(3)?,
                    last_played: last_played.map(|dt| dt.into()),
                    created_at: created_at.into(),
                    updated_at: updated_at.into(),
                })
            })?.collect::<Result<Vec<_>, _>>()?;
            Ok(progress_list)
        })
    }

    /// Get in-progress audiobooks (not completed)
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_in_progress(&self) -> DbResult<Vec<Progress>> {
        self.execute_query(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, audiobook_id, position_seconds, completed, last_played, created_at, updated_at
                 FROM progress WHERE completed = 0 AND position_seconds > 0 
                 ORDER BY updated_at DESC"
            )?;
            let progress_list = stmt.query_map([], |row| {
                let last_played: Option<SqliteDateTime> = row.get(4)?;
                let created_at: SqliteDateTime = row.get(5)?;
                let updated_at: SqliteDateTime = row.get(6)?;
                Ok(Progress {
                    id: row.get(0)?,
                    audiobook_id: row.get(1)?,
                    position_seconds: row.get(2)?,
                    completed: row.get(3)?,
                    last_played: last_played.map(|dt| dt.into()),
                    created_at: created_at.into(),
                    updated_at: updated_at.into(),
                })
            })?.collect::<Result<Vec<_>, _>>()?;
            Ok(progress_list)
        })
    }    /// Update position for an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn update_position(&self, audiobook_id: &str, position_seconds: i64) -> DbResult<bool> {
        let audiobook_id = audiobook_id.to_string();
        self.execute_query(move |conn| {
            let audiobook_id = audiobook_id.clone();
            let now = SqliteDateTime::from(chrono::Utc::now());
            let rows_affected = conn.execute(
                "UPDATE progress SET 
                    position_seconds = ?1,
                    updated_at = ?2,
                    last_played = ?3
                 WHERE audiobook_id = ?4",
                params![position_seconds, now, now, audiobook_id],
            )?;
            Ok(rows_affected > 0)
        })
    }    /// Mark an audiobook as completed or not completed
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn mark_completed(&self, audiobook_id: &str, completed: bool) -> DbResult<bool> {
        let audiobook_id = audiobook_id.to_string();
        self.execute_query(move |conn| {
            let audiobook_id = audiobook_id.clone();
            let now = SqliteDateTime::from(chrono::Utc::now());
            let rows_affected = conn.execute(
                "UPDATE progress SET 
                    completed = ?1,
                    updated_at = ?2
                 WHERE audiobook_id = ?3",
                params![completed, now, audiobook_id],
            )?;
            Ok(rows_affected > 0)
        })
    }

    /// Delete progress by audiobook ID
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete_by_audiobook(&self, audiobook_id: &str) -> DbResult<bool> {
        let audiobook_id = audiobook_id.to_string();
        self.execute_query(move |conn| {
            let audiobook_id = audiobook_id.clone();
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
    /// Returns [`DatabaseError::Sqlite`] if the SQL execution fails.
    pub fn delete(&self, id: &str) -> DbResult<bool> {
        let id = id.to_string();
        self.execute_query(move |conn| {
            let id = id.clone();
            let rows_affected = conn.execute("DELETE FROM progress WHERE id = ?1", [id])?;
            Ok(rows_affected > 0)
        })
    }

    /// Get statistics about progress
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn get_statistics(&self) -> DbResult<(i64, i64, i64)> {
        self.execute_query(move |conn| {
            let total: i64 =
                conn.query_row("SELECT COUNT(*) FROM progress", [], |row| row.get(0))?;
            let completed: i64 = conn.query_row(
                "SELECT COUNT(*) FROM progress WHERE completed = 1",
                [],
                |row| row.get(0),
            )?;
            let in_progress: i64 = conn.query_row(
                "SELECT COUNT(*) FROM progress WHERE completed = 0 AND position_seconds > 0",
                [],
                |row| row.get(0),
            )?;
            Ok((total, completed, in_progress))
        })
    }

    /// Check if progress exists for an audiobook
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if unable to acquire database connection.
    /// Returns [`DatabaseError::Sqlite`] if the SQL query execution fails.
    pub fn exists_for_audiobook(&self, audiobook_id: &str) -> DbResult<bool> {
        let audiobook_id = audiobook_id.to_string();
        self.execute_query(move |conn| {
            let audiobook_id = audiobook_id.clone();
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
    fn get_connection(&self) -> &Arc<EnhancedConnection> {
        &self.enhanced_connection
    }
}

impl EnhancedRepository for ProgressRepository {}

#[cfg(test)]
mod tests;
