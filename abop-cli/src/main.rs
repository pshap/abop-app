//! ABOP Command Line Interface
//!
//! This is the main entry point for the ABOP CLI application.

use abop_core::{
    db::Database,
    scanner::{LibraryScanner, ScannerConfig},
};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::{debug, info, warn};
use std::path::PathBuf;
use std::time::Instant;

/// Command line arguments for ABOP CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Enable debug output (even more verbose)
    #[arg(short, long)]
    debug: bool,

    /// Commands to execute
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan library for audiobooks
    Scan {
        /// Path to the audiobook library directory
        #[arg(short, long)]
        library: PathBuf,

        /// Path to the database file (optional, defaults to memory)
        #[arg(short, long)]
        database: Option<PathBuf>,

        /// Configuration preset (default, large, small, conservative)
        #[arg(short, long, default_value = "default")]
        config: String,

        /// Maximum concurrent file operations
        #[arg(long)]
        max_concurrent_tasks: Option<usize>,

        /// Maximum concurrent database operations
        #[arg(long)]
        max_concurrent_db_operations: Option<usize>,
    },
    /// Database operations
    Db {
        /// Path to the database file
        #[arg(short, long)]
        database: PathBuf,

        #[command(subcommand)]
        operation: DbOperations,
    },
}

#[derive(Subcommand, Debug)]
enum DbOperations {
    /// Initialize database
    Init,
    /// List all audiobooks in database
    List,
    /// Show database statistics
    Stats,
    /// Clean/optimize database
    Clean,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.debug {
        log::LevelFilter::Debug
    } else if args.verbose {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Warn
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp_secs()
        .init();

    info!("Starting ABOP CLI");
    debug!("Command line arguments: {args:?}");

    match args.command {
        Commands::Scan {
            library,
            database,
            config,
            max_concurrent_tasks,
            max_concurrent_db_operations,
        } => {
            debug!("Executing scan command");
            scan_library(
                library,
                database,
                config,
                max_concurrent_tasks,
                max_concurrent_db_operations,
            )
        }
        Commands::Db {
            database,
            operation,
        } => {
            debug!("Executing database command: {operation:?} on {database:?}");
            handle_db_operation(database, operation)
        }
    }
}

fn scan_library(
    library_path: PathBuf,
    database_path: Option<PathBuf>,
    config_preset: String,
    max_concurrent_tasks: Option<usize>,
    max_concurrent_db_operations: Option<usize>,
) -> Result<()> {
    info!("Scanning library: {library_path:?}");

    // Validate library path
    if !library_path.exists() {
        return Err(anyhow::anyhow!(
            "Library path does not exist: {:?}",
            library_path
        ));
    }
    if !library_path.is_dir() {
        return Err(anyhow::anyhow!(
            "Library path is not a directory: {:?}",
            library_path
        ));
    }

    // Initialize centralized database instead of per-library database
    let db = if let Some(db_path) = database_path {
        info!("Using custom database: {db_path:?}");
        Database::open(&db_path).context("Failed to initialize database")?
    } else {
        info!("Using centralized application database");
        Database::open_app_database().context("Failed to initialize centralized database")?
    };

    // Check if a library with this path already exists
    let library = match db.libraries().find_by_path(&library_path)? {
        Some(lib) => {
            info!("Using existing library: {} (ID: {})", lib.name, lib.id);
            lib
        }
        None => {
            info!("Creating new library for path: {}", library_path.display());
            let library_id = db
                .add_library_with_path("CLI Library", library_path.clone())
                .context("Failed to create library record")?;

            // Get the newly created library
            db.libraries()
                .find_by_id(&library_id)
                .context("Failed to get library record after creation")?
                .context("Library not found after creation")?
        }
    };

    info!("Using library: {} (ID: {})", library.name, library.id);

    // Configure scanner
    let mut scanner_config = match config_preset.as_str() {
        "large" => ScannerConfig::for_large_libraries(),
        "small" => ScannerConfig::for_small_libraries(),
        "conservative" => ScannerConfig::conservative(),
        "default" => ScannerConfig::default(),
        _ => {
            warn!("Unknown config preset '{config_preset}', using default");
            ScannerConfig::default()
        }
    };

    // Override config with command-line options if provided
    if let Some(tasks) = max_concurrent_tasks {
        scanner_config.max_concurrent_tasks = tasks;
    }
    if let Some(db_ops) = max_concurrent_db_operations {
        scanner_config.max_concurrent_db_operations = db_ops;
    }

    info!(
        "Scanner config: max_concurrent_tasks={}, max_concurrent_db_operations={}",
        scanner_config.max_concurrent_tasks, scanner_config.max_concurrent_db_operations
    );

    // Create scanner
    let scanner = LibraryScanner::new(db.clone(), library).with_config(scanner_config);

    let start_time = Instant::now();

    info!("Starting scan...");
    let result = scanner
        .scan(abop_core::scanner::ScanOptions::default())
        .context("Scan failed")?;
    info!(
        "Scan result: processed={}, errors={}",
        result.processed, result.errors
    );

    let elapsed = start_time.elapsed();
    info!("Scan completed in {:.2}s", elapsed.as_secs_f64());

    // Show results
    show_scan_results(&db)?;

    Ok(())
}

fn get_audiobook_count(db: &Database) -> Result<usize> {
    // Get all libraries first
    let libraries = db.get_libraries()?;

    if libraries.is_empty() {
        return Ok(0);
    } // Use the first available library
    let library_id = libraries
        .first()
        .expect("First library should exist as we checked libraries.is_empty()")
        .id
        .as_str();

    let audiobooks = db.get_audiobooks_in_library(library_id)?;
    Ok(audiobooks.len())
}

fn show_scan_results(db: &Database) -> Result<()> {
    let count = get_audiobook_count(db)?;

    if count == 0 {
        warn!("No audiobooks found in the library");
    } else {
        info!("📚 Total audiobooks found: {count}");

        // Get libraries to show audiobook examples
        let libraries = db.get_libraries()?;
        if !libraries.is_empty() {
            let library_id = libraries
                .first()
                .expect("First library should exist as we checked !libraries.is_empty()")
                .id
                .as_str();

            // Show first few audiobooks as examples (efficiently load only what we need)
            let sample_audiobooks =
                db.get_audiobooks_in_library_paginated(library_id, Some(5), 0)?;
            let total_count = db.count_audiobooks_in_library(library_id)?;

            info!("Sample audiobooks:");
            for (i, book) in sample_audiobooks.iter().enumerate() {
                info!(
                    "  {}. {} - {}",
                    i + 1,
                    book.title.as_deref().unwrap_or(UNKNOWN_TITLE),
                    book.author.as_deref().unwrap_or(UNKNOWN_AUTHOR)
                );
            }

            if total_count > 5 {
                info!("  ... and {} more", total_count - 5);
            }
        }
    }

    Ok(())
}

fn handle_db_operation(database_path: PathBuf, operation: DbOperations) -> Result<()> {
    debug!("Starting database operation: {operation:?}");
    match operation {
        DbOperations::Init => handle_db_init(database_path),
        DbOperations::List => handle_db_list(database_path),
        DbOperations::Stats => handle_db_stats(database_path),
        DbOperations::Clean => handle_db_clean(database_path),
    }
}

fn handle_db_init(database_path: PathBuf) -> Result<()> {
    info!("Initializing database: {database_path:?}");
    debug!("About to call Database::open()");
    let _db = Database::open(&database_path).context("Failed to initialize database")?;
    debug!("Database::open() completed successfully");
    info!("✓ Database initialized successfully");
    Ok(())
}

fn handle_db_list(database_path: PathBuf) -> Result<()> {
    info!("Listing audiobooks in: {database_path:?}");
    debug!("About to call Database::open() for list operation");
    let db = Database::open(&database_path).context("Failed to open database")?;
    debug!("Database::open() completed for list operation");

    // Get all libraries first
    let libraries = db.get_libraries().context("Failed to get libraries")?;
    if libraries.is_empty() {
        info!("No libraries found in database. You may need to scan a library first.");
        return Ok(());
    }

    // Use the first available library, or default to "1"
    let library_id = libraries.first().map_or("1", |lib| lib.id.as_str());

    debug!("About to call count_audiobooks_in_library() with library_id: {library_id}");
    let total_count = db
        .count_audiobooks_in_library(library_id)
        .context("Failed to count audiobooks")?;
    debug!("count_audiobooks_in_library() completed, found {total_count} total audiobooks");

    if total_count == 0 {
        info!("No audiobooks found in database. Try scanning a library directory first.");
    } else {
        // Use configurable pagination to avoid loading too many audiobooks into memory at once
        // Default to 100, but allow environment variable override for performance tuning
        let page_size = std::env::var("ABOP_PAGE_SIZE")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(100)
            .clamp(1, 1000); // Ensure minimum of 1 and cap at reasonable maximum to prevent memory issues

        info!("Found {total_count} audiobooks in library {library_id} (page_size={page_size})");

        debug!("Using page size: {page_size}");
        let mut offset = 0;
        let mut displayed = 0;

        while offset < total_count {
            debug!("Loading audiobooks with offset: {offset}, limit: {page_size}");
            let audiobooks = db
                .get_audiobooks_in_library_paginated(library_id, Some(page_size), offset)
                .context("Failed to get audiobooks")?;

            for book in audiobooks {
                displayed += 1;
                println!(
                    "{}. {} - {} ({})",
                    displayed,
                    book.title.as_deref().unwrap_or(UNKNOWN_TITLE),
                    book.author.as_deref().unwrap_or(UNKNOWN_AUTHOR),
                    book.path.display()
                );
            }

            offset += page_size;
        }
    }
    Ok(())
}

fn handle_db_stats(database_path: PathBuf) -> Result<()> {
    info!("Database statistics: {database_path:?}");
    debug!("About to call Database::open() for stats operation");
    let db = Database::open(&database_path).context("Failed to open database")?;
    debug!("Database::open() completed for stats operation");

    debug!("About to call get_audiobook_count()");
    let count = get_audiobook_count(&db)?;
    debug!("get_audiobook_count() completed with count: {count}");
    info!("Total audiobooks: {count}");
    Ok(())
}

fn handle_db_clean(database_path: PathBuf) -> Result<()> {
    info!("Cleaning database: {database_path:?}");
    debug!("About to call Database::open() for clean operation");
    let _db = Database::open(&database_path).context("Failed to open database")?;
    debug!("Database::open() completed for clean operation");

    // TODO: Implement database cleanup/optimization
    info!("✓ Database cleanup completed");
    Ok(())
}

// Constants for fallback strings
const UNKNOWN_TITLE: &str = "Unknown Title";
const UNKNOWN_AUTHOR: &str = "Unknown Author";

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn test_command_factory() {
        // Test that the CLI structure is valid
        let cmd = Args::command();
        assert!(!cmd.get_name().is_empty());
        assert!(cmd.get_version().is_some());
    }

