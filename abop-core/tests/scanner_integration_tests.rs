//! Integration tests for the scanner module
//!
//! These tests verify the interaction between the scanner components and the database,
//! ensuring that the scanning process works correctly with real file system operations.
//! These replace the legacy scanner tests with ones that match
//! the current API design.

use abop_core::{
    audio::AudioFormat,
    db::Database,
    models::{Audiobook, Library},
    scanner::{LibraryScanner, SUPPORTED_AUDIO_EXTENSIONS, ScanSummary},
};
// chrono::Utc is not currently used
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[cfg(test)]
mod library_scanner_tests {
    use super::*;

    #[test]
    fn test_supported_audio_extensions() {
        // Test that our supported extensions constant contains expected formats
        assert!(SUPPORTED_AUDIO_EXTENSIONS.contains(&"mp3"));
        assert!(SUPPORTED_AUDIO_EXTENSIONS.contains(&"m4a"));
        assert!(SUPPORTED_AUDIO_EXTENSIONS.contains(&"m4b"));
        assert!(SUPPORTED_AUDIO_EXTENSIONS.contains(&"flac"));
        assert!(SUPPORTED_AUDIO_EXTENSIONS.contains(&"ogg"));
        assert!(SUPPORTED_AUDIO_EXTENSIONS.contains(&"wav"));
        assert!(SUPPORTED_AUDIO_EXTENSIONS.contains(&"aac"));
    }

    #[test]
    fn test_audio_format_detection() {
        // Test the AudioFormat::from_extension function
        assert_eq!(AudioFormat::from_extension("mp3"), Some(AudioFormat::Mp3));
        assert_eq!(AudioFormat::from_extension("m4a"), Some(AudioFormat::Aac));
        assert_eq!(AudioFormat::from_extension("m4b"), Some(AudioFormat::Aac));
        assert_eq!(AudioFormat::from_extension("flac"), Some(AudioFormat::Flac));
        assert_eq!(AudioFormat::from_extension("ogg"), Some(AudioFormat::Ogg));
        assert_eq!(AudioFormat::from_extension("wav"), Some(AudioFormat::Wav));
        assert_eq!(AudioFormat::from_extension("aac"), Some(AudioFormat::Aac));

        // Test unsupported formats
        assert_eq!(AudioFormat::from_extension("txt"), None);
        assert_eq!(AudioFormat::from_extension("mp4"), None);
        assert_eq!(AudioFormat::from_extension(""), None);
    }

    #[test]
    fn test_audio_format_from_path() {
        // Test the AudioFormat::from_path function
        assert_eq!(AudioFormat::from_path("test.mp3"), Some(AudioFormat::Mp3));
        assert_eq!(
            AudioFormat::from_path("/path/to/file.flac"),
            Some(AudioFormat::Flac)
        );
        assert_eq!(
            AudioFormat::from_path("C:\\Windows\\file.m4a"),
            Some(AudioFormat::Aac)
        );

        // Test unsupported formats and edge cases
        assert_eq!(AudioFormat::from_path("test.txt"), None);
        assert_eq!(AudioFormat::from_path("test"), None);
    }

    #[test]
    fn test_library_scanner_creation() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let db = Database::open(":memory:").expect("Failed to open in-memory database");
        let library = Library::new("Test Library", temp_dir.path());

        let _scanner = LibraryScanner::new(db, library);
        // Test that scanner can be created without panicking
        // This is a basic smoke test
    }
    #[test]
    fn test_scan_empty_directory() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let db = Database::open(":memory:").expect("Failed to open in-memory database");
        let library = Library::new("Test Library", temp_dir.path());

        let scanner = LibraryScanner::new(db, library);
        let scan_summary = scanner
            .scan(abop_core::scanner::ScanOptions::default())
            .expect("Failed to scan empty directory");

        assert_eq!(scan_summary.new_files.len(), 0);
        assert_eq!(scan_summary.processed, 0);
        assert_eq!(scan_summary.errors, 0);
    }
    #[test]
    fn test_scan_with_mixed_files() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let db = Database::open(":memory:").expect("Failed to open in-memory database");
        let library = Library::new("Test Library", temp_dir.path());

        // Create test files - only supported formats should be processed
        let audio_files = ["test1.mp3", "test2.flac", "test3.m4b"];
        let non_audio_files = ["readme.txt", "image.jpg", "video.mp4"];
        for filename in &audio_files {
            let file_path = temp_dir.path().join(filename);
            let mut file = File::create(file_path).expect("Failed to create test audio file");
            file.write_all(b"fake audio data")
                .expect("Failed to write test audio data");
        }

        for filename in &non_audio_files {
            let file_path = temp_dir.path().join(filename);
            let mut file = File::create(file_path).expect("Failed to create test non-audio file");
            file.write_all(b"not audio data")
                .expect("Failed to write test non-audio data");
        }
        let scanner = LibraryScanner::new(db, library);
        let scan_summary = scanner
            .scan(abop_core::scanner::ScanOptions::default())
            .expect("Failed to scan directory with mixed files");

        // Only audio files should be discovered and attempted to be processed
        // Since these are fake files without proper audio metadata,
        // they should be detected as audio files but fail metadata extraction
        assert_eq!(scan_summary.processed, audio_files.len()); // Should attempt to process all audio files
        assert_eq!(scan_summary.errors, audio_files.len()); // All fake files should generate errors
        assert_eq!(scan_summary.new_files.len(), 0); // No valid audiobooks should be created
    }
}

