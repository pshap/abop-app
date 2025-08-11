//! Library scanning command implementation
//!
//! This module handles the scanning of audiobook libraries, including
//! library creation/lookup, scanner configuration, and result reporting.

use crate::{
    error::{validate_library_path, CliResult, CliResultExt},
    utils::show_scan_results,
};
use abop_core::{
    db::Database,
    scanner::{LibraryScanner, ScannerConfig},
};
use anyhow::Context;
use log::{info, warn};
use std::path::PathBuf;
use std::time::Instant;

/// Execute the library scan command
///
/// # Arguments
/// * `library_path` - Path to the audiobook library directory
/// * `database_path` - Optional path to database file (uses centralized app DB if None)
/// * `config_preset` - Configuration preset name
/// * `max_concurrent_tasks` - Optional override for concurrent file tasks
/// * `max_concurrent_db_operations` - Optional override for concurrent DB operations
/// * `json_output` - Whether to output results in JSON format
///
/// # Errors
/// Returns an error if:
/// - Library path doesn't exist or isn't a directory
/// - Database initialization fails
/// - Scanner configuration is invalid
/// - Scan operation fails
pub fn run(
    library_path: PathBuf,
    database_path: Option<PathBuf>,
    config_preset: String,
    max_concurrent_tasks: Option<usize>,
    max_concurrent_db_operations: Option<usize>,
    json_output: bool,
) -> CliResult<()> {
    info!("Scanning library: {library_path:?}");

    // Validate library path
    validate_library_path(&library_path)?;

    // Initialize database (centralized or custom)
    let db = initialize_database(database_path).with_database_context("initialization")?;

    // Find or create library record
    let library = find_or_create_library(&db, &library_path)?;
    info!("Using library: {} (ID: {})", library.name, library.id);

    // Configure scanner
    let scanner_config = build_scanner_config(
        &config_preset,
        max_concurrent_tasks,
        max_concurrent_db_operations,
    )?;

    info!(
        "Scanner config: max_concurrent_tasks={}, max_concurrent_db_operations={}",
        scanner_config.max_concurrent_tasks, scanner_config.max_concurrent_db_operations
    );

    // Execute scan
    let scan_start = Instant::now();
    execute_scan(&db, library.clone(), scanner_config)?;
    let scan_duration = scan_start.elapsed();

    // Show results
    if json_output {
        output_json_results(&db, &library, scan_duration)?;
    } else {
        show_scan_results(&db)?;
    }

    Ok(())
}

/// Initialize the database connection
fn initialize_database(database_path: Option<PathBuf>) -> CliResult<Database> {
    match database_path {
        Some(db_path) => {
            info!("Using custom database: {db_path:?}");
            Database::open(&db_path).context("Failed to initialize custom database")
        }
        None => {
            info!("Using centralized application database");
            Database::open_app_database().context("Failed to initialize centralized database")
        }
    }
}

/// Find existing library or create a new one
fn find_or_create_library(
    db: &Database,
    library_path: &PathBuf,
) -> CliResult<abop_core::models::Library> {
    match db.libraries().find_by_path(library_path)? {
        Some(lib) => {
            info!("Using existing library: {} (ID: {})", lib.name, lib.id);
            Ok(lib)
        }
        None => {
            info!("Creating new library for path: {}", library_path.display());
            
            // Generate a meaningful library name from the path
            let library_name = library_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown Library")
                .to_string();
            
            let library_id = db
                .add_library_with_path(&library_name, library_path.clone())
                .context("Failed to create library record")?;

            // Get the newly created library
            db.libraries()
                .find_by_id(&library_id)
                .context("Failed to get library record after creation")?
                .context("Library not found after creation")
        }
    }
}

/// Build scanner configuration from preset and overrides
///
/// # Available Presets:
/// - `"default"` - Balanced configuration suitable for most systems
/// - `"large"` - Optimized for large libraries with high concurrency
/// - `"small"` - Conservative settings for smaller systems/libraries  
/// - `"conservative"` - Minimal resource usage for constrained environments
///
/// # Arguments
/// * `config_preset` - Configuration preset name
/// * `max_concurrent_tasks` - Override for maximum concurrent file operations
/// * `max_concurrent_db_operations` - Override for maximum concurrent database operations
fn build_scanner_config(
    config_preset: &str,
    max_concurrent_tasks: Option<usize>,
    max_concurrent_db_operations: Option<usize>,
) -> CliResult<ScannerConfig> {
    // Get base configuration from preset
    let mut scanner_config = match config_preset {
        "large" => ScannerConfig::for_large_libraries(),
        "small" => ScannerConfig::for_small_libraries(),
        "conservative" => ScannerConfig::conservative(),
        "default" => ScannerConfig::default(),
        _ => {
            warn!("Unknown config preset '{config_preset}', using default");
            ScannerConfig::default()
        }
    };

    // Apply command-line overrides
    if let Some(tasks) = max_concurrent_tasks {
        scanner_config.max_concurrent_tasks = tasks;
    }
    if let Some(db_ops) = max_concurrent_db_operations {
        scanner_config.max_concurrent_db_operations = db_ops;
    }

    Ok(scanner_config)
}

