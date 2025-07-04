//! Unified Material Design 3 Color System for Iced 0.13.x
//!
//! This module provides the definitive implementation of Material Design 3 colors
//! optimized for Iced applications with comprehensive token coverage.

use crate::styling::material::seed::generate_palette_from_seed;
use iced::Color;

/// Complete tonal palette following Material Design 3 specifications
#[derive(Debug, Clone, PartialEq)]
pub struct TonalPalette {
    /// Array of 24 tonal colors from darkest (tone 0) to lightest (tone 100)
    pub tones: [Color; 24],
}

impl TonalPalette {
    /// Gets the color for a specific tone value (0-100)
    /// Returns the closest available tone from the predefined set
    #[must_use]
    pub fn get_tone(&self, tone: u8) -> Color {
        const TONE_INDICES: [u8; 24] = [
            0, 4, 6, 10, 12, 17, 20, 22, 24, 30, 40, 50, 60, 70, 80, 87, 90, 92, 94, 95, 96, 98,
            99, 100,
        ];
        let idx = TONE_INDICES.iter().position(|&t| t >= tone).unwrap_or(0);
        self.tones[idx]
    }
}

/// Complete Material Design 3 palette with all tonal scales
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialPalette {
    /// Primary color tonal palette
    pub primary: TonalPalette,
    /// Secondary color tonal palette
    pub secondary: TonalPalette,
    /// Tertiary color tonal palette
    pub tertiary: TonalPalette,
    /// Neutral color tonal palette
    pub neutral: TonalPalette,
    /// Neutral variant color tonal palette
    pub neutral_variant: TonalPalette,
    /// Error color tonal palette
    pub error: TonalPalette,
}

impl MaterialPalette {
    /// Creates a new `MaterialPalette` with default Material Design 3 colors
    #[must_use]
    pub fn new() -> Self {
        // Use Material Design 3 baseline color (#6750A4) for better contrast
        Self::from_seed(Color::from_rgb(0.404, 0.314, 0.643))
    }
    /// Creates a `MaterialPalette` from a seed color
    #[must_use]
    pub fn from_seed(seed: Color) -> Self {
        // Use the generate_palette_from_seed function directly since types are identical
        generate_palette_from_seed(seed)
    }
}

impl Default for MaterialPalette {
    fn default() -> Self {
        Self::new()
    }
}

/// Color role representing a complete color family (primary, secondary, tertiary, error)
#[derive(Debug, Clone, PartialEq)]
pub struct ColorRole {
    /// Base color for the role
    pub base: Color,
    /// Color for content on the base color  
    pub on_base: Color,
    /// Container color for the role
    pub container: Color,
    /// Color for content on the container
    pub on_container: Color,
    /// Fixed color variant (for surfaces that don't change between themes)
    pub fixed: Color,
    /// Dimmed fixed color variant
    pub fixed_dim: Color,
    /// Color for content on fixed surfaces
    pub on_fixed: Color,
    /// Variant color for content on fixed surfaces
    pub on_fixed_variant: Color,
}

/// **THE** unified Material Design 3 color system for Iced applications
///
/// This structure contains all MD3 color tokens with field-based access
/// for optimal ergonomics in UI component development.
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialColors {
    // Core color roles
    /// Primary color role (brand colors, key actions)
    pub primary: ColorRole,
    /// Secondary color role (supporting colors)
    pub secondary: ColorRole,
    /// Tertiary color role (accent colors)
    pub tertiary: ColorRole,
    /// Error color role (error states, warnings)
    pub error: ColorRole,

    // Surface colors
    /// Main surface color (primary background)
    pub surface: Color,
    /// Color for content on main surface
    pub on_surface: Color,
    /// Surface variant color (alternate backgrounds)
    pub surface_variant: Color,
    /// Color for content on surface variant
    pub on_surface_variant: Color,

    // Background colors
    /// Background color (behind all surfaces)
    pub background: Color,
    /// Color for content on background
    pub on_background: Color,

    // Outline colors
    /// Outline color for borders and dividers
    pub outline: Color,
    /// Variant outline color for subtle borders
    pub outline_variant: Color,

    // Inverse colors (for contrast)
    /// Inverse surface color
    pub inverse_surface: Color,
    /// Color for content on inverse surface
    pub inverse_on_surface: Color,
    /// Inverse primary color
    pub inverse_primary: Color,

    // System colors
    /// Shadow color for elevation
    pub shadow: Color,
    /// Scrim color for overlays
    pub scrim: Color,
    /// Surface tint color for elevation
    pub surface_tint: Color,

    // Surface container variants (MD3 elevation system)
    /// Standard surface container color
    pub surface_container: Color,
    /// Low emphasis surface container color
    pub surface_container_low: Color,
    /// Lowest emphasis surface container color  
    pub surface_container_lowest: Color,
    /// High emphasis surface container color
    pub surface_container_high: Color,
    /// Highest emphasis surface container color
    pub surface_container_highest: Color,

    // Additional surface variants
    /// Dimmed surface color
    pub surface_dim: Color,
    /// Bright surface color
    pub surface_bright: Color,
}

