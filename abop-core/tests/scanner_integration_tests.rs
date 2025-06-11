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
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let _scanner = LibraryScanner::new(db, library);
        // Test that scanner can be created without panicking
        // This is a basic smoke test
    }

    #[test]
    fn test_scan_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let scanner = LibraryScanner::new(db, library);
        let scan_summary = scanner
            .scan(abop_core::scanner::ScanOptions::default())
            .unwrap();

        assert_eq!(scan_summary.new_files.len(), 0);
        assert_eq!(scan_summary.processed, 0);
        assert_eq!(scan_summary.errors, 0);
    }

    #[test]
    fn test_scan_with_mixed_files() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create test files - only supported formats should be processed
        let audio_files = ["test1.mp3", "test2.flac", "test3.m4b"];
        let non_audio_files = ["readme.txt", "image.jpg", "video.mp4"];

        for filename in &audio_files {
            let file_path = temp_dir.path().join(filename);
            let mut file = File::create(file_path).unwrap();
            file.write_all(b"fake audio data").unwrap();
        }

        for filename in &non_audio_files {
            let file_path = temp_dir.path().join(filename);
            let mut file = File::create(file_path).unwrap();
            file.write_all(b"not audio data").unwrap();
        }

        let scanner = LibraryScanner::new(db, library);
        let scan_summary = scanner
            .scan(abop_core::scanner::ScanOptions::default())
            .unwrap();

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
        let db = Database::open(":memory:").unwrap();
        let temp_dir = tempdir().unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Use our mock scanner instead of the real one
        let mock_scanner = MockScanner::new(db, library);
        let scan_result = mock_scanner.scan().unwrap();

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
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create a nested directory structure
        for i in 0..5 {
            let subdir = temp_dir.path().join(format!("subdir_{i}"));
            std::fs::create_dir_all(&subdir).unwrap();

            // Create some test files in each subdirectory
            for j in 0..3 {
                let file_path = subdir.join(format!("test_{i}_{j}.mp3"));
                let mut file = File::create(file_path).unwrap();
                file.write_all(b"fake audio data").unwrap();
            }
        }

        let scanner = LibraryScanner::new(db, library);
        let start_time = std::time::Instant::now();
        let scan_summary = scanner
            .scan(abop_core::scanner::ScanOptions::default())
            .unwrap();
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
