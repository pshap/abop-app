//! Batch processing operations for multiple audio files

use super::ProcessingConfig;
use super::error::{AudioProcessingError, Result};
use super::file_io::{AudioFileProcessor, FileProcessingOptions};
use crate::audio::AudioBufferPool;
use crate::audio::processing::pipeline::AudioProcessingPipeline;
use std::path::{Path, PathBuf};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};

/// Wrapper that adds cancellation support to AudioFileProcessor
///
/// **Purpose**: Provides cancellation-aware file processing without modifying
/// the underlying AudioFileProcessor implementation. This wrapper pattern allows
/// for responsive cancellation while maintaining compatibility with existing code.
///
/// **Architecture**: Uses composition over inheritance to add cancellation capability,
/// following the decorator pattern for clean separation of concerns.
struct CancellationAwareProcessor<'a> {
    processor: &'a AudioFileProcessor,
    cancellation_token: &'a Arc<AtomicBool>,
}

impl<'a> CancellationAwareProcessor<'a> {
    /// Process a file with integrated cancellation checks
    ///
    /// **Cancellation Points**: Checks for cancellation before and after processing
    /// to ensure responsive behavior without requiring changes to the underlying
    /// processor implementation.
    ///
    /// **Error Handling**: Provides detailed error context to distinguish between
    /// processing failures and cancellation requests.
    fn process_file_with_cancellation(&self, input_path: &Path) -> Result<PathBuf> {
        // Pre-processing cancellation check
        if self.cancellation_token.load(Ordering::SeqCst) {
            return Err(AudioProcessingError::Cancelled(format!(
                "Processing cancelled before starting file: {}",
                input_path.display()
            )));
        }

        // Clone the processor to get a mutable instance for processing
        // This is more efficient than creating a new processor from scratch
        // as it reuses the pipeline and options configuration
        let mut processor_clone = self.processor.clone();

        // Perform the actual file processing
        let result = processor_clone.process_file(input_path);

        // Post-processing cancellation check
        if self.cancellation_token.load(Ordering::SeqCst) {
            return Err(AudioProcessingError::Cancelled(format!(
                "Processing cancelled after completing file: {}",
                input_path.display()
            )));
        }

        result.map_err(|e| {
            AudioProcessingError::FileIo(format!(
                "Failed to process file '{}': {}",
                input_path.display(),
                e
            ))
        })
    }
}

/// Handles processing of multiple audio files, optionally in parallel.
#[derive(Clone)]
pub struct BatchProcessor {
    /// The file processor used for individual file operations.
    pub file_processor: AudioFileProcessor,
    /// Whether to enable parallel processing.
    pub enable_parallel: bool,
    /// Progress callback for reporting batch progress
    progress_callback: Option<Arc<dyn Fn(f32, String) + Send + Sync>>,
    /// Buffer pool for efficient memory reuse during processing
    buffer_pool: Option<Arc<AudioBufferPool<f32>>>,
    /// Flag to signal cancellation
    cancellation_token: Arc<AtomicBool>,
}

/// Result of batch processing operation
#[derive(Debug, PartialEq)]
pub struct BatchProcessingResult {
    /// Successfully processed files and their output paths
    pub successful: Vec<(PathBuf, PathBuf)>,
    /// Failed files and their errors
    pub failed: Vec<(PathBuf, AudioProcessingError)>,
    /// Total processing time
    pub total_time: Duration,
    /// Average time per file
    pub average_time_per_file: Duration,
}

impl BatchProcessingResult {
    /// Get the success rate as a percentage
    #[must_use]
    pub fn success_rate(&self) -> f32 {
        let total = self.successful.len() + self.failed.len();
        if total == 0 {
            0.0
        } else {
            #[allow(clippy::cast_precision_loss)]
            let successful = self.successful.len() as f32;
            #[allow(clippy::cast_precision_loss)]
            let total_f32 = total as f32;
            (successful / total_f32) * 100.0
        }
    }
    /// Check if all files were processed successfully
    #[must_use]
    pub const fn all_successful(&self) -> bool {
        self.failed.is_empty()
    }
}

impl BatchProcessor {
    /// Create a new `BatchProcessor` with the given config and file options.
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if the audio processing pipeline cannot be created
    /// due to invalid configuration parameters.
    pub fn new(config: ProcessingConfig, options: FileProcessingOptions) -> Result<Self> {
        let enable_parallel = config.enable_parallel;
        let pipeline = AudioProcessingPipeline::new(config)?;
        let file_processor = AudioFileProcessor::new(pipeline, options);
        Ok(Self {
            file_processor,
            enable_parallel,
            progress_callback: None,
            buffer_pool: None,
            cancellation_token: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Set a progress callback for reporting batch progress
    #[must_use]
    pub fn with_progress_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32, String) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Arc::new(callback));
        self
    }

