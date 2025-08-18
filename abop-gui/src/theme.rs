//! Professional theme system for ABOP GUI
//!
//! This module provides a comprehensive theming system using Material Design 3 (MD3)
//! color standards with multiple theme modes including dark, light, and dynamic themes.

use crate::styling::material::{MaterialColors, MaterialPalette, MaterialTokens};
use iced::{Color, Theme as IcedTheme, theme::Palette};

// ================================================================================================
// CONSTANTS
// ================================================================================================

/// Default seed color for Material Dynamic theme (rich purple)
const DEFAULT_MATERIAL_SEED_COLOR: Color = Color::from_rgb(0.5, 0.2, 0.8);

// ================================================================================================
// THEME TYPES
// ================================================================================================

/// Theme modes available in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    /// Professional dark sunset theme with warm oranges and cool blues
    Dark,
    /// Professional light theme matching the dark sunset palette
    Light,
    /// System theme that follows OS preference
    System,
    /// Material Design dark theme
    MaterialDark,
    /// Material Design light theme
    MaterialLight,
    /// Dynamic Material theme based on seed color
    MaterialDynamic,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Dark
    }
}

impl ThemeMode {
    /// Get the Iced theme for this mode
    #[must_use]
    pub fn theme(&self) -> IcedTheme {
        match self {
            Self::Dark => dark_sunset_theme(),
            Self::Light => light_sunset_theme(),
            Self::System => {
                // For now, default to dark. Could be enhanced to detect OS theme
                dark_sunset_theme()
            }
            Self::MaterialDark => {
                let colors = MaterialColors::dark(&MaterialPalette::default());
                material_theme_from_colors(&colors)
            }
            Self::MaterialLight => {
                let colors = MaterialColors::light(&MaterialPalette::default());
                material_theme_from_colors(&colors)
            }
            Self::MaterialDynamic => {
                // Use a default purple seed color for dynamic theme - could be made configurable
                let seed_color = DEFAULT_MATERIAL_SEED_COLOR;
                let colors = MaterialColors::from_seed(seed_color, true);
                material_theme_from_colors(&colors)
            }
        }
    }
    /// Get the display name for this theme
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Dark => "Dark Sunset",
            Self::Light => "Light Sunset",
            Self::System => "System",
            Self::MaterialDark => "Material Dark",
            Self::MaterialLight => "Material Light",
            Self::MaterialDynamic => "Material Dynamic",
        }
    }
    /// Check if this is a dark theme
    #[must_use]
    pub const fn is_dark(&self) -> bool {
        matches!(
            self,
            Self::Dark | Self::System | Self::MaterialDark | Self::MaterialDynamic
        )
    }
    /// Centralized helper to resolve System theme to appropriate palette values
    /// Currently defaults to Dark palette for System mode as temporary behavior.
    /// TODO: Enhance to detect actual OS theme preference
    pub fn resolve_palette<T>(&self, dark_value: T, light_value: T) -> T {
        match self {
            Self::Dark | Self::System | Self::MaterialDark | Self::MaterialDynamic => dark_value,
            Self::Light | Self::MaterialLight => light_value,
        }
    }

    /// Get background color for the current theme mode
    #[must_use]
    pub fn background_color(&self) -> Color {
        self.get_colors().background
    }

    /// Get surface color for the current theme mode
    #[must_use]
    pub fn surface_color(&self) -> Color {
        self.get_colors().surface
    }

    /// Get surface variant color for the current theme mode
    #[must_use]
    pub fn surface_variant_color(&self) -> Color {
        self.get_colors().surface_variant
    }

    /// Get primary color for the current theme mode
    #[must_use]
    pub fn primary_color(&self) -> Color {
        self.get_colors().primary.base
    }

    /// Get primary light color for the current theme mode
    #[must_use]
    pub fn primary_light_color(&self) -> Color {
        self.get_colors().primary.container
    }

    /// Get secondary color for the current theme mode
    #[must_use]
    pub fn secondary_color(&self) -> Color {
        self.get_colors().secondary.base
    }

    /// Get primary text color for the current theme mode
    #[must_use]
    pub fn text_primary_color(&self) -> Color {
        self.get_colors().on_surface
    }

    /// Get secondary text color for the current theme mode
    #[must_use]
    pub fn text_secondary_color(&self) -> Color {
        self.get_colors().on_surface_variant
    }

    /// Get border color for the current theme mode
    #[must_use]
    pub fn border_color(&self) -> Color {
        self.get_colors().outline
    }

    /// Get outline color for the current theme mode
    #[must_use]
    pub fn outline_color(&self) -> Color {
        self.get_colors().outline_variant
    }

    /// Get error color for the current theme mode
    #[must_use]
    pub fn error_color(&self) -> Color {
        self.get_colors().error.base
    }

    /// Get success color for the current theme mode
    #[must_use]
    pub fn success_color(&self) -> Color {
        self.semantic_colors().success
    }

    /// Get info color for the current theme mode
    #[must_use]
    pub fn info_color(&self) -> Color {
        self.get_colors().primary.base // Use primary for info
    }

    /// Get orange accent color for rare/special use
    #[must_use]
    pub fn orange_accent_color(&self) -> Color {
        self.get_colors().secondary.base // Use secondary for accent
    }

    /// Get warning color for the current theme mode
    #[must_use]
    pub fn warning_color(&self) -> Color {
        self.get_colors().error.container // Use error container for warning
    }

    /// Get disabled text color for the current theme mode
    #[must_use]
    pub fn text_disabled_color(&self) -> Color {
        self.get_colors().on_surface_variant // Use surface variant for disabled text
    }

    /// Helper method to get the appropriate MaterialColors for this theme mode
    /// This centralizes the dark/light theme color selection logic
    #[must_use]
    fn get_colors(&self) -> MaterialColors {
        if self.is_dark() {
            MaterialColors::dark_default()
        } else {
            MaterialColors::light_default()
        }
    }
    /// Get semantic colors for the current theme mode
    #[must_use]
    pub fn semantic_colors(&self) -> crate::styling::material::SemanticColors {
        // Use the appropriate semantic color scheme for the theme
        if self.is_dark() {
            crate::styling::material::SemanticColors::dark()
        } else {
            crate::styling::material::SemanticColors::light()
        }
    }

    /// Get Material Design tokens for this theme mode if it's a Material theme
    /// Returns None for non-Material themes
    #[must_use]
    pub fn material_tokens_if_material(&self) -> Option<MaterialTokens> {
        match self {
            Self::MaterialDark => Some(MaterialTokens::dark()),
            Self::MaterialLight => Some(MaterialTokens::light()),
            Self::MaterialDynamic => {
                // Use purple seed color for dynamic Material Design theme
                let seed_color = DEFAULT_MATERIAL_SEED_COLOR;
                Some(MaterialTokens::from_seed_color(seed_color, true))
            }
            _ => None,
        }
    }

    /// Check if this is a Material Design theme
    #[must_use]
    pub const fn is_material(&self) -> bool {
        matches!(
            self,
            Self::MaterialDark | Self::MaterialLight | Self::MaterialDynamic
        )
    }

    /// Get Material tokens for the current theme
    ///
    /// # Returns
    /// Material design tokens for the current theme
    #[must_use]
    pub fn material_tokens(&self) -> MaterialTokens {
        match self {
            Self::Light | Self::MaterialLight => MaterialTokens::light(),
            Self::Dark | Self::System | Self::MaterialDark | Self::MaterialDynamic => {
                MaterialTokens::dark()
            } // TODO: Implement dynamic
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_modes_map_to_iced_theme() {
        // Ensure all primary modes produce an IcedTheme without panic
        let _ = ThemeMode::Dark.theme();
        let _ = ThemeMode::Light.theme();
        let _ = ThemeMode::System.theme();
    }

    #[test]
    fn is_dark_reflects_mode_intent() {
        assert!(ThemeMode::Dark.is_dark());
        assert!(!ThemeMode::Light.is_dark());
        // System currently resolves to dark by default
        assert!(ThemeMode::System.is_dark());
    }
}
/// Calculate the perceived luminance of a color using the standard formula
///
/// This uses the ITU-R BT.709 standard for calculating perceived brightness
/// which is more accurate than simple RGB averaging.
fn calculate_luminance(color: Color) -> f32 {
    // Convert to linear RGB first (assuming sRGB input)
    let linear_r = if color.r <= 0.03928 {
        color.r / 12.92
    } else {
        ((color.r + 0.055) / 1.055).powf(2.4)
    };
    let linear_g = if color.g <= 0.03928 {
        color.g / 12.92
    } else {
        ((color.g + 0.055) / 1.055).powf(2.4)
    };
    let linear_b = if color.b <= 0.03928 {
        color.b / 12.92
    } else {
        ((color.b + 0.055) / 1.055).powf(2.4)
    };

    // Calculate luminance using ITU-R BT.709 coefficients
    0.2126 * linear_r + 0.7152 * linear_g + 0.0722 * linear_b
}

// ================================================================================================
// THEME CONSTRUCTORS
// ================================================================================================

/// Create the dark sunset theme using Material Design colors
#[must_use]
pub fn dark_sunset_theme() -> IcedTheme {
    let colors = MaterialColors::dark(&MaterialPalette::from_seed(DEFAULT_MATERIAL_SEED_COLOR));
    let semantic = crate::styling::material::SemanticColors::dark();

    let palette = Palette {
        background: colors.background,
        text: colors.on_background,
        primary: colors.primary.base,
        success: semantic.success, // Use proper semantic green
        danger: colors.error.base,
    };

    IcedTheme::custom("Dark Sunset".to_string(), palette)
}

/// Create the light sunset theme using Material Design colors
#[must_use]
pub fn light_sunset_theme() -> IcedTheme {
    let colors = MaterialColors::light(&MaterialPalette::from_seed(DEFAULT_MATERIAL_SEED_COLOR));
    let semantic = crate::styling::material::SemanticColors::light();

    let palette = Palette {
        background: colors.background,
        text: colors.on_background,
        primary: colors.primary.base,
        success: semantic.success, // Use proper semantic green
        danger: colors.error.base,
    };

    IcedTheme::custom("Light Sunset".to_string(), palette)
}

/// Create a Material Design theme from `MaterialColors`
fn material_theme_from_colors(colors: &MaterialColors) -> IcedTheme {
    // Use proper luminance-based dark theme detection instead of just red channel
    let background_luminance = calculate_luminance(colors.background);
    let is_dark = background_luminance < 0.5;

    let semantic = if is_dark {
        crate::styling::material::SemanticColors::dark()
    } else {
        crate::styling::material::SemanticColors::light()
    };

    let palette = Palette {
        background: colors.background,
        text: colors.on_background,
        primary: colors.primary.base,
        success: semantic.success, // Use proper semantic green
        danger: colors.error.base,
    };
    IcedTheme::custom("Material".to_string(), palette)
}
