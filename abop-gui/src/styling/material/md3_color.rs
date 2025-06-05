//! Enhanced Material Design 3 Color System
//!
//! This module provides an enhanced implementation of the Material Design 3 color system
//! with improved support for dynamic theming, accessibility, and color manipulation.

use crate::material::color::{
    Theme, ThemeVariant,
    palette::{MaterialPalette as CoreMaterialPalette, TonalPalette as CoreTonalPalette},
    scheme::Theme as CoreTheme,
};
use iced::Color;

/// Material Design 3 color scheme
/// Provides semantic color roles for theming
pub struct MaterialColors {
    theme: CoreTheme,
    is_dark: bool,
}

/// Enhanced TonalPalette that wraps the core implementation
#[derive(Debug, Clone, PartialEq)]
pub struct TonalPalette {
    inner: CoreTonalPalette,
}

impl TonalPalette {
    /// Create a new TonalPalette from hue and chroma
    pub fn new(hue: f64, chroma: f64) -> Self {
        Self {
            inner: CoreTonalPalette::new(hue, chroma),
        }
    }

    /// Get the color for a specific tone (0-100)
    pub fn get_tone(&self, tone: u8) -> Color {
        let tone = tone as f64 / 100.0;
        let srgb = self.inner.get(tone);
        Color::from_rgba(srgb.r as f32, srgb.g as f32, srgb.b as f32, 1.0)
    }
}

/// Enhanced MaterialPalette that wraps the core implementation
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialPalette {
    inner: CoreMaterialPalette,
}

impl MaterialPalette {
    /// Create a new MaterialPalette from a seed color
    pub fn from_seed(seed: Color) -> Self {
        let srgb = crate::material::color::Srgb::new(seed.r as f64, seed.g as f64, seed.b as f64);
        Self {
            inner: CoreMaterialPalette::from_seed(srgb),
        }
    }

    /// Get the primary tonal palette
    pub fn primary(&self) -> TonalPalette {
        TonalPalette {
            inner: self.inner.primary.clone(),
        }
    }

    /// Get the secondary tonal palette
    pub fn secondary(&self) -> TonalPalette {
        TonalPalette {
            inner: self.inner.secondary.clone(),
        }
    }

    /// Get the tertiary tonal palette
    pub fn tertiary(&self) -> TonalPalette {
        TonalPalette {
            inner: self.inner.tertiary.clone(),
        }
    }

    /// Get the neutral tonal palette
    pub fn neutral(&self) -> TonalPalette {
        TonalPalette {
            inner: self.inner.neutral.clone(),
        }
    }

    /// Get the neutral variant tonal palette
    pub fn neutral_variant(&self) -> TonalPalette {
        TonalPalette {
            inner: self.inner.neutral_variant.clone(),
        }
    }
}

impl MaterialColors {
    /// Create a light theme color scheme
    pub fn light() -> Self {
        Self {
            theme: CoreTheme::light(),
            is_dark: false,
        }
    }

    /// Create a dark theme color scheme
    pub fn dark() -> Self {
        Self {
            theme: CoreTheme::dark(),
            is_dark: true,
        }
    }

    /// Create a color scheme from a seed color
    pub fn from_seed(seed: Color, is_dark: bool) -> Self {
        let srgb = crate::material::color::Srgb::new(seed.r as f64, seed.g as f64, seed.b as f64);
        let variant = if is_dark {
            ThemeVariant::Dark
        } else {
            ThemeVariant::Light
        };
        Self {
            theme: CoreTheme::from_seed(srgb, variant),
            is_dark,
        }
    }

    // Primary colors
    pub fn primary(&self) -> Color {
        self.theme.core.primary.into()
    }

    pub fn on_primary(&self) -> Color {
        self.theme.core.on_primary.into()
    }

    pub fn primary_container(&self) -> Color {
        self.theme.core.primary_container.into()
    }

    pub fn on_primary_container(&self) -> Color {
        self.theme.core.on_primary_container.into()
    }

    // Secondary colors
    pub fn secondary(&self) -> Color {
        self.theme.core.secondary.into()
    }

    pub fn on_secondary(&self) -> Color {
        self.theme.core.on_secondary.into()
    }

    pub fn secondary_container(&self) -> Color {
        self.theme.core.secondary_container.into()
    }

    pub fn on_secondary_container(&self) -> Color {
        self.theme.core.on_secondary_container.into()
    }

    // Tertiary colors
    pub fn tertiary(&self) -> Color {
        self.theme.core.tertiary.into()
    }

    pub fn on_tertiary(&self) -> Color {
        self.theme.core.on_tertiary.into()
    }

    pub fn tertiary_container(&self) -> Color {
        self.theme.core.tertiary_container.into()
    }

    pub fn on_tertiary_container(&self) -> Color {
        self.theme.core.on_tertiary_container.into()
    }

    // Error colors
    pub fn error(&self) -> Color {
        self.theme.core.error.into()
    }

    pub fn on_error(&self) -> Color {
        self.theme.core.on_error.into()
    }

    pub fn error_container(&self) -> Color {
        self.theme.core.error_container.into()
    }

    pub fn on_error_container(&self) -> Color {
        self.theme.core.on_error_container.into()
    }

    // Background colors
    pub fn background(&self) -> Color {
        self.theme.core.background.into()
    }

