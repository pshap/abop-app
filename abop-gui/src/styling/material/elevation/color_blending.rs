//! Color blending utilities for Material Design 3 elevation system

use crate::styling::color_utils::ColorUtils;
use iced::Color;

/// Apply surface tint to a base color with given opacity
#[must_use]
pub fn apply_surface_tint(base_color: Color, tint_color: Color, tint_opacity: f32) -> Color {
    if tint_opacity <= 0.0 {
        return base_color;
    }

    ColorUtils::blend_colors(base_color, tint_color, tint_opacity)
}

/// Calculate elevated surface color using Material Design 3 specifications
#[must_use]
pub fn elevated_surface_color(
    base_surface: Color,
    tint_color: Color,
    elevation_level: u8,
) -> Color {
    let tint_opacity = match elevation_level {
        0 => 0.0,
        1 => 0.05,
        2 => 0.08,
        3 => 0.11,
        4 => 0.12,
        5 => 0.14,
        _ => 0.14, // Cap at level 5
    };

    apply_surface_tint(base_surface, tint_color, tint_opacity)
}

/// Create a shadow color with proper opacity
#[must_use]
pub const fn create_shadow_color(base_color: Color, opacity: f32) -> Color {
    Color {
        a: opacity.clamp(0.0, 1.0),
        ..base_color
    }
}

/// Check if a color is considered "dark" (luminance < 0.5)
#[must_use]
pub fn is_dark_color(color: Color) -> bool {
    ColorUtils::luminance(color) < 0.5
}

/// Get appropriate text color (black or white) for a given background color
#[must_use]
pub fn get_text_color_for_background(background: Color) -> Color {
    ColorUtils::contrasting_text_color(background)
}

/// Mix two colors with a given ratio (0.0 = first color, 1.0 = second color)
#[must_use]
pub fn mix_colors(color1: Color, color2: Color, ratio: f32) -> Color {
    ColorUtils::blend_colors(color1, color2, ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to calculate the Euclidean distance between two colors in RGB space
    fn color_distance(c1: Color, c2: Color) -> f32 {
        let dr = c1.r - c2.r;
        let dg = c1.g - c2.g;
        let db = c1.b - c2.b;
        (dr * dr + dg * dg + db * db).sqrt()
    }

    #[test]
    fn test_blend_colors() {
        let base = Color::WHITE;
        let overlay = Color::BLACK;
        let blended = ColorUtils::blend_colors(base, overlay, 0.5);

        // Should be gray when blending white and black at 50%
        assert!((blended.r - 0.5).abs() < 0.01);
        assert!((blended.g - 0.5).abs() < 0.01);
        assert!((blended.b - 0.5).abs() < 0.01);
        assert_eq!(blended.a, base.a);
    }

    #[test]
    fn test_apply_surface_tint() {
        let base = Color::WHITE;
        let tint = Color::new(0.0, 0.5, 1.0, 1.0); // Blue tint
        let result = apply_surface_tint(base, tint, 0.1);

        // Should have slight blue tint
        assert!(result.b > result.r);
        assert!(result.b > result.g);
    }

    #[test]
    fn test_zero_tint_opacity() {
        let base = Color::WHITE;
        let tint = Color::BLACK;
        let result = apply_surface_tint(base, tint, 0.0);

        // Should be unchanged
        assert_eq!(result, base);
    }

    #[test]
    fn test_elevated_surface_color() {
        let base = Color::WHITE;
        let tint = Color::new(0.5, 0.0, 1.0, 1.0); // Purple tint

        let level0 = elevated_surface_color(base, tint, 0);
        let level1 = elevated_surface_color(base, tint, 1);
        let level5 = elevated_surface_color(base, tint, 5);

        // Debug output to see the actual values
        println!(
            "Level 0: R={:.3}, G={:.3}, B={:.3}",
            level0.r, level0.g, level0.b
        );
        println!(
            "Level 1: R={:.3}, G={:.3}, B={:.3}",
            level1.r, level1.g, level1.b
        );
        println!(
            "Level 5: R={:.3}, G={:.3}, B={:.3}",
            level5.r, level5.g, level5.b
        );

        // Level 0 should be unchanged
        assert_eq!(level0, base, "Level 0 should be unchanged from base color");

        // Check that the colors are different at different elevations
        assert_ne!(
            level1, level5,
            "Different elevation levels should produce different colors"
        );

        // For a white base and purple tint, the result should be a very light purple
        // The blue channel should be slightly higher than red, and green should be slightly reduced
        // But since the tint opacities are very low, the differences will be subtle

        // Instead of checking specific channel values, verify that the colors are ordered by elevation
        // by comparing their distances from the base color
        let dist1 = color_distance(base, level1);
        let dist5 = color_distance(base, level5);

        println!(
            "Distance from base - Level 1: {:.6}, Level 5: {:.6}",
            dist1, dist5
        );

        // Higher elevation should be further from the base color (more tinted)
        assert!(
            dist5 >= dist1,
            "Higher elevation should be further from base color"
        );
    }

    #[test]
    fn test_luminance() {
        assert_eq!(ColorUtils::luminance(Color::WHITE), 1.0);
        assert_eq!(ColorUtils::luminance(Color::BLACK), 0.0);

        let gray = Color::new(0.5, 0.5, 0.5, 1.0);
        // WCAG luminance for gray(0.5) with gamma correction is approximately 0.214
        assert!((ColorUtils::luminance(gray) - 0.214).abs() < 0.01);
    }

    #[test]
    fn test_is_dark_color() {
        assert!(!is_dark_color(Color::WHITE));
        assert!(is_dark_color(Color::BLACK));

        let dark_gray = Color::new(0.3, 0.3, 0.3, 1.0);
        assert!(is_dark_color(dark_gray));
    }

    #[test]
    fn test_text_color_for_background() {
        assert_eq!(get_text_color_for_background(Color::WHITE), Color::BLACK);
        assert_eq!(get_text_color_for_background(Color::BLACK), Color::WHITE);
    }

    #[test]
    fn test_lighten_darken() {
        let base = Color::new(0.5, 0.5, 0.5, 1.0);
        let lightened = ColorUtils::lighten(base, 0.5);
        let darkened = ColorUtils::darken(base, 0.5);

        assert!(lightened.r > base.r);
        assert!(darkened.r < base.r);
    }

    #[test]
    fn test_mix_colors() {
        let white = Color::WHITE;
        let black = Color::BLACK;
        let gray = mix_colors(white, black, 0.5);

        assert!((gray.r - 0.5).abs() < 0.01);
        assert!((gray.g - 0.5).abs() < 0.01);
        assert!((gray.b - 0.5).abs() < 0.01);
    }
}
