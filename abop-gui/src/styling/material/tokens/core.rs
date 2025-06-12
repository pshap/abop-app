//! Core Material Design token system
//!
//! This module contains the main `MaterialTokens` struct that serves as the central
//! hub for all Material Design tokens and helper functionality.

use iced::Color;

use crate::styling::material::{
    MaterialColors,
    elevation::MaterialElevation,
    helpers::{AnimationHelpers, ComponentHelpers, ElevationHelpers},
    shapes::{self, MaterialShapes},
    sizing::SizingTokens,
    spacing::SpacingTokens,
    tokens::{semantic::SemanticColors, states::MaterialStates},
    typography::MaterialTypography,
    visual::VisualTokens,
};

/// Constants for Material Design token system
mod constants {
    /// Default theme is dark for better contrast and readability
    pub const DEFAULT_THEME_IS_DARK: bool = true;
}

// Macro to generate getter functions for token types
macro_rules! token_getter {
    ($field:ident, $type:ty, $doc:expr) => {
        #[doc = $doc]
        #[must_use]
        pub const fn $field(&self) -> &$type {
            &self.$field
        }
    };
}

/// Complete Material Design 3 token system
///
/// This struct serves as the central hub for all Material Design tokens,
/// providing access to colors, typography, elevation, shapes, and other
/// design system components.
#[derive(Debug, Clone)]
pub struct MaterialTokens {
    /// Material Design color system with semantic roles
    pub colors: MaterialColors,
    /// Material Design typography scale
    pub typography: MaterialTypography,
    /// Material Design elevation system
    pub elevation: MaterialElevation,
    /// Material Design shape system (including M3 Expressive)
    pub shapes: shapes::MaterialShapes,
    /// State tokens for interactive elements
    pub states: MaterialStates,
    /// Application-specific sizing tokens
    pub sizing: SizingTokens,
    /// Application-specific spacing tokens
    pub spacing: SpacingTokens,
    /// Application-specific UI effect tokens
    pub ui: VisualTokens,
}

impl Default for MaterialTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialTokens {
    /// Create a new Material Design token system with default values (dark theme)
    #[must_use]
    pub fn new() -> Self {
        // Use dark theme by default for better contrast and readability
        if constants::DEFAULT_THEME_IS_DARK {
            Self::dark()
        } else {
            Self::light()
        }
    }

    /// Create Material tokens for dark theme
    #[must_use]
    pub fn dark() -> Self {
        Self::with_theme_colors(MaterialColors::dark_default())
    }

    /// Create Material tokens for light theme
    #[must_use]
    pub fn light() -> Self {
        Self::with_theme_colors(MaterialColors::light_default())
    }

    /// Create dynamic Material tokens from a seed color
    #[must_use]
    pub fn from_seed_color(seed: Color, is_dark: bool) -> Self {
        // Use real seed color generation to create a dynamic Material Design 3 palette
        Self::with_theme_colors(MaterialColors::from_seed(seed, is_dark))
    }

    /// Internal helper to create tokens with given colors
    fn with_theme_colors(colors: MaterialColors) -> Self {
        let elevation = MaterialElevation::new(&colors);
        Self {
            colors,
            elevation,
            typography: MaterialTypography::default(),
            shapes: shapes::MaterialShapes::default(),
            states: MaterialStates::default(),
            sizing: SizingTokens::default(),
            spacing: SpacingTokens::default(),
            ui: VisualTokens::default(),
        }
    }

    // Generated getter functions using macro to eliminate repetition
    token_getter!(colors, MaterialColors, "Get Material colors");
    token_getter!(typography, MaterialTypography, "Get Material typography");
    token_getter!(
        elevation,
        MaterialElevation,
        "Get Material elevation system"
    );
    token_getter!(shapes, MaterialShapes, "Get Material shapes");
    token_getter!(states, MaterialStates, "Get Material states");
    token_getter!(sizing, SizingTokens, "Get sizing tokens");
    token_getter!(spacing, SpacingTokens, "Get spacing tokens");
    token_getter!(ui, VisualTokens, "Get visual effect tokens");

