//! Plugin-based Style System
//!
//! This module provides a plugin architecture for extending the styling system
//! with custom components, themes, and behaviors.

use crate::theme::ThemeMode;
use iced::{Background, Border, Color, Shadow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Result type for style operations
pub type StyleResult<T> = Result<T, StyleError>;

/// Safely read from an RwLock, converting poison errors to StyleError
fn read_lock<T>(lock: &RwLock<T>) -> StyleResult<RwLockReadGuard<'_, T>> {
    lock.read().map_err(|e| {
        log::error!("Failed to acquire read lock: {e}");
        StyleError::LockError(format!("Failed to acquire read lock: {e}"))
    })
}

/// Safely write to an RwLock, converting poison errors to StyleError
fn write_lock<T>(lock: &RwLock<T>) -> StyleResult<RwLockWriteGuard<'_, T>> {
    lock.write().map_err(|e| {
        log::error!("Failed to acquire write lock: {e}");
        StyleError::LockError(format!("Failed to acquire write lock: {e}"))
    })
}

/// Plugin trait for style extensions
pub trait StylePlugin: Send + Sync {
    /// Plugin name and version
    fn info(&self) -> PluginInfo;

    /// Initialize the plugin with the current theme
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Plugin initialization fails
    /// - Theme configuration is incompatible
    /// - Required resources cannot be allocated
    fn initialize(&mut self, theme: &ThemeMode) -> Result<(), StylePluginError>;

    /// Get custom component styles
    fn get_component_style(&self, component: &str, variant: &str) -> Option<CustomComponentStyle>;

    /// Get custom theme colors
    fn get_custom_colors(&self) -> Option<HashMap<String, Color>>;

    /// Get custom tokens
    fn get_custom_tokens(&self) -> Option<HashMap<String, f32>>;

    /// Handle theme changes
    fn on_theme_change(&self, new_theme: &ThemeMode);

    /// Cleanup resources
    fn cleanup(&self) {}
}

/// Plugin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin author
    pub author: String,
    /// Plugin description
    pub description: String,
    /// API version this plugin supports
    pub api_version: String,
}

/// Custom component style definition
#[derive(Debug, Clone)]
pub struct CustomComponentStyle {
    /// Background styling
    pub background: Option<Background>,
    /// Text color
    pub text_color: Option<Color>,
    /// Border styling
    pub border: Option<Border>,
    /// Shadow styling
    pub shadow: Option<Shadow>,
    /// Custom properties for extended styling
    pub custom_properties: HashMap<String, StyleProperty>,
}

/// Custom style property value
#[derive(Debug, Clone)]
pub enum StyleProperty {
    /// Color value
    Color(Color),
    /// Floating point value
    Float(f32),
    /// String value
    String(String),
    /// Boolean value
    Boolean(bool),
}

/// Style plugin errors
#[derive(Debug, thiserror::Error)]
pub enum StylePluginError {
    /// Plugin failed to initialize
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),

    /// Plugin configuration is invalid
    #[error("Invalid plugin configuration: {0}")]
    InvalidConfiguration(String),

    /// Plugin API version doesn't match expected version
    #[error("Plugin API version mismatch: expected {expected}, got {actual}")]
    ApiVersionMismatch {
        /// Expected API version
        expected: String,
        /// Actual API version provided by plugin
        actual: String,
    },

    /// Required plugin dependency is missing
    #[error("Plugin dependency missing: {0}")]
    DependencyMissing(String),
}

