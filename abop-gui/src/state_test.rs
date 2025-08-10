//! Tests for the application state management

use super::*;
use abop_core::{
    models::{AppState, Audiobook, AudiobookId},
    scanner::{progress::ScanProgress, ScannerState},
};
use iced::Color;
use std::path::Path;
use std::time::{Duration, SystemTime};

/// Test creating a new UiState with default values
#[test]
fn test_ui_state_default() {
    let state = AppState::default();
    
    assert_eq!(state.ui.theme_mode, ThemeMode::Dark); // Updated to match new default
    // Material tokens are now always initialized in ui domain
    assert!(state.library.scan_progress.is_none());
    assert!(!state.library.scanning); // scanning replaces scan_state
    assert!(state.library.recent_directories.is_empty());
    // directories field no longer exists in new architecture
}

/// Test creating UiState from core AppState
#[test]
fn test_from_core_state() {
    let mut core_state = AppState::default();
    core_state.audiobooks.insert(
        AudiobookId::new(),
        Audiobook {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            ..Default::default()
        },
    );

    let ui_state = AppState::from_core_state(core_state);
    
    assert_eq!(ui_state.library.audiobooks.len(), 1);
    assert_eq!(ui_state.ui.theme_mode, ThemeMode::Dark);
}

/// Test theme mode changes
#[test]
fn test_set_theme_mode() {
    let mut state = AppState::default();
    
    // Test changing to dark mode
    state.ui.set_theme_mode(ThemeMode::Dark);
    assert_eq!(state.ui.theme_mode, ThemeMode::Dark);
    
    // Test changing to light mode
    state.ui.set_theme_mode(ThemeMode::Light);
    assert_eq!(state.ui.theme_mode, ThemeMode::Light);
    
    // Test changing to system mode
    state.set_theme_mode(ThemeMode::System);
    assert_eq!(state.theme_mode, ThemeMode::System);
}

/// Test seed color updates
#[test]
fn test_set_seed_color() {
    let mut state = AppState::default();
    let test_color = Color::from_rgb(0.5, 0.2, 0.8);
    
    state.set_seed_color(test_color, false);
    
    // Verify the material tokens were regenerated
    assert!(state.material_tokens.is_some());
}

/// Test scan operations
#[test]
fn test_scan_operations() {
    let mut state = AppState::default();
    
    // Test starting a scan
    state.start_scan("/test/path".into());
    assert_eq!(state.scan_state, ScannerState::Scanning);
    
    // Test updating scan progress
    let progress = ScanProgress {
        current: 5,
        total: 10,
        current_path: Some("/test/path".into()),
    };
    state.update_scan_progress(progress.clone());
    assert_eq!(state.scan_progress, Some(progress));
    
    // Test cancelling a scan
    state.cancel_scan();
    assert_eq!(state.scan_state, ScannerState::Cancelled);
}

/// Test progress text caching
#[test]
fn test_progress_text_caching() {
    let mut state = AppState::default();
    
    // First call should update the cache
    let text1 = state.get_scan_progress_text(0.5);
    assert!(!text1.is_empty());
    
    // Second call with similar value should use cache
    let text2 = state.get_scan_progress_text(0.5001);
    assert_eq!(text1, text2);
    
    // Call with different value should update cache
    let text3 = state.get_scan_progress_text(0.6);
    assert_ne!(text1, text3);
}

/// Test directory metadata synchronization
#[test]
fn test_sync_directory_metadata() {
    let mut state = AppState::default();
    
    // Add an audiobook to the state
    let mut audiobook = Audiobook::default();
    audiobook.path = "/test/path/to/book".into();
    state.audiobooks.insert(AudiobookId::new(), audiobook);
    
    // Synchronize directory metadata
    state.sync_directory_metadata();
    
    // Verify the directory was added
    assert!(!state.directories.is_empty());
    assert_eq!(state.directories[0].path, Path::new("/test/path/to"));
}

/// Test table state management
#[test]
fn test_table_state() {
    let mut table_state = TableState::default();
    
    // Test initial state
    assert_eq!(table_state.selected(), None);
    
    // Test selection
    table_state.select(Some(5));
    assert_eq!(table_state.selected(), Some(5));
    
    // Test clearing selection
    table_state.select(None);
    assert_eq!(table_state.selected(), None);
}

/// Test task info
#[test]
fn test_task_info() {
    let task = TaskInfo {
        id: 1,
        task_type: TaskType::LibraryScan,
        description: "Test task".to_string(),
        progress: Some(0.5),
        status: "Running".to_string(),
        created_at: SystemTime::now(),
        started_at: Some(SystemTime::now()),
        completed_at: None,
    };
    
    assert_eq!(task.id, 1);
    assert_eq!(task.task_type, TaskType::LibraryScan);
    assert_eq!(task.description, "Test task");
    assert_eq!(task.progress, Some(0.5));
    assert_eq!(task.status, "Running");
}
