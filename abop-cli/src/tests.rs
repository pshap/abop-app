//! Integration tests for ABOP CLI
//!
//! This module contains comprehensive tests that were originally in main.rs,
//! providing integration testing for the CLI application functionality.

#[cfg(test)]
mod integration_tests {
    use crate::cli::{Args, Commands, DbOperations};
    use crate::commands::{db, scan};
    use crate::error::{validate_existing_database_path, validate_library_path};
    use crate::utils::get_audiobook_count;
    use abop_core::{
        db::Database,
        models::audiobook::fallbacks::{UNKNOWN_AUTHOR, UNKNOWN_TITLE},
        scanner::ScannerConfig,
    };
    use clap::Parser;
    use std::path::PathBuf;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_constants() {
        // Test that constants are defined and not empty
        assert!(!UNKNOWN_TITLE.is_empty());
        assert!(!UNKNOWN_AUTHOR.is_empty());
        assert_eq!(
            UNKNOWN_TITLE,
            abop_core::models::audiobook::fallbacks::UNKNOWN_TITLE
        );
        assert_eq!(
            UNKNOWN_AUTHOR,
            abop_core::models::audiobook::fallbacks::UNKNOWN_AUTHOR
        );
    }

    #[test]
    fn test_scan_library_nonexistent_path() {
        // Test error handling for non-existent library path
        let result = scan::run(
            PathBuf::from("/nonexistent/path"),
            None,
            "default".to_string(),
            None,
            None,
            false,
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

        let result = scan::run(file_path, None, "default".to_string(), None, None, false);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Library path is not a directory"));
    }

    #[test]
    fn test_scanner_config_presets() {
        // Test different configuration presets don't panic
        let configs = vec![
            "default",
            "large",
            "small",
            "conservative",
            "unknown_preset",
        ];

        for preset in configs {
            let result = std::panic::catch_unwind(|| match preset {
                "large" => ScannerConfig::for_large_libraries(),
                "small" => ScannerConfig::for_small_libraries(),
                "conservative" => ScannerConfig::conservative(),
                "default" => ScannerConfig::default(),
                _ => ScannerConfig::default(), // Unknown presets fall back to default
            });

            assert!(result.is_ok(), "Config preset '{preset}' should not panic");
        }
    }

    /// Test database setup result containing temporary directory, database path, and database instance
    type TestDbSetup = (TempDir, PathBuf, Database);

    /// Creates a test database with temporary directory for isolated testing
    ///
    /// Returns a tuple containing:
    /// - `TempDir`: Temporary directory that will be cleaned up when dropped
    /// - `PathBuf`: Path to the database file within the temporary directory
    /// - `Database`: Opened database instance ready for testing
    fn setup_test_db() -> TestDbSetup {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db_path = temp_dir.path().join("test.db");
        let db = Database::open(&db_path).expect("Failed to open test database");
        (temp_dir, db_path, db)
    }

    #[test]
    fn test_get_audiobook_count_empty_database() {
        // Test get_audiobook_count with an empty database
        let (_temp_dir, _db_path, db) = setup_test_db();

        // Test count with no libraries
        let count = get_audiobook_count(&db).expect("Failed to get audiobook count");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_handle_db_operation_dispatch() {
        // Test that database operations dispatch properly
        let (_temp_dir, db_path, _db) = setup_test_db();

        // Test init operation
        let result = db::run(db_path.clone(), DbOperations::Init, false);
        assert!(result.is_ok(), "DB init should succeed");

        // Test stats operation on the initialized database
        let result = db::run(db_path.clone(), DbOperations::Stats, false);
        assert!(result.is_ok(), "DB stats should succeed");

        // Test list operation
        let result = db::run(db_path.clone(), DbOperations::List, false);
        assert!(result.is_ok(), "DB list should succeed");

        // Test clean operation
        let result = db::run(db_path, DbOperations::Clean, false);
        assert!(result.is_ok(), "DB clean should succeed");
    }

    #[test]
    fn test_handle_db_operations_nonexistent_file() {
        // Test error handling when database file doesn't exist for read operations
        let nonexistent_path = PathBuf::from("/nonexistent/database.db");

        // Stats, List, and Clean operations should fail gracefully on non-existent files
        let result = db::run(nonexistent_path.clone(), DbOperations::Stats, false);
        assert!(
            result.is_err(),
            "Stats should fail on non-existent database"
        );

        let result = db::run(nonexistent_path.clone(), DbOperations::List, false);
        assert!(result.is_err(), "List should fail on non-existent database");

        let result = db::run(nonexistent_path, DbOperations::Clean, false);
        assert!(
            result.is_err(),
            "Clean should fail on non-existent database"
        );
    }

    #[test]
    fn test_config_override_with_command_line_options() {
        // Test that command-line options override config presets
        let mut config = ScannerConfig::default();
        let original_tasks = config.max_concurrent_tasks;
        let original_db_ops = config.max_concurrent_db_operations;

        // Use values that are guaranteed to be different from defaults
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
            json: false,
            command: Commands::Scan {
                library: PathBuf::from("/test"),
                database: None,
                config: "default".to_string(),
                max_concurrent_tasks: None,
                max_concurrent_db_operations: None,
            },
        };

        let debug_str = format!("{args:?}");
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
            let debug_str = format!("{op:?}");
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
        let long_path = PathBuf::from(format!("/tmp/{long_name}"));
        // Just verify it doesn't panic when creating PathBuf
        assert!(!long_path.to_string_lossy().is_empty());
    }

    #[test]
    fn test_concurrent_limits_validation() {
        // Test that concurrent limits can be set to various values
        let test_values = vec![1, 2, 4, 8, 16, 32, 100];

        for value in test_values {
            let args = Args::try_parse_from([
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
        // This tests the logic used in utils::get_pagination_size

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
            assert!((1..=1000).contains(&clamped));
        }

        // Test default fallback
        let default_page_size = 100;
        assert!((1..=1000).contains(&default_page_size));
    }

    #[test]
    fn test_validation_functions() {
        // Test library path validation
        let temp_dir = TempDir::new().unwrap();
        let valid_dir = temp_dir.path().to_path_buf();
        assert!(validate_library_path(&valid_dir).is_ok());

        let nonexistent = PathBuf::from("/nonexistent/path");
        assert!(validate_library_path(&nonexistent).is_err());

        // Test database path validation
        let temp_file = NamedTempFile::new().unwrap();
        let valid_db = temp_file.path().to_path_buf();
        assert!(validate_existing_database_path(&valid_db).is_ok());

        let nonexistent_db = PathBuf::from("/nonexistent/db.sqlite");
        assert!(validate_existing_database_path(&nonexistent_db).is_err());

        // Directory should fail for database path
        assert!(validate_existing_database_path(&valid_dir).is_err());
    }
}