impl MaterialColors {
    /// Creates a light theme color scheme from the given palette
    #[must_use]
    pub fn light(palette: &MaterialPalette) -> Self {
        // Light theme tone mappings based on Material Design 3 specifications
        Self::create_from_palette(palette, false)
    }

    /// Creates a dark theme color scheme from the given palette
    #[must_use]
    pub fn dark(palette: &MaterialPalette) -> Self {
        // Dark theme tone mappings based on Material Design 3 specifications
        Self::create_from_palette(palette, true)
    }

    /// Creates a `MaterialColors` scheme from a seed color
    #[must_use]
    pub fn from_seed(seed: Color, is_dark: bool) -> Self {
        let palette = MaterialPalette::from_seed(seed);
        if is_dark {
            Self::dark(&palette)
        } else {
            Self::light(&palette)
        }
    }

    /// Creates a light theme using the default palette
    #[must_use]
    pub fn light_default() -> Self {
        Self::light(&MaterialPalette::default())
    }

    /// Creates a dark theme using the default palette
    #[must_use]
    pub fn dark_default() -> Self {
        Self::dark(&MaterialPalette::default())
    }

    /// Internal method to create colors from palette with theme-specific tone mappings
    fn create_from_palette(palette: &MaterialPalette, is_dark: bool) -> Self {
        if is_dark {
            Self::create_dark_scheme(palette)
        } else {
            Self::create_light_scheme(palette)
        }
    }
    /// Creates light theme color scheme with proper tone mappings
    ///
    /// Maps Material Design 3 tonal palette to semantic color roles for light themes.
    /// Tone values follow MD3 specifications:
    /// - Surface colors use high tones (90-100) for bright backgrounds
    /// - Text colors use low tones (10-30) for sufficient contrast
    /// - Container surfaces use mid-high tones (85-95) for subtle elevation
    fn create_light_scheme(palette: &MaterialPalette) -> Self {
        Self {
            // Color roles using helper function
            primary: Self::create_light_color_role(&palette.primary),
            secondary: Self::create_light_color_role(&palette.secondary),
            tertiary: Self::create_light_color_role(&palette.tertiary),
            error: Self::create_light_color_role(&palette.error),

            // Surface and background colors - MD3 light theme specification
            surface: palette.neutral.tones[19], // tone 98 - main surface, very light
            on_surface: palette.neutral.tones[3], // tone 10 - text on surface, high contrast
            surface_variant: palette.neutral_variant.tones[16], // tone 90 - subtle surface variation
            on_surface_variant: palette.neutral_variant.tones[9], // tone 30 - lower contrast text
            background: palette.neutral.tones[19], // tone 98 - app background, matches surface
            on_background: palette.neutral.tones[3], // tone 10 - text on background

            // Outline colors for borders and dividers
            outline: palette.neutral_variant.tones[10], // tone 50 - medium contrast borders
            outline_variant: palette.neutral_variant.tones[14], // tone 80 - subtle borders

            // Inverse colors for high-contrast elements (dark-on-light inversion)
            inverse_surface: palette.neutral.tones[6], // tone 20 - dark surface for contrast
            inverse_on_surface: palette.neutral.tones[17], // tone 95 - light text on dark
            inverse_primary: palette.primary.tones[14], // tone 80 - accessible primary on dark

            // System colors for shadows and overlays
            shadow: palette.neutral.tones[0], // tone 0 - pure black for shadows
            scrim: palette.neutral.tones[0],  // tone 0 - overlay backgrounds
            surface_tint: palette.primary.tones[10], // tone 40 - primary color for elevation tinting            // Surface container variants for different elevation levels
            surface_container: palette.neutral.tones[15], // tone 94 - elevated containers
            surface_container_low: palette.neutral.tones[17], // tone 96 - subtle elevation
            surface_container_lowest: palette.neutral.tones[20], // tone 100 - no elevation
            surface_container_high: palette.neutral.tones[13], // tone 92 - moderate elevation
            surface_container_highest: palette.neutral.tones[11], // tone 90 - highest elevation

            // Additional surface variants for light/dark surface distinction
            surface_dim: palette.neutral.tones[14], // tone 87 - dimmed surface variant
            surface_bright: palette.neutral.tones[19], // tone 98 - bright surface (matches main surface)
        }
    }

