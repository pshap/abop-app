//! Application-level error types for ABOP CLI
//!
//! This module defines the error types used throughout the CLI application,
//! following modern Rust error handling practices with anyhow for application
//! code and clear error messages for users.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Application-level errors for the ABOP CLI
///
/// These are high-level errors that can be displayed to users with
/// meaningful messages. Lower-level errors from abop_core are wrapped
/// with context using anyhow.
pub type CliResult<T> = Result<T>;

/// Extension trait for adding CLI-specific context to errors
pub trait CliResultExt<T> {
    /// Add context for database operation errors
    fn with_database_context(self, operation: &str) -> CliResult<T>;
    
    /// Add context for scan operation errors
    fn with_scan_context(self) -> CliResult<T>;
}

impl<T, E> CliResultExt<T> for std::result::Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn with_database_context(self, operation: &str) -> CliResult<T> {
        self.map_err(|e| e.into())
            .with_context(|| format!("Database operation '{operation}' failed"))
    }
    
    fn with_scan_context(self) -> CliResult<T> {
        self.map_err(|e| e.into())
            .with_context(|| "Library scan operation failed")
    }
}

/// Validate that a library path exists and is a directory
pub fn validate_library_path(path: &Path) -> CliResult<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Library path does not exist: {}",
            path.display()
        ));
    }
    
    if !path.is_dir() {
        return Err(anyhow::anyhow!(
            "Library path is not a directory: {}",
            path.display()
        ));
    }
    
    Ok(())
}

/// Validate that a database path is valid for operations that require an existing file
pub fn validate_existing_database_path(path: &Path) -> CliResult<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Database file does not exist: {}",
            path.display()
        ));
    }
    
    if path.is_dir() {
        return Err(anyhow::anyhow!(
            "Database path is a directory, expected a file: {}",
            path.display()
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_validate_library_path_success() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();
        
        let result = validate_library_path(path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_library_path_not_exists() {
        let path = std::path::Path::new("/nonexistent/path");
        
        let result = validate_library_path(path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_validate_library_path_not_directory() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("not_a_directory.txt");
        fs::write(&file_path, "test content").unwrap();
        
        let result = validate_library_path(&file_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not a directory"));
    }

    #[test]
    fn test_validate_existing_database_path_success() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        fs::write(&db_path, "test database").unwrap();
        
        let result = validate_existing_database_path(&db_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_existing_database_path_not_exists() {
        let path = std::path::Path::new("/nonexistent/database.db");
        
        let result = validate_existing_database_path(path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_validate_existing_database_path_is_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();
        
        let result = validate_existing_database_path(dir_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is a directory"));
    }

    #[test]
    fn test_cli_result_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let _path = temp_dir.path().to_path_buf();
        
        // Test with_database_context
        let error: std::result::Result<(), std::io::Error> = 
            Err(std::io::Error::new(std::io::ErrorKind::Other, "test error"));
        let result = error.with_database_context("test operation");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database operation 'test operation' failed"));
        
        // Test with_scan_context
        let error: std::result::Result<(), std::io::Error> = 
            Err(std::io::Error::new(std::io::ErrorKind::Other, "test error"));
        let result = error.with_scan_context();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Library scan operation failed"));
    }
}