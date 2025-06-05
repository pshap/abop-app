//! Library scanning functionality

use abop_core::{
    db::Database, 
    models::Audiobook, 
    scanner::{LibraryScanner, LibraryScanResult, performance::PerformanceMonitor}
};
use iced::Task;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::Arc;

/// Progress callback type for scanning operations
pub type ProgressCallback = Box<dyn Fn(f32) + Send + Sync>;

/// Result of a library scan operation including timing information
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// List of successfully scanned audiobooks
    pub audiobooks: Vec<Audiobook>,
    /// Total time taken for the scan
    pub scan_duration: Duration,
    /// Number of files processed
    pub processed_count: usize,
    /// Number of files that had errors
    pub error_count: usize,
    /// Performance metrics if monitoring was enabled
    pub performance_monitor: Option<Arc<PerformanceMonitor>>,
}

/// Enhanced scanning result with detailed metrics
#[derive(Debug, Clone)]
pub struct EnhancedScanResult {
    /// Base scan result
    pub scan_result: LibraryScanResult,
    /// Performance metrics
    pub performance_metrics: Option<abop_core::scanner::performance::PerformanceMetrics>,
    /// ETA information during scan
    pub eta_history: Vec<Duration>,
    /// Throughput history (files per second)
    pub throughput_history: Vec<f64>,
}

/// Progress information for ongoing scans
#[derive(Debug, Clone)]
pub struct ScanProgress {
    /// Current file being processed
    pub current_file: Option<String>,
    /// Number of files processed so far
    pub processed: usize,
    /// Total number of files to process
    pub total: usize,
    /// Current throughput (files per second)
    pub throughput: f64,
    /// Estimated time to completion
    pub eta: Option<Duration>,
    /// Current progress percentage (0.0 to 1.0)
    pub progress_percentage: f32,
}

impl ScanProgress {
    /// Create new scan progress
    pub fn new(total: usize) -> Self {
        Self {
            current_file: None,
            processed: 0,
            total,
            throughput: 0.0,
            eta: None,
            progress_percentage: 0.0,
        }
    }

    /// Update progress with new file
    pub fn update(&mut self, processed: usize, current_file: Option<String>, throughput: f64) {
        self.processed = processed;
        self.current_file = current_file;
        self.throughput = throughput;
        
        if self.total > 0 {
            self.progress_percentage = processed as f32 / self.total as f32;
            
            // Calculate ETA
            if throughput > 0.0 {
                let remaining = self.total.saturating_sub(processed) as f64;
                let eta_seconds = remaining / throughput;
                self.eta = Some(Duration::from_secs_f64(eta_seconds));
            }
        }
    }

    /// Check if scan is complete
    pub fn is_complete(&self) -> bool {
        self.processed >= self.total
    }
}

/// Open a directory dialog and return the selected path
pub async fn open_directory_dialog() -> Option<PathBuf> {
    use rfd::AsyncFileDialog;

    AsyncFileDialog::new()
        .set_title("Select Audiobook Library Directory")
        .pick_folder()
        .await
        .map(|handle| handle.path().to_path_buf())
}

/// Scan the library directory for audiobooks with performance monitoring
///
/// # Errors
///
/// Returns an error if:
/// - Library scanning fails
/// - Database operations fail
/// - File system access is denied
pub fn scan_library_async(_library_path: PathBuf, db: abop_core::db::Database, library: abop_core::models::Library) -> Task<LibraryScanResult> {
    let scanner = LibraryScanner::new(db, library)
        .with_performance_monitoring(); // Enable performance monitoring for GUI
    let audio_files = scanner.find_audio_files();
    scanner.scan_with_tasks(audio_files)
}

/// Scan the library directory for audiobooks with optional progress reporting
///
/// # Errors
///
/// Returns an error if:
/// - Library scanning fails
/// - Database operations fail
/// - File system access is denied
/// - Data directory creation fails
///
/// # Panics
///
/// This function contains an `unwrap()` call on the progress callback, but it is logically safe
/// because the function returns early if the callback is None.
pub async fn scan_library_with_progress(
    library_path: PathBuf,
    progress_callback: Option<ProgressCallback>,
) -> Result<Vec<Audiobook>, String> {
    // Use a persistent database file in the user's data directory
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("abop");
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let db_path = data_dir.join("library.db");

    let db = Database::open(&db_path).map_err(|e| e.to_string())?;

    // Check if library already exists, if not create it
    let library = match db
        .libraries()
        .find_by_name("Default Library")
        .map_err(|e| e.to_string())?
    {
        Some(existing_library) => existing_library,
        None => db
            .add_library("Default Library", &library_path)
            .map_err(|e| e.to_string())?,
    };

    // If no progress callback, use the simple scanner
    if progress_callback.is_none() {
        let scanner = LibraryScanner::new(db, library);
        let scan_result = scanner.scan().map_err(|e| e.to_string())?;
        return Ok(scan_result.audiobooks);
    }

    // Custom scanning with progress reporting
    scan_with_progress_reporting(db, library, progress_callback.unwrap()).await
}

