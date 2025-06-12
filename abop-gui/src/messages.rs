//! Message and command definitions for the GUI application

use std::path::PathBuf;

use abop_core::models::Audiobook;
use serde::{Deserialize, Serialize};

use crate::{router::Route, state::DirectoryInfo, theme::ThemeMode};

// ================================================================================================
// MESSAGE SYSTEM
// ================================================================================================

/// Top-level application message enum
#[derive(Debug, Clone)]
pub enum Message {
    /// Navigation messages
    Navigate(Route),
    NavigateBack,

    /// Settings messages
    ShowSettings,
    CloseSettings,
    ToggleTheme,
    ToggleAutoSaveLibrary,
    ToggleScanSubdirectories,
    SetTheme(ThemeMode),

    /// Library management messages
    DirectorySelected(Option<PathBuf>),
    SelectRecentDirectory(PathBuf),
    ShowRecentDirectories,
    QuickScanDirectory(PathBuf),
    QuickScanComplete(Result<DirectoryInfo, String>),
    ScanComplete(Result<crate::library::ScanResult, String>),
    ScanProgress(f32),
    ScanProgressEnhanced(abop_core::scanner::ScanProgress),

    /// Audiobook selection messages
    SelectAudiobook(String),
    ToggleAudiobookSelection(String),
    ToggleSelectAll,
    DeselectAll,
    SortBy(String),

    /// Playback control messages
    StartPlayback,
    StopPlayback,
    PlayPause,
    Previous,
    Next,
    Stop,
    ProcessSelected,

    /// System messages
    AudioProcessingComplete(Result<String, String>),
    PlaybackStarted(Result<String, String>),
    PlaybackStopped,
    StateSaveComplete(Result<String, String>),
    StateSaveProgress(f32),
    ResetRedrawFlag,

    /// Command execution
    ExecuteCommand(Command),

    /// No operation - used for handlers that don't produce meaningful results
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    /// Library commands
    ScanLibrary {
        library_path: PathBuf,
    },
    BrowseDirectory,
    QuickScanDirectory {
        directory_path: PathBuf,
    },

    /// Audio processing commands
    ConvertToMono {
        selected_ids: Vec<String>,
        audiobooks: Vec<Audiobook>,
    },
    PlayAudio {
        selected_ids: Vec<String>,
        audiobooks: Vec<Audiobook>,
    },
    StopAudio,

    /// System commands
    SaveState,
    LoadState,
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
