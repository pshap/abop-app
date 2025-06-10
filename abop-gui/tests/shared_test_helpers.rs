//! Shared Test Utilities
//!
//! This module provides common utilities and helpers used across multiple test files.

use iced::Color;

/// Common test message types for UI testing
#[derive(Clone, Debug, PartialEq)]
pub enum TestMessage {
    Save,
    Cancel,
    Export,
    ButtonPressed,
    Submit,
    Reset,
}

/// Test color constants for consistent testing
pub mod test_colors {
    use super::Color;

    pub const WHITE: Color = Color::WHITE;
    pub const BLACK: Color = Color::BLACK;
    pub const MATERIAL_BLUE: Color = Color {
        r: 0.404,
        g: 0.314,
        b: 0.643,
        a: 1.0,
    };
    pub const MATERIAL_RED: Color = Color {
        r: 0.729,
        g: 0.071,
        b: 0.212,
        a: 1.0,
    };
    pub const TRANSPARENT: Color = Color::TRANSPARENT;
}

/// Test utilities for color validation
pub mod color_test_utils {
    use super::Color;

    /// Calculate relative luminance according to WCAG 2.1
    pub fn calculate_luminance(color: Color) -> f32 {
        fn gamma_correct(c: f32) -> f32 {
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }

        let r = gamma_correct(color.r);
        let g = gamma_correct(color.g);
        let b = gamma_correct(color.b);

        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Calculate contrast ratio between two colors using WCAG 2.1 formula
    pub fn calculate_contrast_ratio(color1: Color, color2: Color) -> f32 {
        let luminance1 = calculate_luminance(color1);
        let luminance2 = calculate_luminance(color2);

        let (brighter, darker) = if luminance1 > luminance2 {
            (luminance1, luminance2)
        } else {
            (luminance2, luminance1)
        };

        (brighter + 0.05) / (darker + 0.05)
    }

    /// Check if contrast ratio meets WCAG AA standards
    pub fn meets_wcag_aa_contrast(color1: Color, color2: Color) -> bool {
        calculate_contrast_ratio(color1, color2) >= 4.5
    }

    /// Check if contrast ratio meets WCAG AA standards for large text/UI components
    pub fn meets_wcag_aa_large_contrast(color1: Color, color2: Color) -> bool {
        calculate_contrast_ratio(color1, color2) >= 3.0
    }
}

/// Test assertions and validation helpers
pub mod test_assertions {
    use super::Color;

    /// Assert that a color is not transparent
    pub fn assert_not_transparent(color: Color) {
        assert!(color.a > 0.0, "Color should not be transparent: {color:?}");
    }

    /// Assert that colors are sufficiently different
    pub fn assert_colors_different(color1: Color, color2: Color) {
        assert_ne!(
            color1, color2,
            "Colors should be different: {color1:?} vs {color2:?}"
        );
    }

    /// Assert that a value is within a reasonable range
    pub fn assert_reasonable_size(value: f32, min: f32, max: f32) {
        assert!(
            value >= min && value <= max,
            "Value {value} should be between {min} and {max}"
        );
    }
}
