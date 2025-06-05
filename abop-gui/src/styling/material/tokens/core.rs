//! Core Material Design token system
//!
//! This module contains the main `MaterialTokens` struct that serves as the central
//! hub for all Material Design tokens and helper functionality.

use iced::Color;

use crate::styling::material::{
    colors::{self, MaterialColors, MaterialPalette},
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
    pub colors: colors::MaterialColors,
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
        Self::with_theme_colors(colors::MaterialColors::dark(
            &colors::MaterialPalette::default(),
        ))
    }

    /// Create Material tokens for light theme
    #[must_use]
    pub fn light() -> Self {
        Self::with_theme_colors(MaterialColors::light(&MaterialPalette::default()))
    }

    /// Create dynamic Material tokens from a seed color
    #[must_use]
    pub fn from_seed_color(seed: Color, is_dark: bool) -> Self {
        // Use real seed color generation to create a dynamic Material Design 3 palette
        Self::with_theme_colors(MaterialColors::from_seed(seed, is_dark))
    }

    /// Internal helper to create tokens with given colors
    fn with_theme_colors(colors: colors::MaterialColors) -> Self {
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
        let bg = self.colors.background;
        bg.b.mul_add(0.114, bg.r.mul_add(0.299, bg.g * 0.587)) < 0.5
    }

    /// Get semantic colors using Material Design roles
    ///
    /// This provides a mapping between Material Design color roles and
    /// the application's semantic color system.
    #[must_use]
    pub const fn semantic_colors(&self) -> SemanticColors {
        let colors = &self.colors;
        SemanticColors {
            primary: colors.primary.base,
            secondary: colors.secondary.base,
            success: colors.tertiary.base,
            warning: colors.secondary.base,
            error: colors.error.base,
            info: colors.primary.base,
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
}

// Implement helper traits for MaterialTokens
impl ElevationHelpers for MaterialTokens {
    fn elevation(&self) -> &MaterialElevation {
        &self.elevation
    }
}

impl AnimationHelpers for MaterialTokens {}

impl ComponentHelpers for MaterialTokens {}

// Backward compatibility methods that delegate to trait implementations
impl MaterialTokens {
    /// Get elevation style for cards (Level 1) - backward compatibility
    #[must_use]
    pub fn card_elevation(&self) -> &crate::styling::material::elevation::ElevationStyle {
        ElevationHelpers::card_elevation(self)
    }

    /// Get elevation style for menus and dialogs (Level 2) - backward compatibility
    #[must_use]
    pub fn menu_elevation(&self) -> &crate::styling::material::elevation::ElevationStyle {
        ElevationHelpers::menu_elevation(self)
    }

    /// Get elevation style for floating action buttons (Level 3) - backward compatibility
    #[must_use]
    pub fn fab_elevation(&self) -> &crate::styling::material::elevation::ElevationStyle {
        ElevationHelpers::fab_elevation(self)
    }

    /// Get elevation style for navigation bars (Level 2) - backward compatibility
    #[must_use]
    pub fn navigation_elevation(&self) -> &crate::styling::material::elevation::ElevationStyle {
        ElevationHelpers::navigation_elevation(self)
    }

    /// Get elevation style for modals and overlays (Level 3) - backward compatibility
    #[must_use]
    pub fn modal_elevation(&self) -> &crate::styling::material::elevation::ElevationStyle {
        ElevationHelpers::modal_elevation(self)
    }

    /// Get elevation style for tooltips and temporary surfaces (Level 2) - backward compatibility
    #[must_use]
    pub fn tooltip_elevation(&self) -> &crate::styling::material::elevation::ElevationStyle {
        ElevationHelpers::tooltip_elevation(self)
    }

    /// Get no elevation (Level 0) - backward compatibility
    #[must_use]
    pub fn no_elevation(&self) -> &crate::styling::material::elevation::ElevationStyle {
        ElevationHelpers::no_elevation(self)
    }

    /// Get elevation style for a specific level (0-5) - backward compatibility
    #[must_use]
    pub fn elevation_style(
        &self,
        level: u8,
    ) -> &crate::styling::material::elevation::ElevationStyle {
        ElevationHelpers::elevation_style(self, level)
    }

    /// Get just the shadow for a specific elevation level - backward compatibility
    #[must_use]
    pub fn elevation_shadow(&self, level: u8) -> iced::Shadow {
        ElevationHelpers::elevation_shadow(self, level)
    }

    /// Get the surface tint opacity for a specific elevation level - backward compatibility
    #[must_use]
    pub fn elevation_tint_opacity(&self, level: u8) -> f32 {
        ElevationHelpers::elevation_tint_opacity(self, level)
    }

    /// Get elevation transition for state changes - backward compatibility
    #[must_use]
    pub fn elevation_transition(
        &self,
        from_level: u8,
        to_level: u8,
    ) -> (iced::Shadow, iced::Shadow) {
        ElevationHelpers::elevation_transition(self, from_level, to_level)
    }

    /// Get elevation for button hover state (Level 1 -> Level 2) - backward compatibility
    #[must_use]
    pub fn button_hover_elevation(&self) -> (iced::Shadow, iced::Shadow) {
        ElevationHelpers::button_hover_elevation(self)
    }

    /// Get elevation for FAB hover state (Level 3 -> Level 4) - backward compatibility
    #[must_use]
    pub fn fab_hover_elevation(&self) -> (iced::Shadow, iced::Shadow) {
        ElevationHelpers::fab_hover_elevation(self)
    }

    // Animation helper methods - backward compatibility
    /// Create a fade in/out animation - backward compatibility
    #[must_use]
    pub fn fade_animation(&self) -> crate::styling::material::motion::Animation {
        AnimationHelpers::fade_animation(self)
    }

    /// Create a button press animation - backward compatibility
    #[must_use]
    pub fn button_animation(&self) -> crate::styling::material::motion::Animation {
        AnimationHelpers::button_animation(self)
    }

    /// Create a modal/dialog animation - backward compatibility
    #[must_use]
    pub fn modal_animation(&self) -> crate::styling::material::motion::Animation {
        AnimationHelpers::modal_animation(self)
    }

    /// Create a slide animation - backward compatibility
    #[must_use]
    pub fn slide_animation(&self) -> crate::styling::material::motion::Animation {
        AnimationHelpers::slide_animation(self)
    }

    /// Create a scale animation - backward compatibility
    #[must_use]
    pub fn scale_animation(&self) -> crate::styling::material::motion::Animation {
        AnimationHelpers::scale_animation(self)
    }

    /// Create a dismiss animation - backward compatibility
    #[must_use]
    pub fn dismiss_animation(&self) -> crate::styling::material::motion::Animation {
        AnimationHelpers::dismiss_animation(self)
    }

    /// Create a loading animation - backward compatibility
    #[must_use]
    pub fn loading_animation(&self) -> crate::styling::material::motion::Animation {
        AnimationHelpers::loading_animation(self)
    }

    /// Create a hover state animation - backward compatibility
    #[must_use]
    pub fn hover_animation(&self) -> crate::styling::material::motion::Animation {
        AnimationHelpers::hover_animation(self)
    }

    // Component helper methods - backward compatibility
    /// Create a Material Design card with proper elevation - backward compatibility
    #[must_use]
    pub fn card(&self) -> crate::styling::material::components::containers::MaterialCard {
        ComponentHelpers::card(self)
    }

    /// Create a Material Design progress indicator - backward compatibility
    #[must_use]
    pub fn progress_indicator(
        &self,
    ) -> crate::styling::material::components::feedback::MaterialProgressIndicator {
        ComponentHelpers::progress_indicator(self)
    }

    /// Create a Material Design notification - backward compatibility
    pub fn notification(
        &self,
        message: impl Into<String>,
    ) -> crate::styling::material::components::feedback::MaterialNotification {
        ComponentHelpers::notification(self, message)
    }

    /// Create a Material Design badge - backward compatibility
    #[must_use]
    pub fn badge(&self) -> crate::styling::material::components::feedback::MaterialBadge {
        ComponentHelpers::badge(self)
    }

    /// Create a Material Design status indicator - backward compatibility
    #[must_use]
    pub fn status_indicator(
        &self,
    ) -> crate::styling::material::components::feedback::MaterialStatusIndicator {
        ComponentHelpers::status_indicator(self)
    }
}
