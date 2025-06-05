//! Design token system for ABOP GUI
//!
//! This module provides a centralized design token system for consistent
//! spacing, typography, sizing, and other design values across the application.

pub mod colors;
pub mod components;
pub mod constants;
pub mod elevation;
pub mod material;
pub mod radius;
pub mod shadows;
pub mod sizing;
pub mod spacing;
pub mod typography;
pub mod ui;

use crate::styling::material::MaterialTokens;

pub use colors::SemanticColors;
pub use components::ComponentTokens;
pub use elevation::ElevationTokens;
pub use radius::RadiusTokens;
pub use sizing::SizingTokens;
pub use spacing::SpacingTokens;
pub use typography::TypographyTokens;
pub use ui::VisualTokens;

// Re-export constants for convenience
pub use constants::*;
pub use shadows::*;

/// Design token system containing all design constants
#[derive(Debug, Clone)]
pub struct DesignTokens {
    /// Spacing tokens for consistent padding, margins, and gaps
    pub spacing: SpacingTokens,
    /// Typography tokens for consistent font sizes
    pub typography: TypographyTokens,
    /// Border radius tokens for consistent rounded corners
    pub radius: RadiusTokens,
    /// Elevation tokens for consistent shadows and depth
    pub elevation: ElevationTokens,
    /// Sizing tokens for consistent component dimensions
    pub sizing: SizingTokens,
    /// Semantic color tokens for consistent color usage
    pub semantic_colors: SemanticColors,
    /// Component-specific tokens for specialized styling
    pub components: ComponentTokens,
    /// Visual treatment tokens for UI effects
    pub ui: VisualTokens,
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl DesignTokens {
    /// Create a new set of design tokens with default values
    #[must_use]
    pub fn new() -> Self {
        Self {
            spacing: SpacingTokens::default(),
            typography: TypographyTokens::default(),
            radius: RadiusTokens::default(),
            elevation: ElevationTokens::default(),
            sizing: SizingTokens::default(),
            semantic_colors: SemanticColors::default(),
            components: ComponentTokens::default(),
            ui: VisualTokens::default(),
        }
    }

    /// Create design tokens based on Material Design system
    #[must_use]
    pub fn material() -> Self {
        let material_tokens = MaterialTokens::new();
        Self::from_material_tokens(&material_tokens)
    }

    /// Create design tokens from Material Design tokens
    #[must_use]
    pub fn from_material_tokens(material_tokens: &MaterialTokens) -> Self {
        material::create_from_material_tokens(material_tokens)
    }

    /// Merge Material Design colors with existing design tokens
    #[must_use]
    pub const fn with_material_colors(self, material_tokens: &MaterialTokens) -> Self {
        material::merge_material_colors(self, material_tokens)
    }
}