    /// Creates dark theme color scheme with proper tone mappings
    ///
    /// Maps Material Design 3 tonal palette to semantic color roles for dark themes.
    /// Tone values follow MD3 dark theme specifications:
    /// - Surface colors use low tones (6-30) for dark backgrounds
    /// - Text colors use high tones (80-95) for sufficient contrast on dark
    /// - Container surfaces use mid-low tones (10-25) for subtle elevation hierarchy
    fn create_dark_scheme(palette: &MaterialPalette) -> Self {
        Self {
            // Color roles using helper function
            primary: Self::create_dark_color_role(&palette.primary),
            secondary: Self::create_dark_color_role(&palette.secondary),
            tertiary: Self::create_dark_color_role(&palette.tertiary),
            error: Self::create_dark_color_role(&palette.error),

            // Surface and background colors - MD3 dark theme specification
            surface: palette.neutral.tones[2], // tone 6 - main dark surface
            on_surface: palette.neutral.tones[17], // tone 90 - light text on dark surface
            surface_variant: palette.neutral_variant.tones[7], // tone 30 - subtle dark surface variation
            on_surface_variant: palette.neutral_variant.tones[14], // tone 80 - medium contrast text
            background: palette.neutral.tones[2], // tone 6 - app background, matches surface
            on_background: palette.neutral.tones[17], // tone 90 - light text on dark background

            // Outline colors for borders and dividers in dark theme
            outline: palette.neutral_variant.tones[12], // tone 60 - visible borders on dark
            outline_variant: palette.neutral_variant.tones[7], // tone 30 - subtle borders

            // Inverse colors for light-on-dark elements (light-on-dark inversion)
            inverse_surface: palette.neutral.tones[17], // tone 90 - light surface for contrast
            inverse_on_surface: palette.neutral.tones[5], // tone 20 - dark text on light
            inverse_primary: palette.primary.tones[10], // tone 40 - accessible primary on light

            // System colors for shadows and overlays
            shadow: palette.neutral.tones[0], // tone 0 - pure black for shadows
            scrim: palette.neutral.tones[0],  // tone 0 - overlay backgrounds
            surface_tint: palette.primary.tones[14], // tone 80 - primary color for dark elevation tinting            // Surface container variants for different elevation levels in dark theme
            surface_container: palette.neutral.tones[4], // tone 12 - elevated containers
            surface_container_low: palette.neutral.tones[3], // tone 10 - subtle elevation
            surface_container_lowest: palette.neutral.tones[1], // tone 4 - no elevation
            surface_container_high: palette.neutral.tones[6], // tone 17 - moderate elevation
            surface_container_highest: palette.neutral.tones[8], // tone 22 - highest elevation

            // Additional surface variants for light/dark surface distinction
            surface_dim: palette.neutral.tones[2], // tone 6 - dimmed surface (matches main surface)
            surface_bright: palette.neutral.tones[8], // tone 24 - bright surface variant
        }
    }

    /// Helper function to create a ColorRole with proper tone mappings for light theme
    fn create_light_color_role(palette: &TonalPalette) -> ColorRole {
        ColorRole {
            base: palette.tones[10],            // tone 40
            on_base: palette.tones[23],         // tone 100
            container: palette.tones[16],       // tone 90
            on_container: palette.tones[3],     // tone 10
            fixed: palette.tones[16],           // tone 90
            fixed_dim: palette.tones[14],       // tone 80
            on_fixed: palette.tones[3],         // tone 10
            on_fixed_variant: palette.tones[7], // tone 30
        }
    }

