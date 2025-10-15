//! Schema initialization for the ABOP database.
//!
//! This module contains the low-level schema bootstrap used by tests and
//! fallback paths. Migrations remain the primary mechanism for schema changes.

use rusqlite::Connection;

use super::error::DatabaseError;
use crate::error::{AppError, Result};

/// Initialize fundamental tables when needed.
///
/// Prefer running migrations via `migrations::run_migrations` from the facade constructor.
#[allow(dead_code)]
pub(crate) fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS libraries (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS audiobooks (
            id TEXT PRIMARY KEY,
            library_id TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            title TEXT,
            author TEXT,
            narrator TEXT,
            description TEXT,
            duration_seconds INTEGER,
            size_bytes INTEGER,
            cover_art BLOB,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (library_id) REFERENCES libraries(id),
            UNIQUE(library_id, path)
        );
        CREATE INDEX IF NOT EXISTS idx_audiobooks_library_id ON audiobooks(library_id);
        CREATE INDEX IF NOT EXISTS idx_audiobooks_path ON audiobooks(path);
        CREATE INDEX IF NOT EXISTS idx_audiobooks_title ON audiobooks(title);
        CREATE INDEX IF NOT EXISTS idx_audiobooks_author ON audiobooks(author);",
    )
    .map_err(|e| AppError::Database(DatabaseError::from(e)))?;

    Ok(())
}