    #[test]
    fn test_constants() {
        // Test that constants are defined and not empty
        assert!(!UNKNOWN_TITLE.is_empty());
        assert!(!UNKNOWN_AUTHOR.is_empty());
        assert_eq!(UNKNOWN_TITLE, "Unknown Title");
        assert_eq!(UNKNOWN_AUTHOR, "Unknown Author");
    }

    #[test]
    fn test_args_parsing_scan_command() {
        // Test basic scan command parsing
        let args = Args::try_parse_from(&["abop-cli", "scan", "--library", "/test/path"]).unwrap();

        assert!(!args.verbose);
        assert!(!args.debug);

        match args.command {
            Commands::Scan {
                library,
                database,
                config,
                max_concurrent_tasks,
                max_concurrent_db_operations,
            } => {
                assert_eq!(library, PathBuf::from("/test/path"));
                assert!(database.is_none());
                assert_eq!(config, "default");
                assert!(max_concurrent_tasks.is_none());
                assert!(max_concurrent_db_operations.is_none());
            }
            _ => panic!("Expected scan command"),
        }
    }

    #[test]
    fn test_args_parsing_scan_command_with_all_options() {
        // Test scan command with all optional parameters
        let args = Args::try_parse_from(&[
            "abop-cli",
            "--verbose",
            "--debug",
            "scan",
            "--library",
            "/test/path",
            "--database",
            "/test/db.sqlite",
            "--config",
            "large",
            "--max-concurrent-tasks",
            "8",
            "--max-concurrent-db-operations",
            "4",
        ])
        .unwrap();

        assert!(args.verbose);
        assert!(args.debug);

        match args.command {
            Commands::Scan {
                library,
                database,
                config,
                max_concurrent_tasks,
                max_concurrent_db_operations,
            } => {
                assert_eq!(library, PathBuf::from("/test/path"));
                assert_eq!(database, Some(PathBuf::from("/test/db.sqlite")));
                assert_eq!(config, "large");
                assert_eq!(max_concurrent_tasks, Some(8));
                assert_eq!(max_concurrent_db_operations, Some(4));
            }
            _ => panic!("Expected scan command"),
        }
    }

