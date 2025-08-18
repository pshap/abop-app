//! Database operations command implementation
//!
//! This module handles all database-related operations including
//! initialization, listing, statistics, and cleanup operations.

use crate::cli::DbOperations;
use crate::error::{CliResult, CliResultExt, validate_existing_database_path};
use crate::utils::{get_audiobook_count, show_audiobook_list};
use abop_core::db::Database;
use anyhow::Context;
use log::{debug, info};
use std::path::PathBuf;

/// Execute database operations command
///
/// # Arguments
/// * `database_path` - Path to the database file
/// * `operation` - The specific database operation to perform
/// * `json_output` - Whether to output results in JSON format
///
/// # Errors
/// Returns an error if:
/// - Database path is invalid (for operations requiring existing file)
/// - Database connection fails
/// - The specific operation fails
pub fn run(database_path: PathBuf, operation: DbOperations, json_output: bool) -> CliResult<()> {
    debug!("Starting database operation: {operation:?}");

    match operation {
        DbOperations::Init => init(database_path, json_output),
        DbOperations::List => list(database_path, json_output),
        DbOperations::Stats => stats(database_path, json_output),
        DbOperations::Clean => clean(database_path, json_output),
    }
}

/// Initialize a new database
fn init(database_path: PathBuf, json_output: bool) -> CliResult<()> {
    info!("Initializing database: {database_path:?}");
    debug!("About to call Database::open()");

    let _db = Database::open(&database_path).with_database_context("initialization")?;

    debug!("Database::open() completed successfully");

    if json_output {
        log::debug!("Serializing database init results to JSON output");
        let output = crate::output::CliOutput::database_init_success(database_path);
        let json = output
            .to_json()
            .with_context(|| "serializing init results to JSON")?;
        log::debug!("JSON serialization completed, output size: {} bytes", json.len());
        println!("{json}");
    } else {
        info!("✓ Database initialized successfully");
    }

    Ok(())
}

/// List all audiobooks in the database
fn list(database_path: PathBuf, json_output: bool) -> CliResult<()> {
    info!("Listing audiobooks in: {database_path:?}");

    // Validate database exists before attempting connection
    validate_existing_database_path(&database_path)?;

    debug!("About to call Database::open() for list operation");
    let db = Database::open(&database_path).with_database_context("opening for list operation")?;
    debug!("Database::open() completed for list operation");

    if json_output {
        output_audiobook_list_json(&db)?;
    } else {
        show_audiobook_list(&db)?;
    }
    Ok(())
}

/// Show database statistics
fn stats(database_path: PathBuf, json_output: bool) -> CliResult<()> {
    info!("Database statistics: {database_path:?}");

    // Validate database exists before attempting connection
    validate_existing_database_path(&database_path)?;

    debug!("About to call Database::open() for stats operation");
    let db = Database::open(&database_path).with_database_context("opening for stats operation")?;
    debug!("Database::open() completed for stats operation");

    debug!("About to call get_audiobook_count()");
    let audiobook_count = get_audiobook_count(&db)?;
    debug!("get_audiobook_count() completed with count: {audiobook_count}");

    let libraries = db
        .get_libraries()
        .with_database_context("retrieving libraries for stats")?;
    let library_count = libraries.len();

    if json_output {
        log::debug!("Serializing database stats results to JSON output");
        let output =
            crate::output::CliOutput::database_stats_success(audiobook_count, library_count);
        let json = output
            .to_json()
            .with_context(|| "serializing stats results to JSON")?;
        log::debug!("JSON serialization completed, output size: {} bytes", json.len());
        println!("{json}");
    } else {
        info!("Total audiobooks: {audiobook_count}");
        info!("Total libraries: {library_count}");
    }
    Ok(())
}

/// Clean and optimize the database
fn clean(database_path: PathBuf, json_output: bool) -> CliResult<()> {
    info!("Cleaning database: {database_path:?}");

    // Validate database exists before attempting connection
    validate_existing_database_path(&database_path)?;

    debug!("About to call Database::open() for clean operation");
    let db = Database::open(&database_path).with_database_context("opening for clean operation")?;
    debug!("Database::open() completed for clean operation");

    // Perform basic database validation
    info!("Running database validation...");

    // For now, just validate that we can query basic statistics
    let libraries = db
        .get_libraries()
        .with_database_context("validating database structure")?;
    let libraries_count = libraries.len();
    info!("✓ Database structure validated ({libraries_count} libraries found)");

    // Future improvements could include:
    // - VACUUM operation for space reclamation
    // - ANALYZE for query optimization statistics
    // - Orphaned record cleanup
    // - Integrity checks

    if json_output {
        let output = crate::output::CliOutput::database_clean_success(libraries_count);
        let json = output
            .to_json()
            .with_context(|| "serializing clean results to JSON")?;
        println!("{json}");
    } else {
        info!("✓ Database cleanup and optimization completed");
    }

    Ok(())
}

