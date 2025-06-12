//! Enhanced Material Design 3 Color System
//!
//! This module provides an enhanced implementation of the Material Design 3 color system
//! with improved support for dynamic theming, accessibility, and color manipulation.

use iced::Color;
use palette::{Srgb, Hsl, IntoColor, FromColor};

/// Material Design 3 color scheme
/// Provides semantic color roles for theming
#[derive(Debug, Clone)]
pub struct MaterialColors {
    primary_palette: TonalPalette,
    secondary_palette: TonalPalette,
    tertiary_palette: TonalPalette,
    neutral_palette: TonalPalette,
    neutral_variant_palette: TonalPalette,
    error_palette: TonalPalette,
    is_dark: bool,
}

/// Tonal palette for Material Design 3
/// Generates colors across different tones (0-100) based on hue and chroma
#[derive(Debug, Clone, PartialEq)]
pub struct TonalPalette {
    hue: f64,
    chroma: f64,
}

impl TonalPalette {
    /// Create a new TonalPalette from hue and chroma
    pub fn new(hue: f64, chroma: f64) -> Self {
        Self { hue, chroma }
    }

    /// Get the color for a specific tone (0-100)
    pub fn get_tone(&self, tone: u8) -> Color {
        let tone_value = tone as f64 / 100.0;
        
        // Create HSL color from hue, chroma (saturation), and tone (lightness)
        let hsl = Hsl::new(self.hue, self.chroma / 100.0, tone_value);
        let srgb: Srgb = hsl.into_color();
        
        Color::from_rgba(
            srgb.red as f32,
            srgb.green as f32, 
            srgb.blue as f32,
            1.0
        )
    }
}

/// Material palette containing all tonal palettes
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialPalette {
    primary: TonalPalette,
    secondary: TonalPalette,
    tertiary: TonalPalette,
    neutral: TonalPalette,
    neutral_variant: TonalPalette,
    error: TonalPalette,
}

impl MaterialPalette {
    /// Create a new MaterialPalette from a seed color
    pub fn from_seed(seed: Color) -> Self {
        // Convert seed color to HSL to extract hue
        let srgb = Srgb::new(seed.r as f64, seed.g as f64, seed.b as f64);
        let hsl: Hsl = srgb.into_color();
        
        let primary_hue = hsl.hue.into_positive_degrees();
        
        Self {
            primary: TonalPalette::new(primary_hue, 48.0),
            secondary: TonalPalette::new((primary_hue + 60.0) % 360.0, 16.0),
            tertiary: TonalPalette::new((primary_hue + 120.0) % 360.0, 24.0),
            neutral: TonalPalette::new(primary_hue, 4.0),
            neutral_variant: TonalPalette::new(primary_hue, 8.0),
            error: TonalPalette::new(25.0, 84.0), // Standard error hue and chroma
        }
    }

    /// Get the primary tonal palette
    pub fn primary(&self) -> TonalPalette {
        self.primary.clone()
    }

    /// Get the secondary tonal palette
    pub fn secondary(&self) -> TonalPalette {
        self.secondary.clone()
    }

    /// Get the tertiary tonal palette
    pub fn tertiary(&self) -> TonalPalette {
        self.tertiary.clone()
    }

    /// Get the neutral tonal palette
    pub fn neutral(&self) -> TonalPalette {
        self.neutral.clone()
    }

    /// Get the neutral variant tonal palette
    pub fn neutral_variant(&self) -> TonalPalette {
        self.neutral_variant.clone()
    }
    
    /// Get the error tonal palette
    pub fn error(&self) -> TonalPalette {
        self.error.clone()
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

    /// Create default light theme color scheme
    pub fn light_default() -> Self {
        Self::light()
    }

    /// Create default dark theme color scheme
    pub fn dark_default() -> Self {
        Self::dark()
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

    pub fn outline_variant(&self) -> Color {
        self.theme.core.outline_variant.into()
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

    /// Get the shadow color for elevation system
    pub fn shadow(&self) -> Color {
        if self.is_dark {
            Color::from_rgba(0.0, 0.0, 0.0, 0.5)
        } else {
            Color::from_rgba(0.0, 0.0, 0.0, 0.2)
        }
    }

    /// Get the surface tint color for elevation system
    pub fn surface_tint(&self) -> Color {
        // Return the primary color with some opacity for surface tint
        let mut color = self.primary();
        color.a = 0.1; // Adjust opacity as needed
        color
    }
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
