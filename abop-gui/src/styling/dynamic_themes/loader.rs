//! Theme loading functionality

use super::{
    config::{CustomThemeMode, ThemeConfig},
    errors::ThemeLoadError,
    serialization::SerializableMaterialTokens,
};
use crate::styling::material::{spacing::SpacingTokens, tokens::core::MaterialTokens};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Dynamic theme loader for loading themes from files
pub struct ThemeLoader {
    /// Cache of loaded themes
    #[cfg(test)]
    pub theme_cache: HashMap<String, ThemeConfig>,
    #[cfg(not(test))]
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
    /// - Material token conversion fails
    /// - Theme configuration is invalid
    /// - Component override validation fails
    pub fn create_theme_mode(
        &self,
        config: &ThemeConfig,
    ) -> Result<CustomThemeMode, ThemeLoadError> {
        let semantic_colors = config.semantic_colors.to_semantic_colors()?;

        // Validate component overrides
        for override_config in &config.component_overrides {
            override_config.validate().map_err(|e| {
                ThemeLoadError::ValidationError(format!(
                    "Component override validation failed: {e}"
                ))
            })?;
        }

        Ok(CustomThemeMode {
            metadata: config.metadata.clone(),
            semantic_colors,
            material_tokens: Self::convert_material_tokens(&config.material_tokens)?,
            component_overrides: config.component_overrides.clone(),
        })
    }

    /// Validate a loaded theme configuration
    #[cfg(test)]
    pub fn validate_theme(config: &ThemeConfig) -> Result<(), ThemeLoadError> {
        if config.metadata.name.is_empty() {
            return Err(ThemeLoadError::MissingField("metadata.name".to_string()));
        }

        // Try to parse colors to validate them
        config.semantic_colors.to_semantic_colors()?;

        // Validate component overrides
        for override_config in &config.component_overrides {
            override_config.validate().map_err(|e| {
                ThemeLoadError::ValidationError(format!(
                    "Component override validation failed: {e}"
                ))
            })?;
        }

        Ok(())
    }

    /// Validate a loaded theme configuration
    #[cfg(not(test))]
    fn validate_theme(config: &ThemeConfig) -> Result<(), ThemeLoadError> {
        if config.metadata.name.is_empty() {
            return Err(ThemeLoadError::MissingField("metadata.name".to_string()));
        }

        // Try to parse colors to validate them
        config.semantic_colors.to_semantic_colors()?;

        // Validate component overrides
        for override_config in &config.component_overrides {
            override_config.validate().map_err(|e| {
                ThemeLoadError::ValidationError(format!(
                    "Component override validation failed: {e}"
                ))
            })?;
        }

        Ok(())
    }

    /// Convert serializable material tokens to runtime tokens
    ///
    /// This unified method replaces the old convert_design_tokens method
    /// and handles the complete material token conversion process.
    fn convert_material_tokens(
        tokens: &SerializableMaterialTokens,
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
            typography: tokens.typography.to_material_typography(),
            ..Default::default()
        };

        // TODO: Add conversion for radius, elevation, sizing in Phase 3
        // For now, these remain as default values

        Ok(material_tokens)
    }
}

impl Default for ThemeLoader {
    fn default() -> Self {
        Self::new()
    }
}
