//! Custom Style Builder
//!
//! This module provides a fluent builder pattern for creating custom component
//! styles with plugin integration and theme override support.

use super::types::*;
use crate::theme::ThemeMode;
use iced::{Background, Border, Color, Shadow};

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
