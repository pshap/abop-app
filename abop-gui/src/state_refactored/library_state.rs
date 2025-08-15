//! Library management state
//!
//! This module handles all library-related state including audiobooks, directories,
//! and scanning operations.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;

use crate::utils::platform;
use abop_core::models::{AppState, Audiobook};
use abop_core::scanner::progress::ScanProgress;
use abop_core::scanner::{LibraryScanner, ScannerState};

/// Directory information with scan metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryInfo {
    /// Path to the directory
    pub path: PathBuf,
    /// When this directory was last scanned
    pub last_scan: SystemTime,
    /// Number of audiobooks found in this directory
    pub book_count: usize,
    /// How long the scan took
    pub scan_duration: Duration,
}

/// Table state for sorting and selection compatibility
#[derive(Debug, Clone)]
pub struct TableState {
    /// Column to sort by
    pub sort_column: String,
    /// Whether to sort ascending or descending
    pub sort_ascending: bool,
}

impl Default for TableState {
    fn default() -> Self {
        Self {
            sort_column: "title".to_string(),
            sort_ascending: true,
        }
    }
}

/// Library management state (audiobooks, directories, scanning)
#[derive(Clone)]
pub struct LibraryState {
    /// Path to the currently loaded audiobook library
    pub library_path: PathBuf,
    /// List of recently accessed library directories with metadata
    pub recent_directories: Vec<DirectoryInfo>,
    /// List of audiobooks currently loaded in the GUI
    pub audiobooks: Vec<Audiobook>,
    /// Selected audiobook IDs for GUI operations
    pub selected_audiobooks: HashSet<String>,
    /// State of the audiobook table (sorting, selection, etc.)
    pub table_state: TableState,

    // User preferences
    /// Whether to automatically save library state after scanning
    pub auto_save_library: bool,
    /// Whether to include subdirectories when scanning
    pub scan_subdirectories: bool,

    // Scanning state
    /// Current state of the library scanner
    scanner_state: ScannerState,
    /// Current progress information for an active scan
    pub scanner_progress: Option<ScanProgress>,
    /// Active library scanner instance if a scan is in progress
    scanner: Option<Arc<Mutex<LibraryScanner>>>,

    /// Flag to indicate library state needs UI redraw
    needs_redraw: bool,
}

impl LibraryState {
    /// Create library state from core application state
    #[must_use]
    pub fn from_core_state(core_state: &AppState) -> Self {
        let default_directory = platform::get_default_audiobook_directory();

        Self {
            library_path: core_state
                .user_preferences
                .most_recent_directory()
                .cloned()
                .unwrap_or(default_directory),
            recent_directories: core_state
                .user_preferences
                .recent_directories
                .iter()
                .map(|path| DirectoryInfo {
                    path: path.clone(),
                    last_scan: SystemTime::UNIX_EPOCH,
                    book_count: 0,
                    scan_duration: Duration::from_secs(0),
                })
                .collect(),
            audiobooks: core_state.app_data.audiobooks.clone(),
            selected_audiobooks: HashSet::new(),
            table_state: TableState::default(),
            auto_save_library: true,
            scan_subdirectories: true,
            scanner_state: ScannerState::Idle,
            scanner_progress: None,
            scanner: None,
            needs_redraw: false,
        }
    }

    /// Synchronize directory information with actual scan data
    /// This updates directory metadata based on currently loaded audiobooks
    pub fn sync_directory_metadata(&mut self) {
        for dir_info in &mut self.recent_directories {
            // Count audiobooks for this directory
            let book_count = self
                .audiobooks
                .iter()
                .filter(|audiobook| {
                    audiobook
                        .path
                        .parent()
                        .is_some_and(|parent| parent == dir_info.path)
                })
                .count();

            // Update the book count if we have audiobooks for this directory
            if book_count > 0 {
                dir_info.book_count = book_count;
                // Set a reasonable last_scan time if it's still at UNIX_EPOCH
                if dir_info.last_scan == SystemTime::UNIX_EPOCH {
                    dir_info.last_scan = SystemTime::now();
                }
            }
        }
        self.mark_for_redraw();
    }

    /// Set the current library path
    pub fn set_library_path(&mut self, path: PathBuf) {
        if self.library_path != path {
            self.library_path = path;
            self.mark_for_redraw();
        }
    }

    /// Add or update a directory in recent directories
    pub fn add_recent_directory(&mut self, path: PathBuf, scan_duration: Duration) {
        let now = SystemTime::now();

        // Check if directory already exists and update it
        if let Some(existing) = self.recent_directories.iter_mut().find(|d| d.path == path) {
            existing.last_scan = now;
            existing.scan_duration = scan_duration;
        } else {
            // Add new directory
            let dir_info = DirectoryInfo {
                path,
                last_scan: now,
                book_count: 0,
                scan_duration,
            };
            self.recent_directories.push(dir_info);
        }
        self.mark_for_redraw();
    }

    /// Update audiobooks list
    pub fn set_audiobooks(&mut self, audiobooks: Vec<Audiobook>) {
        self.audiobooks = audiobooks;
        self.sync_directory_metadata();
        self.mark_for_redraw();
    }

    /// Add an audiobook to the selection
    pub fn select_audiobook(&mut self, id: String) {
        if self.selected_audiobooks.insert(id) {
            self.mark_for_redraw();
        }
    }

