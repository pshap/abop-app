//! Shared handle to the application database
//!
//! Opening the database builds a connection pool and runs migrations, so the
//! GUI opens it once and hands out clones (which share the same pool).

use std::sync::OnceLock;

use abop_core::db::Database;

static APP_DATABASE: OnceLock<Database> = OnceLock::new();

/// The application database, opened on first use
///
/// A failed open is not cached, so a later call retries.
///
/// # Errors
///
/// Returns an error if the database cannot be opened.
pub fn app_database() -> abop_core::error::Result<Database> {
    if let Some(db) = APP_DATABASE.get() {
        return Ok(db.clone());
    }
    let db = Database::open_app_database()?;
    Ok(APP_DATABASE.get_or_init(|| db).clone())
}