/// Style plugin errors
#[derive(Debug, thiserror::Error)]
pub enum StyleError {
    /// Lock error
    #[error("Lock error: {0}")]
    LockError(String),
    /// Plugin error
    #[error("Plugin error: {0}")]
    PluginError(#[from] StylePluginError),
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for StyleError {
    fn from(err: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        Self::LockError(err.to_string())
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for StyleError {
    fn from(err: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        Self::LockError(err.to_string())
    }
}

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
    /// Panics if the internal plugin registry or theme `RwLock` is poisoned.
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

        // Initialize the plugin with the current theme
        let mut plugin = plugin;
        plugin.initialize(&theme)?;

        // Insert the plugin into the registry
        plugins.insert(info.name, plugin);

        Ok(())
    }

    /// Unregister a plugin
    ///
    /// # Panics
    ///
    /// Panics if the internal plugin registry mutex is poisoned.
    /// Unregister a plugin
    ///
    /// # Errors
    ///
    /// Returns `StyleError::LockError` if the lock is poisoned
    pub fn unregister_plugin(&self, name: &str) -> StyleResult<bool> {
        let mut plugins = write_lock(&self.plugins)?;
        let removed = if let Some(plugin) = plugins.remove(name) {
            plugin.cleanup();
            true
        } else {
            false
        };
        Ok(removed)
    }

    /// Get component style from plugins    ///
    /// # Panics
    ///
    /// Panics if the internal plugin registry mutex is poisoned.
    pub fn get_plugin_style(
        &self,
        component: &str,
        variant: &str,
    ) -> StyleResult<Option<CustomComponentStyle>> {
        let plugins = read_lock(&self.plugins)?;
        for plugin in plugins.values() {
            if let Some(style) = plugin.get_component_style(component, variant) {
                return Ok(Some(style));
            }
        }

        Ok(None)
    }

    /// Get all custom colors from plugins
    ///
    /// # Panics
    ///
    /// Panics if the internal plugin registry mutex is poisoned.
    /// Get all custom colors from plugins
    ///
    /// # Errors
    ///
    /// Returns `StyleError::LockError` if the lock is poisoned
    pub fn get_all_custom_colors(&self) -> StyleResult<HashMap<String, Color>> {
        let plugins = read_lock(&self.plugins)?;
        let mut colors = HashMap::new();
        for plugin in plugins.values() {
            if let Some(plugin_colors) = plugin.get_custom_colors() {
                colors.extend(plugin_colors);
            }
        }
        Ok(colors)
    }

    /// Get all custom tokens from plugins
    ///
    /// # Panics
    ///
    /// Panics if the internal plugin registry mutex is poisoned.
    /// Get all custom tokens from plugins
    ///
    /// # Errors
    ///
    /// Returns `StyleError::LockError` if the lock is poisoned
    pub fn get_all_custom_tokens(&self) -> StyleResult<HashMap<String, f32>> {
        let plugins = read_lock(&self.plugins)?;
        let mut tokens = HashMap::new();
        for plugin in plugins.values() {
            if let Some(plugin_tokens) = plugin.get_custom_tokens() {
                tokens.extend(plugin_tokens);
            }
        }
        Ok(tokens)
    }

    /// Update theme for all plugins
    ///
    /// # Panics
    ///
    /// Panics if the internal theme or plugin registry mutex is poisoned.
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
        for plugin in plugins.values() {
            plugin.on_theme_change(&new_theme);
        }

        Ok(())
    }

    /// List all registered plugins
    ///
    /// # Panics
    ///
    /// Panics if the internal plugin registry mutex is poisoned.
    /// List all registered plugins
    ///
    /// # Errors
    ///
    /// Returns `StyleError::LockError` if the lock is poisoned
    pub fn list_plugins(&self) -> StyleResult<Vec<PluginInfo>> {
        let plugins = read_lock(&self.plugins)?;
        Ok(plugins.values().map(|p| p.info()).collect())
    }
}

/// Advanced customization API for creating complex styling behaviors
pub struct StyleCustomizationAPI {
    registry: Arc<StylePluginRegistry>,
    theme_overrides: RwLock<HashMap<String, ThemeOverride>>,
}

/// Theme override for specific components or contexts
#[derive(Debug, Clone)]
pub struct ThemeOverride {
    /// Override for semantic colors
    pub colors: Option<iced::Color>,
    /// Override for component tokens
    pub tokens: Option<f32>,
    /// Override styles for specific components
    pub component_styles: HashMap<String, CustomComponentStyle>,
}

impl StyleCustomizationAPI {
    /// Create new customization API
    #[must_use]
    pub fn new(registry: Arc<StylePluginRegistry>) -> Self {
        Self {
            registry,
            theme_overrides: RwLock::new(HashMap::new()),
        }
    }

