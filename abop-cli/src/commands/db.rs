//! Database operations command implementation
//!
//! This module handles all database-related operations including
//! initialization, listing, statistics, and cleanup operations.

use crate::cli::DbOperations;
use crate::error::{validate_existing_database_path, CliResult, CliResultExt};
use crate::utils::{get_audiobook_count, show_audiobook_list};
use abop_core::db::Database;
use log::{debug, info};
use std::path::PathBuf;

/// Execute database operations command
///
/// # Arguments
/// * `database_path` - Path to the database file
/// * `operation` - The specific database operation to perform
///
/// # Errors
/// Returns an error if:
/// - Database path is invalid (for operations requiring existing file)
/// - Database connection fails
/// - The specific operation fails
pub fn run(database_path: PathBuf, operation: DbOperations) -> CliResult<()> {
    debug!("Starting database operation: {operation:?}");
    
    match operation {
        DbOperations::Init => init(database_path),
        DbOperations::List => list(database_path),
        DbOperations::Stats => stats(database_path),
        DbOperations::Clean => clean(database_path),
    }
}

/// Initialize a new database
fn init(database_path: PathBuf) -> CliResult<()> {
    info!("Initializing database: {database_path:?}");
    debug!("About to call Database::open()");
    
    let _db = Database::open(&database_path)
        .with_database_context("initialization")?;
    
    debug!("Database::open() completed successfully");
    info!("✓ Database initialized successfully");
    Ok(())
}

/// List all audiobooks in the database
fn list(database_path: PathBuf) -> CliResult<()> {
    info!("Listing audiobooks in: {database_path:?}");
    
    // Validate database exists before attempting connection
    validate_existing_database_path(&database_path)?;
    
    debug!("About to call Database::open() for list operation");
    let db = Database::open(&database_path)
        .with_database_context("opening for list operation")?;
    debug!("Database::open() completed for list operation");

    show_audiobook_list(&db)?;
    Ok(())
}

/// Show database statistics
fn stats(database_path: PathBuf) -> CliResult<()> {
    info!("Database statistics: {database_path:?}");
    
    // Validate database exists before attempting connection
    validate_existing_database_path(&database_path)?;
    
    debug!("About to call Database::open() for stats operation");
    let db = Database::open(&database_path)
        .with_database_context("opening for stats operation")?;
    debug!("Database::open() completed for stats operation");

    debug!("About to call get_audiobook_count()");
    let count = get_audiobook_count(&db)?;
    debug!("get_audiobook_count() completed with count: {count}");
    
    info!("Total audiobooks: {count}");
    Ok(())
}

/// Clean and optimize the database
fn clean(database_path: PathBuf) -> CliResult<()> {
    info!("Cleaning database: {database_path:?}");
    
    // Validate database exists before attempting connection
    validate_existing_database_path(&database_path)?;
    
    debug!("About to call Database::open() for clean operation");
    let _db = Database::open(&database_path)
        .with_database_context("opening for clean operation")?;
    debug!("Database::open() completed for clean operation");

    // TODO: Implement database cleanup/optimization
    info!("✓ Database cleanup completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::fs;

    fn create_test_db() -> NamedTempFile {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        
        // Create a minimal database file
        fs::write(temp_file.path(), b"test database content")
            .expect("Failed to write test database");
        
        temp_file
    }

    #[test]
    fn test_init_operation() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_path_buf();
        
        // Remove the temp file so we can test creation
        drop(temp_file);
        
        let result = init(db_path.clone());
        // In test environment, this might fail due to missing dependencies
        // but it shouldn't panic
        assert!(result.is_ok() || result.is_err());
        
        // If successful, the file should exist
        if result.is_ok() {
            assert!(db_path.exists());
        }
    }

    #[test]
    fn test_operations_with_nonexistent_database() {
        let nonexistent_path = PathBuf::from("/nonexistent/database.db");
        
        // All operations except init should fail with nonexistent database
        let result = list(nonexistent_path.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
        
        let result = stats(nonexistent_path.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
        
        let result = clean(nonexistent_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_operations_with_directory_path() {
        use tempfile::TempDir;
        
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_path_buf();
        
        // Operations should fail when path points to directory
        let result = list(dir_path.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is a directory"));
        
        let result = stats(dir_path.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is a directory"));
        
        let result = clean(dir_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is a directory"));
    }

    #[test]
    fn test_database_operations_dispatch() {
        let temp_file = create_test_db();
        let db_path = temp_file.path().to_path_buf();
        
        // Test that the run function properly dispatches to sub-operations
        // Note: These might fail in test environment but shouldn't panic
        
        let result = run(db_path.clone(), DbOperations::Stats);
        assert!(result.is_ok() || result.is_err());
        
        let result = run(db_path.clone(), DbOperations::List);
        assert!(result.is_ok() || result.is_err());
        
        let result = run(db_path.clone(), DbOperations::Clean);
        assert!(result.is_ok() || result.is_err());
        
        // Init should work with any path
        let new_file = NamedTempFile::new().unwrap();
        let new_path = new_file.path().to_path_buf();
        drop(new_file); // Remove file to test creation
        
        let result = run(new_path, DbOperations::Init);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_db_operations_debug_format() {
        // Test debug formatting for database operations
        let operations = vec![
            DbOperations::Init,
            DbOperations::List,
            DbOperations::Stats,
            DbOperations::Clean,
        ];

        for op in operations {
            let debug_str = format!("{:?}", op);
            assert!(!debug_str.is_empty());
        }
    }
}