//! Connection statistics tracking for database operations
//!
//! This module provides functionality for tracking and reporting database
//! connection statistics for monitoring and performance analysis.

use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

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

/// Statistics collector for database operations
#[derive(Debug)]
pub struct StatisticsCollector {
    /// Internal statistics storage
    stats: Arc<RwLock<ConnectionStats>>,
    /// Connection establishment timestamp
    connected_at: Arc<RwLock<Option<Instant>>>,
    /// Total number of successful operations
    successful_operations: Arc<RwLock<usize>>,
    /// Total number of failed operations
    failed_operations: Arc<RwLock<usize>>,
    /// Total number of reconnection attempts
    reconnection_attempts: Arc<RwLock<usize>>,
    /// Total number of connections established
    connections_established: Arc<RwLock<usize>>,
    /// Total time spent in successful operations (in nanoseconds)
    total_success_time_ns: Arc<RwLock<u128>>,
    /// Total time spent in failed operations (in nanoseconds)
    total_failure_time_ns: Arc<RwLock<u128>>,
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
            successful_operations: Arc::new(RwLock::new(0)),
            failed_operations: Arc::new(RwLock::new(0)),
            reconnection_attempts: Arc::new(RwLock::new(0)),
            connections_established: Arc::new(RwLock::new(0)),
            total_success_time_ns: Arc::new(RwLock::new(0)),
            total_failure_time_ns: Arc::new(RwLock::new(0)),
        }
    }

    /// Record a successful connection
    ///
    /// # Panics
    ///
    /// Panics if the `connected_at` `RwLock` is poisoned due to a panic in another thread.
    pub fn record_connection(&self) {
        *self.connected_at.write().unwrap() = Some(Instant::now());
        *self.connections_established.write().unwrap() += 1;
    }

    /// Record a successful operation
    ///
    /// # Panics
    ///
    /// Panics if the statistics `RwLock` is poisoned due to a panic in another thread.
    pub fn record_success(&self, duration: Duration) {
        let mut stats = self.stats.write().unwrap();
        stats.successful_operations += 1;
        stats.last_successful_operation = Some(Instant::now());
        Self::update_average_duration(&mut stats, duration);
        drop(stats);

        *self.successful_operations.write().unwrap() += 1;
        *self.total_success_time_ns.write().unwrap() += duration.as_nanos();
    }

    /// Record a failed operation
    ///
    /// # Panics
    ///
    /// Panics if the statistics `RwLock` is poisoned due to a panic in another thread.
    pub fn record_failure(&self, duration: Duration) {
        let mut stats = self.stats.write().unwrap();
        stats.failed_operations += 1;
        stats.last_failed_operation = Some(Instant::now());
        Self::update_average_duration(&mut stats, duration);
        drop(stats);

        *self.failed_operations.write().unwrap() += 1;
        *self.total_failure_time_ns.write().unwrap() += duration.as_nanos();
    }

    /// Record a reconnection attempt
    ///
    /// # Panics
    ///
    /// Panics if the statistics `RwLock` is poisoned due to a panic in another thread.
    pub fn record_reconnection_attempt(&self) {
        self.stats.write().unwrap().reconnection_attempts += 1;
        *self.reconnection_attempts.write().unwrap() += 1;
    }

    /// Update the average operation duration using exponential moving average
    ///
    /// # Precision Notes
    /// - Converts `Duration` to milliseconds as `f64`
    /// - May lose precision for durations > 2^53 milliseconds (≈285,616 years)
    /// - Precision loss is acceptable for statistical purposes
    fn update_average_duration(stats: &mut ConnectionStats, duration: Duration) {
        let duration_ms_u128 = duration.as_millis();

        // Convert to f64 with explicit handling of precision loss
        // Cap at u32::MAX (≈49.7 days) to avoid excessive precision loss
        let duration_ms = if duration_ms_u128 > u128::from(u32::MAX) {
            log::debug!(
                "Capping large duration {}ms to {}ms to avoid precision loss",
                duration_ms_u128,
                u32::MAX
            );
            f64::from(u32::MAX)
        } else {
            #[allow(clippy::cast_precision_loss)] // Intentional and documented
            let ms = duration_ms_u128 as f64;
            ms
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
    /// # Panics
    ///
    /// Panics if the statistics or `connected_at` `RwLocks` are poisoned due to a panic in another thread.
    #[must_use]
    pub fn get_stats(&self) -> ConnectionStats {
        let mut result = self.stats.read().unwrap().clone();

        // Calculate uptime if connected
        let value = *self.connected_at.read().unwrap();
        if let Some(connected_at) = value {
            result.connection_uptime = connected_at.elapsed();
        }

        result
    }

    /// Get the connection establishment time
    ///
    /// # Panics
    ///
    /// Panics if the `connected_at` `RwLock` is poisoned due to a panic in another thread.
    #[must_use]
    pub fn connected_at(&self) -> Option<Instant> {
        *self.connected_at.read().unwrap()
    }
}

impl Clone for StatisticsCollector {
    fn clone(&self) -> Self {
        Self {
            stats: Arc::clone(&self.stats),
            connected_at: Arc::clone(&self.connected_at),
            successful_operations: Arc::clone(&self.successful_operations),
            failed_operations: Arc::clone(&self.failed_operations),
            reconnection_attempts: Arc::clone(&self.reconnection_attempts),
            connections_established: Arc::clone(&self.connections_established),
            total_success_time_ns: Arc::clone(&self.total_success_time_ns),
            total_failure_time_ns: Arc::clone(&self.total_failure_time_ns),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_statistics_collector_creation() {
        let collector = StatisticsCollector::new();
        let stats = collector.get_stats();

        assert_eq!(stats.successful_operations, 0);
        assert_eq!(stats.failed_operations, 0);
        assert_eq!(stats.reconnection_attempts, 0);
        assert_eq!(stats.avg_operation_duration_ms, 0.0);
    }

    #[test]
    fn test_record_operations() {
        let collector = StatisticsCollector::new();

        // Record a successful operation
        collector.record_success(Duration::from_millis(100));

        // Record a failed operation
        collector.record_failure(Duration::from_millis(200));

        // Record a reconnection attempt
        collector.record_reconnection_attempt();

        let stats = collector.get_stats();
        assert_eq!(stats.successful_operations, 1);
        assert_eq!(stats.failed_operations, 1);
        assert_eq!(stats.reconnection_attempts, 1);

        // Using EMA formula: first value is 100, second is 0.9*100 + 0.1*200 = 110
        assert_eq!(stats.avg_operation_duration_ms, 110.0);
    }

    #[test]
    fn test_connection_uptime() {
        let collector = StatisticsCollector::new();

        // Record connection
        collector.record_connection();

        // Wait a bit
        thread::sleep(Duration::from_millis(10));

        // Check uptime
        let stats = collector.get_stats();
        assert!(stats.connection_uptime.as_millis() >= 10);
    }

    #[test]
    fn test_exponential_moving_average() {
        let collector = StatisticsCollector::new();

        // First operation sets the baseline
        collector.record_success(Duration::from_millis(100));
        let stats1 = collector.get_stats();
        assert_eq!(stats1.avg_operation_duration_ms, 100.0);

        // Second operation uses EMA formula: 0.9 * 100 + 0.1 * 200 = 90 + 20 = 110
        collector.record_success(Duration::from_millis(200));
        let stats2 = collector.get_stats();
        assert_eq!(stats2.avg_operation_duration_ms, 110.0);

        // Third operation: 0.9 * 110 + 0.1 * 300 = 99 + 30 = 129
        collector.record_success(Duration::from_millis(300));
        let stats3 = collector.get_stats();
        assert_eq!(stats3.avg_operation_duration_ms, 129.0);
    }
}
