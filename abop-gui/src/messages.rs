//! Message and command definitions for the GUI application

use std::path::PathBuf;

use abop_core::models::Audiobook;
use serde::{Deserialize, Serialize};

use crate::{router::Route, state::DirectoryInfo, theme::ThemeMode};

// ================================================================================================
// MESSAGE SYSTEM
// ================================================================================================

/// Top-level application message enum
///
/// This enum represents all possible messages that can be processed by the application's
/// update loop. Each variant corresponds to a specific user action or system event.
#[derive(Debug, Clone)]
pub enum Message {
    // ===== Navigation =====
    /// Navigate to a specific route in the application
    Navigate(Route),
    /// Navigate back to the previous route
    NavigateBack,

    // ===== Settings =====
    /// Show the settings dialog
    ShowSettings,
    /// Close the settings dialog
    CloseSettings,
    /// Toggle between light and dark theme
    ToggleTheme,
    /// Toggle automatic saving of the library state
    ToggleAutoSaveLibrary,
    /// Toggle whether to scan subdirectories when importing
    ToggleScanSubdirectories,
    /// Set the application theme to a specific mode
    SetTheme(ThemeMode),

    // ===== Library Management =====
    /// A directory was selected for scanning/import
    DirectorySelected(Option<PathBuf>),
    /// Select a recently used directory
    SelectRecentDirectory(PathBuf),
    /// Show the recent directories dialog
    ShowRecentDirectories,
    /// Perform a quick scan of a directory
    QuickScanDirectory(PathBuf),
    /// Result of a quick directory scan
    QuickScanComplete(Result<DirectoryInfo, String>),
    /// Result of a full library scan
    ScanComplete(Result<crate::library::ScanResult, String>),
    /// Progress update for a scan operation (0.0 to 1.0)
    ScanProgress(f32),
    /// Enhanced progress information for a scan operation
    ScanProgressEnhanced(abop_core::scanner::ScanProgress),

    // ===== Audiobook Selection =====
    /// Select a single audiobook by ID
    SelectAudiobook(String),
    /// Toggle selection state of an audiobook by ID
    ToggleAudiobookSelection(String),
    /// Toggle selection state of all audiobooks
    ToggleSelectAll,
    /// Deselect all audiobooks
    DeselectAll,
    /// Sort audiobooks by the specified column
    SortBy(String),

    // ===== Playback Control =====
    /// Start playback of selected audiobooks
    StartPlayback,
    /// Stop the current playback
    StopPlayback,
    /// Toggle between play and pause
    PlayPause,
    /// Play the previous track
    Previous,
    /// Play the next track
    Next,
    /// Stop all playback
    Stop,
    /// Process the selected audiobooks
    ProcessSelected,

    // ===== System Messages =====
    /// Result of an audio processing operation
    AudioProcessingComplete(Result<String, String>),
    /// Result of starting playback
    PlaybackStarted(Result<String, String>),
    /// Notification that playback has stopped
    PlaybackStopped,
    /// Result of saving application state
    StateSaveComplete(Result<String, String>),
    /// Progress update for state saving (0.0 to 1.0)
    StateSaveProgress(f32),
    /// Reset the redraw flag after rendering
    ResetRedrawFlag,

    // ===== Command Execution =====
    /// Execute a command asynchronously
    ExecuteCommand(Command),

    // ===== Utility =====
    /// No operation - used as a placeholder when no action is needed
    NoOp,
}

impl Message {
    /// Creates a navigation message to the specified route
    pub fn navigate(route: Route) -> Self {
        Self::Navigate(route)
    }

    /// Creates a command execution message
    pub fn command(command: Command) -> Self {
        Self::ExecuteCommand(command)
    }
}

// ================================================================================================
// COMMAND SYSTEM
// ================================================================================================

/// Represents high-level asynchronous operations the GUI can trigger.
///
/// These commands are typically executed in a background thread and can perform
/// potentially long-running operations without blocking the UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    // ===== Library Commands =====
    /// Scan a library directory for audiobooks
    ScanLibrary {
        /// Path to the library directory to scan
        library_path: PathBuf,
    },
    
    /// Open a directory browser dialog
    BrowseDirectory,
    
    /// Quickly scan a single directory
    QuickScanDirectory {
        /// Path to the directory to scan
        directory_path: PathBuf,
    },

    // ===== Audio Processing =====
    /// Convert selected audiobooks to mono
    ConvertToMono {
        /// IDs of the selected audiobooks
        selected_ids: Vec<String>,
        /// Full list of audiobooks for reference
        audiobooks: Vec<Audiobook>,
    },
    
    /// Start playing the selected audiobooks
    PlayAudio {
        /// IDs of the selected audiobooks
        selected_ids: Vec<String>,
        /// Full list of audiobooks for reference
        audiobooks: Vec<Audiobook>,
    },
    
    /// Stop any currently playing audio
    StopAudio,

    // ===== System Commands =====
    /// Save the current application state
    SaveState,
    
    /// Load the saved application state
    LoadState,
    
    /// Quit the application
    Quit,
}

impl Command {
    /// Creates a new scan library command
    pub fn scan_library(library_path: PathBuf) -> Self {
        Self::ScanLibrary { library_path }
    }

    /// Creates a new play audio command
    pub fn play_audio(selected_ids: Vec<String>, audiobooks: Vec<Audiobook>) -> Self {
        Self::PlayAudio {
            selected_ids,
            audiobooks,
        }
    }
}
