//! Retry logic for database operations
//!
//! This module provides configurable retry policies and execution mechanisms
//! for handling transient database failures with exponential backoff.

use crate::db::error::{DatabaseError, DbResult};
use std::thread;
use std::time::Duration;

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_retry_attempts: u32,
    /// Initial retry delay in milliseconds
    pub initial_retry_delay_ms: u64,
    /// Maximum retry delay in milliseconds
    pub max_retry_delay_ms: u64,
    /// Whether to add jitter to retry delays
    pub use_jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retry_attempts: 3,
            initial_retry_delay_ms: 100,
            max_retry_delay_ms: 5000,
            use_jitter: true,
        }
    }
}

impl RetryPolicy {
    /// Create a new retry policy with custom settings
    #[must_use]
    pub const fn new(max_attempts: u32, initial_delay_ms: u64, max_delay_ms: u64) -> Self {
        Self {
            max_retry_attempts: max_attempts,
            initial_retry_delay_ms: initial_delay_ms,
            max_retry_delay_ms: max_delay_ms,
            use_jitter: true,
        }
    }

    /// Calculate the delay for a specific attempt
    #[must_use]
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::from_millis(0);
        }

        // Calculate exponential backoff with overflow protection
        let base_delay = self
            .initial_retry_delay_ms
            .saturating_mul(2u64.saturating_pow(attempt - 1));
        let capped_delay = std::cmp::min(base_delay, self.max_retry_delay_ms);

        // Add jitter if enabled (±10%)
        if self.use_jitter {
            // Generate a jitter factor between 0.9 and 1.1
            let jitter_factor = rand::random::<f64>().mul_add(0.2, 0.9);

            // For retry scenarios, capping at u32::MAX (~49 days) is reasonable since:
            // 1. Most retry operations should succeed within minutes/hours
            // 2. Delays exceeding 49 days indicate a fundamental system issue
            // 3. This preserves precision in f64 calculations while being practical
            const MAX_REASONABLE_RETRY_DELAY_MS: u64 = u32::MAX as u64;
            let delay_for_jitter = std::cmp::min(capped_delay, MAX_REASONABLE_RETRY_DELAY_MS);

            // Convert to f64 for jitter calculation
            // Safe due to the cap above - u32::MAX can be exactly represented in f64
            #[allow(clippy::cast_precision_loss)] // Intentional and documented
            let delay_f64 = delay_for_jitter as f64 * jitter_factor;

            // Safely convert back to u64 with bounds checking
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let jittered_delay = if delay_f64 < 0.0 {
                0
            } else if delay_f64 > f64::from(u32::MAX) {
                MAX_REASONABLE_RETRY_DELAY_MS
            } else {
                delay_f64.round() as u64
            };

            Duration::from_millis(jittered_delay)
        } else {
            Duration::from_millis(capped_delay)
        }
    }
}

/// Executes operations with retry logic
pub struct RetryExecutor {
    /// Retry policy configuration
    policy: RetryPolicy,
}

impl RetryExecutor {
    /// Create a new retry executor with the specified policy
    #[must_use]
    pub const fn new(policy: RetryPolicy) -> Self {
        Self { policy }
    }