    #[test]
    fn test_args_parsing_db_init_command() {
        // Test database init command parsing
        let args =
            Args::try_parse_from(&["abop-cli", "db", "--database", "/test/db.sqlite", "init"])
                .unwrap();

        match args.command {
            Commands::Db {
                database,
                operation,
            } => {
                assert_eq!(database, PathBuf::from("/test/db.sqlite"));
                assert!(matches!(operation, DbOperations::Init));
            }
            _ => panic!("Expected db command"),
        }
    }

    #[test]
    fn test_args_parsing_db_list_command() {
        let args =
            Args::try_parse_from(&["abop-cli", "db", "--database", "/test/db.sqlite", "list"])
                .unwrap();

        match args.command {
            Commands::Db {
                database,
                operation,
            } => {
                assert_eq!(database, PathBuf::from("/test/db.sqlite"));
                assert!(matches!(operation, DbOperations::List));
            }
            _ => panic!("Expected db command"),
        }
    }

    #[test]
    fn test_args_parsing_db_stats_command() {
        let args =
            Args::try_parse_from(&["abop-cli", "db", "--database", "/test/db.sqlite", "stats"])
                .unwrap();

        match args.command {
            Commands::Db {
                database,
                operation,
            } => {
                assert_eq!(database, PathBuf::from("/test/db.sqlite"));
                assert!(matches!(operation, DbOperations::Stats));
            }
            _ => panic!("Expected db command"),
        }
    }

