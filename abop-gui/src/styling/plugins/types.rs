//! Plugin Types and Error Definitions
//!
//! This module contains core types for the plugin system including
//! plugin information, style definitions, and error types.

use crate::theme::ThemeMode;
use iced::{Background, Border, Color, Shadow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Result type for style operations
pub type StyleResult<T> = Result<T, StyleError>;

/// Safely read from an RwLock, converting poison errors to StyleError
pub fn read_lock<T>(lock: &RwLock<T>) -> StyleResult<RwLockReadGuard<'_, T>> {
    lock.read().map_err(|e| {
        log::error!("Failed to acquire read lock: {e}");
        StyleError::LockError(format!("Failed to acquire read lock: {e}"))
    })
}

/// Safely write to an RwLock, converting poison errors to StyleError
pub fn write_lock<T>(lock: &RwLock<T>) -> StyleResult<RwLockWriteGuard<'_, T>> {
    lock.write().map_err(|e| {
        log::error!("Failed to acquire write lock: {e}");
        StyleError::LockError(format!("Failed to acquire write lock: {e}"))
    })
}

/// Plugin trait for style extensions
pub trait StylePlugin: Send + Sync {
    /// Plugin name and version
    fn info(&self) -> PluginInfo;

    /// Initialize plugin with the current theme
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
