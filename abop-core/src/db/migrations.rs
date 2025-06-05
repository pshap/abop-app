//! Database migrations for ABOP
//!
//! This module handles database schema migrations using an enhanced versioning system
//! with rollback capability and better error handling.

use crate::db::error::{DatabaseError, DbResult};
use rusqlite::{Connection, Result as SqliteResult};
use std::collections::HashMap;

/// Represents a database migration with rollback capability
#[derive(Debug, Clone)]
pub struct Migration {
    /// The version number of this migration
    pub version: u32,
    /// The SQL to execute for this migration (forward)
    pub up_sql: &'static str,
    /// The SQL to execute to rollback this migration (backward)
    pub down_sql: &'static str,
    /// Description of what this migration does
    pub description: &'static str,
}

/// Migration execution result
#[derive(Debug)]
pub struct MigrationResult {
    /// The version that was applied/rolled back
    pub version: u32,
    /// Description of the migration
    pub description: String,
    /// Whether this was a rollback operation
    pub is_rollback: bool,
}

/// Migration manager for enhanced database operations
pub struct MigrationManager {
    migrations: HashMap<u32, Migration>,
}

impl Default for MigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MigrationManager {
    /// Create a new migration manager with all available migrations
    #[must_use]
    pub fn new() -> Self {
        let migrations = get_migrations()
            .into_iter()
            .map(|m| (m.version, m))
            .collect();

        Self { migrations }
    }

    /// Get the current database version
    ///
    /// # Errors
    ///
    /// Returns a database error if:
    /// - Failed to query the migrations table
    /// - Version value cannot be safely converted to u32
    /// - Database query execution fails
    pub fn current_version(&self, conn: &Connection) -> DbResult<u32> {
        let version: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM migrations WHERE applied = 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0); // Safe conversion from i64 to u32 with proper error handling
        u32::try_from(version.max(0)).map_err(|_| DatabaseError::MigrationFailed {
            version: 0,
            message: format!("Version value {version} cannot be safely converted to u32"),
        })
    }

    /// Get all pending migrations that need to be applied
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The current migration version cannot be determined
    /// - Database query fails
    pub fn pending_migrations(&self, conn: &Connection) -> DbResult<Vec<&Migration>> {
        let current_version = self.current_version(conn)?;
        let mut pending: Vec<&Migration> = self
            .migrations
            .values()
            .filter(|m| m.version > current_version)
            .collect();
        pending.sort_by_key(|m| m.version);
        Ok(pending)
    }

    /// Apply all pending migrations
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Migration table setup fails
    /// - Any migration fails to apply
    /// - Database transaction fails
    pub fn migrate_up(&self, conn: &mut Connection) -> DbResult<Vec<MigrationResult>> {
        Self::setup_migrations_table(conn)?;

        let pending = self.pending_migrations(conn)?;
        if pending.is_empty() {
            return Ok(vec![]);
        }

        let mut results = Vec::new();

        for migration in pending {
            let result = Self::apply_migration(conn, migration, false)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Rollback to a specific version
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The target version is invalid or higher than current version
    /// - Current migration version cannot be determined
    /// - Any migration rollback fails
    /// - Database transaction fails
    pub fn migrate_down(
        &self,
        conn: &mut Connection,
        target_version: u32,
    ) -> DbResult<Vec<MigrationResult>> {
        let current_version = self.current_version(conn)?;

        if target_version >= current_version {
            return Err(DatabaseError::MigrationFailed {
                version: target_version,
                message: format!(
                    "Target version {target_version} is not less than current version {current_version}"
                ),
            });
        }

        let mut results = Vec::new();
        let mut version_to_rollback = current_version;

        while version_to_rollback > target_version {
            if let Some(migration) = self.migrations.get(&version_to_rollback) {
                let result = Self::apply_migration(conn, migration, true)?;
                results.push(result);
            }

            if version_to_rollback == 0 {
                break;
            }
            version_to_rollback -= 1;
        }

        Ok(results)
    }

    /// Apply a single migration (up or down)
    fn apply_migration(
        conn: &mut Connection,
        migration: &Migration,
        is_rollback: bool,
    ) -> DbResult<MigrationResult> {
        let tx = conn.transaction().map_err(DatabaseError::from)?;

        let sql = if is_rollback {
            migration.down_sql
        } else {
            migration.up_sql
        };
        let operation = if is_rollback { "rollback" } else { "apply" };

        log::info!(
            "{}ing migration {} - {}",
            operation,
            migration.version,
            migration.description
        );
        log::debug!("Migration SQL: {sql}");

        // Execute the migration SQL
        if let Err(e) = tx.execute_batch(sql) {
            log::error!(
                "Failed to {} migration {}: {}",
                operation,
                migration.version,
                e
            );
            return Err(DatabaseError::MigrationFailed {
                version: migration.version,
                message: format!("Failed to {operation}: {e}"),
            });
        }

        // Update migration tracking
        if is_rollback {
            tx.execute(
                "UPDATE migrations SET applied = 0, rolled_back_at = CURRENT_TIMESTAMP WHERE version = ?",
                [migration.version],
            ).map_err(DatabaseError::from)?;
        } else {
            tx.execute(
                "INSERT OR REPLACE INTO migrations (version, description, applied, applied_at) VALUES (?, ?, 1, CURRENT_TIMESTAMP)",
                [&migration.version.to_string(), migration.description],
            ).map_err(DatabaseError::from)?;
        }

        tx.commit().map_err(DatabaseError::from)?;

        Ok(MigrationResult {
            version: migration.version,
            description: migration.description.to_string(),
            is_rollback,
        })
    }

    /// Setup the migrations table
    fn setup_migrations_table(conn: &Connection) -> DbResult<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                applied INTEGER NOT NULL DEFAULT 1,
                applied_at TIMESTAMP,
                rolled_back_at TIMESTAMP
            )",
        )
        .map_err(DatabaseError::from)?;

        Ok(())
    }
}

