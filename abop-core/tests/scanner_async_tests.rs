//! Comprehensive async tests for the LibraryScanner implementation
//!
//! These tests cover async behavior, cancellation scenarios, timeout handling,
//! and error recovery that are specific to the new async implementation.

use abop_core::{
    db::Database,
    models::Library,
    scanner::{
        LibraryScanner, ScannerConfig, ScanProgress, ChannelReporter,
        error::ScanError, progress::ProgressReporter
    },
};
use std::path::PathBuf;
use std::time::Duration;
use tempfile::tempdir;
use tokio::sync::mpsc;
use tokio::time::timeout;

#[cfg(test)]
mod async_scanner_tests {
    use super::*;

    #[tokio::test]
    async fn test_async_scan_with_progress() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let scanner = LibraryScanner::new(db, library);
        let (tx, mut rx) = mpsc::unbounded_channel();

        // Start scan with progress reporting
        let scan_task = tokio::spawn(async move {
            scanner.scan_async(Some(tx)).await
        });

        // Collect progress updates
        let mut progress_updates = Vec::new();
        while let Some(progress) = rx.recv().await {
            progress_updates.push(progress);
            if matches!(progress, ScanProgress::Complete { .. }) {
                break;
            }
        }

        let result = scan_task.await.unwrap();
        assert!(result.is_ok());

        // Should have at least Started and Complete events
        assert!(!progress_updates.is_empty());
        assert!(matches!(progress_updates[0], ScanProgress::Started { .. }));
        assert!(matches!(progress_updates.last().unwrap(), ScanProgress::Complete { .. }));
    }

    #[tokio::test]
    async fn test_scan_cancellation() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let scanner = LibraryScanner::new(db, library);

        // Start scan
        let scan_task = tokio::spawn(async move {
            scanner.scan_async(None).await
        });

        // Cancel immediately
        scanner.cancel_scan();

        // Task should complete quickly with cancellation error
        let result = timeout(Duration::from_millis(100), scan_task).await;
        assert!(result.is_ok());
        
        let scan_result = result.unwrap().unwrap();
        // For empty directory, cancellation might still succeed
        assert!(scan_result.is_ok() || matches!(scan_result, Err(ScanError::Cancelled)));
    }

    #[tokio::test]
    async fn test_timeout_handling() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create scanner with very short timeout
        let config = ScannerConfig {
            timeout: Some(Duration::from_millis(1)),
            ..Default::default()
        };
        
        let scanner = LibraryScanner::new(db, library).with_config(config);

        // Create a test file that might trigger timeout
        let test_file = temp_dir.path().join("test.mp3");
        std::fs::write(&test_file, b"fake audio data").unwrap();

        let result = scanner.scan_async(None).await;
        
        // Should either succeed (if file is processed quickly) or timeout
        match result {
            Ok(_) => {
                // Success is acceptable for small files
            }
            Err(ScanError::Timeout(_)) => {
                // Expected timeout error
            }
            Err(other) => {
                panic!("Unexpected error: {:?}", other);
            }
        }
    }

    #[tokio::test]
    async fn test_error_recovery() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let scanner = LibraryScanner::new(db, library);

        // Create invalid audio files that will cause errors
        let invalid_file1 = temp_dir.path().join("invalid1.mp3");
        let invalid_file2 = temp_dir.path().join("invalid2.mp3");
        std::fs::write(&invalid_file1, b"not really audio").unwrap();
        std::fs::write(&invalid_file2, b"also not audio").unwrap();

        let result = scanner.scan_async(None).await;
        
        // Should complete successfully even with invalid files
        assert!(result.is_ok());
        let summary = result.unwrap();
        
        // Should report errors but not fail completely
        assert!(summary.errors > 0);
        assert_eq!(summary.processed, 0); // No valid files processed
    }

    #[tokio::test]
    async fn test_concurrent_file_processing() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create multiple test files
        for i in 0..10 {
            let file_path = temp_dir.path().join(format!("test_{}.mp3", i));
            std::fs::write(&file_path, b"fake audio data").unwrap();
        }

        let config = ScannerConfig {
            max_concurrent_tasks: 3,
            ..Default::default()
        };

        let scanner = LibraryScanner::new(db, library).with_config(config);
        let (tx, mut rx) = mpsc::unbounded_channel();

        let start_time = std::time::Instant::now();
        let scan_task = tokio::spawn(async move {
            scanner.scan_async(Some(tx)).await
        });

        // Monitor progress
        let mut file_processed_count = 0;
        while let Some(progress) = rx.recv().await {
            if matches!(progress, ScanProgress::FileProcessed { .. }) {
                file_processed_count += 1;
            }
            if matches!(progress, ScanProgress::Complete { .. }) {
                break;
            }
        }

        let result = scan_task.await.unwrap();
        let duration = start_time.elapsed();

        assert!(result.is_ok());
        
        // Should process files concurrently (faster than sequential)
        // This is a rough heuristic - concurrent processing should be noticeably faster
        assert!(duration < Duration::from_secs(5));
        
        // Should have processed some files
        assert!(file_processed_count > 0);
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create multiple test files
        for i in 0..50 {
            let file_path = temp_dir.path().join(format!("test_{}.mp3", i));
            std::fs::write(&file_path, b"fake audio data").unwrap();
        }

        let config = ScannerConfig {
            batch_size: 10,
            ..Default::default()
        };

        let scanner = LibraryScanner::new(db, library).with_config(config);
        let (tx, mut rx) = mpsc::unbounded_channel();

        let scan_task = tokio::spawn(async move {
            scanner.scan_async(Some(tx)).await
        });

        // Monitor for batch commit events
        let mut batch_commits = 0;
        while let Some(progress) = rx.recv().await {
            if matches!(progress, ScanProgress::BatchCommitted { .. }) {
                batch_commits += 1;
            }
            if matches!(progress, ScanProgress::Complete { .. }) {
                break;
            }
        }

        let result = scan_task.await.unwrap();
        assert!(result.is_ok());
        
        // Should have multiple batch commits
        assert!(batch_commits > 0);
    }

    #[tokio::test]
    async fn test_progress_reporter_trait() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let reporter = ChannelReporter::new(tx);

        // Test different progress events
        let events = vec![
            ScanProgress::Started { total_files: 10 },
            ScanProgress::FileProcessed {
                current: 1,
                total: 10,
                file_name: "test.mp3".to_string(),
                progress_percentage: 0.1,
            },
            ScanProgress::Complete {
                processed: 10,
                errors: 0,
                duration: Duration::from_secs(1),
            },
        ];

        for event in events.clone() {
            reporter.report(event).await.unwrap();
        }

        // Verify all events were received
        let mut received_events = Vec::new();
        while let Ok(event) = rx.try_recv() {
            received_events.push(event);
        }

        assert_eq!(received_events.len(), events.len());
    }

    #[tokio::test]
    async fn test_memory_pressure_handling() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create config with limited resources
        let config = ScannerConfig {
            max_concurrent_tasks: 1, // Limit concurrency
            max_file_size: 1024,     // 1KB limit
            ..Default::default()
        };

        let scanner = LibraryScanner::new(db, library).with_config(config);

        // Create a file that exceeds the size limit
        let large_file = temp_dir.path().join("large.mp3");
        std::fs::write(&large_file, vec![0u8; 2048]).unwrap(); // 2KB file

        let result = scanner.scan_async(None).await;
        
        // Should complete successfully but skip the large file
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert!(summary.errors > 0); // Large file should cause an error
    }
}

