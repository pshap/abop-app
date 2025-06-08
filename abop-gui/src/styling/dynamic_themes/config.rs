//! Theme configuration structures

use super::{
    errors::ThemeLoadError,
    overrides::{ComponentOverride, ComponentType},
    serialization::{SerializableMaterialTokens, SerializableSemanticColors, ThemeMetadata},
};
use crate::styling::material::{
    spacing::SpacingTokens,
    tokens::{core::MaterialTokens, semantic::SemanticColors},
};
use serde::{Deserialize, Serialize};

/// Unified theme configuration for loading and runtime use
///
/// This unified structure eliminates redundancy between ThemeConfig and SerializableTheme
/// while maintaining compatibility with both file loading and runtime theme management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Theme metadata
    pub metadata: ThemeMetadata,
    /// Semantic color definitions
    pub semantic_colors: SerializableSemanticColors,
    /// Material Design tokens
    pub material_tokens: SerializableMaterialTokens,
    /// Component style overrides
    pub component_overrides: Vec<ComponentOverride>,
}

impl ThemeConfig {
    /// Convert to runtime theme
    ///
    /// This method converts the serializable theme configuration into a runtime-ready
    /// theme with proper Material Design tokens and validated color schemes.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Color parsing fails
    /// - Component override validation fails
    pub fn to_runtime_theme(&self) -> Result<Theme, ThemeLoadError> {
        // Validate component overrides
        for override_config in &self.component_overrides {
            override_config.validate().map_err(|e| {
                ThemeLoadError::ValidationError(format!(
                    "Component override validation failed: {}",
                    e
                ))
            })?;
        }

        Ok(Theme {
            name: self.metadata.name.clone(),
            description: self.metadata.description.clone().unwrap_or_default(),
            material_tokens: Self::convert_to_material_tokens(&self.material_tokens)?,
            component_overrides: self.component_overrides.clone(),
        })
    }

    /// Convert serializable material tokens to runtime tokens
    fn convert_to_material_tokens(
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

/// Custom theme mode created from loaded configuration
///
/// This structure represents a fully processed theme with runtime-ready tokens
/// and eliminates redundancy with the base ThemeConfig structure.
#[derive(Debug, Clone)]
pub struct CustomThemeMode {
    /// Theme metadata
    pub metadata: ThemeMetadata,
    /// Semantic colors
    pub semantic_colors: SemanticColors,
    /// Material Design tokens (runtime-ready)
    pub material_tokens: MaterialTokens,
    /// Type-safe component overrides
    pub component_overrides: Vec<ComponentOverride>,
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

    /// Get material tokens
    #[must_use]
    pub const fn material_tokens(&self) -> &MaterialTokens {
        &self.material_tokens
    }

    /// Get component overrides
    #[must_use]
    pub fn component_overrides(&self) -> &[ComponentOverride] {
        &self.component_overrides
    }

    /// Find component override by type and optional variant
    #[must_use]
    pub fn find_component_override(
        &self,
        component_type: ComponentType,
        variant: Option<&str>,
    ) -> Option<&ComponentOverride> {
        self.component_overrides.iter().find(|override_config| {
            override_config.component_type == component_type
                && override_config.variant.as_deref() == variant
        })
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

    /// Get component overrides
    #[must_use]
    pub fn component_overrides(&self) -> &[ComponentOverride] {
        &self.component_overrides
    }

    /// Find component override by type and optional variant
    #[must_use]
    pub fn find_component_override(
        &self,
        component_type: ComponentType,
        variant: Option<&str>,
    ) -> Option<&ComponentOverride> {
        self.component_overrides.iter().find(|override_config| {
            override_config.component_type == component_type
                && override_config.variant.as_deref() == variant
        })
    }

    /// Get theme name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get theme description
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}
