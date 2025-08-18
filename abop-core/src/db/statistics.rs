//! Connection statistics tracking for database operations
//!
//! This module provides functionality for tracking and reporting database
//! connection statistics for monitoring and performance analysis.

use std::sync::{Arc, PoisonError, RwLock};
use std::time::{Duration, Instant};
use thiserror::Error;
use tracing::{debug, warn};

/// Errors that can occur during statistics collection
#[derive(Error, Debug)]
pub enum StatisticsError {
    /// Failed to acquire read lock on statistics
    #[error("Failed to acquire read lock on statistics: {0}")]
    ReadLockFailed(String),

    /// Failed to acquire write lock on statistics
    #[error("Failed to acquire write lock on statistics: {0}")]
    WriteLockFailed(String),

    /// Failed to acquire read lock on connection timestamp
    #[error("Failed to acquire read lock on connection timestamp: {0}")]
    TimestampReadLockFailed(String),

    /// Failed to acquire write lock on connection timestamp
    #[error("Failed to acquire write lock on connection timestamp: {0}")]
    TimestampWriteLockFailed(String),
}

/// Result type for statistics operations
pub type StatisticsResult<T> = std::result::Result<T, StatisticsError>;

/// Connection statistics for monitoring database operations
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    /// Total number of successful operations
    pub successful_operations: u64,
    /// Total number of failed operations  
    pub failed_operations: u64,
    /// Average operation duration in milliseconds
    pub avg_operation_duration_ms: f64,
    /// Last successful operation timestamp
    pub last_successful_operation: Option<Instant>,
    /// Last failed operation timestamp
    pub last_failed_operation: Option<Instant>,
    /// Connection uptime since last successful connection
    pub connection_uptime: Duration,
    /// Number of reconnection attempts
    pub reconnection_attempts: u32,
}

impl Default for ConnectionStats {
    fn default() -> Self {
        Self {
            successful_operations: 0,
            failed_operations: 0,
            avg_operation_duration_ms: 0.0,
            last_successful_operation: None,
            last_failed_operation: None,
            connection_uptime: Duration::ZERO,
            reconnection_attempts: 0,
        }
    }
}

/// Collects and manages statistics for database operations
pub struct StatisticsCollector {
    /// Internal statistics storage
    stats: Arc<RwLock<ConnectionStats>>,
    /// Connection establishment timestamp
    connected_at: Arc<RwLock<Option<Instant>>>,
}

