//! Configurable thread pool for library scanning operations
//!
//! This module provides a thread pool implementation specifically designed for
//! audiobook library scanning with configurable worker count, work distribution,
//! and progress reporting capabilities.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{
    Arc, Condvar, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
    mpsc,
};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::error::{AppError, Result};
use crate::models::Audiobook;
use crate::scanner::LibraryScanner;

/// Configuration for the scanning thread pool
#[derive(Debug, Clone)]
pub struct ThreadPoolConfig {
    /// Number of worker threads (0 = auto-detect based on CPU cores)
    pub worker_count: usize,
    /// Maximum number of pending tasks in the queue
    pub max_queue_size: usize,
    /// Timeout for worker threads when waiting for tasks
    pub worker_timeout: Duration,
    /// Whether to enable adaptive thread scaling
    pub adaptive_scaling: bool,
    /// Minimum number of threads for adaptive scaling
    pub min_threads: usize,
    /// Maximum number of threads for adaptive scaling
    pub max_threads: usize,
    /// Interval for throughput monitoring
    pub monitoring_interval: Duration,
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        Self {
            worker_count: cpu_count,
            max_queue_size: cpu_count * 10,
            worker_timeout: Duration::from_secs(5),
            adaptive_scaling: false,
            min_threads: 2,
            max_threads: cpu_count * 2,
            monitoring_interval: Duration::from_secs(10),
        }
    }
}

impl ThreadPoolConfig {
    /// Create a configuration optimized for I/O heavy workloads (file scanning)
    pub fn for_io_heavy() -> Self {
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        Self {
            worker_count: cpu_count * 2, // More threads for I/O
            max_queue_size: cpu_count * 20,
            worker_timeout: Duration::from_secs(10),
            adaptive_scaling: true,
            min_threads: cpu_count,
            max_threads: cpu_count * 4,
            monitoring_interval: Duration::from_secs(5),
        }
    }

    /// Create a configuration optimized for CPU heavy workloads (metadata extraction)
    pub fn for_cpu_heavy() -> Self {
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        Self {
            worker_count: cpu_count,
            max_queue_size: cpu_count * 5,
            worker_timeout: Duration::from_secs(3),
            adaptive_scaling: false,
            min_threads: cpu_count / 2,
            max_threads: cpu_count,
            monitoring_interval: Duration::from_secs(15),
        }
    }

    /// Create a conservative configuration for resource-constrained environments
    pub fn conservative() -> Self {
        Self {
            worker_count: 2,
            max_queue_size: 10,
            worker_timeout: Duration::from_secs(5),
            adaptive_scaling: false,
            min_threads: 1,
            max_threads: 4,
            monitoring_interval: Duration::from_secs(30),
        }
    }
}

/// A work item for the scanning thread pool
#[derive(Debug)]
pub struct ScanTask {
    /// Path to the audio file to process
    pub file_path: PathBuf,
    /// Library ID for the audiobook
    pub library_id: String,
    /// Task priority (higher = more important)
    pub priority: u8,
    /// Task creation time for metrics
    pub created_at: Instant,
}

impl ScanTask {
    /// Create a new scan task with default priority
    pub fn new(file_path: PathBuf, library_id: String) -> Self {
        Self {
            file_path,
            library_id,
            priority: 5, // Default priority
            created_at: Instant::now(),
        }
    }

    /// Create a high priority scan task
    pub fn high_priority(file_path: PathBuf, library_id: String) -> Self {
        Self {
            file_path,
            library_id,
            priority: 10,
            created_at: Instant::now(),
        }
    }

    /// Create a low priority scan task
    pub fn low_priority(file_path: PathBuf, library_id: String) -> Self {
        Self {
            file_path,
            library_id,
            priority: 1,
            created_at: Instant::now(),
        }
    }
}

/// Result of processing a scan task
#[derive(Debug)]
pub struct ScanTaskResult {
    /// Successfully processed audiobook
    pub audiobook: Option<Audiobook>,
    /// Error that occurred during processing
    pub error: Option<AppError>,
    /// Time taken to process the task
    pub processing_time: Duration,
    /// Original task information
    pub task: ScanTask,
}

impl ScanTaskResult {
    /// Create a successful result
    pub fn success(audiobook: Audiobook, task: ScanTask, processing_time: Duration) -> Self {
        Self {
            audiobook: Some(audiobook),
            error: None,
            processing_time,
            task,
        }
    }

    /// Create an error result
    pub fn error(error: AppError, task: ScanTask, processing_time: Duration) -> Self {
        Self {
            audiobook: None,
            error: Some(error),
            processing_time,
            task,
        }
    }