#[cfg(test)]
mod mock_scanner_tests {
    use super::*;

    /// Mock scanner for testing without real audio files
    struct MockScanner {
        _db: Database,
        _library: Library,
    }

    impl MockScanner {
        fn new(db: Database, library: Library) -> Self {
            Self {
                _db: db,
                _library: library,
            }
        }

        /// Mock scan that returns predefined results
        fn scan(&self) -> Result<ScanSummary, Box<dyn std::error::Error>> {
            // Create mock audiobooks for testing
            let audiobook1 = Audiobook::new("test-library", "/test/book1.mp3");
            let audiobook2 = Audiobook::new("test-library", "/test/book2.flac");

            let result = ScanSummary {
                new_files: vec![audiobook1, audiobook2],
                processed: 2, // Successfully processed MP3 and FLAC
                errors: 2,    // Unsupported formats
                scan_duration: std::time::Duration::from_millis(100),
            };

            Ok(result)
        }
    }
    #[test]
    fn test_mock_scanner() {
        let db = Database::open(":memory:").expect("Failed to open in-memory database");
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let library = Library::new("Test Library", temp_dir.path());

        // Use our mock scanner instead of the real one
        let mock_scanner = MockScanner::new(db, library);
        let scan_result = mock_scanner.scan().expect("Mock scanner should not fail");

        // Verify we have the expected number of audiobooks
        assert_eq!(scan_result.new_files.len(), 2);

        // Verify the expected file formats were processed
        let formats: Vec<&str> = scan_result
            .new_files
            .iter()
            .filter_map(|book| book.path.extension())
            .filter_map(|ext| ext.to_str())
            .collect();

        assert!(formats.contains(&"mp3"));
        assert!(formats.contains(&"flac"));

        // Verify timing and processing stats
        assert_eq!(scan_result.processed, 2);
        assert_eq!(scan_result.errors, 2);
        assert!(scan_result.scan_duration > std::time::Duration::from_secs(0));
    }
}

#[cfg(test)]
mod scanner_performance_tests {
    use super::*;
    #[test]
    fn test_scan_large_directory_structure() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let db = Database::open(":memory:").expect("Failed to open in-memory database");
        let library = Library::new("Test Library", temp_dir.path());

        // Create a nested directory structure
        for i in 0..5 {
            let subdir = temp_dir.path().join(format!("subdir_{i}"));
            std::fs::create_dir_all(&subdir).expect("Failed to create test subdirectory");

            // Create some test files in each subdirectory
            for j in 0..3 {
                let file_path = subdir.join(format!("test_{i}_{j}.mp3"));
                let mut file = File::create(file_path).expect("Failed to create test file");
                file.write_all(b"fake audio data")
                    .expect("Failed to write test data");
            }
        }

        let scanner = LibraryScanner::new(db, library);
        let start_time = std::time::Instant::now();
        let scan_summary = scanner
            .scan(abop_core::scanner::ScanOptions::default())
            .expect("Failed to scan large directory structure");
        let scan_duration = start_time.elapsed();

        // Verify scan completed in reasonable time (adjust threshold as needed)
        assert!(scan_duration < std::time::Duration::from_secs(10));

        // Verify files were discovered
        assert!(scan_summary.processed > 0);

        println!(
            "Scanned {} files in {:?}",
            scan_summary.processed, scan_duration
        );
    }
}

