//! Utility functions for Material Design 3 elevation system

use super::ElevationLevel;

#[cfg(test)]
use super::shadow_calculations;
#[cfg(test)]
use iced::{Color, Vector};

/// Component types for elevation recommendations
///
/// Defines the different component types that have specific elevation
/// level recommendations in Material Design 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    /// Base surface with no elevation
    Surface,
    /// Standard card component
    Card,
    /// Raised button component
    RaisedButton,
    /// Outlined card variant
    OutlinedCard,
    /// Filled card variant
    FilledCard,
    /// Floating action button
    FloatingActionButton,
    /// Top application bar
    AppBar,
    /// Bottom application bar
    BottomAppBar,
    /// Navigation drawer
    NavigationDrawer,
    /// Modal dialog
    ModalDialog,
    /// Context menu
    Menu,
    /// Tooltip component
    Tooltip,
    /// Snackbar notification
    Snackbar,
}

/// Get recommended elevation level for component type
///
/// Returns the Material Design 3 recommended elevation level
/// for different component types.
///
/// # Arguments
/// * `component_type` - The type of component to get elevation for
///
/// # Returns
/// The recommended `ElevationLevel` for the component type
#[must_use]
pub const fn get_recommended_level(component_type: ComponentType) -> ElevationLevel {
    match component_type {
        ComponentType::Surface | ComponentType::OutlinedCard | ComponentType::AppBar => {
            ElevationLevel::Level0
        }
        ComponentType::Card
        | ComponentType::RaisedButton
        | ComponentType::FilledCard
        | ComponentType::NavigationDrawer => ElevationLevel::Level1,
        ComponentType::Menu | ComponentType::Tooltip => ElevationLevel::Level2,
        ComponentType::FloatingActionButton
        | ComponentType::BottomAppBar
        | ComponentType::ModalDialog
        | ComponentType::Snackbar => ElevationLevel::Level3,
    }
}

/// Color blending utilities for elevation
///
/// Provides utilities for blending colors with alpha composition
/// as used in Material Design elevation tinting.
pub mod color_blending {
    use iced::Color;

    /// Blend two colors with alpha compositing
    #[must_use]
    pub fn alpha_blend(foreground: Color, background: Color) -> Color {
        let alpha = foreground.a;
        let inv_alpha = 1.0 - alpha;
        Color {
            r: foreground.r.mul_add(alpha, background.r * inv_alpha),
            g: foreground.g.mul_add(alpha, background.g * inv_alpha),
            b: foreground.b.mul_add(alpha, background.b * inv_alpha),
            a: background.a.mul_add(inv_alpha, alpha),
        }
    }

    /// Apply tint to a base color with given tint color and opacity
    #[must_use]
    pub fn apply_tint(base_color: Color, tint_color: Color, tint_opacity: f32) -> Color {
        if tint_opacity == 0.0 {
            return base_color;
        }

        let tint_with_opacity = Color {
            a: tint_opacity,
            ..tint_color
        };

        alpha_blend(tint_with_opacity, base_color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_recommendations() {
        assert_eq!(
            get_recommended_level(ComponentType::FloatingActionButton),
            ElevationLevel::Level3
        );
        assert_eq!(
            get_recommended_level(ComponentType::Surface),
            ElevationLevel::Level0
        );
        assert_eq!(
            get_recommended_level(ComponentType::Card),
            ElevationLevel::Level1
        );
        assert_eq!(
            get_recommended_level(ComponentType::ModalDialog),
            ElevationLevel::Level3
        );
    }

    #[test]
    fn test_custom_shadow_calculation() {
        let shadow = shadow_calculations::calculate_custom_shadow(6.0, Color::BLACK);
        assert_eq!(shadow.offset.y, 3.0); // 6.0 * 0.5
        assert_eq!(shadow.blur_radius, 6.0); // 6.0 * 1.0
        assert!(shadow.color.a > 0.0);

        // Test zero elevation
        let no_shadow = shadow_calculations::calculate_custom_shadow(0.0, Color::BLACK);
        assert_eq!(no_shadow.color, Color::TRANSPARENT);
        assert_eq!(no_shadow.offset, Vector::ZERO);
        assert_eq!(no_shadow.blur_radius, 0.0);
    }

    #[test]
    fn test_custom_tint_opacity() {
        assert_eq!(shadow_calculations::calculate_custom_tint_opacity(0.0), 0.0);
        let opacity_12dp = shadow_calculations::calculate_custom_tint_opacity(12.0);
        assert!((opacity_12dp - 0.14).abs() < f32::EPSILON); // 12dp should return 0.14
    }

    #[test]
    fn test_alpha_blending() {
        let fg = Color::from_rgba(1.0, 0.0, 0.0, 0.5); // Red with 50% alpha
        let bg = Color::from_rgb(0.0, 1.0, 0.0); // Green background

        let blended = color_blending::alpha_blend(fg, bg);

        // Should be a mix of red and green
        assert!(blended.r > 0.0);
        assert!(blended.g > 0.0);
        assert_eq!(blended.b, 0.0);
    }

    #[test]
    fn test_apply_tint() {
        let base = Color::WHITE;
        let tint = Color::from_rgb(0.5, 0.0, 0.5); // Purple tint

        let tinted = color_blending::apply_tint(base, tint, 0.1);
        assert_ne!(tinted, base); // Should be different

        // No tint should return original color
        let no_tint = color_blending::apply_tint(base, tint, 0.0);
        assert_eq!(no_tint, base);
    }
}