    /// Helper function to create a ColorRole with proper tone mappings for dark theme
    fn create_dark_color_role(palette: &TonalPalette) -> ColorRole {
        ColorRole {
            base: palette.tones[14],            // tone 80
            on_base: palette.tones[5],          // tone 20
            container: palette.tones[7],        // tone 30
            on_container: palette.tones[16],    // tone 90
            fixed: palette.tones[16],           // tone 90
            fixed_dim: palette.tones[14],       // tone 80
            on_fixed: palette.tones[3],         // tone 10
            on_fixed_variant: palette.tones[7], // tone 30
        }
    }
}

impl Default for MaterialColors {
    fn default() -> Self {
        Self::light_default()
    }
}

/// Extension trait providing compatibility methods for components expecting method-based access
impl MaterialColors {
    /// Primary color (method-based access for compatibility)
    #[inline]
    pub fn primary_color(&self) -> Color {
        self.primary.base
    }

    /// On-primary color (method-based access for compatibility)
    #[inline]
    pub fn on_primary_color(&self) -> Color {
        self.primary.on_base
    }

    /// Primary container color (method-based access for compatibility)
    #[inline]
    pub fn primary_container_color(&self) -> Color {
        self.primary.container
    }

    /// On-primary container color (method-based access for compatibility)
    #[inline]
    pub fn on_primary_container_color(&self) -> Color {
        self.primary.on_container
    }

    /// Surface color (method-based access for compatibility)
    #[inline]
    pub fn surface_color(&self) -> Color {
        self.surface
    }

    /// On-surface color (method-based access for compatibility)
    #[inline]
    pub fn on_surface_color(&self) -> Color {
        self.on_surface
    }

    // Convenience methods for common field access patterns
    /// Primary container color (shorthand for primary.container)
    pub fn primary_container(&self) -> Color {
        self.primary.container
    }

    /// On-primary color (shorthand for primary.on_base)
    pub fn on_primary(&self) -> Color {
        self.primary.on_base
    }

    /// On-primary container color (shorthand for primary.on_container)
    pub fn on_primary_container(&self) -> Color {
        self.primary.on_container
    }

    /// Secondary container color (shorthand for secondary.container)
    pub fn secondary_container(&self) -> Color {
        self.secondary.container
    }

    /// On-secondary container color (shorthand for secondary.on_container)
    pub fn on_secondary_container(&self) -> Color {
        self.secondary.on_container
    }

    /// On-error color (shorthand for error.on_base)
    pub fn on_error(&self) -> Color {
        self.error.on_base
    }

    /// Error container color (shorthand for error.container)
    pub fn error_container(&self) -> Color {
        self.error.container
    }

    /// On-error container color (shorthand for error.on_container)
    pub fn on_error_container(&self) -> Color {
        self.error.on_container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_colors_creation() {
        let colors = MaterialColors::light_default();

        // Test that all colors are valid
        assert!(colors.primary.base.r >= 0.0 && colors.primary.base.r <= 1.0);
        assert!(colors.surface.r >= 0.0 && colors.surface.r <= 1.0);
        assert!(colors.background.r >= 0.0 && colors.background.r <= 1.0);
    }

    #[test]
    fn test_dark_vs_light_themes() {
        let light = MaterialColors::light_default();
        let dark = MaterialColors::dark_default();

        // Dark and light themes should be different
        assert_ne!(light.surface, dark.surface);
        assert_ne!(light.primary.base, dark.primary.base);
    }

    #[test]
    fn test_seed_color_generation() {
        let seed = Color::from_rgb(0.2, 0.6, 0.9);
        let colors = MaterialColors::from_seed(seed, false);

        // Should generate valid colors
        assert!(colors.primary.base.r >= 0.0 && colors.primary.base.r <= 1.0);
    }

    #[test]
    fn test_compatibility_methods() {
        let colors = MaterialColors::light_default();

        // Compatibility methods should match field access
        assert_eq!(colors.primary_color(), colors.primary.base);
        assert_eq!(colors.surface_color(), colors.surface);
    }

    #[test]
    fn test_tonal_palette_access() {
        let palette = MaterialPalette::default();

        // Should be able to get different tones
        let tone_0 = palette.primary.get_tone(0);
        let tone_50 = palette.primary.get_tone(50);
        let tone_100 = palette.primary.get_tone(100);

        assert_ne!(tone_0, tone_50);
        assert_ne!(tone_50, tone_100);
    }
}