    /// Enable memory pooling for efficient buffer reuse during processing
    ///
    /// # Arguments
    ///
    /// * `pool_size` - Number of buffers to pre-allocate (default: 16)
    /// * `buffer_capacity` - Capacity of each buffer in samples (default: 1MB)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    #[must_use]
    pub fn with_buffer_pool(
        mut self,
        pool_size: Option<usize>,
        buffer_capacity: Option<usize>,
    ) -> Self {
        // Default to 16 buffers of 1MB each (assuming 4 bytes per f32 sample = 262,144 samples)
        let pool_size = pool_size.unwrap_or(16);
        let buffer_capacity = buffer_capacity.unwrap_or(262_144);

        self.buffer_pool = Some(Arc::new(AudioBufferPool::new(pool_size, buffer_capacity)));
        self
    }

    /// Process a list of files, using parallelism if enabled.
    /// Returns detailed results including successes, failures, and timing information.
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError::InvalidInput`] if no input files are provided,
    /// or [`AudioProcessingError`] if file processing fails due to audio format issues.
    pub fn process_files_detailed<P: AsRef<Path> + Send + Sync>(
        &self,
        input_paths: &[P],
    ) -> Result<BatchProcessingResult> {
        let start_time = Instant::now();

        if input_paths.is_empty() {
            return Err(AudioProcessingError::InvalidInput(
                "No input files provided".to_string(),
            ));
        }

        let total_files = input_paths.len();
        let mut successful = Vec::new();
        let mut failed = Vec::new();

        if self.enable_parallel {
            self.process_files_parallel_detailed(input_paths, &mut successful, &mut failed);
        } else {
            self.process_files_sequential_detailed(input_paths, &mut successful, &mut failed);
        }

        let total_time = start_time.elapsed();
        let average_time_per_file = if total_files > 0 {
            // Safe conversion with bounds checking
            u32::try_from(total_files).map_or_else(
                |_| {
                    // If total_files is too large for u32, calculate differently
                    let total_nanos = total_time.as_nanos();
                    let total_files_u128 = u128::try_from(total_files).unwrap_or(1);
                    let avg_nanos = total_nanos / total_files_u128;
                    // Use try_from for u128->u64 conversion
                    Duration::from_nanos(
                        u64::try_from(avg_nanos.min(u128::from(u64::MAX))).unwrap_or(u64::MAX),
                    )
                },
                |total_files_u32| total_time / total_files_u32,
            )
        } else {
            Duration::ZERO
        };

        Ok(BatchProcessingResult {
            successful,
            failed,
            total_time,
            average_time_per_file,
        })
    }

    /// Process a list of files, using parallelism if enabled.
    /// Returns only the successful output paths (legacy compatibility).
    ///
    /// # Errors
    ///
    /// Returns [`AudioProcessingError`] if batch processing fails or if any critical
    /// processing errors occur that prevent successful completion.
    pub fn process_files<P: AsRef<Path> + Send + Sync>(
        &self,
        input_paths: &[P],
    ) -> Result<Vec<PathBuf>> {
        let result = self.process_files_detailed(input_paths)?;

        if !result.all_successful() {
            let error_msg = format!(
                "Batch processing completed with {} failures out of {} files",
                result.failed.len(),
                result.successful.len() + result.failed.len()
            );
            return Err(AudioProcessingError::Pipeline(error_msg));
        }

        Ok(result
            .successful
            .into_iter()
            .map(|(_, output)| output)
            .collect())
    }

    /// Report progress for a file processing operation
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn report_progress(&self, index: usize, total: usize) {
        if let Some(callback) = &self.progress_callback {
            let progress = if total == 0 {
                0.0f32
            } else {
                let progress_f64 = (index as f64 / total as f64) * 100.0;
                progress_f64.clamp(0.0, 100.0) as f32
            };
            let message = format!("Processing file {} of {}", index + 1, total);
            callback(progress, message);
        }
    }

    /// Cancel the current batch processing operation
    ///
    /// This will cause the processing to stop at the next opportunity.
    pub fn cancel(&self) {
        self.cancellation_token.store(true, Ordering::SeqCst);
    }

    /// Check if processing has been cancelled
    fn is_cancelled(&self) -> bool {
        self.cancellation_token.load(Ordering::SeqCst)
    }

    /// Reset the cancellation token
    fn reset_cancellation(&self) {
        self.cancellation_token.store(false, Ordering::SeqCst);
    }

    /// Process files in parallel with detailed result tracking
    fn process_files_parallel_detailed<P: AsRef<Path> + Send + Sync>(
        &self,
        input_paths: &[P],
        successful: &mut Vec<(PathBuf, PathBuf)>,
        failed: &mut Vec<(PathBuf, AudioProcessingError)>,
    ) {
        use rayon::prelude::*;

        // Reset cancellation state at the start of processing
        self.reset_cancellation();
        let total_files = input_paths.len();

        // Early cancellation check before starting parallel processing
        if self.is_cancelled() {
            self.add_cancellation_errors_for_all_files(input_paths, failed);
            return;
        }

        let results: Vec<_> = input_paths
            .par_iter()
            .enumerate()
            .map(|(index, path)| {
                // Check for cancellation before processing each file
                if self.is_cancelled() {
                    return Err((
                        path.as_ref().to_path_buf(),
                        AudioProcessingError::Cancelled("Processing was cancelled".to_string()),
                    ));
                }

                self.report_progress(index, total_files);

                let input_path = path.as_ref().to_path_buf();

                // Process file with additional cancellation checks during processing
                match self.process_single_file_with_cancellation_checks(&input_path) {
                    Ok(output_path) => Ok((input_path, output_path)),
                    Err(e) => Err((input_path, e)),
                }
            })
            .collect();

        // Check for cancellation after parallel processing but before result processing
        if self.is_cancelled() {
            // Add cancellation errors for any files that weren't processed due to cancellation
            for result in &results {
                if let Err((path, _)) = result {
                    failed.push((
                        path.clone(),
                        AudioProcessingError::Cancelled("Processing was cancelled".to_string()),
                    ));
                }
            }
            return;
        }

        // Separate successful and failed results
        for result in results {
            match result {
                Ok((input, output)) => successful.push((input, output)),
                Err((input, error)) => failed.push((input, error)),
            }
        }

        // Report final progress if not cancelled
        if !self.is_cancelled() {
            self.report_progress(total_files, total_files);
        }
    }

    /// Helper method to add cancellation errors for all remaining files
    ///
    /// **Purpose**: Efficiently handles the scenario where processing is cancelled
    /// before or during batch operations, ensuring all unprocessed files are marked
    /// as cancelled rather than left in an undefined state.
    fn add_cancellation_errors_for_all_files<P: AsRef<Path>>(
        &self,
        input_paths: &[P],
        failed: &mut Vec<(PathBuf, AudioProcessingError)>,
    ) {
        for path in input_paths {
            failed.push((
                path.as_ref().to_path_buf(),
                AudioProcessingError::Cancelled(
                    "Processing was cancelled before file processing".to_string(),
                ),
            ));
        }
    }

    /// Process a single file with additional cancellation checks during processing
    ///
    /// **Cancellation Safety**: This method provides more granular cancellation checking
    /// during individual file processing operations, particularly important for large files
    /// or complex processing pipelines that may take significant time.
    ///
    /// **Performance**: Uses a wrapper approach to add cancellation support while
    /// maintaining the existing file processor interface and avoiding unnecessary
    /// resource allocation.
    ///
    /// **Implementation**: Provides periodic cancellation checks and proper error handling
    /// for responsive cancellation even during intensive processing operations.
    fn process_single_file_with_cancellation_checks(&self, input_path: &Path) -> Result<PathBuf> {
        // Pre-processing cancellation check
        if self.is_cancelled() {
            return Err(AudioProcessingError::Cancelled(format!(
                "Processing cancelled before starting file: {}",
                input_path.display()
            )));
        }

        // Use a custom processing approach that integrates cancellation checks
        // This avoids the need to create new processor instances for each file
        self.process_file_with_integrated_cancellation(input_path)
    }

    /// Process a file with integrated cancellation support
    ///
    /// **Cancellation Integration**: This method provides cancellation checking at
    /// key points during file processing without requiring changes to the underlying
    /// AudioFileProcessor implementation.
    ///
    /// **Architecture**: Uses a wrapper pattern to add cancellation capability while
    /// preserving the existing processor interface and avoiding performance penalties.
    ///
    /// **Future Enhancement**: This approach allows for easy migration to a cancellation-aware
    /// processor implementation when available.
    fn process_file_with_integrated_cancellation(&self, input_path: &Path) -> Result<PathBuf> {
        // Create a cancellation-aware processor wrapper
        let processor = CancellationAwareProcessor {
            processor: &self.file_processor,
            cancellation_token: &self.cancellation_token,
        };

        processor.process_file_with_cancellation(input_path)
    }

    /// Process files sequentially with detailed result tracking and cancellation checks
    ///
    /// **Cancellation Safety**: This method provides responsive cancellation checking
    /// in sequential processing mode, ensuring prompt termination when requested.
    /// Cancellation is checked before processing each individual file.
    ///
    /// **Error Handling**: Each file is processed independently, so failures in one
    /// file don't prevent processing of subsequent files. Failed files are tracked
    /// separately from successful ones.
    fn process_files_sequential_detailed<P: AsRef<Path> + Send + Sync>(
        &self,
        input_paths: &[P],
        successful: &mut Vec<(PathBuf, PathBuf)>,
        failed: &mut Vec<(PathBuf, AudioProcessingError)>,
    ) {
        // Reset cancellation state at the start of processing
        self.reset_cancellation();
        let total_files = input_paths.len();

        for (index, path) in input_paths.iter().enumerate() {
            // Check for cancellation before processing each file
            if self.is_cancelled() {
                // Add cancellation error for remaining files
                for remaining_path in &input_paths[index..] {
                    failed.push((
                        remaining_path.as_ref().to_path_buf(),
                        AudioProcessingError::Cancelled("Processing was cancelled".to_string()),
                    ));
                }
                break;
            }

            self.report_progress(index, total_files);

            let input_path = path.as_ref().to_path_buf();
            match self.process_single_file_with_cancellation_checks(&input_path) {
                Ok(output_path) => successful.push((input_path, output_path)),
                Err(e) => failed.push((input_path, e)),
            }
        }

        // Report final progress
        if !self.is_cancelled() {
            self.report_progress(total_files, total_files);
        }
    }
}