/// Rescans keep audiobook ids stable and forget files that were deleted
#[test]
fn rescan_keeps_ids_and_prunes_deleted_files() {
    let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let library_dir = tempdir().unwrap();
    std::fs::copy(
        fixtures.join("tagged.mp3"),
        library_dir.path().join("one.mp3"),
    )
    .unwrap();
    std::fs::copy(
        fixtures.join("tagged.m4b"),
        library_dir.path().join("two.m4b"),
    )
    .unwrap();

    let db_dir = tempdir().unwrap();
    let db = Database::open(db_dir.path().join("test.db")).unwrap();
    let library_id = db
        .add_library_with_path("Books", library_dir.path().to_path_buf())
        .unwrap();
    let library = db.libraries().find_by_id(&library_id).unwrap().unwrap();

    let scan = || {
        LibraryScanner::new(db.clone(), library.clone())
            .scan(abop_core::scanner::ScanOptions::default())
            .unwrap();
        let mut books = db
            .audiobook_repository()
            .find_by_library(&library_id)
            .unwrap();
        books.sort_by(|a, b| a.path.cmp(&b.path));
        books
    };

    let first = scan();
    assert_eq!(first.len(), 2);

    let second = scan();
    assert_eq!(second.len(), 2, "rescan must not duplicate rows");
    let ids = |books: &[Audiobook]| books.iter().map(|b| b.id.clone()).collect::<Vec<_>>();
    assert_eq!(ids(&first), ids(&second), "rescan must keep ids stable");

    std::fs::remove_file(library_dir.path().join("two.m4b")).unwrap();
    let third = scan();
    assert_eq!(third.len(), 1, "deleted files are removed from the library");
    assert!(third[0].path.ends_with("one.mp3"));
    assert_eq!(third[0].id, first[0].id);
}

/// A missing library folder (e.g. unmounted drive) must not wipe the library
#[test]
fn scan_of_missing_folder_keeps_existing_books() {
    let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let parent = tempdir().unwrap();
    let library_path = parent.path().join("Books");
    std::fs::create_dir(&library_path).unwrap();
    std::fs::copy(fixtures.join("tagged.mp3"), library_path.join("one.mp3")).unwrap();

    let db_dir = tempdir().unwrap();
    let db = Database::open(db_dir.path().join("test.db")).unwrap();
    let library_id = db
        .add_library_with_path("Books", library_path.clone())
        .unwrap();
    let library = db.libraries().find_by_id(&library_id).unwrap().unwrap();
    let options = abop_core::scanner::ScanOptions::default;

    LibraryScanner::new(db.clone(), library.clone())
        .scan(options())
        .unwrap();
    std::fs::rename(&library_path, parent.path().join("Unmounted")).unwrap();
    let _ = LibraryScanner::new(db.clone(), library).scan(options());

    let books = db
        .audiobook_repository()
        .find_by_library(&library_id)
        .unwrap();
    assert_eq!(books.len(), 1);
}

/// An unreadable library folder (permissions, flaky mount) must fail the scan
/// instead of looking empty, which would remove every book from the library
#[cfg(unix)]
#[test]
fn scan_of_unreadable_folder_fails_and_keeps_books() {
    use std::os::unix::fs::PermissionsExt;

    let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let parent = tempdir().unwrap();
    let library_path = parent.path().join("Books");
    std::fs::create_dir(&library_path).unwrap();
    std::fs::copy(fixtures.join("tagged.mp3"), library_path.join("one.mp3")).unwrap();

    let db_dir = tempdir().unwrap();
    let db = Database::open(db_dir.path().join("test.db")).unwrap();
    let library_id = db
        .add_library_with_path("Books", library_path.clone())
        .unwrap();
    let library = db.libraries().find_by_id(&library_id).unwrap().unwrap();
    let options = abop_core::scanner::ScanOptions::default;
    LibraryScanner::new(db.clone(), library.clone())
        .scan(options())
        .unwrap();

    std::fs::set_permissions(&library_path, std::fs::Permissions::from_mode(0o000)).unwrap();
    // Root ignores permissions, so the scenario can't be reproduced there
    let unreadable = std::fs::read_dir(&library_path).is_err();
    let result = LibraryScanner::new(db.clone(), library).scan(options());
    std::fs::set_permissions(&library_path, std::fs::Permissions::from_mode(0o755)).unwrap();

    if unreadable {
        assert!(result.is_err(), "scanning an unreadable folder must fail");
    }
    let books = db
        .audiobook_repository()
        .find_by_library(&library_id)
        .unwrap();
    assert_eq!(books.len(), 1, "books must survive a failed scan");
}