impl Default for StatisticsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl StatisticsCollector {
    /// Create a new statistics collector
    #[must_use]
    pub fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(ConnectionStats::default())),
            connected_at: Arc::new(RwLock::new(None)),
        }
    }
    /// Record a successful connection
    ///
    /// # Errors
    ///
    /// Returns an error if the timestamp lock cannot be acquired
    pub fn record_connection(&self) -> StatisticsResult<()> {
        // Set connection timestamp
        self.connected_at
            .write()
            .map(|mut guard| *guard = Some(Instant::now()))
            .map_err(|e: PoisonError<_>| {
                StatisticsError::TimestampWriteLockFailed(e.to_string())
            })?;

        // Record connection as a successful operation
        self.record_success(Duration::from_millis(0))?;

        Ok(())
    }

    /// Record a successful operation
    ///
    /// # Errors
    ///
    /// Returns an error if the statistics lock cannot be acquired
    pub fn record_success(&self, duration: Duration) -> StatisticsResult<()> {
        let mut stats = self
            .stats
            .write()
            .map_err(|e: PoisonError<_>| StatisticsError::WriteLockFailed(e.to_string()))?;

        stats.successful_operations += 1;
        stats.last_successful_operation = Some(Instant::now());
        Self::update_average_duration(&mut stats, duration);
        drop(stats);
        Ok(())
    }

    /// Record a failed operation
    ///
    /// # Errors
    ///
    /// Returns an error if the statistics lock cannot be acquired
    pub fn record_failure(&self, duration: Duration) -> StatisticsResult<()> {
        let mut stats = self
            .stats
            .write()
            .map_err(|e: PoisonError<_>| StatisticsError::WriteLockFailed(e.to_string()))?;

        stats.failed_operations += 1;
        stats.last_failed_operation = Some(Instant::now());
        Self::update_average_duration(&mut stats, duration);
        drop(stats);
        Ok(())
    }

    /// Record a reconnection attempt
    ///
    /// # Errors
    ///
    /// Returns an error if the statistics lock cannot be acquired
    pub fn record_reconnection_attempt(&self) -> StatisticsResult<()> {
        self.stats
            .write()
            .map(|mut guard| guard.reconnection_attempts += 1)
            .map_err(|e: PoisonError<_>| StatisticsError::WriteLockFailed(e.to_string()))
    }

    /// Update the average operation duration using exponential moving average
    ///
    /// # Precision Notes
    /// - Converts `Duration` to milliseconds as `f64`
    /// - May lose precision for durations > 2^53 milliseconds (≈285,616 years)
    /// - Precision loss is acceptable for statistical purposes
    fn update_average_duration(stats: &mut ConnectionStats, duration: Duration) {
        let duration_ms_u128 = duration.as_millis();

        // Convert to f64 with safe casting and explicit handling of precision loss
        // Cap at u32::MAX (≈49.7 days) to avoid excessive precision loss
        let duration_ms = if duration_ms_u128 > u128::from(u32::MAX) {
            debug!(
                "Capping large duration {}ms to {}ms to avoid precision loss",
                duration_ms_u128,
                u32::MAX
            );
            f64::from(u32::MAX)
        } else {
            // Use safe conversion to avoid direct casting
            // Since we've checked the value is <= u32::MAX, we can safely convert through u32
            let duration_ms_u32 = u32::try_from(duration_ms_u128).unwrap_or_else(|_| {
                warn!(
                    "Duration {} doesn't fit in u32, using max",
                    duration_ms_u128
                );
                u32::MAX
            });
            f64::from(duration_ms_u32)
        };

        if stats.successful_operations + stats.failed_operations == 1 {
            stats.avg_operation_duration_ms = duration_ms;
        } else {
            // Use exponential moving average with alpha = 0.1
            stats.avg_operation_duration_ms =
                0.9f64.mul_add(stats.avg_operation_duration_ms, 0.1 * duration_ms);
        }
    }

    /// Get current statistics
    ///
    /// # Errors
    ///
    /// Returns an error if either lock cannot be acquired
    pub fn get_stats(&self) -> StatisticsResult<ConnectionStats> {
        let mut result = self
            .stats
            .read()
            .map_err(|e: PoisonError<_>| StatisticsError::ReadLockFailed(e.to_string()))?
            .clone();

        // Calculate uptime if connected
        let value = *self
            .connected_at
            .read()
            .map_err(|e: PoisonError<_>| StatisticsError::TimestampReadLockFailed(e.to_string()))?;
        if let Some(connected_at) = value {
            result.connection_uptime = connected_at.elapsed();
        }

        Ok(result)
    }

    /// Get the connection establishment time
    ///
    /// # Errors
    ///
    /// Returns an error if the timestamp lock cannot be acquired
    pub fn connected_at(&self) -> StatisticsResult<Option<Instant>> {
        self.connected_at
            .read()
            .map(|guard| *guard)
            .map_err(|e: PoisonError<_>| StatisticsError::TimestampReadLockFailed(e.to_string()))
    }
}

impl Clone for StatisticsCollector {
    fn clone(&self) -> Self {
        Self {
            stats: self.stats.clone(),
            connected_at: self.connected_at.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_statistics_collector_creation() -> StatisticsResult<()> {
        let collector = StatisticsCollector::new();
        let stats = collector.get_stats()?;

        assert_eq!(stats.successful_operations, 0);
        assert_eq!(stats.failed_operations, 0);
        assert_eq!(stats.reconnection_attempts, 0);
        assert_eq!(stats.avg_operation_duration_ms, 0.0);
        Ok(())
    }

    #[test]
    fn test_record_operations() -> StatisticsResult<()> {
        let collector = StatisticsCollector::new();

        // Record some operations
        collector.record_success(Duration::from_millis(100))?;
        collector.record_success(Duration::from_millis(200))?;
        collector.record_failure(Duration::from_millis(50))?;

        // Verify stats
        let stats = collector.get_stats()?;
        assert_eq!(stats.successful_operations, 2);
        assert_eq!(stats.failed_operations, 1);
        assert!(stats.avg_operation_duration_ms >= 100.0);
        Ok(())
    }

    #[test]
    fn test_connection_tracking() -> StatisticsResult<()> {
        let collector = StatisticsCollector::new();

        // Record connection
        collector.record_connection()?;
        std::thread::sleep(Duration::from_millis(100));

        // Verify stats
        let stats = collector.get_stats()?;
        assert_eq!(stats.successful_operations, 1); // Initial connection
        assert_eq!(stats.failed_operations, 0);
        assert!(stats.avg_operation_duration_ms >= 0.0);

        Ok(())
    }

    #[test]
    fn test_reconnection_attempts() -> StatisticsResult<()> {
        let collector = StatisticsCollector::new();

        // Record some reconnection attempts
        collector.record_reconnection_attempt()?;
        collector.record_reconnection_attempt()?;
        collector.record_success(Duration::from_millis(50))?;

        // Verify stats
        let stats = collector.get_stats()?;
        assert_eq!(stats.successful_operations, 1);
        assert_eq!(stats.failed_operations, 0);
        assert!(stats.avg_operation_duration_ms >= 0.0);

        Ok(())
    }
}
