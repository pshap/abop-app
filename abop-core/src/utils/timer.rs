//! Unified timer utility for performance monitoring and processing timeouts

use std::time::{Duration, Instant};

use crate::error::AppError;

/// Unified timer for measuring performance and handling timeouts
/// Replaces both `ProcessingTimer` and `PerformanceTimer` with a single, efficient implementation
#[derive(Debug, Clone)]
pub struct Timer {
    start_time: Instant,
    label: String,
}

impl Timer {
    /// Create a new timer with a label
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            start_time: Instant::now(),
            label: label.into(),
        }
    }

    /// Create a timer with default label
    #[must_use]
    pub fn start() -> Self {
        Self::new("Timer")
    }

    /// Get elapsed time
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get elapsed time in milliseconds
    ///
    /// # Returns
    /// - `u64`: Elapsed time in milliseconds, capped at `u64::MAX`
    #[must_use]
    pub fn elapsed_ms(&self) -> u64 {
        let elapsed = self.elapsed().as_millis();
        // Cap at u64::MAX to avoid truncation
        match u64::try_from(elapsed) {
            Ok(ms) => ms,
            Err(_) => u64::MAX,
        }
    }

    /// Get the timer label
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Check if timer has exceeded timeout
    #[must_use]
    pub fn has_timeout(&self, timeout: Duration) -> bool {
        self.elapsed() > timeout
    }

    /// Check timeout and return Result
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The elapsed time exceeds the specified timeout
    pub fn check_timeout(&self, timeout_ms: u64) -> Result<(), AppError> {
        let elapsed = self.elapsed_ms();
        if elapsed > timeout_ms {
            Err(AppError::Other(format!(
                "Operation '{}' timed out after {}ms (limit: {}ms)",
                self.label, elapsed, timeout_ms
            )))
        } else {
            Ok(())
        }
    }

    /// Stop timer and return elapsed duration
    #[must_use]
    pub fn stop(self) -> Duration {
        self.elapsed()
    }

    /// Log the elapsed time with the timer label
    pub fn log_elapsed(&self) {
        log::debug!("{} took {:.2?}", self.label, self.elapsed());
    }

    /// Log and consume the timer
    pub fn log_and_stop(self) {
        self.log_elapsed();
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.log_elapsed();
    }
}