    /// Execute an operation with retry logic
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The operation fails after all retry attempts are exhausted
    /// - The error is not retryable (e.g., permanent failures)
    pub fn execute<F, R>(&self, operation: F) -> DbResult<R>
    where
        F: Fn() -> DbResult<R>,
    {
        let mut attempt = 0;

        loop {
            // Apply delay for retry attempts (not on first attempt)
            if attempt > 0 {
                let delay = self.policy.calculate_delay(attempt);
                log::debug!(
                    "Retry attempt {} of {}, waiting {:?}",
                    attempt,
                    self.policy.max_retry_attempts,
                    delay
                );
                thread::sleep(delay);
            }

            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => {
                    // Check if we should retry based on error type
                    if Self::is_retryable_error(&error) && attempt < self.policy.max_retry_attempts
                    {
                        log::warn!(
                            "Operation failed with retryable error (attempt {}/{}): {}",
                            attempt + 1,
                            self.policy.max_retry_attempts + 1,
                            error
                        );
                        attempt += 1;
                        continue;
                    }
                    // Either max attempts reached or non-retryable error
                    if attempt > 0 {
                        return Err(DatabaseError::ConnectionFailed(format!(
                            "Failed after {} attempts: {}",
                            attempt + 1,
                            error
                        )));
                    }
                    return Err(error);
                }
            }
        }
    }

    /// Determine if an error is retryable
    const fn is_retryable_error(error: &DatabaseError) -> bool {
        match error {
            DatabaseError::ConnectionFailed(_) | DatabaseError::LockTimeout { .. } => true,
            DatabaseError::Sqlite(rusqlite_err) => {
                // Retry on busy or locked errors
                matches!(
                    rusqlite_err,
                    rusqlite::Error::SqliteFailure(
                        rusqlite::ffi::Error {
                            code: rusqlite::ffi::ErrorCode::DatabaseBusy
                                | rusqlite::ffi::ErrorCode::DatabaseLocked,
                            ..
                        },
                        _
                    )
                )
            }
            _ => false,
        }
    }
}