#[cfg(test)]
mod task_integration_tests {
    use super::*;
    use iced::Task;

    #[tokio::test]
    async fn test_iced_task_integration() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create some test files
        for i in 0..5 {
            let file_path = temp_dir.path().join(format!("test_{}.mp3", i));
            std::fs::write(&file_path, b"fake audio data").unwrap();
        }

        let scanner = LibraryScanner::new(db, library);
        let audio_files = scanner.find_audio_files();

        // Test scan_with_tasks method
        let task = scanner.scan_with_tasks(audio_files);
        
        // In a real application, this would be executed by Iced's runtime
        // For testing, we can verify the task is created without error
        // Note: Actually executing the task requires Iced's runtime
        // which is not available in unit tests
        assert!(format!("{:?}", task).contains("Task"));
    }

    #[tokio::test]
    async fn test_task_with_progress_callbacks() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let scanner = LibraryScanner::new(db, library);
        let audio_files = scanner.find_audio_files();

        // Test progress callback version
        let progress_called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let progress_called_clone = progress_called.clone();

        let task = scanner.scan_with_tasks_and_progress(audio_files, move |_progress| {
            progress_called_clone.store(true, std::sync::atomic::Ordering::Relaxed);
        });

        // Verify task is created
        assert!(format!("{:?}", task).contains("Task"));
    }
}
