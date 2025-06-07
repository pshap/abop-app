//! Dynamic theme loading system for extensible theming
//!
//! This module provides functionality to load themes from external files,
//! enabling runtime theme customization and plugin-based extensions.

use crate::styling::material::{
    spacing::SpacingTokens,
    tokens::{core::MaterialTokens, semantic::SemanticColors},
    typography::{
        MaterialTypography, TypeStyle,
        font_mapping::{MaterialFont, MaterialWeight},
    },
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

/// Simplified design tokens for file loading - now directly compatible with Material Design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableDesignTokens {
    /// Spacing values
    pub spacing: SerializableSpacing,
    /// Typography values  
    pub typography: SerializableTypography,
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
    /// Label small font size
    pub label_small: u16,
    /// Label medium font size
    pub label_medium: u16,
    /// Label large font size
    pub label_large: u16,
    /// Body small font size
    pub body_small: u16,
    /// Body medium font size
    pub body_medium: u16,
    /// Body large font size
    pub body_large: u16,
    /// Title small font size
    pub title_small: u16,
    /// Title medium font size
    pub title_medium: u16,
    /// Title large font size
    pub title_large: u16,
    /// Headline small font size
    pub headline_small: u16,
    /// Headline medium font size
    pub headline_medium: u16,
    /// Headline large font size
    pub headline_large: u16,
    /// Display small font size
    pub display_small: u16,
    /// Display medium font size
    pub display_medium: u16,
    /// Display large font size
    pub display_large: u16,
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
    ) -> Result<MaterialTokens, ThemeLoadError> {
        // Create Material tokens with proper initialization
        let material_tokens = MaterialTokens {
            spacing: SpacingTokens {
                xs: tokens.spacing.xs,
                sm: tokens.spacing.sm,
                md: tokens.spacing.md,
                lg: tokens.spacing.lg,
                xl: tokens.spacing.xl,
                xxl: tokens.spacing.xxl,
            },
            typography: MaterialTypography {
                display_large: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.display_large as f32,
                    64.0,
                    0.0,
                ),
                display_medium: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.display_medium as f32,
                    52.0,
                    0.0,
                ),
                display_small: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.display_small as f32,
                    44.0,
                    0.0,
                ),
                headline_large: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.headline_large as f32,
                    40.0,
                    0.0,
                ),
                headline_medium: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.headline_medium as f32,
                    36.0,
                    0.0,
                ),
                headline_small: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.headline_small as f32,
                    32.0,
                    0.0,
                ),
                title_large: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Medium,
                    tokens.typography.title_large as f32,
                    28.0,
                    0.0,
                ),
                title_medium: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.title_medium as f32,
                    24.0,
                    0.15,
                ),
                title_small: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.title_small as f32,
                    22.0,
                    0.1,
                ),
                label_large: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.label_large as f32,
                    20.0,
                    0.1,
                ),
                label_medium: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.label_medium as f32,
                    16.0,
                    0.5,
                ),
                label_small: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.label_small as f32,
                    14.0,
                    0.1,
                ),
                body_large: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Regular,
                    tokens.typography.body_large as f32,
                    24.0,
                    0.15,
                ),
                body_medium: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Regular,
                    tokens.typography.body_medium as f32,
                    20.0,
                    0.25,
                ),
                body_small: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Regular,
                    tokens.typography.body_small as f32,
                    16.0,
                    0.4,
                ),
            },
            ..Default::default()
        };

        // Keep other Material Design defaults
        Ok(material_tokens)
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
    pub design_tokens: MaterialTokens,
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
    pub const fn design_tokens(&self) -> &MaterialTokens {
        &self.design_tokens
    }
}

/// Serializable theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTheme {
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: String,
    /// Material Design tokens
    pub material_tokens: SerializableMaterialTokens,
    /// Component style overrides
    pub component_overrides: Vec<ComponentOverride>,
}

/// Serializable Material Design tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableMaterialTokens {
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

impl SerializableTheme {
    /// Convert serializable theme to runtime theme
    pub fn to_theme(&self) -> Result<Theme, ThemeLoadError> {
        Ok(Theme {
            name: self.name.clone(),
            description: self.description.clone(),
            material_tokens: Self::convert_material_tokens(&self.material_tokens)?,
            component_overrides: self.component_overrides.clone(),
        })
    }

