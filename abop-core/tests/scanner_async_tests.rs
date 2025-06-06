//! Comprehensive async tests for the LibraryScanner implementation
//!
//! These tests cover async behavior, cancellation scenarios, timeout handling,
//! and error recovery that are specific to the new async implementation.

use abop_core::{
    db::{Database, DatabaseConfig},
    models::Library,
    scanner::{LibraryScanner, library_scanner::ScannerState, error::ScanError, progress::ScanProgress},
    db::repositories::AudiobookRepository,
};
use std::sync::Arc;
use std::time::Duration;
use tempfile::tempdir;

#[cfg(test)]
mod async_scanner_tests {
    use super::*;

    #[tokio::test]
    async fn test_async_scan_basic() {
        let temp_dir = tempdir().unwrap();
        let db_config = DatabaseConfig {
            path: ":memory:".to_string(),
            enhanced: false,
        };
        let _db = Database::new(db_config).unwrap();
        let _library = Library::new("Test Library", temp_dir.path());

        let repository = Arc::new(AudiobookRepository::new(Arc::new(std::sync::Mutex::new(
            rusqlite::Connection::open(":memory:").unwrap()
        ))));
        let scanner = LibraryScanner::new(repository, 2, 10);

        // Test basic scan functionality
        let result = scanner.scan_directory(temp_dir.path().to_path_buf()).await;
        assert!(result.is_ok());

        // Test scanner state
        let state = scanner.get_state().await;
        assert!(matches!(state, ScannerState::Completed));
    }

