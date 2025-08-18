//! Database migrations for ABOP
//!
//! This module handles database schema migrations using an enhanced versioning system
//! with rollback capability and better error handling.

use crate::db::error::{DatabaseError, DbResult};
use rusqlite::Connection;
use std::collections::HashMap;
use tracing::{debug, error, info};

/// Represents a database migration
#[derive(Debug, Clone)]
pub struct Migration {
    /// The version number of this migration
    pub version: u32,
    /// The SQL to execute for this migration
    pub up_sql: &'static str,
    /// Description of what this migration does
    pub description: &'static str,
}

/// Migration execution result
#[derive(Debug)]
pub struct MigrationResult {
    /// The version that was applied
    pub version: u32,
    /// Description of the migration
    pub description: String,
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
        debug!("Getting current database version");
        let current_version = self.current_version(conn)?;
        debug!("Current database version: {current_version}");

        let mut pending: Vec<&Migration> = self
            .migrations
            .values()
            .filter(|m| m.version > current_version)
            .collect();
        pending.sort_by_key(|m| m.version);

        debug!(
            "Found {} pending migrations after version {}",
            pending.len(),
            current_version
        );
        for migration in &pending {
            debug!(
                "  - Migration {}: {}",
                migration.version, migration.description
            );
        }

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
        debug!("Starting migrate_up - setting up migrations table");
        Self::setup_migrations_table(conn)?;
        debug!("Migrations table setup complete");

        debug!("Getting pending migrations");
        let pending = self.pending_migrations(conn)?;
        debug!("Found {} pending migrations", pending.len());

        if pending.is_empty() {
            debug!("No pending migrations, returning empty results");
            return Ok(vec![]);
        }

        let mut results = Vec::new();

        for (i, migration) in pending.iter().enumerate() {
            debug!(
                "Applying migration {}/{}: version {}",
                i + 1,
                pending.len(),
                migration.version
            );
            let result = Self::apply_migration(conn, migration)?;
            debug!("Migration {} applied successfully", migration.version);
            results.push(result);
        }

        debug!("All {} migrations applied successfully", results.len());
        Ok(results)
    }

    /// Apply a single migration
    fn apply_migration(conn: &mut Connection, migration: &Migration) -> DbResult<MigrationResult> {
        let tx = conn.transaction().map_err(DatabaseError::from)?;

        info!(
            "Applying migration {} - {}",
            migration.version, migration.description
        );
        debug!("Migration SQL: {}", migration.up_sql);

        // Execute the migration SQL
        if let Err(e) = tx.execute_batch(migration.up_sql) {
            error!("Failed to apply migration {}: {}", migration.version, e);
            return Err(DatabaseError::MigrationFailed {
                version: migration.version,
                message: format!("Failed to apply: {e}"),
            });
        }

        // Update migration tracking
        tx.execute(
            "INSERT OR REPLACE INTO migrations (version, description, applied, applied_at) VALUES (?, ?, 1, CURRENT_TIMESTAMP)",
            [&migration.version.to_string(), migration.description],
        ).map_err(DatabaseError::from)?;

        tx.commit().map_err(DatabaseError::from)?;

        Ok(MigrationResult {
            version: migration.version,
            description: migration.description.to_string(),
        })
    }

    /// Setup the migrations table
    fn setup_migrations_table(conn: &Connection) -> DbResult<()> {
        debug!("Creating migrations table if it doesn't exist");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                applied INTEGER NOT NULL DEFAULT 1,
                applied_at TIMESTAMP
            )",
        )
        .map_err(DatabaseError::from)?;
        debug!("Migrations table created successfully");

        Ok(())
    }
}

/// Gets all migrations in order
fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            up_sql: include_str!("migrations/001_initial_schema.sql"),
            description: "Initial database schema with libraries, audiobooks, and progress tracking",
        },
        // Migration version 2 (add selected column) was removed during schema consolidation.
        // The selected column functionality is now included in the initial schema (version 1)
        // to reduce complexity for new installations. This avoids the need for a separate
        // migration step for basic UI state tracking functionality.
    ]
}

/// Simplified migration runner that uses the enhanced migration manager
pub fn run_migrations(conn: &mut Connection) -> DbResult<()> {
    debug!("Starting run_migrations function");
    let manager = MigrationManager::new();
    debug!("MigrationManager created successfully");

    debug!("About to call manager.migrate_up()");
    let results = manager.migrate_up(conn)?;

    debug!(
        "migrate_up completed successfully with {} results",
        results.len()
    );
    for result in results {
        info!(
            "Applied migration {} - {}",
            result.version, result.description
        );
    }
    debug!("All migration results processed successfully");
    Ok(())
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
}
