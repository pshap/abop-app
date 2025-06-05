//! Dynamic theme loading system for extensible theming
//!
//! This module provides functionality to load themes from external files,
//! enabling runtime theme customization and plugin-based extensions.

use crate::design_tokens::{
    ComponentTokens, DesignTokens, SemanticColors, SpacingTokens, TypographyTokens,
};
use iced::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Serializable theme configuration for loading from files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Theme metadata
    pub metadata: ThemeMetadata,
    /// Semantic color definitions
    pub semantic_colors: SerializableSemanticColors,
    /// Design token values
    pub design_tokens: SerializableDesignTokens,
    /// Custom component overrides
    pub component_overrides: HashMap<String, ComponentOverride>,
}

/// Theme metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    /// Theme name
    pub name: String,
    /// Theme version
    pub version: String,
    /// Theme author
    pub author: Option<String>,
    /// Theme description
    pub description: Option<String>,
    /// Whether this is a dark theme
    pub is_dark: bool,
    /// Parent theme to inherit from
    pub extends: Option<String>,
}

/// Serializable semantic colors for file loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSemanticColors {
    /// Primary color as hex string
    pub primary: String,
    /// Secondary color as hex string  
    pub secondary: String,
    /// Success color as hex string
    pub success: String,
    /// Warning color as hex string
    pub warning: String,
    /// Error color as hex string
    pub error: String,
    /// Info color as hex string
    pub info: String,
    /// Surface color as hex string
    pub surface: String,
    /// Text on surface color as hex string
    pub on_surface: String,
}

impl SerializableSemanticColors {
    /// Convert to runtime `SemanticColors`
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any color hex string is invalid or malformed
    /// - Color parsing fails for any semantic color
    pub fn to_semantic_colors(&self) -> Result<SemanticColors, ThemeLoadError> {
        Ok(SemanticColors {
            primary: Self::parse_color(&self.primary)?,
            secondary: Self::parse_color(&self.secondary)?,
            success: Self::parse_color(&self.success)?,
            warning: Self::parse_color(&self.warning)?,
            error: Self::parse_color(&self.error)?,
            info: Self::parse_color(&self.info)?,
            surface: Self::parse_color(&self.surface)?,
            on_surface: Self::parse_color(&self.on_surface)?,
        })
    }

    /// Parse hex color string to Color
    fn parse_color(hex: &str) -> Result<Color, ThemeLoadError> {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 && hex.len() != 8 {
            return Err(ThemeLoadError::InvalidColor(hex.to_string()));
        }
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| ThemeLoadError::InvalidColor(hex.to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| ThemeLoadError::InvalidColor(hex.to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| ThemeLoadError::InvalidColor(hex.to_string()))?;

        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16)
                .map_err(|_| ThemeLoadError::InvalidColor(hex.to_string()))?
        } else {
            255
        };

        Ok(Color::from_rgba(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
            f32::from(a) / 255.0,
        ))
    }
}

/// Serializable design tokens for file loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableDesignTokens {
    /// Spacing values
    pub spacing: SerializableSpacing,
    /// Typography values
    pub typography: SerializableTypography,
    /// Border radius values
    pub radius: HashMap<String, f32>,
    /// Elevation/shadow values
    pub elevation: HashMap<String, f32>,
    /// Component sizing values
    pub sizing: HashMap<String, f32>,
}

/// Serializable spacing tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSpacing {
    /// Extra small spacing
    pub xs: f32,
    /// Small spacing
    pub sm: f32,
    /// Medium spacing
    pub md: f32,
    /// Large spacing
    pub lg: f32,
    /// Extra large spacing
    pub xl: f32,
    /// Extra extra large spacing
    pub xxl: f32,
}

/// Serializable typography tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTypography {
    /// Caption font size
    pub caption: u16,
    /// Body font size
    pub body: u16,
    /// Large body font size
    pub body_large: u16,
    /// Heading 3 font size
    pub heading_3: u16,
    /// Heading 2 font size
    pub heading_2: u16,
    /// Heading 1 font size
    pub heading_1: u16,
    /// Display font size
    pub display: u16,
}