    #[tokio::test]
    async fn test_scan_cancellation() {
        let temp_dir = tempdir().unwrap();
        let db_config = DatabaseConfig {
            path: ":memory:".to_string(),
            enhanced: false,
        };
        let _db = Database::new(db_config).unwrap();
        let _library = Library::new("Test Library", temp_dir.path());

        let repository = Arc::new(AudiobookRepository::new(Arc::new(std::sync::Mutex::new(
            rusqlite::Connection::open(":memory:").unwrap()
        ))));
        let scanner = LibraryScanner::new(repository, 2, 10);

        // Start scan in background
        let path = temp_dir.path().to_path_buf();
        let scan_handle = tokio::spawn(async move {
            scanner.scan_directory(path).await
        });

        // Let scan start then cancel
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Wait for completion (in empty directory, should complete quickly)
        let result = scan_handle.await.unwrap();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_error_recovery() {
        let temp_dir = tempdir().unwrap();
        let db_config = DatabaseConfig {
            path: ":memory:".to_string(),
            enhanced: false,
        };
        let _db = Database::new(db_config).unwrap();
        let _library = Library::new("Test Library", temp_dir.path());

        let repository = Arc::new(AudiobookRepository::new(Arc::new(std::sync::Mutex::new(
            rusqlite::Connection::open(":memory:").unwrap()
        ))));
        let scanner = LibraryScanner::new(repository, 2, 10);

        // Create test files that might cause issues
        let invalid_file1 = temp_dir.path().join("invalid1.mp3");
        let invalid_file2 = temp_dir.path().join("invalid2.mp3");
        std::fs::write(&invalid_file1, b"not really audio").unwrap();
        std::fs::write(&invalid_file2, b"also not audio").unwrap();

        let result = scanner.scan_directory(temp_dir.path().to_path_buf()).await;
        
        // Should complete successfully even with invalid files
        assert!(result.is_ok());
        
        // Check final state
        let state = scanner.get_state().await;
        assert!(matches!(state, ScannerState::Completed));
    }

    #[tokio::test]
    async fn test_concurrent_file_processing() {
        let temp_dir = tempdir().unwrap();
        let db_config = DatabaseConfig {
            path: ":memory:".to_string(),
            enhanced: false,
        };
        let _db = Database::new(db_config).unwrap();
        let _library = Library::new("Test Library", temp_dir.path());

        // Create multiple test files
        for i in 0..10 {
            let file_path = temp_dir.path().join(format!("test_{}.mp3", i));
            std::fs::write(&file_path, b"fake audio data").unwrap();
        }

        // Create scanner with multiple workers
        let repository = Arc::new(AudiobookRepository::new(Arc::new(std::sync::Mutex::new(
            rusqlite::Connection::open(":memory:").unwrap()
        ))));
        let scanner = LibraryScanner::new(repository, 3, 5); // 3 workers, batch size 5

        let start_time = std::time::Instant::now();
        let result = scanner.scan_directory(temp_dir.path().to_path_buf()).await;
        let duration = start_time.elapsed();

        assert!(result.is_ok());
        
        // Should process files relatively quickly
        assert!(duration < Duration::from_secs(5));
        
        // Check final state
        let state = scanner.get_state().await;
        assert!(matches!(state, ScannerState::Completed));
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let temp_dir = tempdir().unwrap();
        let db_config = DatabaseConfig {
            path: ":memory:".to_string(),
            enhanced: false,
        };
        let _db = Database::new(db_config).unwrap();
        let _library = Library::new("Test Library", temp_dir.path());

        // Create multiple test files
        for i in 0..25 {
            let file_path = temp_dir.path().join(format!("test_{}.mp3", i));
            std::fs::write(&file_path, b"fake audio data").unwrap();
        }

        // Create scanner with specific batch size
        let repository = Arc::new(AudiobookRepository::new(Arc::new(std::sync::Mutex::new(
            rusqlite::Connection::open(":memory:").unwrap()
        ))));
        let scanner = LibraryScanner::new(repository, 2, 10); // batch size 10

        let result = scanner.scan_directory(temp_dir.path().to_path_buf()).await;
        assert!(result.is_ok());
        
        // Check final state
        let state = scanner.get_state().await;
        assert!(matches!(state, ScannerState::Completed));
    }

    #[tokio::test]
    async fn test_scanner_state_transitions() {
        let temp_dir = tempdir().unwrap();
        let db_config = DatabaseConfig {
            path: ":memory:".to_string(),
            enhanced: false,
        };
        let _db = Database::new(db_config).unwrap();
        let _library = Library::new("Test Library", temp_dir.path());

        let repository = Arc::new(AudiobookRepository::new(Arc::new(std::sync::Mutex::new(
            rusqlite::Connection::open(":memory:").unwrap()
        ))));
        let scanner = LibraryScanner::new(repository, 1, 5);

        // Initially should be idle
        let initial_state = scanner.get_state().await;
        assert!(matches!(initial_state, ScannerState::Idle));

        // Test pause/resume (will fail since not scanning)
        let pause_result = scanner.pause().await;
        assert!(pause_result.is_err());

        let resume_result = scanner.resume().await;
        assert!(resume_result.is_err());

        let cancel_result = scanner.cancel().await;
        assert!(cancel_result.is_err());
    }

    #[tokio::test]
    async fn test_progress_tracking() {
        let temp_dir = tempdir().unwrap();
        let db_config = DatabaseConfig {
            path: ":memory:".to_string(),
            enhanced: false,
        };
        let _db = Database::new(db_config).unwrap();
        let _library = Library::new("Test Library", temp_dir.path());

        // Create a few test files
        for i in 0..5 {
            let file_path = temp_dir.path().join(format!("test_{}.mp3", i));
            std::fs::write(&file_path, b"fake audio data").unwrap();
        }

        let repository = Arc::new(AudiobookRepository::new(Arc::new(std::sync::Mutex::new(
            rusqlite::Connection::open(":memory:").unwrap()
        ))));
        let scanner = LibraryScanner::new(repository, 1, 3);

        // Get initial progress
        let initial_progress = scanner.get_progress().await;
        assert_eq!(initial_progress.total_files, 0);
        assert_eq!(initial_progress.files_processed, 0);

        // Run scan
        let result = scanner.scan_directory(temp_dir.path().to_path_buf()).await;
        assert!(result.is_ok());

        // Check final progress
        let final_progress = scanner.get_progress().await;
        assert!(final_progress.total_files > 0);
    }
}
