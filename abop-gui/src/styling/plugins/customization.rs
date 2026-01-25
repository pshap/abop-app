//! Style Customization API
//!
//! This module provides advanced customization capabilities including theme overrides,
//! custom component styles, and plugin integration for complex styling behaviors.

use super::types::*;
use super::registry::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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

/// Advanced customization API for creating complex styling behaviors
pub struct StyleCustomizationAPI {
    registry: Arc<StylePluginRegistry>,
    theme_overrides: RwLock<HashMap<String, ThemeOverride>>,
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
    /// Panics if internal theme overrides mutex is poisoned.
    pub fn add_theme_override(&self, context: String, override_def: ThemeOverride) {
        let mut overrides = write_lock(&self.theme_overrides).unwrap();
        overrides.insert(context, override_def);
    }

    /// Remove theme override
    ///
    /// # Panics
    ///
    /// Panics if internal theme overrides mutex is poisoned.
    pub fn remove_theme_override(&self, context: &str) -> bool {
        let mut overrides = write_lock(&self.theme_overrides).unwrap();
        overrides.remove(context).is_some()
    }

    /// Get effective style for component with context
    ///
    /// # Panics
    ///
    /// Panics if internal theme overrides mutex is poisoned.
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