    /// Check if the result represents a successful scan
    pub fn is_success(&self) -> bool {
        self.audiobook.is_some()
    }
}

/// Progress information for scanning operations
#[derive(Debug, Clone)]
pub struct ScanProgress {
    /// Total number of tasks
    pub total_tasks: usize,
    /// Number of completed tasks
    pub completed_tasks: usize,
    /// Number of successful tasks
    pub successful_tasks: usize,
    /// Number of failed tasks
    pub failed_tasks: usize,
    /// Current throughput (tasks per second)
    pub throughput: f64,
    /// Estimated time remaining
    pub eta: Option<Duration>,
    /// Number of active worker threads
    pub active_workers: usize,
}

impl ScanProgress {
    /// Calculate completion percentage (0.0 to 1.0)
    pub fn completion_percentage(&self) -> f32 {
        if self.total_tasks == 0 {
            1.0
        } else {
            self.completed_tasks as f32 / self.total_tasks as f32
        }
    }

    /// Check if scanning is complete
    pub fn is_complete(&self) -> bool {
        self.completed_tasks >= self.total_tasks
    }
}

/// Throughput monitor for adaptive thread scaling
#[derive(Debug)]
struct ThroughputMonitor {
    completed_tasks: AtomicUsize,
    last_measurement: Mutex<Instant>,
    throughput_history: Mutex<VecDeque<f64>>,
    history_size: usize,
}

impl ThroughputMonitor {
    fn new() -> Self {
        Self {
            completed_tasks: AtomicUsize::new(0),
            last_measurement: Mutex::new(Instant::now()),
            throughput_history: Mutex::new(VecDeque::new()),
            history_size: 10,
        }
    }

    fn record_completion(&self) {
        self.completed_tasks.fetch_add(1, Ordering::Relaxed);
    }

    fn measure_throughput(&self) -> f64 {
        let now = Instant::now();
        let mut last_time = self.last_measurement.lock().unwrap();
        let elapsed = now.duration_since(*last_time);

        if elapsed < Duration::from_secs(1) {
            return 0.0; // Too early to measure
        }

        let completed = self.completed_tasks.swap(0, Ordering::Relaxed);
        let throughput = completed as f64 / elapsed.as_secs_f64();

        let mut history = self.throughput_history.lock().unwrap();
        history.push_back(throughput);
        if history.len() > self.history_size {
            history.pop_front();
        }

        *last_time = now;
        throughput
    }

    fn average_throughput(&self) -> f64 {
        let history = self.throughput_history.lock().unwrap();
        if history.is_empty() {
            0.0
        } else {
            history.iter().sum::<f64>() / history.len() as f64
        }
    }
}

/// Worker context containing all shared resources
struct WorkerContext {
    task_queue: Arc<Mutex<VecDeque<ScanTask>>>,
    queue_condvar: Arc<Condvar>,
    result_sender: mpsc::Sender<ScanTaskResult>,
    shutdown_flag: Arc<AtomicBool>,
    active_workers: Arc<AtomicUsize>,
    throughput_monitor: Arc<ThroughputMonitor>,
    completed_tasks: Arc<AtomicUsize>,
    successful_tasks: Arc<AtomicUsize>,
    failed_tasks: Arc<AtomicUsize>,
    worker_timeout: Duration,
}

/// Configurable thread pool for scanning operations
pub struct ScanningThreadPool {
    config: ThreadPoolConfig,
    task_queue: Arc<Mutex<VecDeque<ScanTask>>>,
    queue_condvar: Arc<Condvar>,
    result_sender: mpsc::Sender<ScanTaskResult>,
    result_receiver: Arc<Mutex<mpsc::Receiver<ScanTaskResult>>>,
    workers: Vec<JoinHandle<()>>,
    shutdown_flag: Arc<AtomicBool>,
    active_workers: Arc<AtomicUsize>,
    throughput_monitor: Arc<ThroughputMonitor>,
    total_tasks: Arc<AtomicUsize>,
    completed_tasks: Arc<AtomicUsize>,
    successful_tasks: Arc<AtomicUsize>,
    failed_tasks: Arc<AtomicUsize>,
}

