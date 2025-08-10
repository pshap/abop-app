//! CLI definition and argument parsing for ABOP
//!
//! This module defines the command-line interface structure using clap's derive API.
//! It follows modern Rust CLI patterns with clear separation between command definition
//! and command implementation.

use crate::error::CliResult;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Command line arguments for ABOP CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Enable debug output (even more verbose)
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// Commands to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan library for audiobooks
    Scan {
        /// Path to the audiobook library directory
        #[arg(short, long)]
        library: PathBuf,

        /// Path to the database file (optional, defaults to centralized app database)
        #[arg(short = 'f', long)]
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
        #[arg(short = 'f', long)]
        database: PathBuf,

        #[command(subcommand)]
        operation: DbOperations,
    },
}

#[derive(Subcommand, Debug)]
pub enum DbOperations {
    /// Initialize database
    Init,
    /// List all audiobooks in database
    List,
    /// Show database statistics
    Stats,
    /// Clean/optimize database
    Clean,
}

/// Initialize logging based on CLI arguments
pub fn init_logging(args: &Args) {
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
}

/// Main CLI dispatch function
pub fn run() -> CliResult<()> {
    let args = Args::parse();
    
    init_logging(&args);
    
    log::info!("Starting ABOP CLI");
    log::debug!("Command line arguments: {args:?}");

    match args.command {
        Commands::Scan {
            library,
            database,
            config,
            max_concurrent_tasks,
            max_concurrent_db_operations,
        } => {
            log::debug!("Executing scan command");
            crate::commands::scan::run(
                library,
                database,
                config,
                max_concurrent_tasks,
                max_concurrent_db_operations,
            )
        }
        Commands::Db { database, operation } => {
            log::debug!("Executing database command: {operation:?} on {database:?}");
            crate::commands::db::run(database, operation)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_command_factory() {
        // Test that the CLI structure is valid
        let cmd = Args::command();
        assert!(!cmd.get_name().is_empty());
        assert!(cmd.get_version().is_some());
    }

    #[test]
    fn test_args_parsing_scan_command() {
        // Test basic scan command parsing
        let args = Args::try_parse_from(["abop-cli", "scan", "--library", "/test/path"]).unwrap();

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
        let args = Args::try_parse_from([
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
    fn test_args_parsing_db_commands() {
        let operations = [
            ("init", DbOperations::Init),
            ("list", DbOperations::List),
            ("stats", DbOperations::Stats),
            ("clean", DbOperations::Clean),
        ];

        for (op_name, expected_op) in operations {
            let args = Args::try_parse_from([
                "abop-cli",
                "db",
                "--database",
                "/test/db.sqlite",
                op_name,
            ])
            .unwrap();

            match args.command {
                Commands::Db { database, operation } => {
                    assert_eq!(database, PathBuf::from("/test/db.sqlite"));
                    assert!(std::mem::discriminant(&operation) == std::mem::discriminant(&expected_op));
                }
                _ => panic!("Expected db command"),
            }
        }
    }

    #[test]
    fn test_args_parsing_missing_required_args() {
        // Test that missing required arguments cause parsing to fail
        let result = Args::try_parse_from(["abop-cli", "scan"]);
        assert!(result.is_err());

        let result = Args::try_parse_from(["abop-cli", "db", "init"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_global_flags() {
        // Test that global flags work with any command
        let args = Args::try_parse_from([
            "abop-cli",
            "--verbose",
            "scan",
            "--library", 
            "/test/path"
        ]).unwrap();
        
        assert!(args.verbose);
        
        let args = Args::try_parse_from([
            "abop-cli",
            "--debug",
            "db",
            "--database",
            "/test/db.sqlite",
            "stats"
        ]).unwrap();
        
        assert!(args.debug);
    }
}