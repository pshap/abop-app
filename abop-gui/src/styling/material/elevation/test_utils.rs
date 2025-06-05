//! Test utilities for Material Design 3 elevation system
//
// This module provides helper functions, constants, and assertions for testing
// the Material Design 3 elevation system, including color constants and shadow parameters.
//
// It is intended for use in unit and integration tests for elevation-related code.

#[cfg(test)]
/// Setup utilities for Material Design 3 elevation tests.
///
/// This module provides functions, color constants, and assertion helpers for testing
/// the Material Design 3 elevation system. It is intended for use in unit and integration tests.
pub mod setup {
    use crate::styling::material::MaterialColors;
    use crate::styling::material::elevation::{
        ElevationContext, ElevationStyle, MaterialElevation, ShadowParams,
    };
    use iced::Color;

    /// Create a default MaterialElevation for testing
    pub fn create_default_elevation() -> MaterialElevation {
        MaterialElevation::default()
    }

    /// Create a MaterialElevation with test colors
    pub fn create_test_elevation() -> MaterialElevation {
        let shadow_color = Color::from_rgb(0.1, 0.1, 0.1);
        let tint_color = Color::from_rgb(0.9, 0.9, 0.9);
        MaterialElevation::with_colors(shadow_color, tint_color)
    }

    /// Create an ElevationContext for testing
    pub fn create_test_context() -> ElevationContext {
        let colors = MaterialColors::default();
        ElevationContext::new(&colors)
    }

    /// Create test shadow parameters
    pub fn create_test_shadow_params() -> ShadowParams {
        ShadowParams {
            offset_y: 5.0,
            blur_radius: 10.0,
            opacity: 0.2,
        }
    }

    /// Create a custom elevation style for testing
    pub fn create_custom_elevation_style() -> ElevationStyle {
        ElevationStyle::custom(
            4.0,
            Color::BLACK,
            Color::WHITE,
            Some(create_test_shadow_params()),
        )
    }

    /// Standard test colors
    pub mod colors {
        use iced::Color;

        /// Black color constant for elevation tests.
        pub const BLACK: Color = Color::BLACK;
        /// White color constant for elevation tests.
        pub const WHITE: Color = Color::WHITE;
        /// Red color constant for elevation tests.
        pub const RED: Color = Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        /// Green color constant for elevation tests.
        pub const GREEN: Color = Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        };
        /// Blue color constant for elevation tests.
        pub const BLUE: Color = Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        };
        /// Shadow color used for elevation shadow tests.
        pub const SHADOW_COLOR: Color = Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        };
        /// Tint color used for elevation tint tests.
        pub const TINT_COLOR: Color = Color {
            r: 0.9,
            g: 0.9,
            b: 0.9,
            a: 1.0,
        };
        /// Example Material tint color for elevation tests.
        pub const MATERIAL_TINT: Color = Color {
            r: 0.42,
            g: 0.31,
            b: 0.65,
            a: 1.0,
        };
    }

    /// Test assertion helpers
    pub mod assertions {
        use crate::styling::material::elevation::{ElevationLevel, ElevationStyle};
        use iced::Color;

        /// Assert that two colors are approximately equal (within tolerance)
        pub fn assert_colors_approx_eq(actual: Color, expected: Color, tolerance: f32) {
            assert!(
                (actual.r - expected.r).abs() < tolerance,
                "Red channel mismatch: {} vs {}",
                actual.r,
                expected.r
            );
            assert!(
                (actual.g - expected.g).abs() < tolerance,
                "Green channel mismatch: {} vs {}",
                actual.g,
                expected.g
            );
            assert!(
                (actual.b - expected.b).abs() < tolerance,
                "Blue channel mismatch: {} vs {}",
                actual.b,
                expected.b
            );
            assert!(
                (actual.a - expected.a).abs() < tolerance,
                "Alpha channel mismatch: {} vs {}",
                actual.a,
                expected.a
            );
        }

        /// Assert that an elevation style has expected level and dp
        pub fn assert_elevation_style_level(
            style: &ElevationStyle,
            expected_level: ElevationLevel,
        ) {
            assert_eq!(style.level(), expected_level);
            assert_eq!(style.dp, expected_level.dp());
        }

        /// Assert that a shadow has non-zero values (is elevated)
        pub fn assert_shadow_is_elevated(style: &ElevationStyle) {
            assert!(style.shadow.blur_radius > 0.0 || style.shadow.offset.y > 0.0);
            assert!(style.shadow.color.a > 0.0);
        }

        /// Assert that a style is flat (no elevation)
        pub fn assert_style_is_flat(style: &ElevationStyle) {
            assert!(style.is_flat());
            assert_eq!(style.shadow.blur_radius, 0.0);
            assert_eq!(style.shadow.offset.y, 0.0);
            assert_eq!(style.shadow.color.a, 0.0);
        }
    }
}
