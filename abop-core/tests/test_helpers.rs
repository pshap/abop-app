//! Test helpers for ABOP-Core tests.
//!
//! This module provides common utilities and fixtures for tests.

use abop_core::{AppState, db::Database, models::Library};
use std::path::PathBuf;
use tempfile::TempDir;

/// Creates a default AppState for testing
pub fn create_test_app_state() -> AppState {
    AppState::default()
}

/// Creates a temporary directory for testing file operations
#[allow(dead_code)]
pub fn create_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temporary directory")
}

/// Creates a test database in memory
#[allow(dead_code)]
pub fn create_test_database() -> Database {
    Database::open(":memory:").expect("Failed to create test database")
}

/// Creates a test library
#[allow(dead_code)]
pub fn create_test_library(name: &str, path: &std::path::Path) -> Library {
    Library::new(name, path)
}

/// Creates a test audio file path
#[allow(dead_code)]
pub fn test_audio_path(extension: &str) -> PathBuf {
    PathBuf::from(format!("test_file.{}", extension))
}

/// Creates a test directory structure with audio files
#[allow(dead_code)]
pub fn create_test_directory_structure(temp_dir: &TempDir) -> PathBuf {
    use std::fs::{self, File};
    use std::io::Write;

    let base_path = temp_dir.path().to_path_buf();

    // Create a subdirectory
    let subdir_path = base_path.join("subdir");
    fs::create_dir(&subdir_path).expect("Failed to create subdirectory");

    // Create some test audio files
    let mp3_path = base_path.join("test_file.mp3");
    let mut mp3_file = File::create(&mp3_path).expect("Failed to create MP3 file");
    mp3_file
        .write_all(b"MP3 file content")
        .expect("Failed to write to MP3 file");

    let m4b_path = base_path.join("audiobook.m4b");
    let mut m4b_file = File::create(&m4b_path).expect("Failed to create M4B file");
    m4b_file
        .write_all(b"M4B file content")
        .expect("Failed to write to M4B file");

    // Create a non-audio file
    let txt_path = base_path.join("document.txt");
    let mut txt_file = File::create(&txt_path).expect("Failed to create text file");
    txt_file
        .write_all(b"Text file content")
        .expect("Failed to write to text file");

    // Create a file in the subdirectory
    let subdir_mp3_path = subdir_path.join("subdir_file.mp3");
    let mut subdir_mp3_file =
        File::create(&subdir_mp3_path).expect("Failed to create subdirectory MP3 file");
    subdir_mp3_file
        .write_all(b"Subdirectory MP3 file content")
        .expect("Failed to write to subdirectory MP3 file");

    base_path
}
