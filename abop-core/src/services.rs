//! Service container and dependency injection for ABOP core

use crate::{AppError, Result};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Service container for dependency injection using type-based registration
#[derive(Debug, Clone)]
pub struct ServiceContainer {
    services: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl ServiceContainer {
    /// Create a new service container
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
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
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Example database service
#[derive(Debug)]
pub struct DatabaseService {
    connection_string: String,
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

/// Example configuration service
#[derive(Debug)]
pub struct ConfigService {
    config_path: String,
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
