//! Tests for the application state management

use super::*;
use abop_core::{
    models::{AppState, Audiobook},
    scanner::{progress::ScanProgress, ScannerState},
};
use crate::theme::ThemeMode;
use std::path::Path;

/// Test creating a new AppState with default values
#[test]
fn test_app_state_default() {
    let state = AppState::default();
    
    assert_eq!(state.ui.theme_mode, ThemeMode::Dark);
    assert!(state.library.scanner_progress.is_none());
    assert_eq!(state.library.scanner_state(), ScannerState::Idle);
    assert!(state.library.recent_directories.is_empty());
}

/// Test creating AppState from core AppState
#[test]
fn test_from_core_state() {
    let mut core_state = AppState::default();
    let id = "test-1".to_string();
    core_state.app_data.audiobooks.push(
        Audiobook {
            id: id.clone(),
            title: Some("Test Book".to_string()),
            author: Some("Test Author".to_string()),
            ..Default::default()
        },
    );

    let app_state = AppState::from_core_state(core_state);
    
    assert_eq!(app_state.library.audiobooks.len(), 1);
    assert_eq!(app_state.ui.theme_mode, ThemeMode::Dark);
}

/// Test theme mode changes
#[test]
fn test_set_theme_mode() {
    let mut state = AppState::default();
    
    // Test changing to dark mode
    state.set_theme_mode(ThemeMode::Dark);
    assert_eq!(state.ui.theme_mode, ThemeMode::Dark);
    
    // Test changing to light mode
    state.set_theme_mode(ThemeMode::Light);
    assert_eq!(state.ui.theme_mode, ThemeMode::Light);
    
    // Test changing to system mode
    state.set_theme_mode(ThemeMode::System);
    assert_eq!(state.ui.theme_mode, ThemeMode::System);
}

/// Test scan operations
#[test]
fn test_scan_operations() {
    let mut state = AppState::default();
    
    // Test starting a scan
    state.library.start_scanning();
    assert_eq!(state.library.scanner_state(), ScannerState::Scanning);
    
    // Test updating scan progress
    let progress = ScanProgress::FileProcessed {
        current_file: "test.mp3".to_string(),
        files_processed: 5,
        total_files: 10,
        progress_percentage: 0.5,
    };
    state.library.update_scan_progress(progress.clone());
    assert_eq!(state.library.scanner_progress, Some(progress));
    
    // Test cancelling a scan
    state.library.cancel_scanning();
    assert_eq!(state.library.scanner_state(), ScannerState::Cancelled);
}

/// Test directory metadata synchronization
#[test]
fn test_sync_directory_metadata() {
    let mut state = AppState::default();
    
    // Add a directory to recent
    let test_path = Path::new("/test/path/to").to_path_buf();
    state.library.add_recent_directory(test_path.clone(), std::time::Duration::from_secs(1));
    
    // Add an audiobook to the state in that directory
    let mut audiobook = Audiobook::default();
    audiobook.path = "/test/path/to/book.mp3".into();
    state.library.audiobooks.push(audiobook);
    
    // Synchronize directory metadata
    state.library.sync_directory_metadata();
    
    // Verify the directory was updated
    let dir = state.library.recent_directories.iter().find(|d| d.path == test_path).unwrap();
    assert_eq!(dir.book_count, 1);
}

/// Test table state management
#[test]
fn test_table_state() {
    let mut table_state = TableState::default();
    
    // Test initial state
    assert_eq!(table_state.sort_column, "title");
    assert!(table_state.sort_ascending);
    
    // Test changing sort
    table_state.sort_column = "author".to_string();
    table_state.sort_ascending = false;
    assert_eq!(table_state.sort_column, "author");
    assert!(!table_state.sort_ascending);
}