/// Scan library with enhanced progress tracking and ETA calculation
pub fn scan_library_with_enhanced_progress(
    _library_path: PathBuf, 
    db: Database, 
    library: abop_core::models::Library,
    progress_callback: impl Fn(ScanProgress) + Send + Sync + 'static
) -> Task<EnhancedScanResult> {
    Task::perform(
        async move {
            let scanner = LibraryScanner::new(db, library)
                .with_performance_monitoring();
            
            let audio_files = scanner.find_audio_files();
            let total_files = audio_files.len();
            let mut progress = ScanProgress::new(total_files);
            
            // Initial progress callback
            progress_callback(progress.clone());
            
            let throughput_history = Vec::new();
            let eta_history = Vec::new();
            
            // Use the scanner's manual async scan instead of tasks
            let scan_result = match scanner.scan() {
                Ok(result) => result,
                Err(_e) => {
                    // Return an empty result on error
                    LibraryScanResult::new()
                }
            };
            
            // Get performance metrics if available
            let performance_metrics = scanner.get_performance_monitor()
                .map(|monitor| monitor.get_metrics());
            
            // Final progress update
            progress.update(scan_result.processed_count, None, 0.0);
            progress_callback(progress);
            
            EnhancedScanResult {
                scan_result,
                performance_metrics,
                eta_history,
                throughput_history,
            }
        },
        |result| result
    )
}

/// Enhanced scanning with progress reporting
async fn scan_with_progress_reporting(
    db: Database,
    library: abop_core::models::Library,
    progress_callback: ProgressCallback,
) -> Result<Vec<Audiobook>, String> {
    use abop_core::audio::AudioMetadata;
    use abop_core::scanner::SUPPORTED_AUDIO_EXTENSIONS;
    use walkdir::WalkDir;

    // Report initial progress
    progress_callback(0.0);

    // Step 1: Find all audio files (30% of progress)
    let mut audio_files = Vec::new();
    for entry in WalkDir::new(&library.path)
        .follow_links(true)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if SUPPORTED_AUDIO_EXTENSIONS.iter().any(|e| e == &ext_str) {
                audio_files.push(path.to_path_buf());
            }
        }
    }

    progress_callback(0.3);
    log::info!("Found {} audio files", audio_files.len());

    if audio_files.is_empty() {
        progress_callback(1.0);
        return Ok(Vec::new());
    }

    // Step 2: Process files and save to database (70% of progress)
    let mut audiobooks = Vec::new();
    let total_files = audio_files.len();

    for (index, audio_file) in audio_files.iter().enumerate() {
        // Extract metadata
        let mut audiobook = abop_core::models::Audiobook::new(&library.id, audio_file);

        if let Ok(metadata) = std::fs::metadata(audio_file) {
            audiobook.size_bytes = Some(metadata.len());
        }

        if let Ok(metadata) = AudioMetadata::from_file(audio_file) {
            audiobook.title = metadata.title;
            audiobook.author = metadata.artist;
            audiobook.narrator = metadata.narrator;
            audiobook.duration_seconds = metadata.duration_seconds.map(|d| d.round() as u64);
            if let Some(cover_art) = metadata.cover_art {
                audiobook.cover_art = Some(cover_art);
            }
        }

        // Save to database
        if let Err(e) = db.add_audiobook(&audiobook) {
            log::error!("Error saving {}: {}", audiobook.path.display(), e);
        } else {
            audiobooks.push(audiobook);
        }

        // Report progress (0.3 to 1.0)
        let file_progress = (index + 1) as f32 / total_files as f32;
        let total_progress = file_progress.mul_add(0.7, 0.3);
        progress_callback(total_progress);
    }

    log::info!("Scan completed. Processed: {} audiobooks", audiobooks.len());
    Ok(audiobooks)
}