    /// Update colors while preserving other tokens. Elevation is recreated with new colors.
    #[must_use]
    pub fn with_colors(mut self, colors: MaterialColors) -> Self {
        self.elevation = MaterialElevation::new(&colors);
        self.colors = colors;
        self
    }

    /// Update typography while preserving other tokens
    #[must_use]
    pub const fn with_typography(mut self, typography: MaterialTypography) -> Self {
        self.typography = typography;
        self
    }

    /// Update elevation while preserving other tokens
    #[must_use]
    pub const fn with_elevation(mut self, elevation: MaterialElevation) -> Self {
        self.elevation = elevation;
        self
    }

    /// Update shapes while preserving other tokens
    #[must_use]
    pub const fn with_shapes(mut self, shapes: MaterialShapes) -> Self {
        self.shapes = shapes;
        self
    }

    /// Update states while preserving other tokens
    #[must_use]
    pub const fn with_states(mut self, states: MaterialStates) -> Self {
        self.states = states;
        self
    }

    /// Update sizing tokens while preserving other tokens
    #[must_use]
    pub const fn with_sizing(mut self, sizing: SizingTokens) -> Self {
        self.sizing = sizing;
        self
    }

    /// Update spacing tokens while preserving other tokens
    #[must_use]
    pub const fn with_spacing(mut self, spacing: SpacingTokens) -> Self {
        self.spacing = spacing;
        self
    }

    /// Update visual tokens while preserving other tokens
    #[must_use]
    pub const fn with_ui(mut self, ui: VisualTokens) -> Self {
        self.ui = ui;
        self
    }

    /// Check if tokens are for dark theme
    #[must_use]
    pub fn is_dark_theme(&self) -> bool {
        // Check if background is darker than 0.5 luminance
        let bg = self.colors.background; // Field access, not method call
        bg.b.mul_add(0.114, bg.r.mul_add(0.299, bg.g * 0.587)) < 0.5
    }

    /// Get semantic colors using Material Design roles
    ///
    /// This provides a mapping between Material Design color roles and
    /// the application's semantic color system.
    #[must_use]
    pub fn semantic_colors(&self) -> SemanticColors {
        let colors = &self.colors;
        SemanticColors {
            primary: colors.primary.base,
            secondary: colors.secondary.base,
            success: colors.tertiary.base, // Using tertiary for success
            warning: colors.secondary.base, // Using secondary for warning
            error: colors.error.base,
            info: colors.primary.base, // Using primary for info
            surface: colors.surface,
            on_surface: colors.on_surface,
        }
    }

    /// Get the surface tint color for a specific elevation level
    ///
    /// This combines the surface color with the tint opacity to create the final tinted color.
    #[must_use]
    pub fn elevation_tint_color(&self, level: u8) -> iced::Color {
        let elevation_level = crate::styling::material::elevation::ElevationLevel::from_u8(level)
            .unwrap_or(crate::styling::material::elevation::ElevationLevel::Level0);
        let tint_opacity = self.elevation.get_level(elevation_level).tint_opacity;

        // Apply tint opacity to the surface color
        let surface_color = self.colors.surface;
        iced::Color {
            r: surface_color.r,
            g: surface_color.g,
            b: surface_color.b,
            a: surface_color.a * tint_opacity,
        }
    }

    /// Get the background color for the current theme
    pub fn background_color(&self) -> Color {
        self.colors.background
    }
}

// Implement helper traits for MaterialTokens
impl ElevationHelpers for MaterialTokens {
    fn elevation(&self) -> &MaterialElevation {
        &self.elevation
    }
}

impl AnimationHelpers for MaterialTokens {}

impl ComponentHelpers for MaterialTokens {}