/// Gets all migrations in order with rollback SQL
fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            up_sql: include_str!("migrations/001_initial_schema.sql"),
            down_sql: include_str!("migrations/001_initial_schema_down.sql"),
            description: "Initial database schema with libraries, audiobooks, and progress tracking",
        },
        Migration {
            version: 2,
            up_sql: include_str!("migrations/002_add_bookmarks.sql"),
            down_sql: include_str!("migrations/002_add_bookmarks_down.sql"),
            description: "Add bookmarks table for user bookmarks",
        },
    ]
}

/// Simplified migration runner that uses the enhanced migration manager
pub fn run_migrations(conn: &mut Connection) -> SqliteResult<()> {
    let manager = MigrationManager::new();
    match manager.migrate_up(conn) {
        Ok(results) => {
            for result in results {
                log::info!(
                    "Applied migration {} - {}",
                    result.version,
                    result.description
                );
            }
            Ok(())
        }
        Err(e) => {
            log::error!("Migration failed: {e}");
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_ABORT),
                Some(format!("Migration failed: {e}")),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migrations() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();

        // Verify that the migrations table was created
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='migrations'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        // Verify that the migrations were recorded
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM migrations WHERE applied = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(count > 0, "Migrations were not recorded");
    }

    #[test]
    fn test_migration_manager() {
        let mut conn = Connection::open_in_memory().unwrap();
        let manager = MigrationManager::new();

        // Apply all migrations
        let results = manager.migrate_up(&mut conn).unwrap();
        assert!(!results.is_empty(), "Should have applied migrations");

        // Check current version
        let version = manager.current_version(&conn).unwrap();
        assert!(version > 0, "Version should be greater than 0");

        // Check no pending migrations
        let pending = manager.pending_migrations(&conn).unwrap();
        assert!(pending.is_empty(), "Should have no pending migrations");
    }

    #[test]
    fn test_migration_rollback() {
        let mut conn = Connection::open_in_memory().unwrap();
        let manager = MigrationManager::new();

        // Apply all migrations first
        manager.migrate_up(&mut conn).unwrap();
        let _initial_version = manager.current_version(&conn).unwrap();

        // Rollback to version 1
        let rollback_results = manager.migrate_down(&mut conn, 1).unwrap();
        assert!(
            !rollback_results.is_empty(),
            "Should have rolled back migrations"
        );

        // Check version is now 1
        let current_version = manager.current_version(&conn).unwrap();
        assert_eq!(current_version, 1, "Version should be 1 after rollback");

        // Rollback to version 0 (empty database)
        let rollback_results = manager.migrate_down(&mut conn, 0).unwrap();
        assert!(
            !rollback_results.is_empty(),
            "Should have rolled back to version 0"
        );

        let current_version = manager.current_version(&conn).unwrap();
        assert_eq!(
            current_version, 0,
            "Version should be 0 after complete rollback"
        );
    }
}