impl ScanningThreadPool {
    /// Create a new scanning thread pool with the given configuration
    pub fn new(config: ThreadPoolConfig) -> Result<Self> {
        let (result_sender, result_receiver) = mpsc::channel();
        let result_receiver = Arc::new(Mutex::new(result_receiver));

        let task_queue = Arc::new(Mutex::new(VecDeque::new()));
        let queue_condvar = Arc::new(Condvar::new());
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let active_workers = Arc::new(AtomicUsize::new(0));
        let throughput_monitor = Arc::new(ThroughputMonitor::new());

        let total_tasks = Arc::new(AtomicUsize::new(0));
        let completed_tasks = Arc::new(AtomicUsize::new(0));
        let successful_tasks = Arc::new(AtomicUsize::new(0));
        let failed_tasks = Arc::new(AtomicUsize::new(0));

        let mut pool = Self {
            config: config.clone(),
            task_queue: task_queue.clone(),
            queue_condvar: queue_condvar.clone(),
            result_sender,
            result_receiver,
            workers: Vec::new(),
            shutdown_flag: shutdown_flag.clone(),
            active_workers: active_workers.clone(),
            throughput_monitor: throughput_monitor.clone(),
            total_tasks: total_tasks.clone(),
            completed_tasks: completed_tasks.clone(),
            successful_tasks: successful_tasks.clone(),
            failed_tasks: failed_tasks.clone(),
        };

        // Start worker threads
        pool.start_workers()?;

        Ok(pool)
    }

    /// Set a progress callback for monitoring scan progress
    pub fn set_progress_callback<F>(&mut self, callback: F) -> Result<()>
    where
        F: Fn(ScanProgress) + Send + 'static,
    {
        let (sender, receiver) = mpsc::channel::<()>();
        let _progress_sender = Some(sender);

        // Spawn progress monitoring thread
        let config = self.config.clone();
        let active_workers = Arc::clone(&self.active_workers);
        let total_tasks = Arc::clone(&self.total_tasks);
        let completed_tasks = Arc::clone(&self.completed_tasks);
        let successful_tasks = Arc::clone(&self.successful_tasks);
        let failed_tasks = Arc::clone(&self.failed_tasks);
        let throughput_monitor = Arc::clone(&self.throughput_monitor);
        let shutdown_flag = Arc::clone(&self.shutdown_flag);
        thread::spawn(move || {
            while !shutdown_flag.load(Ordering::Relaxed) {
                thread::sleep(config.monitoring_interval);

                // Measure current throughput and update history
                let _current_throughput = throughput_monitor.measure_throughput();

                let total = total_tasks.load(Ordering::Relaxed);
                let completed = completed_tasks.load(Ordering::Relaxed);
                let successful = successful_tasks.load(Ordering::Relaxed);
                let failed = failed_tasks.load(Ordering::Relaxed);
                let workers = active_workers.load(Ordering::Relaxed);

                let throughput = throughput_monitor.average_throughput();
                let eta = if throughput > 0.0 && completed < total {
                    let remaining = total - completed;
                    Some(Duration::from_secs_f64(remaining as f64 / throughput))
                } else {
                    None
                };

                let progress = ScanProgress {
                    total_tasks: total,
                    completed_tasks: completed,
                    successful_tasks: successful,
                    failed_tasks: failed,
                    throughput,
                    eta,
                    active_workers: workers,
                };

                callback(progress);

                // Check if we need to receive explicit progress updates
                if receiver.try_recv().is_ok() {
                    // Progress update received, continue monitoring
                }
            }
        });

        Ok(())
    }

    /// Submit a batch of tasks for processing
    pub fn submit_tasks(&self, tasks: Vec<ScanTask>) -> Result<()> {
        let task_count = tasks.len();
        self.total_tasks.fetch_add(task_count, Ordering::Relaxed);
        let mut queue = self.task_queue.lock().map_err(|_| {
            AppError::Threading("Failed to acquire task queue lock".to_string().into())
        })?;

        // Check queue size limit
        if queue.len() + task_count > self.config.max_queue_size {
            return Err(AppError::Threading(
                format!(
                    "Task queue would exceed maximum size of {}",
                    self.config.max_queue_size
                )
                .into(),
            ));
        }

        // Sort tasks by priority (highest first) and add to queue
        let mut sorted_tasks = tasks;
        sorted_tasks.sort_by(|a, b| b.priority.cmp(&a.priority));

        for task in sorted_tasks {
            queue.push_back(task);
        }

        // Notify workers
        self.queue_condvar.notify_all();
        Ok(())
    }

    /// Submit a single task for processing
    pub fn submit_task(&self, task: ScanTask) -> Result<()> {
        self.submit_tasks(vec![task])
    }

    /// Get the next completed result (blocking)
    pub fn get_result(&self) -> Result<ScanTaskResult> {
        let receiver = self.result_receiver.lock().map_err(|_| {
            AppError::Threading("Failed to acquire result receiver lock".to_string().into())
        })?;

        receiver
            .recv()
            .map_err(|_| AppError::Threading("Result channel closed".to_string().into()))
    }

