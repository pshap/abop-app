//! Performance monitoring and instrumentation for scanner operations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Performance metrics for scanner operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total operation time
    pub total_duration: Duration,
    /// Time spent on file I/O operations
    pub io_duration: Duration,
    /// Time spent on metadata extraction
    pub metadata_duration: Duration,
    /// Time spent on database operations
    pub database_duration: Duration,
    /// Number of files processed
    pub files_processed: usize,
    /// Number of errors encountered
    pub error_count: usize,
    /// Average time per file
    pub avg_time_per_file: Duration,
    /// Throughput (files per second)
    pub throughput: f64,
    /// Slowest operations (top 10)
    pub slowest_operations: Vec<SlowOperation>,
}

/// Information about a slow operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlowOperation {
    /// File path that was slow to process
    pub file_path: String,
    /// Duration of the operation
    pub duration: Duration,
    /// Type of operation that was slow
    pub operation_type: OperationType,
    /// Error message if the operation failed
    pub error: Option<String>,
}

/// Types of operations we track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    FileRead,
    MetadataExtraction,
    DatabaseInsert,
    ImageProcessing,
    AudioAnalysis,
}

/// Performance monitor for tracking scanner operations
#[derive(Debug)]
pub struct PerformanceMonitor {
    start_time: Instant,
    metrics: Arc<Mutex<PerformanceMetrics>>,
    operation_times: Arc<Mutex<HashMap<String, Vec<(OperationType, Duration)>>>>,
    slowest_threshold: Duration,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            metrics: Arc::new(Mutex::new(PerformanceMetrics::new())),
            operation_times: Arc::new(Mutex::new(HashMap::new())),
            slowest_threshold: Duration::from_millis(500), // Track operations > 500ms
        }
    }

    /// Record the start of an operation
    pub fn start_operation(
        &self,
        file_path: &str,
        operation_type: OperationType,
    ) -> OperationTimer {
        OperationTimer::new(
            file_path.to_string(),
            operation_type,
            self.operation_times.clone(),
            self.slowest_threshold,
        )
    }

    /// Record a completed file processing
    pub fn record_file_processed(&self, duration: Duration, success: bool) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.files_processed += 1;

        if !success {
            metrics.error_count += 1;
        }

        // Update running averages
        let total_files = metrics.files_processed;
        if total_files == 1 {
            metrics.avg_time_per_file = duration;
        } else {
            // Exponential moving average
            let alpha = 0.1;
            let current_avg_ms = metrics.avg_time_per_file.as_millis() as f64;
            let new_duration_ms = duration.as_millis() as f64;
            let new_avg_ms = current_avg_ms * (1.0 - alpha) + new_duration_ms * alpha;
            metrics.avg_time_per_file = Duration::from_millis(new_avg_ms as u64);
        }

        // Update throughput
        let elapsed = self.start_time.elapsed();
        if elapsed.as_secs() > 0 {
            metrics.throughput = total_files as f64 / elapsed.as_secs_f64();
        }
    }

    /// Get current performance metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        let mut metrics = self.metrics.lock().unwrap().clone();
        metrics.total_duration = self.start_time.elapsed();

        // Collect slowest operations
        let operation_times = self.operation_times.lock().unwrap();
        let mut all_operations = Vec::new();

        for (file_path, operations) in operation_times.iter() {
            for (op_type, duration) in operations {
                if *duration > self.slowest_threshold {
                    all_operations.push(SlowOperation {
                        file_path: file_path.clone(),
                        duration: *duration,
                        operation_type: op_type.clone(),
                        error: None,
                    });
                }
            }
        }

        // Sort by duration (slowest first) and take top 10
        all_operations.sort_by(|a, b| b.duration.cmp(&a.duration));
        all_operations.truncate(10);
        metrics.slowest_operations = all_operations;

        metrics
    }

    /// Log performance summary
    pub fn log_summary(&self) {
        let metrics = self.get_metrics();

        info!(
            "Scanner Performance Summary: {} files in {:?} ({:.2} files/sec, {:.1}% errors)",
            metrics.files_processed,
            metrics.total_duration,
            metrics.throughput,
            if metrics.files_processed > 0 {
                metrics.error_count as f64 / metrics.files_processed as f64 * 100.0
            } else {
                0.0
            }
        );

        if !metrics.slowest_operations.is_empty() {
            warn!("Slowest operations detected:");
            for (i, op) in metrics.slowest_operations.iter().enumerate().take(5) {
                warn!(
                    "  {}: {:?} on {} took {:?}",
                    i + 1,
                    op.operation_type,
                    op.file_path,
                    op.duration
                );
            }
        }
    }

    /// Get performance recommendations
    pub fn get_recommendations(&self) -> Vec<String> {
        let metrics = self.get_metrics();
        let mut recommendations = Vec::new();

        // Check throughput
        if metrics.throughput < 5.0 && metrics.files_processed > 10 {
            recommendations.push(
                "Low throughput detected. Consider increasing concurrency or checking for I/O bottlenecks.".to_string()
            );
        }

        // Check error rate
        let error_rate = if metrics.files_processed > 0 {
            metrics.error_count as f64 / metrics.files_processed as f64
        } else {
            0.0
        };

        if error_rate > 0.1 {
            recommendations.push(format!(
                "High error rate ({:.1}%). Check file permissions and format support.",
                error_rate * 100.0
            ));
        }

        // Check for slow operations
        let slow_io_ops = metrics
            .slowest_operations
            .iter()
            .filter(|op| matches!(op.operation_type, OperationType::FileRead))
            .count();

        if slow_io_ops > 3 {
            recommendations.push(
                "Multiple slow file I/O operations detected. Consider using faster storage or checking disk health.".to_string()
            );
        }

        recommendations
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer for tracking individual operations
pub struct OperationTimer {
    file_path: String,
    operation_type: OperationType,
    start_time: Instant,
    operation_times: Arc<Mutex<HashMap<String, Vec<(OperationType, Duration)>>>>,
    slowest_threshold: Duration,
}

impl OperationTimer {
    fn new(
        file_path: String,
        operation_type: OperationType,
        operation_times: Arc<Mutex<HashMap<String, Vec<(OperationType, Duration)>>>>,
        slowest_threshold: Duration,
    ) -> Self {
        Self {
            file_path,
            operation_type,
            start_time: Instant::now(),
            operation_times,
            slowest_threshold,
        }
    }

    /// Complete the operation and record its duration
    pub fn complete(self) -> Duration {
        let duration = self.start_time.elapsed();

        // Record the operation time
        {
            let mut times = self.operation_times.lock().unwrap();
            times
                .entry(self.file_path.clone())
                .or_default()
                .push((self.operation_type.clone(), duration));
        }

        // Log slow operations
        if duration > self.slowest_threshold {
            debug!(
                "Slow operation detected: {:?} on {} took {:?}",
                self.operation_type, self.file_path, duration
            );
        }

        duration
    }
}

impl PerformanceMetrics {
    fn new() -> Self {
        Self {
            total_duration: Duration::ZERO,
            io_duration: Duration::ZERO,
            metadata_duration: Duration::ZERO,
            database_duration: Duration::ZERO,
            files_processed: 0,
            error_count: 0,
            avg_time_per_file: Duration::ZERO,
            throughput: 0.0,
            slowest_operations: Vec::new(),
        }
    }

    /// Get a human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "Processed {} files in {:?} ({:.2} files/sec, {} errors)\nAvg time per file: {:?}\nSlowest operations: {}",
            self.files_processed,
            self.total_duration,
            self.throughput,
            self.error_count,
            self.avg_time_per_file,
            self.slowest_operations.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        let metrics = monitor.get_metrics();

        assert_eq!(metrics.files_processed, 0);
        assert_eq!(metrics.error_count, 0);
        assert!(metrics.total_duration < Duration::from_millis(100));
    }

    #[test]
    fn test_operation_timer() {
        let monitor = PerformanceMonitor::new();

        let timer = monitor.start_operation("test.mp3", OperationType::FileRead);
        thread::sleep(Duration::from_millis(10));
        let duration = timer.complete();

        assert!(duration >= Duration::from_millis(10));
        assert!(duration < Duration::from_millis(50));
    }

    #[test]
    fn test_file_processing_metrics() {
        let monitor = PerformanceMonitor::new();

        // Record some file processing
        monitor.record_file_processed(Duration::from_millis(100), true);
        monitor.record_file_processed(Duration::from_millis(200), false);

        let metrics = monitor.get_metrics();
        assert_eq!(metrics.files_processed, 2);
        assert_eq!(metrics.error_count, 1);
        assert!(metrics.avg_time_per_file > Duration::ZERO);
    }

    #[test]
    fn test_slowest_operations_tracking() {
        let monitor = PerformanceMonitor::new();

        // Create a slow operation
        let timer = monitor.start_operation("slow.mp3", OperationType::MetadataExtraction);
        thread::sleep(Duration::from_millis(10));
        timer.complete();

        let metrics = monitor.get_metrics();
        // The operation should be tracked if it exceeds the threshold
        // (in our test, the threshold is 500ms, so this won't be considered slow)
        assert_eq!(metrics.slowest_operations.len(), 0);
    }

    #[test]
    fn test_performance_recommendations() {
        let monitor = PerformanceMonitor::new();

        // Simulate many errors
        for _ in 0..10 {
            monitor.record_file_processed(Duration::from_millis(100), false);
        }

        let recommendations = monitor.get_recommendations();
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().any(|r| r.contains("error rate")));
    }
}
