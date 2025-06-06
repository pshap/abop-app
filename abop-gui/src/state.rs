//! Application state management for the GUI

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::styling::material::MaterialTokens;
use abop_core::scanner::progress::ScanProgress;
use crate::theme::ThemeMode;
use abop_core::ViewType;
use abop_core::scanner::{LibraryScanner, ScannerState};

use abop_core::models::Audiobook;
use abop_core::audio::player::PlayerState;
use abop_core::models::AppState;

// ================================================================================================
// HELPER FUNCTIONS
// ================================================================================================

/// Determines a sensible default directory for audiobooks when no recent directories exist
fn get_default_audiobook_directory() -> PathBuf {
    // Only use as fallback when no previous directories exist
    // Try to find a good default directory in order of preference:
    // 1. User's Documents/Audiobooks folder
    // 2. User's home directory
    // 3. Current directory as ultimate fallback

    if let Some(docs_dir) = dirs::document_dir() {
        return docs_dir.join("Audiobooks");
    }

    if let Some(home_dir) = dirs::home_dir() {
        return home_dir;
    }

    // Ultimate fallback
    PathBuf::from(".")
}

// ================================================================================================
// DIRECTORY METADATA
// ================================================================================================

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

// ================================================================================================
// GUI-SPECIFIC STATE WRAPPER
// ================================================================================================

/// GUI-specific wrapper around the centralized `AppState`
#[derive(Debug, Clone)]
pub struct UiState {
    /// Core application state from abop-core
    pub core_state: AppState,
    /// Current theme mode for the GUI (light or dark)
    pub theme_mode: ThemeMode,
    /// Whether the settings dialog is currently open
    pub settings_open: bool,
    /// Whether the recent directories dropdown is currently open
    pub recent_directories_open: bool,
    /// State of the audiobook table (sorting, selection, etc.)
    pub table_state: TableState,
    /// Path to the currently loaded audiobook library
    pub library_path: PathBuf,
    /// List of recently accessed library directories with metadata
    pub recent_directories: Vec<DirectoryInfo>,
    /// List of audiobooks currently loaded in the GUI
    pub audiobooks: Vec<Audiobook>,
    /// Selected audiobook IDs for GUI operations
    pub selected_audiobooks: HashSet<String>,
    /// Whether a library scan is in progress
    pub scanning: bool,
    /// Progress of the current scan (0.0 to 1.0)
    pub scan_progress: Option<f32>,
    /// Enhanced scan progress with detailed information
    pub enhanced_scan_progress: Option<ScanProgress>,
    /// Whether state saving is in progress
    pub saving: bool,
    /// Progress of the current state save (0.0 to 1.0)
    pub save_progress: Option<f32>,
    /// Whether audio processing is in progress
    pub processing_audio: bool,
    /// Progress of the current audio processing (0.0 to 1.0)
    pub processing_progress: Option<f32>,
    /// Current audio processing status message
    pub processing_status: Option<String>,
    /// Current audio player state for UI updates
    pub player_state: PlayerState,
    /// Currently playing file path
    pub current_playing_file: Option<PathBuf>,
    /// Material Design tokens for UI styling - contains all design tokens
    pub material_tokens: MaterialTokens,
    /// Flag to force a UI redraw when state changes
    pub needs_redraw: bool,
    /// Current active task if any
    pub active_task: Option<TaskInfo>,
    /// List of recent tasks
    pub recent_tasks: Vec<TaskInfo>,
    /// Whether to show task history
    pub show_task_history: bool,
    pub scanner_state: ScannerState,
    pub scanner_progress: Option<ScanProgress>,
    pub scanner: Option<Arc<Mutex<LibraryScanner>>>,
}