impl Default for RetryExecutor {
    /// Create a new retry executor with default policy
    fn default() -> Self {
        Self {
            policy: RetryPolicy::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_retry_policy_delay_calculation() {
        let policy = RetryPolicy::new(3, 100, 1000);

        // First attempt has no delay
        assert_eq!(policy.calculate_delay(0), Duration::from_millis(0));

        // Subsequent attempts follow exponential backoff
        // Without jitter, would be exactly 100ms
        let delay1 = policy.calculate_delay(1);
        assert!(delay1.as_millis() >= 90 && delay1.as_millis() <= 110);

        // Without jitter, would be exactly 200ms
        let delay2 = policy.calculate_delay(2);
        assert!(delay2.as_millis() >= 180 && delay2.as_millis() <= 220);

        // Without jitter, would be exactly 400ms
        let delay3 = policy.calculate_delay(3);
        assert!(delay3.as_millis() >= 360 && delay3.as_millis() <= 440);
    }

    #[test]
    fn test_retry_policy_max_delay() {
        let policy = RetryPolicy::new(5, 100, 300);

        // Third attempt would be 400ms without cap, but is capped at 300ms
        let delay = policy.calculate_delay(3);
        // With jitter, should be between 270-330ms
        assert!(delay.as_millis() >= 270 && delay.as_millis() <= 330);
    }

    #[test]
    fn test_retry_executor_success_first_try() {
        let executor = RetryExecutor::default();

        let result = executor.execute(|| Ok::<_, DatabaseError>(42));

        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_retry_executor_success_after_retries() {
        let executor = RetryExecutor::new(RetryPolicy {
            max_retry_attempts: 3,
            initial_retry_delay_ms: 10, // Small delay for tests
            max_retry_delay_ms: 50,
            use_jitter: false, // Disable jitter for predictable tests
        });

        let attempt_counter = Rc::new(RefCell::new(0));
        let counter_clone = attempt_counter.clone();

        let result = executor.execute(move || {
            let mut counter = counter_clone.borrow_mut();
            *counter += 1;

            if *counter < 3 {
                Err(DatabaseError::ConnectionFailed(
                    "Simulated failure".to_string(),
                ))
            } else {
                Ok(42)
            }
        });

        assert_eq!(result.unwrap(), 42);
        assert_eq!(*attempt_counter.borrow(), 3);
    }

    #[test]
    fn test_retry_executor_failure_after_max_attempts() {
        let executor = RetryExecutor::new(RetryPolicy {
            max_retry_attempts: 2,
            initial_retry_delay_ms: 10,
            max_retry_delay_ms: 50,
            use_jitter: false,
        });

        let attempt_counter = Rc::new(RefCell::new(0));
        let counter_clone = attempt_counter.clone();

        let result: DbResult<i32> = executor.execute(move || {
            let mut counter = counter_clone.borrow_mut();
            *counter += 1;

            Err(DatabaseError::ConnectionFailed("Always fails".to_string()))
        });

        assert!(result.is_err());
        assert_eq!(*attempt_counter.borrow(), 3); // Initial + 2 retries
    }

    #[test]
    fn test_non_retryable_errors_dont_retry() {
        let executor = RetryExecutor::default();

        let attempt_counter = Rc::new(RefCell::new(0));
        let counter_clone = attempt_counter.clone();

        let result: DbResult<i32> = executor.execute(move || {
            let mut counter = counter_clone.borrow_mut();
            *counter += 1;

            Err(DatabaseError::ValidationFailed {
                field: "test".to_string(),
                message: "Non-retryable error".to_string(),
            })
        });

        assert!(result.is_err());
        assert_eq!(*attempt_counter.borrow(), 1); // Only tried once
    }

    #[test]
    fn test_jitter_delay_cap_behavior() {
        // Test the behavior of jitter delay capping at u32::MAX
        let policy = RetryPolicy::new(10, u64::MAX / 2, u64::MAX);

        // Calculate delay for a high attempt that would exceed u32::MAX without cap
        let delay = policy.calculate_delay(5);

        // Should be capped to u32::MAX milliseconds
        assert!(delay.as_millis() <= u32::MAX as u128);

        // With jitter, should be between 90% and 110% of the capped value
        let max_reasonable_ms = u32::MAX as u128;
        assert!(delay.as_millis() >= (max_reasonable_ms * 9) / 10);
        assert!(delay.as_millis() <= (max_reasonable_ms * 11) / 10);
    }

    #[test]
    fn test_jitter_disabled_produces_exact_delays() {
        let mut policy = RetryPolicy::new(3, 100, 1000);
        policy.use_jitter = false;

        // Without jitter, delays should be exactly the exponential backoff values
        assert_eq!(policy.calculate_delay(0), Duration::from_millis(0));
        assert_eq!(policy.calculate_delay(1), Duration::from_millis(100));
        assert_eq!(policy.calculate_delay(2), Duration::from_millis(200));
        assert_eq!(policy.calculate_delay(3), Duration::from_millis(400));
    }

    #[test]
    fn test_jitter_range_validation() {
        let policy = RetryPolicy::new(3, 1000, 10000);

        // Run multiple iterations to validate jitter range (±10%)
        for _i in 0..50 {
            let delay = policy.calculate_delay(1);
            let delay_ms = delay.as_millis();

            // Expected base delay is 1000ms, jitter should be ±10%
            assert!(delay_ms >= 900, "Delay {delay_ms}ms is below 900ms minimum");
            assert!(
                delay_ms <= 1100,
                "Delay {delay_ms}ms is above 1100ms maximum"
            );
        }
    }

    #[test]
    fn test_extreme_delay_values() {
        // Test with very large initial delays
        let policy = RetryPolicy::new(3, u32::MAX as u64, u64::MAX);

        let delay = policy.calculate_delay(1);

        // Should handle large values without panicking
        // Should be capped at u32::MAX due to our jitter cap
        assert!(delay.as_millis() <= u32::MAX as u128);
        assert!(delay.as_millis() > 0);
    }

    #[test]
    fn test_zero_delay_handling() {
        let policy = RetryPolicy::new(3, 0, 1000);

        // Zero initial delay should remain zero even with jitter
        let delay = policy.calculate_delay(1);
        assert_eq!(delay, Duration::from_millis(0));
    }

    #[test]
    fn test_delay_cap_rationale() {
        // Test that demonstrates why u32::MAX cap is reasonable for retry scenarios
        let policy = RetryPolicy::new(50, 1000, u64::MAX);

        // Even with a very high attempt number, delay should be reasonable
        let delay = policy.calculate_delay(30);

        // Should be capped to prevent unreasonably long delays
        let max_reasonable_days = u32::MAX as f64 / (1000.0 * 60.0 * 60.0 * 24.0);
        assert!(
            max_reasonable_days < 50.0,
            "Cap allows delays up to ~49 days, which is reasonable for retry scenarios"
        );

        // Actual delay should respect the cap
        assert!(delay.as_millis() <= u32::MAX as u128);
    }
}
