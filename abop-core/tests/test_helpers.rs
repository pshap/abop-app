//! Test helpers for ABOP-Core tests.
//!
//! This module provides common utilities and fixtures for tests.
//! All helpers are designed to be thread-safe and can be used in parallel tests.

use abop_core::{AppState, db::Database, models::Library};
use std::path::PathBuf;
use tempfile::TempDir;

/// Creates a default AppState for testing.
///
/// # Examples
/// ```
/// use abop_core::tests::test_helpers::create_test_app_state;
///
/// let app_state = create_test_app_state();
/// assert_eq!(app_state.libraries().len(), 0);
/// ```
pub fn create_test_app_state() -> AppState {
    AppState::default()
}

/// Creates a temporary directory for testing file operations.
/// The directory is automatically cleaned up when the returned `TempDir` is dropped.
///
/// # Examples
/// ```
/// use abop_core::tests::test_helpers::create_temp_dir;
///
/// let temp_dir = create_temp_dir();
/// let file_path = temp_dir.path().join("test.txt");
/// std::fs::write(&file_path, "test content").unwrap();
/// assert!(file_path.exists());
/// ```
#[allow(dead_code)]
pub fn create_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temporary directory")
}

/// Creates a test database in memory.
/// This is useful for tests that need database access without persisting data.
///
/// # Examples
/// ```
/// use abop_core::tests::test_helpers::create_test_database;
///
/// let db = create_test_database();
/// assert!(db.is_connected());
/// ```
#[allow(dead_code)]
pub fn create_test_database() -> Database {
    Database::open(":memory:").expect("Failed to create test database")
}

/// Creates a test library with the given name and path.
///
/// # Arguments
/// * `name` - The name of the library
/// * `path` - The path where the library's files are stored
///
/// # Examples
/// ```
/// use abop_core::tests::test_helpers::{create_test_library, create_temp_dir};
///
/// let temp_dir = create_temp_dir();
/// let library = create_test_library("Test Library", temp_dir.path());
/// assert_eq!(library.name, "Test Library");
/// ```
#[allow(dead_code)]
pub fn create_test_library(name: &str, path: &std::path::Path) -> Library {
    Library::new(name, path)
}

/// Creates a test audio file path with the given extension.
/// This is a convenience function for creating test file paths.
///
/// # Arguments
/// * `extension` - The file extension (e.g., "mp3", "wav")
///
/// # Examples
/// ```
/// use abop_core::tests::test_helpers::test_audio_path;
///
/// let path = test_audio_path("mp3");
/// assert!(path.extension().unwrap() == "mp3");
/// ```
#[allow(dead_code)]
pub fn test_audio_path(extension: &str) -> PathBuf {
    PathBuf::from(format!("test_file.{extension}"))
}

/// Creates a test directory structure with audio files.
/// This is useful for testing file scanning and library operations.
///
/// # Arguments
/// * `temp_dir` - The temporary directory to create the structure in
///
/// # Returns
/// The path to the base directory of the created structure
///
/// # Examples
/// ```
/// use abop_core::tests::test_helpers::{create_temp_dir, create_test_directory_structure};
///
/// let temp_dir = create_temp_dir();
/// let base_path = create_test_directory_structure(&temp_dir);
/// assert!(base_path.join("test_file.mp3").exists());
/// assert!(base_path.join("subdir").is_dir());
/// ```
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

/// A trait for test fixtures that need setup and teardown.
///
/// This trait provides a standard way to create and clean up test fixtures.
/// It's particularly useful for tests that need complex setup or cleanup.
///
/// # Examples
/// ```
/// use abop_core::tests::test_helpers::{TestFixture, cleanup_fixture};
///
/// struct MyTestFixture {
///     temp_dir: TempDir,
///     db: Database,
/// }
///
/// impl TestFixture for MyTestFixture {
///     fn setup() -> Self {
///         let temp_dir = create_temp_dir();
///         let db = create_test_database();
///         Self { temp_dir, db }
///     }
///
///     fn teardown(&mut self) {
///         // Cleanup if needed
///     }
/// }
///
/// // In your test:
/// let mut fixture = MyTestFixture::setup();
/// // ... use fixture ...
/// cleanup_fixture(&mut fixture);
/// ```
#[allow(dead_code)]
pub trait TestFixture {
    /// Creates a new instance of the fixture.
    fn setup() -> Self;

    /// Performs any necessary cleanup.
    fn teardown(&mut self) {}
}

/// Helper function to clean up a test fixture.
/// This should be called at the end of tests that use fixtures.
///
/// # Arguments
/// * `fixture` - The fixture to clean up
#[allow(dead_code)]
pub fn cleanup_fixture<F: TestFixture>(fixture: &mut F) {
    fixture.teardown();
}
