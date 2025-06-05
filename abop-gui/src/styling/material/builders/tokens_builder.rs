//! Material Design tokens builder pattern
//!
//! This module provides a builder pattern for creating Material Design tokens
//! with customizable options. This is the foundation for Phase 3 implementation.

use iced::Color;

use crate::styling::material::{
    colors::MaterialColors, themes::theme_mode::ThemeMode, tokens::core::MaterialTokens,
};

/// Builder for creating customized Material Design tokens
///
/// This builder provides a fluent API for creating Material Design tokens
/// with various customization options. It will be expanded in Phase 3 to
/// support advanced configuration options.
#[derive(Debug, Default)]
pub struct MaterialTokensBuilder {
    /// Theme mode (light/dark/auto/custom)
    theme_mode: Option<ThemeMode>,
    /// Optional seed color for dynamic theming
    seed_color: Option<Color>,
    /// Optional custom color palette
    custom_colors: Option<MaterialColors>,
}

impl MaterialTokensBuilder {
    /// Create a new `MaterialTokensBuilder`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the theme mode
    #[must_use]
    pub const fn with_theme_mode(mut self, mode: ThemeMode) -> Self {
        self.theme_mode = Some(mode);
        self
    }

    /// Set a seed color for dynamic theming
    #[must_use]
    pub const fn with_seed_color(mut self, color: Color) -> Self {
        self.seed_color = Some(color);
        self
    }

    /// Set custom colors
    #[must_use]
    pub const fn with_custom_colors(mut self, colors: MaterialColors) -> Self {
        self.custom_colors = Some(colors);
        self
    }

    /// Build the `MaterialTokens` with the configured options
    #[must_use]
    pub fn build(self) -> MaterialTokens {
        // Phase 3 will implement full builder logic
        // For now, return default tokens
        match (self.theme_mode, self.seed_color, self.custom_colors) {
            (Some(ThemeMode::Light), None, None) => MaterialTokens::light(),
            (Some(ThemeMode::Dark), None, None) => MaterialTokens::dark(),
            (_, Some(seed), None) => {
                let is_dark = matches!(self.theme_mode, Some(ThemeMode::Dark));
                MaterialTokens::from_seed_color(seed, is_dark)
            }
            (_, _, Some(colors)) => MaterialTokens::default().with_colors(colors),
            _ => MaterialTokens::default(),
        }
    }
}

impl From<MaterialTokensBuilder> for MaterialTokens {
    fn from(builder: MaterialTokensBuilder) -> Self {
        builder.build()
    }
}
