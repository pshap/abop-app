//! Health monitoring for database connections
//!
//! Provides health status tracking and background health checks for database connections.

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

/// Connection health status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionHealth {
    /// Connection is healthy and ready for use
    Healthy,
    /// Connection is experiencing issues but may recover
    Degraded,
    /// Connection is failed and needs reconnection
    Failed,
    /// Connection is in the process of being established
    Connecting,
}

/// Monitors the health of a database connection in the background
pub struct HealthMonitor {
    health: Arc<RwLock<ConnectionHealth>>,
    check_interval: Duration,
    last_check: Arc<RwLock<Option<Instant>>>,
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthMonitor {
    /// Create a new health monitor with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            health: Arc::new(RwLock::new(ConnectionHealth::Failed)),
            check_interval: Duration::from_secs(60),
            last_check: Arc::new(RwLock::new(None)),
        }
    }

    /// Create a new health monitor with custom settings
    #[must_use]
    pub fn with_config(check_interval_secs: u64) -> Self {
        Self {
            health: Arc::new(RwLock::new(ConnectionHealth::Failed)),
            check_interval: Duration::from_secs(check_interval_secs),
            last_check: Arc::new(RwLock::new(None)),
        }
    }

    /// Starts background health checks in a new thread
    pub fn start(
        self: Arc<Self>,
        check_fn: Arc<dyn Fn() -> ConnectionHealth + Send + Sync + 'static>,
    ) {
        let health = Arc::clone(&self.health);
        let last_check = Arc::clone(&self.last_check);
        let interval = self.check_interval;
        thread::spawn(move || {
            loop {
                thread::sleep(interval);
                let status = check_fn();
                if let Ok(mut health_guard) = health.write() {
                    *health_guard = status.clone();
                }
                if let Ok(mut last_check_guard) = last_check.write() {
                    *last_check_guard = Some(Instant::now());
                }
            }
        });
    }

    /// Gets the current health status
    #[must_use]
    pub fn status(&self) -> ConnectionHealth {
        self.health
            .read()
            .map(|guard| guard.clone())
            .unwrap_or(ConnectionHealth::Failed)
    }

    /// Gets the last health check time
    #[must_use]
    pub fn last_check(&self) -> Option<Instant> {
        self.last_check.read().map(|guard| *guard).unwrap_or(None)
    }

    /// Set health status to healthy
    pub fn set_healthy(&self) {
        if let Ok(mut health_guard) = self.health.write() {
            *health_guard = ConnectionHealth::Healthy;
        }
        if let Ok(mut last_check_guard) = self.last_check.write() {
            *last_check_guard = Some(Instant::now());
        }
    }

    /// Set health status to degraded
    pub fn set_degraded(&self) {
        if let Ok(mut health_guard) = self.health.write() {
            *health_guard = ConnectionHealth::Degraded;
        }
        if let Ok(mut last_check_guard) = self.last_check.write() {
            *last_check_guard = Some(Instant::now());
        }
    }

    /// Set health status to failed
    pub fn set_failed(&self) {
        if let Ok(mut health_guard) = self.health.write() {
            *health_guard = ConnectionHealth::Failed;
        }
        if let Ok(mut last_check_guard) = self.last_check.write() {
            *last_check_guard = Some(Instant::now());
        }
    }

    /// Set health status to connecting
    ///
    /// # Errors
    ///
    /// Returns an error if either lock is poisoned.
    pub fn set_connecting(&self) -> Result<(), String> {
        if let Err(e) = self
            .health
            .write()
            .map(|mut guard| *guard = ConnectionHealth::Connecting)
        {
            return Err(format!("Failed to update health status: {e}"));
        }

        if let Err(e) = self
            .last_check
            .write()
            .map(|mut guard| *guard = Some(Instant::now()))
        {
            return Err(format!("Failed to update last check time: {e}"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_health_monitor_creation() {
        let monitor = HealthMonitor::new();
        assert_eq!(monitor.status(), ConnectionHealth::Failed);
        assert_eq!(monitor.last_check(), None);
    }

    #[test]
    fn test_health_status_changes() {
        let monitor = HealthMonitor::new();

        monitor.set_connecting().unwrap();
        assert_eq!(monitor.status(), ConnectionHealth::Connecting);
        assert!(monitor.last_check().is_some());

        monitor.set_healthy();
        assert_eq!(monitor.status(), ConnectionHealth::Healthy);

        monitor.set_degraded();
        assert_eq!(monitor.status(), ConnectionHealth::Degraded);

        monitor.set_failed();
        assert_eq!(monitor.status(), ConnectionHealth::Failed);
    }

    #[test]
    fn test_background_health_checks() {
        let monitor = Arc::new(HealthMonitor::with_config(1)); // 1 second interval
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
}
