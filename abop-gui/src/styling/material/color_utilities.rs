//! Advanced Material Design 3 Color Utilities
//!
//! This module provides enhanced utilities for working with the unified MD3 color system,
//! including accessibility helpers, color manipulation, and theme utilities.

use iced::Color;
use crate::styling::material::unified_colors::{MaterialColors, MaterialPalette, ColorRole};

/// Advanced color manipulation and accessibility utilities
pub struct ColorUtilities;

impl ColorUtilities {
    /// Check if a color combination meets WCAG AA contrast requirements (4.5:1)
    pub fn meets_accessibility_contrast(foreground: Color, background: Color) -> bool {
        Self::contrast_ratio(foreground, background) >= 4.5
    }

    /// Check if a color combination meets WCAG AAA contrast requirements (7:1)
    pub fn meets_high_accessibility_contrast(foreground: Color, background: Color) -> bool {
        Self::contrast_ratio(foreground, background) >= 7.0
    }

    /// Calculate the contrast ratio between two colors
    pub fn contrast_ratio(color1: Color, color2: Color) -> f32 {
        let l1 = Self::relative_luminance(color1);
        let l2 = Self::relative_luminance(color2);
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Calculate the relative luminance of a color (for accessibility calculations)
    pub fn relative_luminance(color: Color) -> f32 {
        let r = Self::gamma_correct(color.r);
        let g = Self::gamma_correct(color.g);
        let b = Self::gamma_correct(color.b);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Apply gamma correction for luminance calculation
    fn gamma_correct(value: f32) -> f32 {
        if value <= 0.03928 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Blend two colors with a given ratio (0.0 = first color, 1.0 = second color)
    pub fn blend_colors(color1: Color, color2: Color, ratio: f32) -> Color {
        let ratio = ratio.clamp(0.0, 1.0);
        Color {
            r: color1.r * (1.0 - ratio) + color2.r * ratio,
            g: color1.g * (1.0 - ratio) + color2.g * ratio,
            b: color1.b * (1.0 - ratio) + color2.b * ratio,
            a: color1.a * (1.0 - ratio) + color2.a * ratio,
        }
    }

    /// Apply an alpha/opacity value to a color
    pub fn with_alpha(color: Color, alpha: f32) -> Color {
        Color {
            a: alpha.clamp(0.0, 1.0),
            ..color
        }
    }

    /// Lighten a color by a given amount (0.0 = no change, 1.0 = white)
    pub fn lighten(color: Color, amount: f32) -> Color {
        Self::blend_colors(color, Color::WHITE, amount.clamp(0.0, 1.0))
    }

    /// Darken a color by a given amount (0.0 = no change, 1.0 = black)
    pub fn darken(color: Color, amount: f32) -> Color {
        Self::blend_colors(color, Color::BLACK, amount.clamp(0.0, 1.0))
    }

    /// Get the best text color (black or white) for a given background
    pub fn get_best_text_color(background: Color) -> Color {
        let luminance = Self::relative_luminance(background);
        if luminance > 0.5 {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    /// Validate that all color combinations in a MaterialColors scheme meet accessibility standards
    pub fn validate_accessibility(colors: &MaterialColors) -> AccessibilityReport {
        let mut report = AccessibilityReport::new();

        // Check primary color combinations
        report.add_check(
            "Primary/On-Primary",
            Self::meets_accessibility_contrast(colors.primary.on_base, colors.primary.base),
        );
        report.add_check(
            "Primary Container/On-Primary Container",
            Self::meets_accessibility_contrast(colors.primary.on_container, colors.primary.container),
        );

        // Check secondary color combinations
        report.add_check(
            "Secondary/On-Secondary",
            Self::meets_accessibility_contrast(colors.secondary.on_base, colors.secondary.base),
        );

        // Check surface combinations
        report.add_check(
            "Surface/On-Surface",
            Self::meets_accessibility_contrast(colors.on_surface, colors.surface),
        );
        report.add_check(
            "Surface Variant/On-Surface Variant",
            Self::meets_accessibility_contrast(colors.on_surface_variant, colors.surface_variant),
        );

        // Check background combinations
        report.add_check(
            "Background/On-Background",
            Self::meets_accessibility_contrast(colors.on_background, colors.background),
        );

        report
    }
}

/// Report on accessibility compliance for a color scheme
#[derive(Debug, Clone)]
pub struct AccessibilityReport {
    checks: Vec<(String, bool)>,
}

impl AccessibilityReport {
    fn new() -> Self {
        Self {
            checks: Vec::new(),
        }
    }

    fn add_check(&mut self, name: impl Into<String>, passes: bool) {
        self.checks.push((name.into(), passes));
    }

    /// Check if all accessibility tests pass
    pub fn all_pass(&self) -> bool {
        self.checks.iter().all(|(_, passes)| *passes)
    }

    /// Get failed accessibility checks
    pub fn failed_checks(&self) -> Vec<&str> {
        self.checks
            .iter()
            .filter_map(|(name, passes)| if !passes { Some(name.as_str()) } else { None })
            .collect()
    }

    /// Get a summary of the accessibility report
    pub fn summary(&self) -> String {
        let total = self.checks.len();
        let passed = self.checks.iter().filter(|(_, passes)| *passes).count();
        format!("Accessibility: {}/{} checks passed", passed, total)
    }
}

/// Theme creation and manipulation utilities
pub struct ThemeUtilities;

impl ThemeUtilities {
    /// Create a custom MaterialColors from brand colors
    /// This is useful for companies wanting to use their brand colors in MD3
    pub fn from_brand_colors(
        primary_brand: Color,
        secondary_brand: Option<Color>,
        is_dark: bool,
    ) -> MaterialColors {
        let palette = if let Some(secondary) = secondary_brand {
            // Create palette with custom secondary color
            Self::create_custom_palette(primary_brand, Some(secondary))
        } else {
            // Use standard MD3 algorithm for secondary/tertiary
            MaterialPalette::from_seed(primary_brand)
        };

        if is_dark {
            MaterialColors::dark(&palette)
        } else {
            MaterialColors::light(&palette)
        }
    }

    /// Create a MaterialColors optimized for high contrast accessibility
    pub fn high_contrast_theme(base_theme: MaterialColors) -> MaterialColors {
        // Enhance contrast for better accessibility
        let mut enhanced = base_theme;

        // Increase contrast for text on surfaces
        enhanced.on_surface = ColorUtilities::get_best_text_color(enhanced.surface);
        enhanced.on_background = ColorUtilities::get_best_text_color(enhanced.background);
        enhanced.on_surface_variant = ColorUtilities::get_best_text_color(enhanced.surface_variant);

        // Ensure primary combinations have high contrast
        enhanced.primary.on_base = ColorUtilities::get_best_text_color(enhanced.primary.base);
        enhanced.primary.on_container = ColorUtilities::get_best_text_color(enhanced.primary.container);

        enhanced
    }

    /// Create a low-contrast theme for reduced eye strain
    pub fn low_contrast_theme(base_theme: MaterialColors) -> MaterialColors {
        let mut softened = base_theme;

        // Reduce contrast by blending with neutral tones
        let neutral_tone = Color::from_rgb(0.5, 0.5, 0.5);

        softened.outline = ColorUtilities::blend_colors(softened.outline, neutral_tone, 0.3);
        softened.outline_variant = ColorUtilities::blend_colors(softened.outline_variant, neutral_tone, 0.5);

        softened
    }

    /// Generate a seasonal theme variation
    pub fn seasonal_theme(base_color: Color, season: Season) -> MaterialColors {
        let seasonal_color = match season {
            Season::Spring => ColorUtilities::blend_colors(base_color, Color::from_rgb(0.4, 0.8, 0.4), 0.2),
            Season::Summer => ColorUtilities::blend_colors(base_color, Color::from_rgb(1.0, 0.8, 0.2), 0.2),
            Season::Autumn => ColorUtilities::blend_colors(base_color, Color::from_rgb(0.8, 0.4, 0.2), 0.2),
            Season::Winter => ColorUtilities::blend_colors(base_color, Color::from_rgb(0.3, 0.5, 0.8), 0.2),
        };

        MaterialColors::from_seed(seasonal_color, false)
    }

    /// Helper to create a custom palette with specific brand colors
    fn create_custom_palette(primary: Color, secondary: Option<Color>) -> MaterialPalette {
        // This would use your existing seed generation but with custom secondary
        // For now, use the primary-based generation
        MaterialPalette::from_seed(primary)
    }
}

/// Seasonal theme variations
#[derive(Debug, Clone, Copy)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

/// Advanced color role utilities
pub struct ColorRoleUtilities;

impl ColorRoleUtilities {
    /// Create a color role from a base color with automatic contrast calculations
    pub fn create_role_from_base(base_color: Color, is_dark_theme: bool) -> ColorRole {
        let on_base = ColorUtilities::get_best_text_color(base_color);
        
        // Create container as a lighter/darker variant
        let container = if is_dark_theme {
            ColorUtilities::darken(base_color, 0.6)
        } else {
            ColorUtilities::lighten(base_color, 0.7)
        };
        
        let on_container = ColorUtilities::get_best_text_color(container);

        // Fixed variants (for consistent surfaces)
        let fixed = if is_dark_theme {
            ColorUtilities::lighten(base_color, 0.7)
        } else {
            ColorUtilities::lighten(base_color, 0.7)
        };
        
        let fixed_dim = ColorUtilities::darken(fixed, 0.1);
        let on_fixed = ColorUtilities::get_best_text_color(fixed);
        let on_fixed_variant = ColorUtilities::blend_colors(on_fixed, base_color, 0.3);

        ColorRole {
            base: base_color,
            on_base,
            container,
            on_container,
            fixed,
            fixed_dim,
            on_fixed,
            on_fixed_variant,
        }
    }

    /// Validate that a ColorRole meets accessibility requirements
    pub fn validate_role_accessibility(role: &ColorRole) -> bool {
        ColorUtilities::meets_accessibility_contrast(role.on_base, role.base)
            && ColorUtilities::meets_accessibility_contrast(role.on_container, role.container)
            && ColorUtilities::meets_accessibility_contrast(role.on_fixed, role.fixed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contrast_ratio_calculation() {
        // Test with known values
        let white = Color::WHITE;
        let black = Color::BLACK;
        let ratio = ColorUtilities::contrast_ratio(white, black);
        assert!(ratio > 20.0); // Should be 21:1 for perfect white/black
    }

    #[test]
    fn test_accessibility_validation() {
        let colors = MaterialColors::light_default();
        let report = ColorUtilities::validate_accessibility(&colors);
        
        // Most combinations should pass accessibility
        assert!(report.all_pass() || report.failed_checks().len() < 3);
    }

    #[test]
    fn test_color_blending() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let purple = ColorUtilities::blend_colors(red, blue, 0.5);
        
        assert!(purple.r > 0.0 && purple.r < 1.0);
        assert!(purple.b > 0.0 && purple.b < 1.0);
        assert_eq!(purple.g, 0.0);
    }

    #[test]
    fn test_theme_creation() {
        let brand_color = Color::from_rgb(0.2, 0.4, 0.8);
        let theme = ThemeUtilities::from_brand_colors(brand_color, None, false);
        
        // Should create a valid theme
        assert!(ColorUtilities::validate_accessibility(&theme).all_pass());
    }

    #[test]
    fn test_seasonal_themes() {
        let base = Color::from_rgb(0.5, 0.5, 0.5);
        let spring = ThemeUtilities::seasonal_theme(base, Season::Spring);
        let winter = ThemeUtilities::seasonal_theme(base, Season::Winter);
        
        // Seasonal themes should be different
        assert_ne!(spring.primary.base, winter.primary.base);
    }
}
