//! Application database utilities: centralized DB path and opener.
//!
//! Provides `Database::open_app_database()` and related helpers used by the GUI.

use std::path::{Path, PathBuf};
use tracing::info;

use crate::error::{AppError, Result};

use super::facade::Database;

impl Database {
    /// Opens the centralized application database.
    ///
    /// This creates a single database file in the app's data directory,
    /// avoiding the need for separate databases per library.
    pub fn open_app_database() -> Result<Self> {
        let db_path = Self::get_app_database_path()?;

        // Ensure the parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AppError::Config(format!("Failed to create database directory: {e}"))
            })?;
        }

        info!("Using centralized database at: {}", db_path.display());
        Self::open(&db_path)
    }

    /// Gets the path to the centralized application database
    pub fn get_app_database_path() -> Result<PathBuf> {
        let mut path = dirs::data_dir()
            .ok_or_else(|| AppError::Config("Could not find data directory".to_string()))?;
        path.push("abop-iced");
        path.push("database.db");
        Ok(path)
    }

    /// Opens a database at the specified path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = super::facade::PoolConfig {
            path: path.as_ref().to_string_lossy().to_string(),
            ..Default::default()
        };

        Self::new(config)
    }
}