    pub fn on_background(&self) -> Color {
        self.theme.core.on_background.into()
    }

    // Surface colors
    pub fn surface(&self) -> Color {
        self.theme.core.surface.into()
    }

    pub fn on_surface(&self) -> Color {
        self.theme.core.on_surface.into()
    }

    pub fn surface_variant(&self) -> Color {
        self.theme.core.surface_variant.into()
    }

    pub fn on_surface_variant(&self) -> Color {
        self.theme.core.on_surface_variant.into()
    }

    // Outline colors
    pub fn outline(&self) -> Color {
        self.theme.core.outline.into()
    }

    // Inverse colors
    pub fn inverse_surface(&self) -> Color {
        self.theme.core.inverse_surface.into()
    }

    pub fn inverse_on_surface(&self) -> Color {
        self.theme.core.inverse_on_surface.into()
    }

    pub fn inverse_primary(&self) -> Color {
        self.theme.core.inverse_primary.into()
    }

    // Shadow and scrim
    pub fn shadow(&self) -> Color {
        self.theme.core.shadow.into()
    }

    pub fn scrim(&self) -> Color {
        self.theme.core.scrim.into()
    }

    // Surface tint
    pub fn surface_tint(&self) -> Color {
        self.theme.core.surface_tint.into()
    }

    // Surface dim/bright
    pub fn surface_dim(&self) -> Color {
        if self.is_dark {
            Color::from_rgb(0.12, 0.12, 0.12)
        } else {
            Color::from_rgb(0.93, 0.93, 0.93)
        }
    }

    pub fn surface_bright(&self) -> Color {
        if self.is_dark {
            Color::from_rgb(0.24, 0.24, 0.24)
        } else {
            Color::from_rgb(0.98, 0.98, 0.98)
        }
    }

    // Surface container colors
    pub fn surface_container_lowest(&self) -> Color {
        if self.is_dark {
            Color::from_rgb(0.12, 0.12, 0.12)
        } else {
            Color::WHITE
        }
    }

    pub fn surface_container_low(&self) -> Color {
        if self.is_dark {
            Color::from_rgb(0.26, 0.26, 0.26)
        } else {
            Color::from_rgb(0.98, 0.97, 0.98)
        }
    }

    pub fn surface_container(&self) -> Color {
        if self.is_dark {
            Color::from_rgb(0.30, 0.30, 0.30)
        } else {
            Color::from_rgb(0.97, 0.96, 0.97)
        }
    }

    pub fn surface_container_high(&self) -> Color {
        if self.is_dark {
            Color::from_rgb(0.36, 0.36, 0.36)
        } else {
            Color::from_rgb(0.95, 0.94, 0.95)
        }
    }

    pub fn surface_container_highest(&self) -> Color {
        if self.is_dark {
            Color::from_rgb(0.42, 0.42, 0.42)
        } else {
            Color::from_rgb(0.93, 0.92, 0.93)
        }
    }
}

impl MaterialColors {
    /// Create a light theme color scheme
    pub fn light() -> Self {
        Self {
            theme: CoreTheme::light(),
        }
    }

    /// Create a dark theme color scheme
    pub fn dark() -> Self {
        Self {
            theme: CoreTheme::dark(),
        }
    }

    /// Create a color scheme from a seed color
    pub fn from_seed(seed: Color, is_dark: bool) -> Self {
        let srgb = crate::material::color::Srgb::new(seed.r as f64, seed.g as f64, seed.b as f64);
        let variant = if is_dark {
            ThemeVariant::Dark
        } else {
            ThemeVariant::Light
        };
        Self {
            theme: CoreTheme::from_seed(srgb, variant),
        }
    }

    /// Get the primary color
    pub fn primary(&self) -> Color {
        self.theme.core.primary.into()
    }

    /// Get the primary container color
    pub fn primary_container(&self) -> Color {
        self.theme.core.primary_container.into()
    }

    /// Get the on primary color
    pub fn on_primary(&self) -> Color {
        self.theme.core.on_primary.into()
    }

    /// Get the on primary container color
    pub fn on_primary_container(&self) -> Color {
        self.theme.core.on_primary_container.into()
    }

    // Add more color getters as needed...
}

impl From<Color> for MaterialPalette {
    fn from(color: Color) -> Self {
        Self::from_seed(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tonal_palette() {
        let palette = TonalPalette::new(120.0, 40.0);
        let color = palette.get_tone(40);
        assert!(color.r > 0.0);
        assert!(color.g > 0.0);
        assert!(color.b > 0.0);
    }

    #[test]
    fn test_material_palette() {
        let palette = MaterialPalette::from_seed(Color::new(0.5, 0.2, 0.8, 1.0));
        let primary = palette.primary();
        let color = primary.get_tone(40);
        assert!(color.r > 0.0);
        assert!(color.g > 0.0);
        assert!(color.b > 0.0);
    }

    #[test]
    fn test_material_colors() {
        let colors = MaterialColors::light();
        let primary = colors.primary();
        assert!(primary.r > 0.0);
        assert!(primary.g > 0.0);
        assert!(primary.b > 0.0);

        let dark_colors = MaterialColors::dark();
        let dark_primary = dark_colors.primary();
        assert_ne!(primary, dark_primary);
    }
}
