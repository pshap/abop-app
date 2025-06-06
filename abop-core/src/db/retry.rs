//! Retry logic for database operations
//!
//! This module provides configurable retry policies and execution mechanisms
//! for handling transient database failures with exponential backoff.

use crate::db::error::DatabaseError;
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tokio::time;
use tokio::runtime::Runtime;

/// Retry policy configuration
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: u64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    /// Exponential backoff factor
    pub backoff_factor: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_factor: 2.0,
        }
    }
}

/// Executor for retrying operations with configurable policy
#[derive(Debug)]
pub struct RetryExecutor {
    /// Retry policy configuration
    policy: RetryPolicy,
    runtime: Arc<Runtime>,
}

impl RetryExecutor {
    /// Create a new retry executor with the given policy
    #[must_use]
    pub fn new(policy: RetryPolicy) -> Self {
        Self {
            policy,
            runtime: Arc::new(Runtime::new().unwrap()),
        }
    }

    /// Execute an operation with retry logic
    pub async fn execute_async<F, Fut, T>(&self, operation: F) -> Result<T, DatabaseError>
    where
        F: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, DatabaseError>> + Send + 'static,
        T: Send + 'static,
    {
        let mut attempt = 0;
        let mut delay = self.policy.initial_delay_ms;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;
                    if attempt >= self.policy.max_attempts {
                        return Err(e);
                    }

                    // Check if error is retryable
                    if !self.should_retry(&e) {
                        return Err(e);
                    }

                    // Calculate next delay with exponential backoff
                    delay = (delay as f64 * self.policy.backoff_factor) as u64;
                    delay = delay.min(self.policy.max_delay_ms);

                    // Wait before retrying
                    time::sleep(Duration::from_millis(delay)).await;
                }
            }
        }
    }

    /// Execute an operation with retry logic in a blocking manner
    pub fn execute<F, T>(&self, operation: F) -> Result<T, DatabaseError>
    where
        F: Fn() -> Result<T, DatabaseError> + Send + 'static,
        T: Send + 'static,
    {
        self.runtime.block_on(async {
            let mut attempt = 0;
            let mut delay = self.policy.initial_delay_ms;

            loop {
                match operation() {
                    Ok(result) => return Ok(result),
                    Err(e) => {
                        attempt += 1;
                        if attempt >= self.policy.max_attempts {
                            return Err(e);
                        }

                        // Check if error is retryable
                        if !self.should_retry(&e) {
                            return Err(e);
                        }

                        // Calculate next delay with exponential backoff
                        delay = (delay as f64 * self.policy.backoff_factor) as u64;
                        delay = delay.min(self.policy.max_delay_ms);

                        // Wait before retrying
                        time::sleep(Duration::from_millis(delay)).await;
                    }
                }
            }
        })
    }

    fn should_retry(&self, error: &DatabaseError) -> bool {
        match error {
            &DatabaseError::Sqlite(ref e) => match e {
                rusqlite::Error::SqliteFailure(err, _) => {
                    err.code == rusqlite::ErrorCode::DatabaseBusy
                        || err.code == rusqlite::ErrorCode::DatabaseLocked
                }
                _ => false,
            },
            &DatabaseError::Query(_) => true,
            &DatabaseError::ConnectionError { .. } | &DatabaseError::Internal { .. } => false,
            &DatabaseError::ConnectionFailed(_) => true,
            &DatabaseError::TransactionFailed { .. } => true,
            &DatabaseError::LockTimeout { .. } => true,
            &DatabaseError::QueryPreparationFailed { .. } => true,
            &DatabaseError::ExecutionFailed { .. } => true,
            &DatabaseError::MigrationFailed { .. } => false,
            &DatabaseError::RecordNotFound { .. } => false,
            &DatabaseError::ConstraintViolation { .. } => false,
            &DatabaseError::ValidationFailed { .. } => false,
            &DatabaseError::DuplicateEntry { .. } => false,
            &DatabaseError::SchemaMismatch { .. } => false,
        }
    }
}

impl Clone for RetryExecutor {
    fn clone(&self) -> Self {
        Self {
            policy: self.policy.clone(),
            runtime: Arc::clone(&self.runtime),
        }
    }
}

/// Check if an error is retryable
fn is_retryable_error(error: &DatabaseError) -> bool {
    match error {
        DatabaseError::ConnectionFailed(_) => true,
        DatabaseError::Sqlite(_) => true,
        DatabaseError::TransactionFailed { .. } => true,
        DatabaseError::MigrationFailed { .. } => false,
        DatabaseError::RecordNotFound { .. } => false,
        DatabaseError::ConstraintViolation { .. } => false,
        DatabaseError::ValidationFailed { .. } => false,
        DatabaseError::DuplicateEntry { .. } => false,
        DatabaseError::SchemaMismatch { .. } => false,
        DatabaseError::LockTimeout { .. } => true,
        DatabaseError::QueryPreparationFailed { .. } => true,
        DatabaseError::ExecutionFailed { .. } => true,
        DatabaseError::Query(_) => true,
        DatabaseError::ConnectionError { .. } => false,
        DatabaseError::Internal { .. } => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;    #[tokio::test]
    async fn test_retry_success() {
        let policy = RetryPolicy::default();
        let executor = RetryExecutor::new(policy);
        let counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

        let result = executor
            .execute_async(|| {
                let counter = Arc::clone(&counter);
                async move {
                    let count = counter.fetch_add(1, Ordering::SeqCst);
                    if count < 2 {
                        Err(DatabaseError::ConnectionFailed("test".into()))
                    } else {
                        Ok(())
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_failure() {        let policy = RetryPolicy::default();
        let executor = RetryExecutor::new(policy);
        let counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

        let result = executor
            .execute_async(|| {
                let counter = Arc::clone(&counter);
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Err(DatabaseError::ConnectionFailed("test".into()))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_no_retry_on_non_retryable_error() {
        let policy = RetryPolicy::default();        let executor = RetryExecutor::new(policy);
        let counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

        let result = executor
            .execute_async(|| {
                let counter = Arc::clone(&counter);
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Err(DatabaseError::ValidationFailed {
                        field: "test".into(),
                        message: "test".into(),
                    })
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]    fn test_blocking_retry_success() {
        let policy = RetryPolicy::default();
        let executor = RetryExecutor::new(policy);
        let counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

        let result = executor.execute(|| {
            let count = counter.fetch_add(1, Ordering::SeqCst);
            if count < 2 {
                Err(DatabaseError::ConnectionFailed("test".into()))
            } else {
                Ok(())
            }
        });

        assert!(result.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_blocking_retry_failure() {
        let policy = RetryPolicy::default();
        let executor = RetryExecutor::new(policy);
        let counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

        let result = executor.execute(|| {
            counter.fetch_add(1, Ordering::SeqCst);
            Err(DatabaseError::ConnectionFailed("test".into()))
        });

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
}
