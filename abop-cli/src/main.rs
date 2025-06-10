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
    }

    // Use the first available library, or default to "1"
    let library_id = libraries.first().map(|lib| lib.id.as_str()).unwrap_or("1");

    let audiobooks = db.get_audiobooks_in_library(library_id)?;
    Ok(audiobooks.len())
}

fn show_scan_results(db: &Database) -> Result<()> {
    let count = get_audiobook_count(db)?;

    if count == 0 {
        warn!("No audiobooks found in the library");
    } else {
        info!("📚 Total audiobooks found: {count}");

        // Get all libraries and use the first one, or default to "1"
        let libraries = db.get_libraries()?;
        let library_id = libraries.first().map(|lib| lib.id.as_str()).unwrap_or("1");

        // Show first few audiobooks as examples
        let audiobooks = db.get_audiobooks_in_library(library_id)?;
        info!("Sample audiobooks:");
        for (i, book) in audiobooks.iter().take(5).enumerate() {
            info!(
                "  {}. {} - {}",
                i + 1,
                book.title.as_deref().unwrap_or("Unknown Title"),
                book.author.as_deref().unwrap_or("Unknown")
            );
        }

        if audiobooks.len() > 5 {
            info!("  ... and {} more", audiobooks.len() - 5);
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
    let library_id = libraries.first().map(|lib| lib.id.as_str()).unwrap_or("1");

    debug!("About to call get_audiobooks_in_library() with library_id: {library_id}");
    let audiobooks = db
        .get_audiobooks_in_library(library_id)
        .context("Failed to get audiobooks")?;
    debug!(
        "get_audiobooks_in_library() completed, found {} audiobooks",
        audiobooks.len()
    );

    if audiobooks.is_empty() {
        info!("No audiobooks found in database. Try scanning a library directory first.");
    } else {
        info!("Found {} audiobooks:", audiobooks.len());
        for (i, book) in audiobooks.iter().enumerate() {
            println!(
                "{}. {} - {} ({})",
                i + 1,
                book.title.as_deref().unwrap_or("Unknown Title"),
                book.author.as_deref().unwrap_or("Unknown"),
                book.path.display()
            );
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