/// Component style override configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentOverride {
    /// Component type (button, input, container, etc.)
    pub component_type: String,
    /// Style variant override
    pub variant: Option<String>,
    /// Custom properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// Theme loading errors
#[derive(Debug, Clone)]
pub enum ThemeLoadError {
    /// File not found or cannot be read
    FileError(String),
    /// Invalid JSON or TOML format
    ParseError(String),
    /// Invalid color format
    InvalidColor(String),
    /// Required field missing
    MissingField(String),
    /// Theme validation failed
    ValidationError(String),
}

impl std::fmt::Display for ThemeLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileError(msg) => write!(f, "File error: {msg}"),
            Self::ParseError(msg) => write!(f, "Parse error: {msg}"),
            Self::InvalidColor(color) => write!(f, "Invalid color: {color}"),
            Self::MissingField(field) => write!(f, "Missing field: {field}"),
            Self::ValidationError(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}

impl std::error::Error for ThemeLoadError {}

/// Dynamic theme loader for loading themes from files
pub struct ThemeLoader {
    /// Cache of loaded themes
    theme_cache: HashMap<String, ThemeConfig>,
}

impl ThemeLoader {
    /// Create a new theme loader
    #[must_use]
    pub fn new() -> Self {
        Self {
            theme_cache: HashMap::new(),
        }
    }

    /// Create a theme loader with a specific directory
    #[must_use]
    pub fn with_directory<P: AsRef<Path>>(_directory: P) -> Self {
        Self {
            theme_cache: HashMap::new(),
        }
    }

    /// Load theme from JSON file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - JSON parsing fails
    /// - Theme validation fails
    pub fn load_from_json<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<ThemeConfig, ThemeLoadError> {
        let content = fs::read_to_string(&path).map_err(|e| {
            ThemeLoadError::FileError(format!("{}: {}", path.as_ref().display(), e))
        })?;
        let config: ThemeConfig = serde_json::from_str(&content)
            .map_err(|e| ThemeLoadError::ParseError(e.to_string()))?;

        Self::validate_theme(&config)?;

        // Cache the theme
        self.theme_cache
            .insert(config.metadata.name.clone(), config.clone());

        Ok(config)
    }

    /// Load theme from TOML file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - TOML parsing fails
    /// - Theme validation fails
    pub fn load_from_toml<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<ThemeConfig, ThemeLoadError> {
        let content = fs::read_to_string(&path).map_err(|e| {
            ThemeLoadError::FileError(format!("{}: {}", path.as_ref().display(), e))
        })?;
        let config: ThemeConfig =
            toml::from_str(&content).map_err(|e| ThemeLoadError::ParseError(e.to_string()))?;

        Self::validate_theme(&config)?;

        // Cache the theme
        self.theme_cache
            .insert(config.metadata.name.clone(), config.clone());

        Ok(config)
    }

    /// Get cached theme by name
    #[must_use]
    pub fn get_theme(&self, name: &str) -> Option<&ThemeConfig> {
        self.theme_cache.get(name)
    }

    /// List all loaded themes
    #[must_use]
    pub fn list_themes(&self) -> Vec<&ThemeConfig> {
        self.theme_cache.values().collect()
    }

