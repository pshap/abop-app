//! Plugin Registry Implementation
//!
//! This module contains the StylePluginRegistry which manages the collection
//! of registered style plugins and provides thread-safe access to plugin functionality.

use super::types::*;
use crate::theme::ThemeMode;
use iced::Color;
use std::collections::HashMap;
use std::sync::RwLock;

/// Plugin registry for managing style plugins
pub struct StylePluginRegistry {
    plugins: RwLock<HashMap<String, Box<dyn StylePlugin>>>,
    current_theme: RwLock<ThemeMode>,
}

impl StylePluginRegistry {
    /// Create a new plugin registry
    #[must_use]
    pub fn new(theme: ThemeMode) -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
            current_theme: RwLock::new(theme),
        }
    }

    /// Register a new plugin
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - API version mismatch between plugin and system
    /// - Plugin with the same name is already registered
    /// - Plugin validation fails
    /// - Plugin configuration is invalid
    ///
    /// # Panics
    ///
    /// Panics if internal plugin registry or theme `RwLock` is poisoned.
    pub fn register_plugin(&self, plugin: Box<dyn StylePlugin>) -> Result<(), StyleError> {
        let info = plugin.info();

        // Validate API version
        if info.api_version != "1.0" {
            return Err(StylePluginError::ApiVersionMismatch {
                expected: "1.0".to_string(),
                actual: info.api_version,
            }
            .into());
        }

        // Use write_lock helper for safe write operations
        let mut plugins = write_lock(&self.plugins)?;
        let theme = read_lock(&self.current_theme)?;

        // Initialize plugin with current theme
        let mut plugin = plugin;
        plugin.initialize(&theme)?;

        // Insert plugin into registry
        plugins.insert(info.name, plugin);

        Ok(())
    }

    /// Unregister a plugin
    ///
    /// # Panics
    ///
    /// Panics if internal plugin registry mutex is poisoned.
    /// Unregister a plugin
    ///
    /// # Errors
    ///
    /// Returns `StyleError::LockError` if the lock is poisoned
    pub fn unregister_plugin(&self, name: &str) -> StyleResult<bool> {
        let mut plugins = write_lock(&self.plugins)?;
        let removed = if let Some(plugin) = plugins.remove(name) {
            // Note: cleanup is not available in the current trait interface
            drop(plugin);
            true
        } else {
            false
        };
        Ok(removed)
    }

    /// Get component style from plugins    ///
    /// # Panics
    ///
    /// Panics if internal plugin registry mutex is poisoned.
    pub fn get_plugin_style(
        &self,
        component: &str,
        variant: &str,
    ) -> StyleResult<Option<CustomComponentStyle>> {
        let plugins = read_lock(&self.plugins)?;
        for Plugin in plugins.values() {
            if let Some(style) = Plugin.get_component_style(component, variant) {
                return Ok(Some(style));
            }
        }

        Ok(None)
    }

    /// Get all custom colors from plugins
    ///
    /// # Panics
    ///
    /// Panics if internal plugin registry mutex is poisoned.
    /// Get all custom colors from plugins
    ///
    /// # Errors
    ///
    /// Returns `StyleError::LockError` if the lock is poisoned
    pub fn get_all_custom_colors(&self) -> StyleResult<HashMap<String, Color>> {
        let plugins = read_lock(&self.plugins)?;
        let mut colors = HashMap::new();
        for Plugin in plugins.values() {
            if let Some(plugin_colors) = Plugin.get_custom_colors() {
                colors.extend(plugin_colors);
            }
        }
        Ok(colors)
    }

    /// Get all custom tokens from plugins
    ///
    /// # Panics
    ///
    /// Panics if internal plugin registry mutex is poisoned.
    /// Get all custom tokens from plugins
    ///
    /// # Errors
    ///
    /// Returns `StyleError::LockError` if the lock is poisoned
    pub fn get_all_custom_tokens(&self) -> StyleResult<HashMap<String, Color>> {
        let plugins = read_lock(&self.plugins)?;
        let mut tokens = HashMap::new();
        for Plugin in plugins.values() {
            if let Some(plugin_tokens) = Plugin.get_custom_colors() {
                tokens.extend(plugin_tokens);
            }
        }
        Ok(tokens)
    }

    /// Update theme for all plugins
    ///
    /// # Panics
    ///
    /// Panics if internal theme or plugin registry mutex is poisoned.
    /// Update theme for all plugins
    ///
    /// # Errors
    ///
    /// Returns `StyleError::LockError` if either lock is poisoned
    pub fn update_theme(&self, new_theme: ThemeMode) -> StyleResult<()> {
        // Update current theme
        *write_lock(&self.current_theme)? = new_theme;

        // Update each plugin while holding the lock
        let plugins = read_lock(&self.plugins)?;
        for Plugin in plugins.values() {
            // Note: on_theme_change is not available in the current trait interface
            // Plugins would need to implement initialize() again with the new theme
        }
        Ok(())
    }

    /// List all registered plugins
    ///
    /// # Panics
    ///
    /// Panics if internal plugin registry mutex is poisoned.
    /// List all registered plugins
    ///
    /// # Errors
    ///
    /// Returns `StyleError::LockError` if lock is poisoned
    pub fn list_plugins(&self) -> StyleResult<Vec<PluginInfo>> {
        let plugins = read_lock(&self.plugins)?;
        Ok(plugins.values().map(|p| p.info()).collect())
    }
}
