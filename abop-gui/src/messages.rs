//! Message and command definitions for the GUI application

// use std::collections::HashSet; // Commented out, unused import
use std::path::PathBuf;
use iced::widget::button;
use abop_core::{
    scanner::{ScannerState, ScanProgress},
    models::Audiobook,
    error::AppError,
};

use crate::{
    state::DirectoryInfo,
    theme::ThemeMode,
};

// ================================================================================================
// GUI MESSAGE SYSTEM
// ================================================================================================

/// GUI-specific message enum that wraps core `AppMessage` and adds GUI-specific messages
#[derive(Debug, Clone)]
pub enum Message {
    /// Opens the settings dialog.
    ShowSettings,
    /// Closes the settings dialog.
    CloseSettings,

    /// Indicates a directory was selected (or not) by the user.
    DirectorySelected(Option<PathBuf>),
    /// Selects a recently used directory.
    SelectRecentDirectory(PathBuf),
    /// Shows the recent directories dropdown.
    ShowRecentDirectories,
    /// Quick scan a directory for metadata without full library processing.
    QuickScanDirectory(PathBuf),
    /// Quick scan completed with directory metadata.
    QuickScanComplete(Result<DirectoryInfo, String>),

    /// Start scanning a library
    StartScan(PathBuf),

    /// Update scan progress
    ScanProgress(ScanProgress),

    /// Scan completed with result
    ScanComplete(Result<Vec<Audiobook>, String>),

    /// Scan was cancelled
    ScanCancelled,

    /// Error occurred
    Error(String),

    /// Close the application
    Close,

    /// Selects an audiobook by its ID.
    SelectAudiobook(String),
    /// Deselects all selected audiobooks.
    DeselectAll,
    /// Sorts the audiobook table by the given column.
    SortBy(String),

    /// Processes the currently selected audiobooks.
    ProcessSelected,
    /// Starts audio playback.
    StartPlayback,
    /// Stops audio playback.
    StopPlayback,
    /// Toggles between play and pause.
    PlayPause,
    /// Plays the previous track.
    Previous,
    /// Plays the next track.
    Next,
    /// Stops audio playback (same as `StopPlayback` but for consistency).
    Stop,

    /// Indicates audio processing has completed.
    AudioProcessingComplete(Result<String, String>),
    /// Indicates playback has started.
    PlaybackStarted(Result<String, String>),
    /// Indicates playback has stopped.
    PlaybackStopped,

    /// Sets the application theme.
    SetTheme(ThemeMode),

    /// Internal message to reset the `needs_redraw` flag after rendering
    ResetRedrawFlag,

    /// Indicates async state save has completed
    StateSaveComplete(Result<String, String>),

    /// Reports progress during state saving (0.0 to 1.0).
    StateSaveProgress(f32),

    /// Executes a high-level command.
    ExecuteCommand(Command),

    /// Task management messages
    /// Starts a new task
    StartTask(TaskType),
    /// Updates task progress
    TaskProgress {
        /// Task ID
        task_id: String,
        /// Progress value (0.0 to 1.0)
        progress: f32,
        /// Status message
        status: String,
    },
    /// Marks a task as completed
    TaskComplete {
        /// Task ID
        task_id: String,
        /// Final status message
        status: String,
    },
    /// Marks a task as failed
    TaskFailed {
        /// Task ID
        task_id: String,
        /// Error message
        error: String,
    },
    /// Cancels the current task
    CancelTask,
    /// Toggles task history visibility
    ToggleTaskHistory,
    /// Clears task history
    ClearTaskHistory,

    // Scanner Messages
    ScanStateChanged(ScannerState),
    PauseScan,
    ResumeScan,
    CancelScan,
    ScanCompleted,
    ScanError(String),
}

impl From<AppError> for Message {
    fn from(error: AppError) -> Self {
        Message::Error(error.to_string())
    }
}

// ================================================================================================
// COMMAND DEFINITIONS
// ================================================================================================

/// Represents high-level asynchronous operations the GUI can trigger.
#[derive(Debug, Clone)]
pub enum Command {
    /// Command to scan the library at the given path.
    ScanLibrary {
        /// Path to the library directory to scan.
        library_path: PathBuf,
    },
    /// Command to open a directory browser dialog.
    BrowseDirectory,
    /// Command to quickly scan a directory for metadata.
    QuickScanDirectory {
        /// Path to the directory to scan.
        directory_path: PathBuf,
    },
    /// Command to convert selected audiobooks to mono.
    ConvertToMono {
        /// IDs of selected audiobooks to convert.
        selected_ids: Vec<String>,
        /// Audiobook data for conversion.
        audiobooks: Vec<Audiobook>,
    },
    /// Command to play selected audiobooks.
    PlayAudio {
        /// IDs of selected audiobooks to play.
        selected_ids: Vec<String>,
        /// Audiobook data for playback.
        audiobooks: Vec<Audiobook>,
    },
    /// Command to stop audio playback.
    StopAudio,
}

/// Represents different types of tasks that can be performed
#[derive(Debug, Clone)]
pub enum TaskType {
    /// Scanning a library for audiobooks
    ScanLibrary(PathBuf),
    /// Converting audiobooks to mono
    ConvertToMono(Vec<String>),
    /// Playing audio files
    PlayAudio(Vec<String>),
}