    /// Convert serializable tokens to runtime tokens
    fn convert_material_tokens(
        tokens: &SerializableMaterialTokens,
    ) -> Result<MaterialTokens, ThemeLoadError> {
        let material_tokens = MaterialTokens {
            spacing: SpacingTokens {
                xs: tokens.spacing.xs,
                sm: tokens.spacing.sm,
                md: tokens.spacing.md,
                lg: tokens.spacing.lg,
                xl: tokens.spacing.xl,
                xxl: tokens.spacing.xxl,
            },
            typography: MaterialTypography {
                display_large: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.display_large as f32,
                    64.0, // line height
                    0.0,  // letter spacing
                ),
                display_medium: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.display_medium as f32,
                    52.0,
                    0.0,
                ),
                display_small: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.display_small as f32,
                    44.0,
                    0.0,
                ),
                headline_large: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.headline_large as f32,
                    40.0,
                    0.0,
                ),
                headline_medium: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.headline_medium as f32,
                    36.0,
                    0.0,
                ),
                headline_small: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Regular,
                    tokens.typography.headline_small as f32,
                    32.0,
                    0.0,
                ),
                title_large: TypeStyle::new(
                    MaterialFont::Brand,
                    MaterialWeight::Medium,
                    tokens.typography.title_large as f32,
                    28.0,
                    0.0,
                ),
                title_medium: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.title_medium as f32,
                    24.0,
                    0.15,
                ),
                title_small: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.title_small as f32,
                    22.0,
                    0.1,
                ),
                label_large: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.label_large as f32,
                    20.0,
                    0.1,
                ),
                label_medium: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.label_medium as f32,
                    16.0,
                    0.5,
                ),
                label_small: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Medium,
                    tokens.typography.label_small as f32,
                    14.0,
                    0.1,
                ),
                body_large: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Regular,
                    tokens.typography.body_large as f32,
                    24.0,
                    0.15,
                ),
                body_medium: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Regular,
                    tokens.typography.body_medium as f32,
                    20.0,
                    0.25,
                ),
                body_small: TypeStyle::new(
                    MaterialFont::Plain,
                    MaterialWeight::Regular,
                    tokens.typography.body_small as f32,
                    16.0,
                    0.4,
                ),
            },
            ..Default::default()
        };

        // Update other tokens as needed
        // TODO: Add conversion for radius, elevation, sizing

        Ok(material_tokens)
    }
}

/// Runtime theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: String,
    /// Material Design tokens
    pub material_tokens: MaterialTokens,
    /// Component style overrides
    pub component_overrides: Vec<ComponentOverride>,
}

impl Theme {
    /// Get the Material Design tokens
    #[must_use]
    pub const fn material_tokens(&self) -> &MaterialTokens {
        &self.material_tokens
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
                    label_small: 12,
                    label_medium: 14,
                    label_large: 16,
                    body_small: 14,
                    body_medium: 16,
                    body_large: 18,
                    title_small: 18,
                    title_medium: 20,
                    title_large: 24,
                    headline_small: 20,
                    headline_medium: 22,
                    headline_large: 26,
                    display_small: 24,
                    display_medium: 26,
                    display_large: 32,
                },
            },
            component_overrides: HashMap::new(),
        };

        assert!(ThemeLoader::validate_theme(&config).is_ok());
    }

    #[test]
    fn test_theme_serialization() {
        let theme = SerializableTheme {
            name: "Test Theme".to_string(),
            description: "A test theme".to_string(),
            material_tokens: SerializableMaterialTokens {
                spacing: SerializableSpacing {
                    xs: 4.0,
                    sm: 8.0,
                    md: 16.0,
                    lg: 24.0,
                    xl: 32.0,
                    xxl: 48.0,
                },
                typography: SerializableTypography {
                    label_small: 12,
                    label_medium: 14,
                    label_large: 16,
                    body_small: 14,
                    body_medium: 16,
                    body_large: 18,
                    title_small: 18,
                    title_medium: 20,
                    title_large: 24,
                    headline_small: 20,
                    headline_medium: 22,
                    headline_large: 26,
                    display_small: 24,
                    display_medium: 26,
                    display_large: 32,
                },
                radius: HashMap::new(),
                elevation: HashMap::new(),
                sizing: HashMap::new(),
            },
            component_overrides: Vec::new(),
        };

        let runtime_theme = theme.to_theme().unwrap();
        assert_eq!(runtime_theme.name, "Test Theme");
        assert_eq!(runtime_theme.description, "A test theme");
        assert_eq!(runtime_theme.material_tokens.spacing.xs, 4.0);
        assert_eq!(runtime_theme.material_tokens.spacing.xxl, 48.0);
    }
}