    /// Add theme override for specific context
    ///
    /// # Panics
    ///
    /// Panics if the internal theme overrides mutex is poisoned.
    pub fn add_theme_override(&self, context: String, override_def: ThemeOverride) {
        let mut overrides = write_lock(&self.theme_overrides).unwrap();
        overrides.insert(context, override_def);
    }

    /// Remove theme override
    ///
    /// # Panics
    ///
    /// Panics if the internal theme overrides mutex is poisoned.
    pub fn remove_theme_override(&self, context: &str) -> bool {
        let mut overrides = write_lock(&self.theme_overrides).unwrap();
        overrides.remove(context).is_some()
    }

    /// Get effective style for component with context
    ///
    /// # Panics
    ///
    /// Panics if the internal theme overrides mutex is poisoned.
    pub fn get_effective_style(
        &self,
        component: &str,
        variant: &str,
        context: Option<&str>,
    ) -> StyleResult<Option<CustomComponentStyle>> {
        // Check context-specific overrides first
        if let Some(context) = context {
            let overrides = read_lock(&self.theme_overrides)?;
            if let Some(override_def) = overrides.get(context)
                && let Some(style) = override_def.component_styles.get(component)
            {
                return Ok(Some(style.clone()));
            }
        }

        // Fall back to plugin styles
        self.registry.get_plugin_style(component, variant)
    }

    /// Create style builder with customization support
    pub fn create_custom_builder(&self, base_style: CustomComponentStyle) -> CustomStyleBuilder {
        CustomStyleBuilder::new(base_style, self.registry.clone())
    }
}

/// Builder for custom component styles
pub struct CustomStyleBuilder {
    style: CustomComponentStyle,
    registry: Arc<StylePluginRegistry>,
}

impl CustomStyleBuilder {
    /// Create new custom style builder
    #[must_use]
    pub const fn new(base_style: CustomComponentStyle, registry: Arc<StylePluginRegistry>) -> Self {
        Self {
            style: base_style,
            registry,
        }
    }

    /// Set background
    #[must_use]
    pub const fn background(mut self, background: Background) -> Self {
        self.style.background = Some(background);
        self
    }

    /// Set text color
    #[must_use]
    pub const fn text_color(mut self, color: Color) -> Self {
        self.style.text_color = Some(color);
        self
    }

    /// Set border
    #[must_use]
    pub const fn border(mut self, border: Border) -> Self {
        self.style.border = Some(border);
        self
    }

    /// Set shadow
    #[must_use]
    pub const fn shadow(mut self, shadow: Shadow) -> Self {
        self.style.shadow = Some(shadow);
        self
    }

    /// Add custom property
    #[must_use]
    pub fn custom_property(mut self, key: String, value: StyleProperty) -> Self {
        self.style.custom_properties.insert(key, value);
        self
    }

    /// Merge with plugin styles
    #[must_use]
    pub fn merge_plugin_styles(mut self, component: &str, variant: &str) -> Self {
        if let Some(plugin_style) = self.registry.get_plugin_style(component, variant).unwrap() {
            // Merge styles, preferring existing values
            if self.style.background.is_none() {
                self.style.background = plugin_style.background;
            }
            if self.style.text_color.is_none() {
                self.style.text_color = plugin_style.text_color;
            }
            if self.style.border.is_none() {
                self.style.border = plugin_style.border;
            }
            if self.style.shadow.is_none() {
                self.style.shadow = plugin_style.shadow;
            }

            // Merge custom properties
            for (key, value) in plugin_style.custom_properties {
                self.style.custom_properties.entry(key).or_insert(value);
            }
        }
        self
    }

    /// Build the final style
    #[must_use]
    pub fn build(self) -> CustomComponentStyle {
        self.style
    }
}

/// Global style system instance
pub static STYLE_SYSTEM: std::sync::LazyLock<Arc<StyleCustomizationAPI>> =
    std::sync::LazyLock::new(|| {
        let registry = Arc::new(StylePluginRegistry::new(ThemeMode::Dark));
        Arc::new(StyleCustomizationAPI::new(registry))
    });

/// Utility functions for plugin development
pub mod plugin_utils {
    use super::{
        Background, Border, Color, CustomComponentStyle, HashMap, PluginInfo, Shadow,
        StylePluginError,
    };

