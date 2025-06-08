//! Orchestration layer for coordinating scanning operations
//!
//! This module provides high-level coordination between core scanning,
//! database operations, progress reporting, and performance monitoring.

use std::sync::Arc;
use std::time::Instant;

use futures::{stream, StreamExt};
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};

use crate::{
    db::Database,
    models::{Audiobook, Library},
    scanner::{
        config::ScannerConfig,
        core_scanner::CoreScanner,
        error::{ScanError, ScanResult},
        performance::PerformanceMonitor,
        progress::ProgressReporter,
        result::ScanSummary,
    },
};

/// Options for scan operations
#[derive(Debug, Clone)]
pub struct ScanOptions {
    /// Whether to enable progress reporting
    pub enable_progress: bool,
    /// Whether to enable performance monitoring
    pub enable_monitoring: bool,
    /// Custom batch size (if None, uses config default)
    pub batch_size: Option<usize>,
    /// Maximum concurrent operations (if None, uses config default)
    pub max_concurrent: Option<usize>,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            enable_progress: true,
            enable_monitoring: true,
            batch_size: None,
            max_concurrent: None,
        }
    }
}

/// High-level orchestrator for scan operations
pub struct ScanOrchestrator {
    /// Core scanner for file operations
    core_scanner: CoreScanner,
    /// Database for persistence
    database: Arc<Database>,
    /// Library being scanned
    library: Library,
    /// Progress reporter
    progress_reporter: Option<Arc<dyn ProgressReporter>>,
    /// Performance monitor
    performance_monitor: Option<Arc<PerformanceMonitor>>,
    /// Cancellation token
    cancel_token: CancellationToken,
    /// Configuration
    config: ScannerConfig,
}

impl ScanOrchestrator {
    /// Creates a new scan orchestrator
    pub fn new(
        database: Arc<Database>,
        library: Library,
        config: ScannerConfig,
    ) -> Self {
        let core_scanner = CoreScanner::with_config(config.clone());
        
        Self {
            core_scanner,
            database,
            library,
            progress_reporter: None,
            performance_monitor: None,
            cancel_token: CancellationToken::new(),
            config,
        }
    }

    /// Sets the progress reporter
    pub fn with_progress_reporter(mut self, reporter: Arc<dyn ProgressReporter>) -> Self {
        self.progress_reporter = Some(reporter);
        self
    }

    /// Sets the performance monitor
    pub fn with_performance_monitor(mut self, monitor: Arc<PerformanceMonitor>) -> Self {
        self.performance_monitor = Some(monitor);
        self
    }

    /// Sets the cancellation token
    pub fn with_cancellation_token(mut self, token: CancellationToken) -> Self {
        self.cancel_token = token;
        self
    }