impl UiState {    /// Create a new GUI state from a core `AppState`
    #[must_use]
    pub fn from_core_state(core_state: AppState) -> Self {
        let theme_mode = ThemeMode::Dark;
        let material_tokens = MaterialTokens::new();

        Self {
            core_state: core_state.clone(), // Clone core_state first
            theme_mode,
            material_tokens,
            settings_open: false,
            recent_directories_open: false,
            table_state: TableState::default(),
            selected_audiobooks: HashSet::new(),
            scanning: false,
            scan_progress: None,
            enhanced_scan_progress: None,
            saving: false,
            save_progress: None,
            processing_audio: false,
            processing_progress: None,
            processing_status: None,
            player_state: PlayerState::Stopped,
            current_playing_file: None,
            library_path: core_state
                .user_preferences
                .most_recent_directory()
                .cloned()
                .unwrap_or_else(get_default_audiobook_directory), // Use most recent or default
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
            audiobooks: core_state.data.audiobooks,
            needs_redraw: false,
            active_task: None,
            recent_tasks: Vec::new(),
            show_task_history: false,
            scanner_state: ScannerState::Idle,
            scanner_progress: None,
            scanner: None,
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
    }    /// Create UI state from core state and ensure metadata is synchronized
    #[must_use]
    pub fn from_core_state_synced(core_state: AppState) -> Self {
        let mut ui_state = Self::from_core_state(core_state);
        ui_state.sync_directory_metadata();
        ui_state
    }

    /// Update the theme mode and regenerate material tokens
    pub fn set_theme_mode(&mut self, theme_mode: ThemeMode) {
        self.theme_mode = theme_mode;
        self.material_tokens = match theme_mode {
            ThemeMode::Dark | ThemeMode::System | ThemeMode::MaterialDark => MaterialTokens::dark(),
            ThemeMode::Light | ThemeMode::MaterialLight => MaterialTokens::light(),
            ThemeMode::MaterialDynamic => {
                // Use purple seed color for dynamic Material Design theme
                let seed_color = iced::Color::from_rgb(0.4, 0.2, 0.8);
                MaterialTokens::from_seed_color(seed_color, true)
            }
        };
    }

    /// Update theme with a seed color for dynamic Material Design themes
    pub fn set_seed_color(&mut self, seed: iced::Color, is_dark: bool) {
        self.theme_mode = ThemeMode::MaterialDynamic;
        self.material_tokens = MaterialTokens::from_seed_color(seed, is_dark);
    }    pub async fn start_scan(&mut self, path: PathBuf) {
        if let Some(scanner) = &self.scanner {
            let mut scanner = scanner.lock().await;
            if let Err(_e) = scanner.scan_directory(path).await {
                self.scanner_state = ScannerState::Error;
            }
        }
    }

    pub async fn update_scan_progress(&mut self, progress: ScanProgress) {
        self.scanner_progress = Some(progress);
    }

    pub async fn update_scan_state(&mut self, state: ScannerState) {
        self.scanner_state = state;
    }

    pub async fn pause_scan(&mut self) {
        if let Some(scanner) = &self.scanner {
            let mut scanner = scanner.lock().await;
            scanner.pause().await;
        }
    }

    pub async fn resume_scan(&mut self) {
        if let Some(scanner) = &self.scanner {
            let mut scanner = scanner.lock().await;
            scanner.resume().await;
        }
    }

    pub async fn cancel_scan(&mut self) {
        if let Some(scanner) = &self.scanner {
            let mut scanner = scanner.lock().await;
            scanner.cancel().await;
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        let mut state = Self::from_core_state_synced(CoreAppState::default());
        state.scanner_state = ScannerState::Idle;
        state.scanner_progress = None;
        state.scanner = None;
        state
    }
}

/// Table state for sorting and selection compatibility with existing table component
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

/// Information about a task
#[derive(Debug, Clone)]
pub struct TaskInfo {
    /// Unique identifier for the task
    pub id: String,
    /// Task type
    pub task_type: TaskType,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Task status message
    pub status: String,
    /// Whether the task is currently running
    pub is_running: bool,
    /// Whether the task has completed
    pub is_completed: bool,
    /// Error message if task failed
    pub error: Option<String>,
    /// Start time of the task
    pub start_time: chrono::DateTime<chrono::Local>,
    /// End time of the task if completed
    pub end_time: Option<chrono::DateTime<chrono::Local>>,
}

/// Types of tasks that can be performed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskType {
    Scan,
    Process,
    Import,
    Export,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Scan => write!(f, "Scanning"),
            TaskType::Process => write!(f, "Processing"),
            TaskType::Import => write!(f, "Importing"),
            TaskType::Export => write!(f, "Exporting"),
        }
    }
}

impl Default for TaskInfo {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            task_type: TaskType::Scan,
            progress: 0.0,
            status: "Ready".to_string(),
            is_running: false,
            is_completed: false,
            error: None,
            start_time: chrono::Local::now(),
            end_time: None,
        }    }
}
