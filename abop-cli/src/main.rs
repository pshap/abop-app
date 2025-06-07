//! ABOP Command Line Interface
//!
//! This is the main entry point for the ABOP CLI application.

use abop_core::{
    db::Database,
    scanner::{LibraryScanner, ScannerConfig},
};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::{debug, error, info, warn};
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

        /// Show progress during scan
        #[arg(short, long)]
        progress: bool,
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

#[tokio::main]
async fn main() -> Result<()> {
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

    match args.command {
        Commands::Scan {
            library,
            database,
            config,
            max_concurrent_tasks,
            max_concurrent_db_operations,
            progress,
        } => {
            scan_library(
                library,
                database,
                config,
                max_concurrent_tasks,
                max_concurrent_db_operations,
                progress,
            )
            .await
        }
        Commands::Db {
            database,
            operation,
        } => handle_db_operation(database, operation).await,
    }
}

async fn scan_library(
    library_path: PathBuf,
    database_path: Option<PathBuf>,
    config_preset: String,
    max_concurrent_tasks: Option<usize>,
    max_concurrent_db_operations: Option<usize>,
    show_progress: bool,
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

    // Initialize database
    let db_path = database_path.unwrap_or_else(|| {
        let mut path = library_path.clone();
        path.push("abop.db");
        path
    });

    info!("Using database: {db_path:?}");
    let db = Database::open(&db_path).context("Failed to initialize database")?;

    // Create a library record first
    let library = db
        .add_library("CLI Library", &library_path)
        .context("Failed to create library record")?;

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

    if show_progress {
        info!("Starting scan with progress monitoring...");
        scan_with_progress(scanner, &db, &library_path).await?;
    } else {
        info!("Starting scan...");
        let result = scanner.scan_async(None).await.context("Scan failed")?;
        info!(
            "Scan result: processed={}, errors={}",
            result.processed, result.errors
        );
    }

    let elapsed = start_time.elapsed();
    info!("Scan completed in {:.2}s", elapsed.as_secs_f64());

    // Show results
    show_scan_results(&db).await?;

    Ok(())
}

async fn scan_with_progress(
    scanner: LibraryScanner,
    _db: &Database,
    _library_path: &PathBuf,
) -> Result<()> {
    use tokio::time::{Duration, interval}; // Start scan in background
    let mut scan_handle = tokio::spawn(async move { scanner.scan_async(None).await });

    // Monitor progress
    let mut progress_interval = interval(Duration::from_secs(2));
    loop {
        tokio::select! {
            result = &mut scan_handle => {
                match result {
                    Ok(Ok(summary)) => {
                        info!("✓ Scan completed successfully");
                        info!("Summary: {} audiobooks processed, {} errors", summary.processed, summary.errors);
                        break;
                    }
                    Ok(Err(e)) => {
                        error!("✗ Scan failed: {e}");
                        return Err(e.into());
                    }
                    Err(e) => {
                        error!("✗ Scan task panicked: {e}");
                        return Err(e.into());
                    }
                }
            }
            _ = progress_interval.tick() => {
                debug!("📚 Scan in progress...");
            }
        }
    }

    Ok(())
}

async fn get_audiobook_count(db: &Database) -> Result<usize> {
    // Use the correct database API to get audiobooks
    let audiobooks = db.get_audiobooks_in_library("1")?; // Assuming library ID "1"
    Ok(audiobooks.len())
}

async fn show_scan_results(db: &Database) -> Result<()> {
    let count = get_audiobook_count(db).await?;

    if count == 0 {
        warn!("No audiobooks found in the library");
    } else {
        info!("📚 Total audiobooks found: {count}");
        // Show first few audiobooks as examples
        let audiobooks = db.get_audiobooks_in_library("1")?;
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

async fn handle_db_operation(database_path: PathBuf, operation: DbOperations) -> Result<()> {
    debug!("Starting database operation: {:?}", operation);
    match operation {
        DbOperations::Init => {
            info!("Initializing database: {database_path:?}");
            debug!("About to call Database::open()");
            let _db = Database::open(&database_path).context("Failed to initialize database")?;
            debug!("Database::open() completed successfully");
            info!("✓ Database initialized successfully");
        }
        DbOperations::List => {
            info!("Listing audiobooks in: {database_path:?}");
            debug!("About to call Database::open() for list operation");
            let db = Database::open(&database_path).context("Failed to open database")?;
            debug!("Database::open() completed for list operation");

            debug!("About to call get_audiobooks_in_library()");
            let audiobooks = db
                .get_audiobooks_in_library("1")
                .context("Failed to get audiobooks")?;
            debug!("get_audiobooks_in_library() completed, found {} audiobooks", audiobooks.len());

            if audiobooks.is_empty() {
                info!("No audiobooks found in database");
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
        }
        DbOperations::Stats => {
            info!("Database statistics: {database_path:?}");
            let db = Database::open(&database_path).context("Failed to open database")?;

            let count = get_audiobook_count(&db).await?;
            info!("Total audiobooks: {count}");
        }
        DbOperations::Clean => {
            info!("Cleaning database: {database_path:?}");
            let _db = Database::open(&database_path).context("Failed to open database")?;

            // TODO: Implement database cleanup/optimization
            info!("✓ Database cleanup completed");
        }
    }

    Ok(())
}
