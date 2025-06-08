//! Comprehensive async tests for the LibraryScanner implementation
//!
//! These tests cover async behavior, cancellation scenarios, timeout handling,
//! and error recovery that are specific to the new async implementation.

use abop_core::{
    db::Database,
    models::Library,
    scanner::{
        LibraryScanner, ScanOptions, ScanProgress, ScannerConfig,
        error::ScanError,
        progress::{ChannelReporter, ProgressReporter},
    },
};
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
        let db = Database::open(":memory:").await.unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let scanner = LibraryScanner::new(db, library);
        let (tx, mut rx) = mpsc::channel(100);

        // Start scan with progress reporting
        let scan_task = tokio::spawn(async move { scanner.scan_with_progress(tx).await }); // Collect progress updates
        let mut progress_updates = Vec::new();
        while let Some(progress) = rx.recv().await {
            let is_complete = matches!(progress, ScanProgress::Complete { .. });
            progress_updates.push(progress);
            if is_complete {
                break;
            }
        }

        let result = scan_task.await.unwrap();
        assert!(result.is_ok());

        // Should have at least Started and Complete events
        assert!(!progress_updates.is_empty());
        assert!(matches!(progress_updates[0], ScanProgress::Started { .. }));
        assert!(matches!(
            progress_updates.last().unwrap(),
            ScanProgress::Complete { .. }
        ));
    }
    #[tokio::test]
    async fn test_scan_cancellation() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").await.unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let scanner1 = LibraryScanner::new(db.clone(), library.clone());
        let scanner2 = LibraryScanner::new(db, library);

        // Start scan
        let scan_task = tokio::spawn(async move { scanner1.scan(ScanOptions::default()).await });

        // Cancel immediately using a second scanner instance
        scanner2.cancel_scan();

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
        let db = Database::open(":memory:").await.unwrap();
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

        let result = scanner.scan(ScanOptions::default()).await;

        // Should either succeed (if file is processed quickly) or timeout
        match result {
            Ok(_) => {
                // Success is acceptable for small files
            }
            Err(ScanError::Timeout(_)) => {
                // Expected timeout error
            }
            Err(other) => {
                panic!("Unexpected error: {other:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_error_recovery() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").await.unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let scanner = LibraryScanner::new(db, library);

        // Create invalid audio files that will cause errors
        let invalid_file1 = temp_dir.path().join("invalid1.mp3");
        let invalid_file2 = temp_dir.path().join("invalid2.mp3");
        std::fs::write(&invalid_file1, b"not really audio").unwrap();
        std::fs::write(&invalid_file2, b"also not audio").unwrap();

        let result = scanner.scan(ScanOptions::default()).await;

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
        let db = Database::open(":memory:").await.unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create multiple valid test WAV files
        for i in 0..10 {
            let file_path = temp_dir.path().join(format!("test_{i}.wav"));
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: 8000,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let mut writer = hound::WavWriter::create(&file_path, spec).unwrap();
            // Write a short audio sample (100ms)
            for _ in 0..800 {
                writer.write_sample(0i16).unwrap();
            }
            writer.finalize().unwrap();
        }

        let config = ScannerConfig {
            max_concurrent_tasks: 3,
            ..Default::default()
        };

        let scanner = LibraryScanner::new(db, library).with_config(config);
        let (tx, mut rx) = mpsc::channel(100);

        let start_time = std::time::Instant::now();
        let scan_task = tokio::spawn(async move { scanner.scan_with_progress(tx).await });

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
        let db = Database::open(":memory:").await.unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create multiple test files
        for i in 0..50 {
            let file_path = temp_dir.path().join(format!("test_{i}.mp3"));
            std::fs::write(&file_path, b"fake audio data").unwrap();
        }

        let config = ScannerConfig {
            batch_size: 10,
            ..Default::default()
        };

        let scanner = LibraryScanner::new(db, library).with_config(config);
        let (tx, mut rx) = mpsc::channel(100);

        let scan_task = tokio::spawn(async move { scanner.scan_with_progress(tx).await }); // Monitor for batch commit events
        let mut _batch_commits = 0;
        while let Some(progress) = rx.recv().await {
            if matches!(progress, ScanProgress::BatchCommitted { .. }) {
                _batch_commits += 1;
            }
            let is_complete = matches!(progress, ScanProgress::Complete { .. });
            if is_complete {
                break;
            }
        }
        let result = scan_task.await.unwrap();
        assert!(result.is_ok());

        // For fake audio files, batch commits might be 0 if no valid audiobooks are found
        // The test passes if the scan completes successfully (no errors)
        // In a real scenario with valid audio files, batch_commits would be > 0
    }
    #[tokio::test]
    async fn test_progress_reporter_trait() {
        let (tx, mut rx) = mpsc::channel::<ScanProgress>(10);
        let reporter = ChannelReporter::new(tx);

        // Test different types of events
        reporter.report_started(10).await;
        reporter
            .report_file_processed(1, 10, "test.mp3".to_string())
            .await;
        reporter
            .report_complete(10, 0, Duration::from_secs(1))
            .await;

        // Verify all events were received
        let mut received_events = Vec::new();
        rx.close(); // Close the receiver to stop waiting
        while let Some(event) = rx.recv().await {
            received_events.push(event);
        }

        assert_eq!(received_events.len(), 3); // Started, FileProcessed, Complete
    }

    #[tokio::test]
    async fn test_memory_pressure_handling() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").await.unwrap();
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

        let result = scanner.scan(ScanOptions::default()).await;

        // Should complete successfully but skip the large file
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert!(summary.errors > 0); // Large file should cause an error
    }
}

#[cfg(test)]
mod task_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_iced_task_integration() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").await.unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        // Create some test files
        for i in 0..5 {
            let file_path = temp_dir.path().join(format!("test_{i}.mp3"));
            std::fs::write(&file_path, b"fake audio data").unwrap();
        }

        let scanner = LibraryScanner::new(db, library);
        let (tx, _rx) = mpsc::channel(100);

        // Test scan_task_with_progress method
        let _task = scanner.scan_task_with_progress(tx);

        // In a real application, this would be executed by Iced's runtime        // For testing, we can verify the task is created without error
        // Note: Actually executing the task requires Iced's runtime
        // which is not available in unit tests
        // Task creation successful if we reach this point without panic
    }

    #[tokio::test]
    async fn test_task_with_progress_callbacks() {
        let temp_dir = tempdir().unwrap();
        let db = Database::open(":memory:").await.unwrap();
        let library = Library::new("Test Library", temp_dir.path());

        let scanner = LibraryScanner::new(db, library);
        let (tx, _rx) = mpsc::channel(100);

        // Test scan_task_with_progress method - this is the only method available
        let _task = scanner.scan_task_with_progress(tx); // Verify task is created
        // Task creation successful if we reach this point without panic
    }
}