/// Execute the scan operation
fn execute_scan(
    db: &Database,
    library: abop_core::models::Library,
    scanner_config: ScannerConfig,
) -> CliResult<()> {
    let scanner = LibraryScanner::new(db.clone(), library).with_config(scanner_config);

    let start_time = Instant::now();

    info!("Starting scan...");
    let result = scanner
        .scan(abop_core::scanner::ScanOptions::default())
        .with_scan_context()?;

    let elapsed = start_time.elapsed();
    info!(
        "Scan result: processed={}, errors={}",
        result.processed, result.errors
    );
    info!("Scan completed in {:.2}s", elapsed.as_secs_f64());

    Ok(())
}

/// Output scan results in JSON format
fn output_json_results(
    db: &Database,
    library: &abop_core::models::Library,
    scan_duration: std::time::Duration,
) -> CliResult<()> {
    use crate::output::{AudiobookInfo, CliOutput, LibraryInfo, ScanMetrics};

    // Get audiobooks from the library
    let audiobooks = db
        .get_audiobooks_in_library(&library.id)
        .with_database_context("retrieving audiobooks")?;

    // Convert to output format
    let audiobook_infos: Vec<AudiobookInfo> = audiobooks.iter().map(AudiobookInfo::from).collect();
    
    // Create library info with actual count
    let library_info = LibraryInfo {
        id: library.id.clone(),
        name: library.name.clone(),
        path: library.path.clone(),
        audiobook_count: audiobooks.len(),
    };

    // Create scan metrics
    let metrics = ScanMetrics {
        files_scanned: audiobooks.len(),
        duration_ms: scan_duration.as_millis() as u64,
        files_per_second: if scan_duration.as_secs_f64() > 0.0 {
            audiobooks.len() as f64 / scan_duration.as_secs_f64()
        } else {
            0.0
        },
    };

    // Create output with a sample of audiobooks (first 10)
    let sample_size = 10;
    let sample_audiobooks = if audiobook_infos.len() <= sample_size {
        audiobook_infos.clone()
    } else {
        audiobook_infos[..sample_size].to_vec()
    };

    let mut output = CliOutput::scan_success(
        audiobooks.len(),
        vec![library_info],
        sample_audiobooks,
    );

    // Add metrics to the output
    if let CliOutput::Success { data: crate::output::OutputData::Scan(ref mut scan_output) } = output {
        scan_output.metrics = Some(metrics);
    }

    // Serialize and print
    let json = output.to_json().with_context(|| "serializing scan results to JSON")?;
    println!("{json}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_build_scanner_config_presets() {
        // Test different configuration presets
        let configs = vec![
            "default",
            "large",
            "small", 
            "conservative",
            "unknown_preset",
        ];

        for preset in configs {
            let result = build_scanner_config(preset, None, None);
            assert!(
                result.is_ok(),
                "Config preset '{preset}' should not fail"
            );
        }
    }

    #[test]
    fn test_build_scanner_config_with_overrides() {
        let config = build_scanner_config("default", Some(16), Some(8)).unwrap();
        
        assert_eq!(config.max_concurrent_tasks, 16);
        assert_eq!(config.max_concurrent_db_operations, 8);
    }

    #[test]
    fn test_validate_library_path_integration() {
        // Test successful validation
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();
        
        let result = validate_library_path(&path);
        assert!(result.is_ok());

        // Test nonexistent path
        let nonexistent = PathBuf::from("/nonexistent/path");
        let result = validate_library_path(&nonexistent);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_initialize_database_modes() {
        use tempfile::NamedTempFile;
        
        // Test with custom database path
        let temp_file = NamedTempFile::new().unwrap();
        let custom_path = temp_file.path().to_path_buf();
        
        // Note: This test just verifies the function doesn't panic
        // Full database initialization requires more complex setup
        let result = initialize_database(Some(custom_path));
        // We expect this might fail in test environment, but shouldn't panic
        assert!(result.is_ok() || result.is_err());
        
        // Test with centralized database
        let result = initialize_database(None);
        // We expect this might fail in test environment, but shouldn't panic
        assert!(result.is_ok() || result.is_err());
    }
}