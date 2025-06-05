//! Registry for custom elevation styles

use super::ElevationStyle;
use std::collections::HashMap;

/// Registry for custom elevation styles
#[derive(Debug, Clone)]
pub struct ElevationRegistry {
    /// Map of custom elevation names to styles
    custom_elevations: HashMap<String, ElevationStyle>,
}

impl ElevationRegistry {
    /// Create a new empty registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            custom_elevations: HashMap::new(),
        }
    }

    /// Register a custom elevation style
    pub fn register(&mut self, name: String, style: ElevationStyle) {
        self.custom_elevations.insert(name, style);
    }

    /// Get a custom elevation style by name
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ElevationStyle> {
        self.custom_elevations.get(name)
    }

    /// Remove a custom elevation style
    #[must_use]
    pub fn remove(&mut self, name: &str) -> Option<ElevationStyle> {
        self.custom_elevations.remove(name)
    }

    /// List all registered custom elevation names
    #[must_use]
    pub fn list_names(&self) -> Vec<&String> {
        self.custom_elevations.keys().collect()
    }

    /// Check if a custom elevation is registered
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.custom_elevations.contains_key(name)
    }

    /// Get the number of registered custom elevations
    #[must_use]
    pub fn len(&self) -> usize {
        self.custom_elevations.len()
    }

    /// Check if the registry is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.custom_elevations.is_empty()
    }

    /// Clear all registered custom elevations
    pub fn clear(&mut self) {
        self.custom_elevations.clear();
    }
}

impl Default for ElevationRegistry {
    fn default() -> Self {
        Self::new()
    }
}
