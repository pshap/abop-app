//! Light theme implementation for Material Design 3
//!
//! This module provides the light theme color scheme for Material Design 3.

use super::{Theme, ThemeVariant};
use crate::material::color::{ColorRoles, Srgb};

/// Create a light theme with default colors
pub fn light_theme() -> Theme {
    Theme {
        variant: ThemeVariant::Light,
        colors: ColorRoles {
            // Primary colors
            primary: Srgb::new(0.0, 0.46, 0.71),     // #0077B5
            on_primary: Srgb::new(1.0, 1.0, 1.0),    // White
            primary_container: Srgb::new(0.88, 0.93, 0.99), // Light blue
            on_primary_container: Srgb::new(0.0, 0.27, 0.5), // Dark blue
            
            // Secondary colors
            secondary: Srgb::new(0.3, 0.3, 0.7),     // #4D4DB8
            on_secondary: Srgb::new(1.0, 1.0, 1.0),  // White
            secondary_container: Srgb::new(0.93, 0.93, 0.99), // Light purple
            on_secondary_container: Srgb::new(0.2, 0.2, 0.6), // Dark purple
            
            // Tertiary colors
            tertiary: Srgb::new(0.7, 0.3, 0.6),      // #B34D99
            on_tertiary: Srgb::new(1.0, 1.0, 1.0),  // White
            tertiary_container: Srgb::new(0.99, 0.93, 0.97), // Light pink
            on_tertiary_container: Srgb::new(0.5, 0.1, 0.4), // Dark pink
            
            // Error colors
            error: Srgb::new(0.78, 0.07, 0.17),     // #C7122B
            on_error: Srgb::new(1.0, 1.0, 1.0),     // White
            error_container: Srgb::new(1.0, 0.9, 0.9), // Light red
            on_error_container: Srgb::new(0.4, 0.0, 0.0), // Dark red
            
            // Background colors
            background: Srgb::new(0.98, 0.98, 0.98), // Off-white
            on_background: Srgb::new(0.13, 0.13, 0.13), // Dark gray
            
            // Surface colors
            surface: Srgb::new(1.0, 1.0, 1.0),      // White
            on_surface: Srgb::new(0.13, 0.13, 0.13), // Dark gray
            surface_variant: Srgb::new(0.95, 0.95, 0.95), // Light gray
            on_surface_variant: Srgb::new(0.26, 0.26, 0.26), // Medium gray
            
            // Outline colors
            outline: Srgb::new(0.74, 0.74, 0.74),   // Light gray
            outline_variant: Srgb::new(0.8, 0.8, 0.8), // Lighter gray
            
            // Shadow and scrim
            shadow: Srgb::new(0.0, 0.0, 0.0),       // Black
            scrim: Srgb::new(0.0, 0.0, 0.0),        // Black
            
            // Inverse colors
            inverse_surface: Srgb::new(0.2, 0.2, 0.2), // Dark gray
            inverse_on_surface: Srgb::new(0.95, 0.95, 0.95), // Light gray
            inverse_primary: Srgb::new(0.6, 0.84, 1.0), // Light blue
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::color::ColorRoles;

    #[test]
    fn test_light_theme_creation() {
        let theme = light_theme();
        
        // Verify theme variant
        assert!(theme.is_light());
        assert!(!theme.is_dark());
        
        // Verify some key colors
        assert_eq!(theme.colors.primary.r, 0.0);
        assert_eq!(theme.colors.primary.g, 0.46);
        assert_eq!(theme.colors.primary.b, 0.71);
        
        // Verify text on primary is white
        assert_eq!(theme.colors.on_primary.r, 1.0);
        assert_eq!(theme.colors.on_primary.g, 1.0);
        assert_eq!(theme.colors.on_primary.b, 1.0);
        
        // Verify surface colors
        assert_eq!(theme.colors.surface.r, 1.0);
        assert_eq!(theme.colors.surface.g, 1.0);
        assert_eq!(theme.colors.surface.b, 1.0);
    }
}
