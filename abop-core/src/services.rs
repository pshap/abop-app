//! Service container and dependency injection for ABOP core

use crate::{AppError, Result};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, RwLock, atomic::Ordering};
use tokio::task::JoinHandle;

/// Represents a handle to a background task
#[derive(Debug)]
pub struct TaskHandle {
    id: u64,
    name: String,
    handle: JoinHandle<()>,
}

impl TaskHandle {
    /// Create a new task handle
    pub fn new(id: u64, name: impl Into<String>, handle: JoinHandle<()>) -> Self {
        Self {
            id,
            name: name.into(),
            handle,
        }
    }

    /// Get the task ID
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Get the task name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Abort the task
    pub fn abort(&self) {
        self.handle.abort();
    }
}

/// Service container for dependency injection using type-based registration
#[derive(Debug)]
pub struct ServiceContainer {
    services: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
    next_task_id: std::sync::atomic::AtomicU64,
    tasks: Arc<RwLock<HashMap<u64, TaskHandle>>>,
}

impl ServiceContainer {
    /// Create a new service container
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            next_task_id: std::sync::atomic::AtomicU64::new(1),
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a service in the container by type
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The service container lock cannot be acquired
    /// - The service registration fails
    pub fn register<T: Send + Sync + 'static>(&self, service: T) -> Result<()> {
        let type_id = TypeId::of::<T>();
        self.services
            .write()
            .map_err(|_| AppError::Other("Failed to acquire write lock on services".to_string()))?
            .insert(type_id, Arc::new(service));
        Ok(())
    }

    /// Get a service from the container by type
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The service container lock cannot be acquired
    /// - The service type is not registered
    /// - Service downcast fails
    pub fn get<T: Send + Sync + 'static>(&self) -> Result<Arc<T>> {
        let type_id = TypeId::of::<T>();
        let service = self
            .services
            .read()
            .map_err(|_| AppError::Other("Failed to acquire read lock on services".to_string()))?
            .get(&type_id)
            .ok_or_else(|| {
                AppError::Other(format!(
                    "Service of type '{}' not found",
                    std::any::type_name::<T>()
                ))
            })?
            .clone();

        service.downcast::<T>().map_err(|_| {
            AppError::Other(format!(
                "Service downcast failed for type '{}'",
                std::any::type_name::<T>()
            ))
        })
    }

    /// Check if a service is registered by type
    #[must_use]
    pub fn contains<T: Send + Sync + 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.services
            .read()
            .map(|services| services.contains_key(&type_id))
            .unwrap_or(false)
    }

    /// Get the number of registered services
    #[must_use]
    pub fn service_count(&self) -> usize {
        self.services
            .read()
            .map(|services| services.len())
            .unwrap_or(0)
    }

    /// Spawn a new background task with a name for tracking
    pub fn spawn_named<F, Fut>(&self, name: impl Into<String>, future: F) -> Result<u64>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let task_name = name.into();
        let task_name_for_log = task_name.clone(); // Clone for the async block
        let task_id = self.next_task_id.fetch_add(1, Ordering::SeqCst);

        // Create a task that maps the Result<(), AppError> to ()
        let task = async move {
            if let Err(e) = future().await {
                log::error!("Task '{task_name_for_log}' failed: {e}");
            }
        };

        let handle = tokio::spawn(task);
        let task_handle = TaskHandle::new(task_id, task_name, handle);

        self.tasks
            .write()
            .map_err(|_| AppError::Other("Failed to acquire write lock on tasks".to_string()))?
            .insert(task_id, task_handle);

        Ok(task_id)
    }

    /// Cancel a running task by ID
    pub fn cancel_task(&self, task_id: u64) -> Result<()> {
        if let Some(handle) = self
            .tasks
            .write()
            .map_err(|_| AppError::Other("Failed to acquire write lock on tasks".to_string()))?
            .remove(&task_id)
        {
            handle.handle.abort();
            Ok(())
        } else {
            Err(AppError::Other("Task not found".to_string()))
        }
    }

    /// Cancel all running tasks
    pub fn cancel_all_tasks(&self) -> Result<()> {
        let tasks =
            std::mem::take(&mut *self.tasks.write().map_err(|_| {
                AppError::Other("Failed to acquire write lock on tasks".to_string())
            })?);

        for (_, handle) in tasks {
            handle.abort();
        }

        Ok(())
    }

    /// Get a list of all running tasks
    pub fn list_tasks(&self) -> Result<Vec<(u64, String)>> {
        self.tasks
            .read()
            .map_err(|_| AppError::Other("Failed to acquire read lock on tasks".to_string()))
            .map(|tasks| {
                tasks
                    .values()
                    .map(|t| (t.id(), t.name().to_string()))
                    .collect()
            })
    }

    /// Wait for a task to complete
    pub async fn wait_for_task(&self, task_id: u64) -> Result<()> {
        let handle = {
            let mut tasks = self.tasks.write().map_err(|_| {
                AppError::Other("Failed to acquire write lock on tasks".to_string())
            })?;

            // Remove the task from tracking before awaiting its completion
            // This prevents holding the lock across await points
            if let Some(handle) = tasks.remove(&task_id) {
                handle.handle
            } else {
                return Err(AppError::Other("Task not found".to_string()));
            }
        };

        // Await the task completion and return the result
        handle
            .await
            .map_err(|e| AppError::Other(format!("Task failed: {e}")))?;

        Ok(())
    }
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ServiceContainer {
    fn drop(&mut self) {
        // Try to cancel all tasks on drop
        if let Err(e) = self.cancel_all_tasks() {
            log::error!("Failed to cancel all tasks during drop: {e}");
        }
    }
}