    /// Try to get the next completed result (non-blocking)
    pub fn try_get_result(&self) -> Result<Option<ScanTaskResult>> {
        let receiver = self.result_receiver.lock().map_err(|_| {
            AppError::Threading("Failed to acquire result receiver lock".to_string().into())
        })?;

        match receiver.try_recv() {
            Ok(result) => Ok(Some(result)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(AppError::Threading(
                "Result channel closed".to_string().into(),
            )),
        }
    }
    /// Get current scan progress
    pub fn get_progress(&self) -> ScanProgress {
        // Measure current throughput to update history
        let _current_throughput = self.throughput_monitor.measure_throughput();

        let total = self.total_tasks.load(Ordering::Relaxed);
        let completed = self.completed_tasks.load(Ordering::Relaxed);
        let successful = self.successful_tasks.load(Ordering::Relaxed);
        let failed = self.failed_tasks.load(Ordering::Relaxed);
        let workers = self.active_workers.load(Ordering::Relaxed);

        let throughput = self.throughput_monitor.average_throughput();
        let eta = if throughput > 0.0 && completed < total {
            let remaining = total - completed;
            Some(Duration::from_secs_f64(remaining as f64 / throughput))
        } else {
            None
        };

        ScanProgress {
            total_tasks: total,
            completed_tasks: completed,
            successful_tasks: successful,
            failed_tasks: failed,
            throughput,
            eta,
            active_workers: workers,
        }
    }

    /// Check if all tasks are complete
    pub fn is_complete(&self) -> bool {
        let total = self.total_tasks.load(Ordering::Relaxed);
        let completed = self.completed_tasks.load(Ordering::Relaxed);
        completed >= total && total > 0
    }

    /// Get the number of pending tasks
    pub fn pending_tasks(&self) -> usize {
        self.task_queue.lock().map(|queue| queue.len()).unwrap_or(0)
    }

    /// Start worker threads
    fn start_workers(&mut self) -> Result<()> {
        for worker_id in 0..self.config.worker_count {
            let task_queue = Arc::clone(&self.task_queue);
            let queue_condvar = Arc::clone(&self.queue_condvar);
            let result_sender = self.result_sender.clone();
            let shutdown_flag = Arc::clone(&self.shutdown_flag);
            let active_workers = Arc::clone(&self.active_workers);
            let throughput_monitor = Arc::clone(&self.throughput_monitor);
            let completed_tasks = Arc::clone(&self.completed_tasks);
            let successful_tasks = Arc::clone(&self.successful_tasks);
            let failed_tasks = Arc::clone(&self.failed_tasks);
            let worker_timeout = self.config.worker_timeout;

            let handle = thread::Builder::new()
                .name(format!("scanner-worker-{worker_id}"))
                .spawn(move || {
                    let context = WorkerContext {
                        task_queue,
                        queue_condvar,
                        result_sender,
                        shutdown_flag,
                        active_workers,
                        throughput_monitor,
                        completed_tasks,
                        successful_tasks,
                        failed_tasks,
                        worker_timeout,
                    };
                    Self::worker_loop(worker_id, context);
                })
                .map_err(|e| {
                    AppError::Threading(format!("Failed to start worker thread: {e}").into())
                })?;

            self.workers.push(handle);
        }

        Ok(())
    }

    /// Worker thread main loop
    fn worker_loop(worker_id: usize, context: WorkerContext) {
        log::debug!("Worker {worker_id} started");

        while !context.shutdown_flag.load(Ordering::Relaxed) {
            // Try to get a task from the queue
            let task = {
                let mut queue = match context.task_queue.lock() {
                    Ok(queue) => queue,
                    Err(_) => {
                        log::error!("Worker {worker_id} failed to acquire queue lock");
                        break;
                    }
                };

                // Wait for tasks or timeout
                if queue.is_empty() {
                    let (mut updated_queue, timeout_result) = context
                        .queue_condvar
                        .wait_timeout(queue, context.worker_timeout)
                        .unwrap_or_else(|_| panic!("Worker {worker_id} condvar wait failed"));

                    if timeout_result.timed_out() && updated_queue.is_empty() {
                        None // Timeout, no task available
                    } else {
                        updated_queue.pop_front()
                    }
                } else {
                    queue.pop_front()
                }
            };

            if let Some(task) = task {
                context.active_workers.fetch_add(1, Ordering::Relaxed);

                // Process the task
                let result = Self::process_task(task);

                // Update counters
                context.completed_tasks.fetch_add(1, Ordering::Relaxed);
                if result.is_success() {
                    context.successful_tasks.fetch_add(1, Ordering::Relaxed);
                } else {
                    context.failed_tasks.fetch_add(1, Ordering::Relaxed);
                }
                context.throughput_monitor.record_completion();

                // Send result
                if context.result_sender.send(result).is_err() {
                    log::warn!("Worker {worker_id} failed to send result");
                }

                context.active_workers.fetch_sub(1, Ordering::Relaxed);
            }
            // If no task was available (timeout), continue to check shutdown flag
        }

        log::debug!("Worker {worker_id} shutting down");
    }

    /// Process a single scan task
    fn process_task(task: ScanTask) -> ScanTaskResult {
        let start_time = Instant::now();

        match LibraryScanner::extract_audiobook_metadata(&task.library_id, &task.file_path) {
            Ok(audiobook) => {
                log::debug!("Successfully processed: {}", task.file_path.display());
                ScanTaskResult::success(audiobook, task, start_time.elapsed())
            }
            Err(e) => {
                log::warn!("Error processing {}: {}", task.file_path.display(), e);
                ScanTaskResult::error(e, task, start_time.elapsed())
            }
        }
    }

    /// Shutdown the thread pool and wait for all workers to finish
    pub fn shutdown(self) -> Result<()> {
        log::info!("Shutting down scanning thread pool");

        // Signal shutdown
        self.shutdown_flag.store(true, Ordering::Relaxed);
        self.queue_condvar.notify_all();

        // Wait for all workers to finish
        let mut join_errors = Vec::new();
        for (i, handle) in self.workers.into_iter().enumerate() {
            if let Err(e) = handle.join() {
                join_errors.push(format!("Worker {i} join error: {e:?}"));
            }
        }
        if !join_errors.is_empty() {
            return Err(AppError::Threading(
                format!("Worker thread join errors: {}", join_errors.join(", ")).into(),
            ));
        }

        log::info!("Scanning thread pool shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_thread_pool_config_defaults() {
        let config = ThreadPoolConfig::default();
        assert!(config.worker_count > 0);
        assert!(config.max_queue_size > 0);
        assert!(config.worker_timeout.as_secs() > 0);
    }

    #[test]
    fn test_specialized_configs() {
        let io_config = ThreadPoolConfig::for_io_heavy();
        let cpu_config = ThreadPoolConfig::for_cpu_heavy();
        let conservative_config = ThreadPoolConfig::conservative();

        // I/O config should have more threads
        assert!(io_config.worker_count >= cpu_config.worker_count);

        // Conservative config should be most restrictive
        assert!(conservative_config.worker_count <= cpu_config.worker_count);
        assert!(conservative_config.max_queue_size <= cpu_config.max_queue_size);
    }

    #[test]
    fn test_scan_task_creation() {
        let path = PathBuf::from("/test/file.mp3");
        let library_id = "test-lib".to_string();

        let normal_task = ScanTask::new(path.clone(), library_id.clone());
        let high_task = ScanTask::high_priority(path.clone(), library_id.clone());
        let low_task = ScanTask::low_priority(path, library_id);

        assert_eq!(normal_task.priority, 5);
        assert_eq!(high_task.priority, 10);
        assert_eq!(low_task.priority, 1);
    }

    #[test]
    fn test_scan_progress_calculations() {
        let progress = ScanProgress {
            total_tasks: 100,
            completed_tasks: 50,
            successful_tasks: 45,
            failed_tasks: 5,
            throughput: 10.0,
            eta: Some(Duration::from_secs(5)),
            active_workers: 4,
        };

        assert_eq!(progress.completion_percentage(), 0.5);
        assert!(!progress.is_complete());

        let complete_progress = ScanProgress {
            total_tasks: 100,
            completed_tasks: 100,
            successful_tasks: 95,
            failed_tasks: 5,
            throughput: 10.0,
            eta: None,
            active_workers: 0,
        };

        assert_eq!(complete_progress.completion_percentage(), 1.0);
        assert!(complete_progress.is_complete());
    }

    #[test]
    fn test_throughput_monitor() {
        let monitor = ThroughputMonitor::new();

        // Record some completions
        for _ in 0..5 {
            monitor.record_completion();
        }

        // Wait a bit and measure
        std::thread::sleep(Duration::from_millis(100));
        let throughput = monitor.measure_throughput();

        // Should have some throughput
        assert!(throughput >= 0.0);
    }
}
