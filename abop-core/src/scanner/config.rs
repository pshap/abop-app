//! Configuration for the library scanner

use serde::{Serialize, Deserialize};
use std::time::Duration;

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
    #[serde(
        default = "default_timeout",
        with = "humantime_serde::option"
    )]
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
        .unwrap_or(8)
}

const fn default_batch_size() -> usize { 100 }
const fn default_timeout() -> Option<Duration> { Some(Duration::from_secs(30)) }
const fn default_true() -> bool { true }
const fn default_max_file_size() -> u64 { 1024 * 1024 * 1024 } // 1GB

fn default_extensions() -> Vec<String> {
    vec![
        "mp3".into(),
        "m4a".into(),
        "m4b".into(),
        "flac".into(),
        "ogg".into(),
        "wav".into(),
        "aac".into(),
    ]
}