    /// Remove an audiobook from the selection
    pub fn deselect_audiobook(&mut self, id: &str) {
        if self.selected_audiobooks.remove(id) {
            self.mark_for_redraw();
        }
    }

    /// Clear all selected audiobooks
    pub fn clear_selection(&mut self) {
        if !self.selected_audiobooks.is_empty() {
            self.selected_audiobooks.clear();
            self.mark_for_redraw();
        }
    }

    /// Toggle audiobook selection
    pub fn toggle_audiobook_selection(&mut self, id: String) {
        if self.selected_audiobooks.contains(&id) {
            self.deselect_audiobook(&id);
        } else {
            self.select_audiobook(id);
        }
    }

    /// Update table sorting
    pub fn set_sort_column(&mut self, column: String, ascending: bool) {
        if self.table_state.sort_column != column || self.table_state.sort_ascending != ascending {
            self.table_state.sort_column = column;
            self.table_state.sort_ascending = ascending;
            self.mark_for_redraw();
        }
    }

    /// Start a new scanning operation
    pub fn start_scanning(&mut self) {
        self.scanner_state = ScannerState::Scanning;
        self.scanner_progress = None;
        self.mark_for_redraw();
    }

    // Modern API methods - prefer these over legacy fields

    /// Get current scanner state (read-only access)
    #[must_use]
    pub fn scanner_state(&self) -> ScannerState {
        self.scanner_state.clone()
    }

    /// Check if a scan is currently in progress (modern API)
    #[must_use]
    pub fn is_scanning(&self) -> bool {
        matches!(self.scanner_state, ScannerState::Scanning)
    }

    /// Get current scan progress as legacy f32 (modern API with legacy compatibility)
    #[must_use]
    pub fn get_scan_progress_legacy(&self) -> Option<f32> {
        self.scanner_progress.as_ref().and_then(|progress| {
            match progress {
                abop_core::scanner::ScanProgress::FileProcessed {
                    progress_percentage,
                    ..
                } => Some(*progress_percentage),
                abop_core::scanner::ScanProgress::BatchCommitted { .. } => {
                    // BatchCommitted doesn't have total file count, so we can't accurately
                    // calculate percentage. Return None to indicate progress is indeterminate.
                    None
                }
                abop_core::scanner::ScanProgress::Complete { .. } => {
                    Some(1.0) // 100% complete
                }
                abop_core::scanner::ScanProgress::Started { .. } => {
                    Some(0.0) // Just started
                }
                _ => None, // Other progress states don't map to simple percentages
            }
        })
    }

    /// Update scanning progress
    pub fn update_scan_progress(&mut self, progress: ScanProgress) {
        self.scanner_progress = Some(progress);
        self.mark_for_redraw();
    }

    /// Complete scanning operation
    pub fn complete_scanning(&mut self) {
        self.scanner_state = ScannerState::Complete;
        self.mark_for_redraw();
    }

    /// Handle scanning error
    pub fn error_scanning(&mut self) {
        self.scanner_state = ScannerState::Error;
        self.mark_for_redraw();
    }

    /// Cancel scanning operation
    pub fn cancel_scanning(&mut self) {
        self.scanner_state = ScannerState::Cancelled;
        self.mark_for_redraw();
    }

    /// Get scanner instance (read-only access)
    #[must_use]
    pub fn scanner(&self) -> Option<&Arc<Mutex<LibraryScanner>>> {
        self.scanner.as_ref()
    }

    /// Set scanner instance
    pub fn set_scanner(&mut self, scanner: Option<Arc<Mutex<LibraryScanner>>>) {
        self.scanner = scanner;
    }

    /// Check if the library state needs a UI redraw
    #[must_use]
    pub const fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    /// Mark that the library state needs a UI redraw
    pub fn mark_for_redraw(&mut self) {
        self.needs_redraw = true;
    }

    /// Clear the redraw flag (typically called after redraw is complete)
    pub fn clear_redraw_flag(&mut self) {
        self.needs_redraw = false;
    }

    /// Toggle auto-save library preference
    pub fn toggle_auto_save_library(&mut self) {
        self.auto_save_library = !self.auto_save_library;
        self.mark_for_redraw();
    }

    /// Toggle scan subdirectories preference
    pub fn toggle_scan_subdirectories(&mut self) {
        self.scan_subdirectories = !self.scan_subdirectories;
        self.mark_for_redraw();
    }
}

impl std::fmt::Debug for LibraryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LibraryState")
            .field("library_path", &self.library_path)
            .field("recent_directories", &self.recent_directories)
            .field("audiobooks_count", &self.audiobooks.len())
            .field("selected_count", &self.selected_audiobooks.len())
            .field("table_state", &self.table_state)
            .field("auto_save_library", &self.auto_save_library)
            .field("scan_subdirectories", &self.scan_subdirectories)
            .field("scanner_state", &self.scanner_state())
            .field("scanner_progress", &self.scanner_progress)
            .field(
                "scanner",
                &if self.scanner().is_some() {
                    "Some(<LibraryScanner>)"
                } else {
                    "None"
                },
            )
            .field("needs_redraw", &self.needs_redraw())
            .finish()
    }
}

impl Default for LibraryState {
    fn default() -> Self {
        Self::from_core_state(&AppState::default())
    }
}