/// Database service with connection management
#[derive(Debug)]
pub struct DatabaseService {
    connection_string: String,
    // Add connection pool or other resources here
}

impl Drop for DatabaseService {
    fn drop(&mut self) {
        // Clean up database connections
        log::debug!(
            "Dropping DatabaseService with connection: {}",
            self.connection_string
        );
        // Add any necessary cleanup code here
    }
}

impl DatabaseService {
    /// Create new database service with connection
    #[must_use]
    pub const fn new(connection_string: String) -> Self {
        Self { connection_string }
    }

    /// Get the database connection string
    #[must_use]
    pub fn connection_string(&self) -> &str {
        &self.connection_string
    }
}

/// Configuration service with file watching
#[derive(Debug)]
pub struct ConfigService {
    config_path: String,
    // Add file watcher or other resources here
}

impl Drop for ConfigService {
    fn drop(&mut self) {
        // Clean up file watchers
        log::debug!(
            "Dropping ConfigService with config path: {}",
            self.config_path
        );
        // Add any necessary cleanup code here
    }
}

impl ConfigService {
    /// Create new configuration service
    #[must_use]
    pub const fn new(config_path: String) -> Self {
        Self { config_path }
    }

    /// Get the configuration file path
    #[must_use]
    pub fn config_path(&self) -> &str {
        &self.config_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::*;

    #[test]
    fn test_service_registration() {
        let container = ServiceContainer::new();
        let db_service = DatabaseService::new(service::TEST_DB_PATH.to_string());

        assert!(container.register(db_service).is_ok());
        assert!(container.contains::<DatabaseService>());
    }

    #[test]
    fn test_service_retrieval() {
        let container = ServiceContainer::new();
        let db_service = DatabaseService::new(service::TEST_DB_PATH.to_string());

        container.register(db_service).unwrap();

        let retrieved = container.get::<DatabaseService>().unwrap();
        assert_eq!(retrieved.connection_string(), service::TEST_DB_PATH);
    }

    #[test]
    fn test_multiple_services() {
        let container = ServiceContainer::new();

        container
            .register(DatabaseService::new(service::TEST_DB_PATH.to_string()))
            .unwrap();
        container
            .register(ConfigService::new(service::TEST_CONFIG_PATH.to_string()))
            .unwrap();

        assert!(container.contains::<DatabaseService>());
        assert!(container.contains::<ConfigService>());
        assert_eq!(container.service_count(), 2);
    }
}