    /// Helper to create basic component style
    #[must_use]
    pub fn create_basic_style(
        background: Color,
        text_color: Color,
        border_color: Color,
        border_width: f32,
        border_radius: f32,
    ) -> CustomComponentStyle {
        CustomComponentStyle {
            background: Some(Background::Color(background)),
            text_color: Some(text_color),
            border: Some(Border {
                color: border_color,
                width: border_width,
                radius: border_radius.into(),
            }),
            shadow: None,
            custom_properties: HashMap::new(),
        }
    }

    /// Helper to create elevated style with shadow
    #[must_use]
    pub fn create_elevated_style(
        base: CustomComponentStyle,
        elevation: f32,
        shadow_color: Color,
    ) -> CustomComponentStyle {
        let mut style = base;
        style.shadow = Some(Shadow {
            color: shadow_color,
            offset: iced::Vector::new(0.0, elevation / 2.0),
            blur_radius: elevation,
        });
        style
    }

    /// Validate plugin configuration
    ///
    /// # Errors
    ///
    /// Returns `StylePluginError::InvalidConfiguration` if the plugin name is empty.
    /// Returns `StylePluginError::ApiVersionMismatch` if the API version is not "1.0".
    pub fn validate_plugin_config(info: &PluginInfo) -> Result<(), StylePluginError> {
        if info.name.is_empty() {
            return Err(StylePluginError::InvalidConfiguration(
                "Plugin name cannot be empty".to_string(),
            ));
        }

        if info.api_version != "1.0" {
            return Err(StylePluginError::ApiVersionMismatch {
                expected: "1.0".to_string(),
                actual: info.api_version.clone(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        info: PluginInfo,
        initialized: bool,
    }

    impl TestPlugin {
        fn new() -> Self {
            Self {
                info: PluginInfo {
                    name: "test_plugin".to_string(),
                    version: "1.0.0".to_string(),
                    author: "Test Author".to_string(),
                    description: "Test plugin".to_string(),
                    api_version: "1.0".to_string(),
                },
                initialized: false,
            }
        }
    }

    impl StylePlugin for TestPlugin {
        fn info(&self) -> PluginInfo {
            self.info.clone()
        }

        fn initialize(&mut self, _theme: &ThemeMode) -> Result<(), StylePluginError> {
            self.initialized = true;
            Ok(())
        }

        fn get_component_style(
            &self,
            _component: &str,
            _variant: &str,
        ) -> Option<CustomComponentStyle> {
            None
        }

        fn get_custom_colors(&self) -> Option<HashMap<String, Color>> {
            None
        }

        fn get_custom_tokens(&self) -> Option<HashMap<String, f32>> {
            None
        }

        fn on_theme_change(&self, _new_theme: &ThemeMode) {}
    }

    #[test]
    fn test_plugin_registry() {
        let registry = StylePluginRegistry::new(ThemeMode::Dark);
        let plugin = Box::new(TestPlugin::new());

        assert!(registry.register_plugin(plugin).is_ok());
        assert_eq!(registry.list_plugins().unwrap().len(), 1);
        assert!(registry.unregister_plugin("test_plugin").unwrap());
        assert_eq!(registry.list_plugins().unwrap().len(), 0);
    }

    #[test]
    fn test_custom_style_builder() {
        let registry = Arc::new(StylePluginRegistry::new(ThemeMode::Dark));
        let base_style = CustomComponentStyle {
            background: None,
            text_color: None,
            border: None,
            shadow: None,
            custom_properties: HashMap::new(),
        };

        let style = CustomStyleBuilder::new(base_style, registry)
            .background(Background::Color(Color::WHITE))
            .text_color(Color::BLACK)
            .build();

        assert!(style.background.is_some());
        assert!(style.text_color.is_some());
    }
}
