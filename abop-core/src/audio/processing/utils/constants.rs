//! Common audio processing constants

/// Default processing buffer size in samples
pub const DEFAULT_BUFFER_SIZE: usize = 4096;

/// Default processing timeout in milliseconds
pub const DEFAULT_TIMEOUT_MS: u64 = 30_000; // 30 seconds

/// Default number of parallel processing threads
pub const DEFAULT_PARALLEL_THREADS: usize = 4;

/// Silence threshold in dB
pub const DEFAULT_SILENCE_THRESHOLD_DB: f32 = -60.0;

/// Default target loudness in LUFS
pub const DEFAULT_TARGET_LUFS: f32 = -23.0;

/// Maximum allowed audio file size in bytes (500MB)
pub const MAX_AUDIO_FILE_SIZE: usize = 500 * 1024 * 1024;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_BUFFER_SIZE, 4096);
        assert_eq!(DEFAULT_TIMEOUT_MS, 30_000);
        assert_eq!(DEFAULT_PARALLEL_THREADS, 4);
        assert_eq!(DEFAULT_SILENCE_THRESHOLD_DB, -60.0);
        assert_eq!(DEFAULT_TARGET_LUFS, -23.0);
        assert_eq!(MAX_AUDIO_FILE_SIZE, 500 * 1024 * 1024);
    }
}