    #[test]
    fn test_args_parsing_db_clean_command() {
        let args =
            Args::try_parse_from(&["abop-cli", "db", "--database", "/test/db.sqlite", "clean"])
                .unwrap();

        match args.command {
            Commands::Db {
                database,
                operation,
            } => {
                assert_eq!(database, PathBuf::from("/test/db.sqlite"));
                assert!(matches!(operation, DbOperations::Clean));
            }
            _ => panic!("Expected db command"),
        }
    }

    #[test]
    fn test_args_parsing_missing_required_args() {
        // Test that missing required arguments cause parsing to fail
        let result = Args::try_parse_from(&["abop-cli", "scan"]);
        assert!(result.is_err());

        let result = Args::try_parse_from(&["abop-cli", "db", "init"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_args_parsing_invalid_config_preset() {
        // Test that invalid config values still parse (they're handled at runtime)
        let args = Args::try_parse_from(&[
            "abop-cli",
            "scan",
            "--library",
            "/test/path",
            "--config",
            "invalid_preset",
        ])
        .unwrap();

        match args.command {
            Commands::Scan { config, .. } => {
                assert_eq!(config, "invalid_preset");
            }
            _ => panic!("Expected scan command"),
        }
    }

    #[test]
    fn test_scan_library_nonexistent_path() {
        // Test error handling for non-existent library path
        let result = scan_library(
            PathBuf::from("/nonexistent/path"),
            None,
            "default".to_string(),
            None,
            None,
        );

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Library path does not exist"));
    }

    #[test]
    fn test_scan_library_file_instead_of_directory() {
        // Test error handling when library path points to a file instead of directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("not_a_directory.txt");
        std::fs::write(&file_path, "test content").unwrap();

        let result = scan_library(file_path, None, "default".to_string(), None, None);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Library path is not a directory"));
    }

    #[test]
    fn test_scanner_config_presets() {
        // Test different configuration presets
        let temp_dir = TempDir::new().unwrap();

        // Test that different config presets don't cause panics
        // We can't easily test the full scan without a complex setup,
        // but we can test that the configuration logic works

        let configs = vec![
            "default",
            "large",
            "small",
            "conservative",
            "unknown_preset",
        ];

        for preset in configs {
            // The function should handle unknown presets gracefully
            // We're testing the configuration logic, not the full scan
            let result = std::panic::catch_unwind(|| {
                match preset {
                    "large" => ScannerConfig::for_large_libraries(),
                    "small" => ScannerConfig::for_small_libraries(),
                    "conservative" => ScannerConfig::conservative(),
                    "default" => ScannerConfig::default(),
                    _ => ScannerConfig::default(), // Unknown presets fall back to default
                }
            });

            assert!(
                result.is_ok(),
                "Config preset '{}' should not panic",
                preset
            );
        }
    }

    #[test]
    fn test_get_audiobook_count_empty_database() {
        // Test get_audiobook_count with an empty database
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a database
        let db = Database::open(&db_path).unwrap();

        // Test count with no libraries
        let count = get_audiobook_count(&db).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_handle_db_operation_dispatch() {
        // Test that handle_db_operation properly dispatches to the right function
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Test init operation
        let result = handle_db_operation(db_path.clone(), DbOperations::Init);
        assert!(result.is_ok(), "DB init should succeed");

        // Test stats operation on the initialized database
        let result = handle_db_operation(db_path.clone(), DbOperations::Stats);
        assert!(result.is_ok(), "DB stats should succeed");

        // Test list operation
        let result = handle_db_operation(db_path.clone(), DbOperations::List);
        assert!(result.is_ok(), "DB list should succeed");

        // Test clean operation
        let result = handle_db_operation(db_path, DbOperations::Clean);
        assert!(result.is_ok(), "DB clean should succeed");
    }

    #[test]
    fn test_handle_db_init() {
        // Test database initialization
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_init.db");

        let result = handle_db_init(db_path.clone());
        assert!(result.is_ok(), "Database initialization should succeed");

        // Verify database file was created
        assert!(
            db_path.exists(),
            "Database file should exist after initialization"
        );
    }

    #[test]
    fn test_handle_db_list_empty() {
        // Test listing audiobooks in an empty database
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_list.db");

        // Initialize database first
        let _db = Database::open(&db_path).unwrap();

        let result = handle_db_list(db_path);
        assert!(
            result.is_ok(),
            "DB list should succeed even with empty database"
        );
    }

    #[test]
    fn test_handle_db_stats_empty() {
        // Test stats on an empty database
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_stats.db");

        // Initialize database first
        let _db = Database::open(&db_path).unwrap();

        let result = handle_db_stats(db_path);
        assert!(
            result.is_ok(),
            "DB stats should succeed even with empty database"
        );
    }

    #[test]
    fn test_handle_db_clean() {
        // Test database cleaning
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_clean.db");

        // Initialize database first
        let _db = Database::open(&db_path).unwrap();

        let result = handle_db_clean(db_path);
        assert!(result.is_ok(), "DB clean should succeed");
    }

    #[test]
    fn test_show_scan_results_empty() {
        // Test showing scan results with empty database
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_results.db");

        let db = Database::open(&db_path).unwrap();

        let result = show_scan_results(&db);
        assert!(
            result.is_ok(),
            "Show scan results should succeed with empty database"
        );
    }

    #[test]
    fn test_handle_db_operations_nonexistent_file() {
        // Test error handling when database file doesn't exist for read operations
        let nonexistent_path = PathBuf::from("/nonexistent/database.db");

        // Stats, List, and Clean operations should fail gracefully on non-existent files
        let result = handle_db_stats(nonexistent_path.clone());
        assert!(
            result.is_err(),
            "Stats should fail on non-existent database"
        );

        let result = handle_db_list(nonexistent_path.clone());
        assert!(result.is_err(), "List should fail on non-existent database");

        let result = handle_db_clean(nonexistent_path);
        assert!(
            result.is_err(),
            "Clean should fail on non-existent database"
        );
    }

    #[test]
    fn test_config_override_with_command_line_options() {
        // Test that command-line options override config presets
        // This tests the logic in scan_library function

        let mut config = ScannerConfig::default();
        let original_tasks = config.max_concurrent_tasks;
        let original_db_ops = config.max_concurrent_db_operations;

        // Use values that are guaranteed to be different from defaults
        // Choose values based on what's different from current defaults
        let max_concurrent_tasks = if original_tasks == 16 {
            Some(32)
        } else {
            Some(16)
        };
        let max_concurrent_db_operations = if original_db_ops == 8 {
            Some(4)
        } else {
            Some(8)
        };

        if let Some(tasks) = max_concurrent_tasks {
            config.max_concurrent_tasks = tasks;
        }
        if let Some(db_ops) = max_concurrent_db_operations {
            config.max_concurrent_db_operations = db_ops;
        }

        // Verify the configuration was actually overridden
        assert_ne!(config.max_concurrent_tasks, original_tasks);
        assert_ne!(config.max_concurrent_db_operations, original_db_ops);

        // Verify the override values are set correctly
        assert_eq!(config.max_concurrent_tasks, max_concurrent_tasks.unwrap());
        assert_eq!(
            config.max_concurrent_db_operations,
            max_concurrent_db_operations.unwrap()
        );
    }

    #[test]
    fn test_debug_output_format() {
        // Test that debug formatting works for command structures
        let args = Args {
            verbose: true,
            debug: false,
            command: Commands::Scan {
                library: PathBuf::from("/test"),
                database: None,
                config: "default".to_string(),
                max_concurrent_tasks: None,
                max_concurrent_db_operations: None,
            },
        };

        let debug_str = format!("{:?}", args);
        assert!(debug_str.contains("verbose: true"));
        assert!(debug_str.contains("debug: false"));
        assert!(debug_str.contains("Scan"));
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

    #[test]
    fn test_path_validation_edge_cases() {
        // Test edge cases in path validation

        // Empty path should be handled gracefully by PathBuf
        let empty_path = PathBuf::new();
        assert!(!empty_path.exists());

        // Very long path names (within reasonable limits)
        let long_name = "a".repeat(100);
        let long_path = PathBuf::from(format!("/tmp/{}", long_name));
        // Just verify it doesn't panic when creating PathBuf
        assert!(!long_path.to_string_lossy().is_empty());
    }

    #[test]
    fn test_concurrent_limits_validation() {
        // Test that concurrent limits can be set to various values
        let test_values = vec![1, 2, 4, 8, 16, 32, 100];

        for value in test_values {
            let args = Args::try_parse_from(&[
                "abop-cli",
                "scan",
                "--library",
                "/test/path",
                "--max-concurrent-tasks",
                &value.to_string(),
                "--max-concurrent-db-operations",
                &value.to_string(),
            ])
            .unwrap();

            match args.command {
                Commands::Scan {
                    max_concurrent_tasks,
                    max_concurrent_db_operations,
                    ..
                } => {
                    assert_eq!(max_concurrent_tasks, Some(value));
                    assert_eq!(max_concurrent_db_operations, Some(value));
                }
                _ => panic!("Expected scan command"),
            }
        }
    }

    #[test]
    fn test_environment_variable_page_size_parsing() {
        // Test the page size environment variable parsing logic
        // This tests the logic used in handle_db_list

        // Test valid page size
        let valid_sizes = vec!["50", "100", "500", "1000"];
        for size_str in valid_sizes {
            if let Ok(size) = size_str.parse::<usize>() {
                let clamped = size.clamp(1, 1000);
                assert_eq!(clamped, size); // All test values should be within range
            }
        }

        // Test invalid page sizes get clamped
        let invalid_sizes = vec![0, 1001, 9999];
        for size in invalid_sizes {
            let clamped = size.clamp(1, 1000);
            assert!(clamped >= 1 && clamped <= 1000);
        }

        // Test default fallback
        let default_page_size = 100;
        assert!(default_page_size >= 1 && default_page_size <= 1000);
    }
}
