//! Constants used by the scanner module

use std::time::Duration;

/// Default concurrency level for scanning operations
pub const DEFAULT_CONCURRENCY: usize = 8;

/// Default batch size for database operations
pub const DEFAULT_BATCH_SIZE: usize = 100;

/// Default timeout for scanning operations
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default progress buffer size for channel-based progress reporting
pub const PROGRESS_BUFFER_SIZE: usize = 100;

/// Default maximum file size (1GB)
pub const DEFAULT_MAX_FILE_SIZE: u64 = 1024 * 1024 * 1024;

/// Default worker timeout for thread pool operations
pub const DEFAULT_WORKER_TIMEOUT: Duration = Duration::from_secs(5);

/// Default monitoring interval for performance metrics
pub const DEFAULT_MONITORING_INTERVAL: Duration = Duration::from_secs(10);

/// Default minimum number of threads for adaptive scaling
pub const DEFAULT_MIN_THREADS: usize = 2;

/// Default maximum number of threads for adaptive scaling
pub const DEFAULT_MAX_THREADS_MULTIPLIER: usize = 2;

/// Default queue size multiplier for thread pool
pub const DEFAULT_QUEUE_SIZE_MULTIPLIER: usize = 10;

/// Supported audio file extensions for scanning
pub const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["mp3", "m4a", "m4b", "flac", "ogg", "wav", "aac"];

/// Default priority for scan tasks
pub const DEFAULT_TASK_PRIORITY: u8 = 5;

/// High priority for scan tasks
pub const HIGH_TASK_PRIORITY: u8 = 10;

/// Low priority for scan tasks
pub const LOW_TASK_PRIORITY: u8 = 1;

/// Default timeout for spawn_blocking operations
pub const DEFAULT_SPAWN_TIMEOUT: Duration = Duration::from_secs(5);

/// Default timeout for metadata extraction operations
pub const DEFAULT_METADATA_TIMEOUT: Duration = Duration::from_secs(30);

/// Warning threshold for slow operations (as a percentage of timeout)
pub const SLOW_OPERATION_WARNING_THRESHOLD: f32 = 0.5;