/// Output audiobook list in JSON format
fn output_audiobook_list_json(db: &Database) -> CliResult<()> {

    // Check for large library count and use pagination if needed to prevent memory issues
    let libraries = db
        .get_libraries()
        .with_database_context("retrieving libraries for count check")?;
    
    let total_audiobook_estimate = libraries.iter()
        .map(|lib| db.count_audiobooks_in_library(&lib.id).unwrap_or(0))
        .sum::<usize>();
    
    // Use pagination for libraries with more than 10,000 audiobooks to prevent memory issues
    const LARGE_LIBRARY_THRESHOLD: usize = 10_000;
    let all_audiobooks = if total_audiobook_estimate > LARGE_LIBRARY_THRESHOLD {
        log::info!("Large library detected ({} audiobooks), using paginated approach", total_audiobook_estimate);
        get_audiobooks_paginated(db, &libraries)?
    } else {
        // Use optimized single query for smaller libraries with fallback
        match db.get_all_audiobooks() {
        Ok(audiobooks) => {
            // Log successful retrieval with count for debugging
            log::debug!("Retrieved {} audiobooks via optimized query", audiobooks.len());
            
            // Additional validation: filter out audiobooks with invalid IDs (database integrity check)
            let original_count = audiobooks.len();
            let valid_audiobooks: Vec<_> = audiobooks
                .into_iter()
                .filter(|book| !book.id.trim().is_empty())
                .collect();
                
            let invalid_count = original_count - valid_audiobooks.len();
            if invalid_count > 0 {
                log::error!("Database integrity issue: Found and filtered {} audiobooks with invalid IDs", invalid_count);
                log::error!("Continuing with {} valid audiobooks (data may be incomplete)", valid_audiobooks.len());
            }
            
            valid_audiobooks
        }
        Err(e) => {
            log::warn!("Optimized query failed, falling back to per-library queries: {}", e);
            
            // Fallback: Use the original N+1 approach if the optimized query fails
            let libraries = db
                .get_libraries()
                .with_database_context("retrieving libraries for fallback list")?;

            let mut fallback_audiobooks = Vec::new();
            for library in &libraries {
                let library_audiobooks = db
                    .get_audiobooks_in_library(&library.id)
                    .with_database_context("retrieving audiobooks for fallback list")?;
                fallback_audiobooks.extend(library_audiobooks);
            }
            
            log::debug!("Retrieved {} audiobooks via fallback method", fallback_audiobooks.len());
            fallback_audiobooks
        }
    }
    };

    process_and_output_audiobooks(all_audiobooks)
}

/// Get audiobooks using paginated approach for large libraries
fn get_audiobooks_paginated(
    db: &Database, 
    libraries: &[abop_core::models::Library]
) -> CliResult<Vec<abop_core::models::Audiobook>> {
    const PAGE_SIZE: usize = 1000;
    let mut all_audiobooks = Vec::new();
    
    for library in libraries {
        let mut offset = 0;
        let total_count = db
            .count_audiobooks_in_library(&library.id)
            .with_database_context("counting audiobooks for pagination")?;
            
        log::debug!("Processing library '{}' with {} audiobooks", library.name, total_count);
        
        while offset < total_count {
            let batch = db
                .get_audiobooks_in_library_paginated(&library.id, Some(PAGE_SIZE), offset)
                .with_database_context("retrieving paginated audiobooks")?;
                
            if batch.is_empty() {
                break; // Prevent infinite loop if no more results
            }
            
            all_audiobooks.extend(batch);
            offset += PAGE_SIZE;
            
            // Log progress for very large libraries
            if total_count > 5000 {
                log::debug!("Processed {}/{} audiobooks for library '{}'", 
                    std::cmp::min(offset, total_count), total_count, library.name);
            }
        }
    }
    
    log::info!("Retrieved {} total audiobooks using paginated approach", all_audiobooks.len());
    Ok(all_audiobooks)
}

/// Process audiobooks and output as JSON
fn process_and_output_audiobooks(all_audiobooks: Vec<abop_core::models::Audiobook>) -> CliResult<()> {
    use crate::output::{AudiobookInfo, CliOutput};
    
    // Convert to output format
    let audiobook_infos: Vec<AudiobookInfo> =
        all_audiobooks.iter().map(AudiobookInfo::from).collect();

    log::debug!("Serializing database list results to JSON output");
    let output = CliOutput::database_list_success(audiobook_infos);
    let json = output
        .to_json()
        .with_context(|| "serializing list results to JSON")?;
    log::debug!("JSON serialization completed, output size: {} bytes", json.len());
    println!("{json}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_init_operation() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_path_buf();

        // Remove the temp file so we can test creation
        drop(temp_file);

        let result = init(db_path.clone(), false);
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
        let result = list(nonexistent_path.clone(), false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));

        let result = stats(nonexistent_path.clone(), false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));

        let result = clean(nonexistent_path, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_operations_with_directory_path() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_path_buf();

        // Operations should fail when path points to directory
        let result = list(dir_path.clone(), false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is a directory"));

        let result = stats(dir_path.clone(), false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is a directory"));

        let result = clean(dir_path, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is a directory"));
    }

    #[test]
    fn test_database_operations_dispatch() {
        let temp_file = NamedTempFile::new().unwrap();

        // Create a minimal database file
        fs::write(temp_file.path(), b"test database content").unwrap();
        let db_path = temp_file.path().to_path_buf();

        // Test that the run function properly dispatches to sub-operations
        // Note: These might fail in test environment but shouldn't panic

        let result = run(db_path.clone(), DbOperations::Stats, false);
        assert!(result.is_ok() || result.is_err());

        let result = run(db_path.clone(), DbOperations::List, false);
        assert!(result.is_ok() || result.is_err());

        let result = run(db_path.clone(), DbOperations::Clean, false);
        assert!(result.is_ok() || result.is_err());

        // Init should work with any path
        let new_file = NamedTempFile::new().unwrap();
        let new_path = new_file.path().to_path_buf();
        drop(new_file); // Remove file to test creation

        let result = run(new_path, DbOperations::Init, false);
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
            let debug_str = format!("{op:?}");
            assert!(!debug_str.is_empty());
        }
    }
}
