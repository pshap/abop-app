//! Shared color utilities for consistent color manipulation
//!
//! This module provides utility functions for common color operations
//! to reduce code duplication and ensure consistent color handling.

use iced::Color;

/// Color utility functions for consistent color manipulation
pub struct ColorUtils;

impl ColorUtils {
    /// Apply alpha transparency to a color
    ///
    /// # Arguments
    /// * `color` - The base color
    /// * `alpha` - Alpha value between 0.0 and 1.0
    ///
    /// # Returns
    /// Color with the specified alpha value
    #[must_use]
    pub const fn with_alpha(color: Color, alpha: f32) -> Color {
        Color {
            a: alpha.clamp(0.0, 1.0),
            ..color
        }
    }

    /// Lighten a color by a specified factor
    ///
    /// # Arguments
    /// * `color` - The base color
    /// * `factor` - Lightening factor between 0.0 and 1.0
    ///
    /// # Returns
    /// Lightened color
    #[must_use]
    pub fn lighten(color: Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: (1.0 - color.r).mul_add(factor, color.r).clamp(0.0, 1.0),
            g: (1.0 - color.g).mul_add(factor, color.g).clamp(0.0, 1.0),
            b: (1.0 - color.b).mul_add(factor, color.b).clamp(0.0, 1.0),
            a: color.a,
        }
    }

    /// Darken a color by a specified factor
    ///
    /// # Arguments
    /// * `color` - The base color
    /// * `factor` - Darkening factor between 0.0 and 1.0
    ///
    /// # Returns
    /// Darkened color
    #[must_use]
    pub fn darken(color: Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: (color.r * (1.0 - factor)).clamp(0.0, 1.0),
            g: (color.g * (1.0 - factor)).clamp(0.0, 1.0),
            b: (color.b * (1.0 - factor)).clamp(0.0, 1.0),
            a: color.a,
        }
    }

    /// Blend two colors together
    ///
    /// # Arguments
    /// * `base` - The base color
    /// * `overlay` - The overlay color
    /// * `factor` - Blending factor between 0.0 (all base) and 1.0 (all overlay)
    ///
    /// # Returns
    /// Blended color
    #[must_use]
    pub fn blend_colors(base: Color, overlay: Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        let inv_factor = 1.0 - factor;

        Color {
            r: base
                .r
                .mul_add(inv_factor, overlay.r * factor)
                .clamp(0.0, 1.0),
            g: base
                .g
                .mul_add(inv_factor, overlay.g * factor)
                .clamp(0.0, 1.0),
            b: base
                .b
                .mul_add(inv_factor, overlay.b * factor)
                .clamp(0.0, 1.0),
            a: base
                .a
                .mul_add(inv_factor, overlay.a * factor)
                .clamp(0.0, 1.0),
        }
    }

    /// Create a hover state color by slightly lightening or darkening
    ///
    /// # Arguments
    /// * `color` - The base color
    /// * `is_dark_theme` - Whether we're in a dark theme
    ///
    /// # Returns
    /// Hover state color
    #[must_use]
    pub fn hover_color(color: Color, is_dark_theme: bool) -> Color {
        if is_dark_theme {
            Self::lighten(color, 0.1)
        } else {
            Self::darken(color, 0.1)
        }
    }

    /// Create an active/pressed state color
    ///
    /// # Arguments
    /// * `color` - The base color
    /// * `is_dark_theme` - Whether we're in a dark theme
    ///
    /// # Returns
    /// Active state color
    #[must_use]
    pub fn active_color(color: Color, is_dark_theme: bool) -> Color {
        if is_dark_theme {
            Self::lighten(color, 0.2)
        } else {
            Self::darken(color, 0.2)
        }
    }

    /// Create a disabled state color by reducing opacity
    ///
    /// # Arguments
    /// * `color` - The base color
    ///
    /// # Returns
    /// Disabled state color with reduced opacity
    #[must_use]
    pub const fn disabled_color(color: Color) -> Color {
        Self::with_alpha(color, 0.4)
    }

    /// Get contrasting text color for a given background
    ///
    /// Automatically chooses between white and black text for best readability.
    ///
    /// # Returns
    /// Either white or black text color for best contrast
    #[must_use]
    pub fn contrasting_text_color(background: Color) -> Color {
        let luminance = Self::luminance(background);
        if luminance > 0.5 {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    /// Calculate contrast ratio between two colors
    ///
    /// Follows WCAG guidelines for contrast ratio calculation.
    ///
    /// # Arguments
    /// * `foreground` - The foreground/text color
    /// * `background` - The background color
    ///
    /// # Returns
    /// Contrast ratio from 1.0 (no contrast) to 21.0 (maximum contrast)
    #[must_use]
    pub fn contrast_ratio(foreground: Color, background: Color) -> f32 {
        let l1 = Self::luminance(foreground);
        let l2 = Self::luminance(background);

        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };

        (lighter + 0.05) / (darker + 0.05)
    }

    /// Calculate relative luminance of a color
    ///
    /// Uses the WCAG formula for relative luminance calculation.
    ///
    /// # Arguments
    /// * `color` - The color to calculate luminance for
    ///
    /// # Returns
    /// Relative luminance value between 0.0 and 1.0
    #[must_use]
    pub fn luminance(color: Color) -> f32 {
        let r = Self::linear_rgb_component(color.r);
        let g = Self::linear_rgb_component(color.g);
        let b = Self::linear_rgb_component(color.b);

        0.0722f32.mul_add(b, 0.2126f32.mul_add(r, 0.7152 * g))
    }

    /// Convert sRGB component to linear RGB
    fn linear_rgb_component(component: f32) -> f32 {
        if component <= 0.03928 {
            component / 12.92
        } else {
            ((component + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Convert RGB color to HSV for better color manipulation
    ///
    /// # Arguments
    /// * `color` - RGB color to convert
    ///
    /// # Returns
    /// (hue, saturation, value) tuple
    #[must_use]
    pub fn rgb_to_hsv(color: Color) -> (f32, f32, f32) {
        let max = color.r.max(color.g).max(color.b);
        let min = color.r.min(color.g).min(color.b);
        let diff = max - min;

        let hue = if (diff).abs() < f32::EPSILON {
            0.0
        } else if (max - color.r).abs() < f32::EPSILON {
            60.0 * ((color.g - color.b) / diff) % 360.0
        } else if (max - color.g).abs() < f32::EPSILON {
            60.0f32.mul_add((color.b - color.r) / diff, 120.0)
        } else {
            60.0f32.mul_add((color.r - color.g) / diff, 240.0)
        };

        let saturation = if (max).abs() < f32::EPSILON {
            0.0
        } else {
            diff / max
        };
        let value = max;

        (hue, saturation, value)
    }

    /// Convert HSV color to RGB
    ///
    /// # Arguments
    /// * `hue` - Hue value (0-360)
    /// * `saturation` - Saturation value (0-1)
    /// * `value` - Value/brightness (0-1)
    /// * `alpha` - Alpha value (0-1)
    ///
    /// # Returns
    /// RGB Color
    #[must_use]
    pub fn hsv_to_rgb(hue: f32, saturation: f32, value: f32, alpha: f32) -> Color {
        let c = value * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = value - c;

        let (r, g, b) = if hue < 60.0 {
            (c, x, 0.0)
        } else if hue < 120.0 {
            (x, c, 0.0)
        } else if hue < 180.0 {
            (0.0, c, x)
        } else if hue < 240.0 {
            (0.0, x, c)
        } else if hue < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Color {
            r: (r + m).clamp(0.0, 1.0),
            g: (g + m).clamp(0.0, 1.0),
            b: (b + m).clamp(0.0, 1.0),
            a: alpha.clamp(0.0, 1.0),
        }
    }
}

/// Commonly used color constants
pub mod colors {
    use iced::Color;

    /// Transparent color
    pub const TRANSPARENT: Color = Color::TRANSPARENT;

    /// Semi-transparent overlay
    pub const OVERLAY: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.5,
    };

    /// Light overlay for dark themes
    pub const LIGHT_OVERLAY: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.1,
    };

    /// Dark overlay for light themes
    pub const DARK_OVERLAY: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.1,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_alpha() {
        let color = Color::from_rgb(1.0, 0.0, 0.0);
        let result = ColorUtils::with_alpha(color, 0.5);
        assert_eq!(result.a, 0.5);
        assert_eq!(result.r, 1.0);
    }

    #[test]
    fn test_lighten() {
        let color = Color::from_rgb(0.5, 0.5, 0.5);
        let result = ColorUtils::lighten(color, 0.2);
        assert!(result.r > color.r);
        assert!(result.g > color.g);
        assert!(result.b > color.b);
    }

    #[test]
    fn test_darken() {
        let color = Color::from_rgb(0.5, 0.5, 0.5);
        let result = ColorUtils::darken(color, 0.2);
        assert!(result.r < color.r);
        assert!(result.g < color.g);
        assert!(result.b < color.b);
    }

    #[test]
    fn test_contrasting_text_color() {
        let white_bg = Color::WHITE;
        let black_bg = Color::BLACK;

        assert_eq!(ColorUtils::contrasting_text_color(white_bg), Color::BLACK);
        assert_eq!(ColorUtils::contrasting_text_color(black_bg), Color::WHITE);
    }
}
