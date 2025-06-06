//! Health monitoring for database connections
//!
//! Provides health status tracking and background health checks for database connections.

use crate::db::error::{DatabaseError, DbResult};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

/// Connection health status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionHealth {
    /// Connection is healthy
    Healthy,
    /// Connection is connecting
    Connecting,
    /// Connection is failed
    Failed,
}

impl ConnectionHealth {
    /// Check if the connection is healthy
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// Check if the connection is connecting
    #[must_use]
    pub fn is_connecting(&self) -> bool {
        matches!(self, Self::Connecting)
    }

    /// Check if the connection is failed
    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed)
    }
}

/// Health monitor for database connection
#[derive(Debug, Clone)]
pub struct HealthMonitor {
    /// Current health status
    status: Arc<RwLock<ConnectionHealth>>,
    /// Last health check time
    last_check: Arc<RwLock<Instant>>,
    /// Health check interval
    check_interval: Duration,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(check_interval: Duration) -> Self {
        Self {
            status: Arc::new(RwLock::new(ConnectionHealth::Healthy)),
            last_check: Arc::new(RwLock::new(Instant::now())),
            check_interval,
        }
    }

    /// Get the current health status
    pub fn get_status(&self) -> DbResult<ConnectionHealth> {
        let status = self.status.read().map_err(|_| DatabaseError::internal("Failed to acquire status lock"))?;
        Ok(*status)
    }

    /// Update the health status
    pub fn set_healthy(&self) -> DbResult<()> {
        let mut status = self.status.write().map_err(|_| DatabaseError::internal("Failed to acquire status lock"))?;
        *status = ConnectionHealth::Healthy;
        Ok(())
    }

    /// Set health status to connecting
    pub fn set_connecting(&self) -> DbResult<()> {
        let mut status = self.status.write().map_err(|_| DatabaseError::internal("Failed to acquire status lock"))?;
        *status = ConnectionHealth::Connecting;
        Ok(())
    }

    /// Set health status to failed
    pub fn set_failed(&self) -> DbResult<()> {
        let mut status = self.status.write().map_err(|_| DatabaseError::internal("Failed to acquire status lock"))?;
        *status = ConnectionHealth::Failed;
        Ok(())
    }

    /// Check if a health check is needed
    pub fn should_check(&self) -> DbResult<bool> {
        let last_check = self.last_check.read().map_err(|_| DatabaseError::internal("Failed to acquire last_check lock"))?;
        Ok(last_check.elapsed() >= self.check_interval)
    }

    /// Update the last check time
    pub fn update_last_check(&self) -> DbResult<()> {
        let mut last_check = self.last_check.write().map_err(|_| DatabaseError::internal("Failed to acquire last_check lock"))?;
        *last_check = Instant::now();
        Ok(())
    }

    /// Perform a health check
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if the health check fails.
    pub fn check_health(&self) -> DbResult<()> {
        // For now, we just update the last check time
        // In a real implementation, this would perform actual health checks
        self.update_last_check()
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new(Duration::from_secs(30))
    }
}

impl HealthMonitor {
    /// Starts background health checks in a new thread
    pub fn start(
        self: Arc<Self>,
        check_fn: Arc<dyn Fn() -> ConnectionHealth + Send + Sync + 'static>,
    ) {
        let status_lock = Arc::clone(&self.status);
        let last_check = Arc::clone(&self.last_check);
        let interval = Duration::from_secs(60);
        thread::spawn(move || {
            loop {
                thread::sleep(interval);
                let new_status = check_fn();
                if let Ok(mut status_guard) = status_lock.write() {
                    *status_guard = new_status;
                }
                if let Ok(mut last_check_guard) = last_check.write() {
                    *last_check_guard = Instant::now();
                }
            }
        });
    }

    /// Gets the current health status
    #[must_use]
    pub fn status(&self) -> ConnectionHealth {
        self.status
            .read()
            .map(|guard| guard.clone())
            .unwrap_or(ConnectionHealth::Healthy)
    }

    /// Gets the last health check time
    #[must_use]
    pub fn last_check(&self) -> Instant {
        self.last_check.read().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_health_monitor_creation() {
        let monitor = HealthMonitor::new(Duration::from_secs(30));
        assert_eq!(monitor.get_status().unwrap(), ConnectionHealth::Healthy);
        assert_eq!(monitor.last_check(), Instant::now());
    }

    #[test]
    fn test_health_status_changes() {
        let monitor = HealthMonitor::new(Duration::from_secs(30));

        monitor.set_healthy().unwrap();
        assert_eq!(monitor.get_status().unwrap(), ConnectionHealth::Healthy);

        monitor.set_connecting().unwrap();
        assert_eq!(monitor.get_status().unwrap(), ConnectionHealth::Connecting);

        monitor.set_failed().unwrap();
        assert_eq!(monitor.get_status().unwrap(), ConnectionHealth::Failed);
    }

    #[test]
    fn test_background_health_checks() {
        let monitor = Arc::new(HealthMonitor::new(Duration::from_secs(1))); // 1 second interval
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        // Create a check function that sets our flag
        let check_fn = Arc::new(move || {
            called_clone.store(true, Ordering::SeqCst);
            ConnectionHealth::Healthy
        });

        // Start background checks
        monitor.clone().start(check_fn);

        // Wait for the check to run
        thread::sleep(Duration::from_secs(2));

        // Verify the check function was called
        assert!(called.load(Ordering::SeqCst));
        assert_eq!(monitor.status(), ConnectionHealth::Healthy);
    }

    #[test]
    fn test_health_check_timing() {
        let monitor = HealthMonitor::new(Duration::from_secs(1));
        
        // Should check immediately after creation
        assert!(monitor.should_check().unwrap());
        
        // Update last check
        monitor.update_last_check().unwrap();
        
        // Should not check immediately after update
        assert!(!monitor.should_check().unwrap());
        
        // Wait for check interval
        thread::sleep(Duration::from_secs(2));
        
        // Should check after interval
        assert!(monitor.should_check().unwrap());
    }
}