    /// Performs a complete scan operation
    pub async fn scan(&self, options: ScanOptions) -> ScanResult<ScanSummary> {
        let start_time = Instant::now();
        
        // Discover all audio files
        let files = self.core_scanner.discover_files(&self.library.path).await?;
        let total_files = files.len();
        
        // Report scan start
        if options.enable_progress {
            if let Some(reporter) = &self.progress_reporter {
                reporter.report_started(total_files).await;
            }
        }

        let mut processed_audiobooks = Vec::new();
        let mut error_count = 0;
        
        // Determine batch size
        let batch_size = options.batch_size.unwrap_or(self.config.batch_size);
        
        // Process files in batches
        for (batch_index, file_chunk) in files.chunks(batch_size).enumerate() {
            if self.cancel_token.is_cancelled() {
                return Err(ScanError::Cancelled);
            }

            let mut batch_audiobooks = Vec::new();            // Process files in parallel within the batch
            let max_concurrent = options.max_concurrent.unwrap_or(self.config.max_concurrent_tasks);
            let core_scanner = self.core_scanner.clone();
            let library_id = self.library.id.clone();
            let monitor = self.performance_monitor.clone();
            
            // Convert to owned data to avoid lifetime issues
            let file_chunk_owned: Vec<_> = file_chunk.iter().enumerate().map(|(i, p)| (i, p.clone())).collect();
            
            let results: Vec<_> = stream::iter(file_chunk_owned.into_iter())
                .map(|(file_index, path)| {
                    let core_scanner = core_scanner.clone();
                    let library_id = library_id.clone();
                    let monitor = monitor.clone();
                    
                    async move {
                        let overall_index = batch_index * batch_size + file_index;
                        let result = if options.enable_monitoring {
                            core_scanner.extract_metadata_with_monitoring(
                                &library_id,
                                &path,
                                monitor.as_deref(),
                            ).await
                        } else {
                            core_scanner.extract_metadata(&library_id, &path).await
                        };
                        
                        (overall_index, path, result)
                    }
                })
                .buffer_unordered(max_concurrent)
                .collect()
                .await;
            
            // Process results and update progress
            for (file_index, path, result) in results {
                if self.cancel_token.is_cancelled() {
                    return Err(ScanError::Cancelled);
                }

                match result {
                    Ok(mut audiobook) => {
                        audiobook.library_id = self.library.id.clone();
                        batch_audiobooks.push(audiobook);
                        
                        // Report individual file progress
                        if options.enable_progress {
                            if let Some(reporter) = &self.progress_reporter {
                                let file_name = path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                
                                reporter.report_file_processed(
                                    file_index + 1,
                                    total_files,
                                    file_name,
                                ).await;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error processing {}: {}", path.display(), e);
                        error_count += 1;
                          // Log error through centralized handler
                        self.handle_scan_error(&e, "metadata_extraction");
                    }
                }
            }
            
            // Persist batch to database
            if !batch_audiobooks.is_empty() {
                if let Err(e) = self.persist_batch(&batch_audiobooks).await {
                    error!("Failed to persist batch: {}", e);
                    error_count += batch_audiobooks.len();
                    self.handle_scan_error(&e, "database_persistence");                } else {
                    processed_audiobooks.extend(batch_audiobooks);
                    
                    // Report progress update instead of batch_committed
                    if options.enable_progress {
                        if let Some(reporter) = &self.progress_reporter {
                            let progress = (processed_audiobooks.len() as f32 / total_files as f32) * 100.0;
                            reporter.report_progress(progress).await;
                        }
                    }
                }
            }
        }
        
        let duration = start_time.elapsed();
        let processed_count = processed_audiobooks.len();
        
        // Report completion
        if options.enable_progress {
            if let Some(reporter) = &self.progress_reporter {
                reporter.report_complete(processed_count, error_count, duration).await;
            }
        }
        
        // Log performance summary
        if options.enable_monitoring {
            if let Some(monitor) = &self.performance_monitor {
                monitor.log_summary();
            }
        }
        
        Ok(ScanSummary {
            new_files: processed_audiobooks,
            scan_duration: duration,
            processed: processed_count,
            errors: error_count,
        })
    }

    /// Persists a batch of audiobooks to the database
    async fn persist_batch(&self, audiobooks: &[Audiobook]) -> ScanResult<()> {
        let db = self.database.clone();
        let max_concurrent_db = self.config.max_concurrent_db_operations;
          // Process database operations with controlled concurrency
        let audiobooks_owned: Vec<Audiobook> = audiobooks.iter().cloned().collect();
        let results: Vec<_> = stream::iter(audiobooks_owned.into_iter())
            .map(|audiobook| {
                let db = db.clone();
                
                async move {
                    let repo = db.audiobook_repository();
                    repo.upsert(&audiobook).map_err(ScanError::from)
                }
            })
            .buffer_unordered(max_concurrent_db)
            .collect()
            .await;
        
        // Check for any errors
        for result in results {
            result?;
        }
        
        Ok(())
    }    /// Centralized error handling
    fn handle_scan_error(&self, error: &dyn std::error::Error, context: &str) {
        error!("Scan error in {}: {}", context, error);
        
        if let Some(_monitor) = &self.performance_monitor {
            // Could extend PerformanceMonitor to track errors
            // monitor.record_error(context, error.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Library;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_scan_orchestrator() {
        let temp_dir = tempdir().unwrap();        let library = Library::new("Test Library", temp_dir.path());
        
        // This would need a mock database for full testing
        // let orchestrator = ScanOrchestrator::new(db, library, ScannerConfig::default());
        // let result = orchestrator.scan(ScanOptions::default()).await;
        // assert!(result.is_ok());
    }
}