    /// Create a runtime theme mode from a theme config
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Semantic color conversion fails
    /// - Design token conversion fails
    /// - Theme configuration is invalid
    pub fn create_theme_mode(
        &self,
        config: &ThemeConfig,
    ) -> Result<CustomThemeMode, ThemeLoadError> {
        let semantic_colors = config.semantic_colors.to_semantic_colors()?;

        Ok(CustomThemeMode {
            metadata: config.metadata.clone(),
            semantic_colors,
            design_tokens: Self::convert_design_tokens(&config.design_tokens)?,
        })
    }
    /// Validate a loaded theme configuration
    fn validate_theme(config: &ThemeConfig) -> Result<(), ThemeLoadError> {
        if config.metadata.name.is_empty() {
            return Err(ThemeLoadError::MissingField("metadata.name".to_string()));
        }

        // Try to parse colors to validate them
        config.semantic_colors.to_semantic_colors()?;

        Ok(())
    }
    /// Convert serializable design tokens to runtime tokens
    fn convert_design_tokens(
        tokens: &SerializableDesignTokens,
    ) -> Result<DesignTokens, ThemeLoadError> {
        let spacing = SpacingTokens {
            xs: tokens.spacing.xs,
            sm: tokens.spacing.sm,
            md: tokens.spacing.md,
            lg: tokens.spacing.lg,
            xl: tokens.spacing.xl,
            xxl: tokens.spacing.xxl,
        };

        let typography = TypographyTokens {
            caption: tokens.typography.caption,
            body: tokens.typography.body,
            body_large: tokens.typography.body_large,
            heading_3: tokens.typography.heading_3,
            heading_2: tokens.typography.heading_2,
            heading_1: tokens.typography.heading_1,
            display: tokens.typography.display,
        };

        Ok(DesignTokens {
            spacing,
            typography,
            radius: Default::default(),
            elevation: Default::default(),
            sizing: Default::default(),
            semantic_colors: SemanticColors::dark(), // Will be overridden
            components: ComponentTokens::new(),
            ui: Default::default(), // Ensure ui is included
        })
    }
}

impl Default for ThemeLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Custom theme mode created from loaded configuration
#[derive(Debug, Clone)]
pub struct CustomThemeMode {
    /// Theme metadata
    pub metadata: ThemeMetadata,
    /// Semantic colors
    pub semantic_colors: SemanticColors,
    /// Design tokens
    pub design_tokens: DesignTokens,
}

impl CustomThemeMode {
    /// Get the theme name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// Check if this is a dark theme
    #[must_use]
    pub const fn is_dark(&self) -> bool {
        self.metadata.is_dark
    }

    /// Get semantic colors
    #[must_use]
    pub const fn semantic_colors(&self) -> &SemanticColors {
        &self.semantic_colors
    }

    /// Get design tokens
    #[must_use]
    pub const fn design_tokens(&self) -> &DesignTokens {
        &self.design_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_parsing() {
        assert!(SerializableSemanticColors::parse_color("#FF0000").is_ok());
        assert!(SerializableSemanticColors::parse_color("#FF0000FF").is_ok());
        assert!(SerializableSemanticColors::parse_color("FF0000").is_ok());
        assert!(SerializableSemanticColors::parse_color("#INVALID").is_err());
    }

    #[test]
    fn test_theme_loader_creation() {
        let loader = ThemeLoader::new();
        assert_eq!(loader.theme_cache.len(), 0);
    }
    #[test]
    fn test_theme_config_validation() {
        let config = ThemeConfig {
            metadata: ThemeMetadata {
                name: "Test Theme".to_string(),
                version: "1.0.0".to_string(),
                author: None,
                description: None,
                is_dark: true,
                extends: None,
            },
            semantic_colors: SerializableSemanticColors {
                primary: "#FF0000".to_string(),
                secondary: "#00FF00".to_string(),
                success: "#00AA00".to_string(),
                warning: "#FFAA00".to_string(),
                error: "#AA0000".to_string(),
                info: "#0000FF".to_string(),
                surface: "#333333".to_string(),
                on_surface: "#FFFFFF".to_string(),
            },
            design_tokens: SerializableDesignTokens {
                spacing: SerializableSpacing {
                    xs: 4.0,
                    sm: 8.0,
                    md: 16.0,
                    lg: 24.0,
                    xl: 32.0,
                    xxl: 48.0,
                },
                typography: SerializableTypography {
                    caption: 12,
                    body: 14,
                    body_large: 16,
                    heading_3: 18,
                    heading_2: 20,
                    heading_1: 24,
                    display: 32,
                },
                radius: HashMap::new(),
                elevation: HashMap::new(),
                sizing: HashMap::new(),
            },
            component_overrides: HashMap::new(),
        };

        assert!(ThemeLoader::validate_theme(&config).is_ok());
    }
}
