//! Configuration for the library scanner

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::constants::*;

/// Configuration for the library scanner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    /// Maximum number of concurrent file operations
    #[serde(default = "default_concurrency")]
    pub max_concurrent_tasks: usize,

    /// Number of items to process before committing to database
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    /// Maximum time to wait for operations to complete
    #[serde(default = "default_timeout", with = "humantime_serde::option")]
    pub timeout: Option<Duration>,

    /// Whether to use memory-mapped I/O where possible
    #[serde(default = "default_true")]
    pub use_mmap: bool,

    /// File extensions to include in the scan (without leading .)
    #[serde(default = "default_extensions")]
    pub extensions: Vec<String>,

    /// Maximum file size to process (in bytes)
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: default_concurrency(),
            batch_size: default_batch_size(),
            timeout: default_timeout(),
            use_mmap: true,
            extensions: default_extensions(),
            max_file_size: default_max_file_size(),
        }
    }
}

fn default_concurrency() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(DEFAULT_CONCURRENCY)
}

const fn default_batch_size() -> usize {
    DEFAULT_BATCH_SIZE
}

const fn default_timeout() -> Option<Duration> {
    Some(DEFAULT_TIMEOUT)
}

const fn default_true() -> bool {
    true
}

const fn default_max_file_size() -> u64 {
    DEFAULT_MAX_FILE_SIZE
}

fn default_extensions() -> Vec<String> {
    SUPPORTED_AUDIO_EXTENSIONS
        .iter()
        .map(|&s| s.to_string())
        .collect()
}

impl ScannerConfig {
    /// Creates a configuration optimized for large libraries
    pub fn for_large_libraries() -> Self {
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get() * 2)
            .unwrap_or(DEFAULT_CONCURRENCY * 2);

        Self {
            max_concurrent_tasks: cpu_count,
            batch_size: DEFAULT_BATCH_SIZE * 2,
            timeout: Some(DEFAULT_TIMEOUT * 2),
            use_mmap: true,
            extensions: default_extensions(),
            max_file_size: DEFAULT_MAX_FILE_SIZE * 2,
        }
    }

    /// Creates a configuration optimized for small libraries
    pub fn for_small_libraries() -> Self {
        Self {
            max_concurrent_tasks: DEFAULT_CONCURRENCY / 2,
            batch_size: DEFAULT_BATCH_SIZE / 2,
            timeout: Some(DEFAULT_TIMEOUT / 2),
            use_mmap: true,
            extensions: default_extensions(),
            max_file_size: DEFAULT_MAX_FILE_SIZE,
        }
    }

    /// Creates a conservative configuration for resource-constrained environments
    pub fn conservative() -> Self {
        Self {
            max_concurrent_tasks: 2,
            batch_size: DEFAULT_BATCH_SIZE / 4,
            timeout: Some(DEFAULT_TIMEOUT / 2),
            use_mmap: false,
            extensions: default_extensions(),
            max_file_size: DEFAULT_MAX_FILE_SIZE / 2,
        }
    }
}
