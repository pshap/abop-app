//! Dark theme implementation for Material Design 3
//!
//! This module provides the dark theme color scheme for Material Design 3.

use super::{Theme, ThemeVariant};
use crate::material::color::{ColorRoles, Srgb};

/// Create a dark theme with default colors
pub fn dark_theme() -> Theme {
    Theme {
        variant: ThemeVariant::Dark,
        colors: ColorRoles {
            // Primary colors
            primary: Srgb::new(0.51, 0.85, 1.0),     // #82D9FF
            on_primary: Srgb::new(0.0, 0.2, 0.35),   // #003357
            primary_container: Srgb::new(0.0, 0.4, 0.6), // #006699
            on_primary_container: Srgb::new(0.88, 0.93, 0.99), // #E0EDFC
            
            // Secondary colors
            secondary: Srgb::new(0.8, 0.8, 1.0),     // #CCCCFF
            on_secondary: Srgb::new(0.2, 0.2, 0.5),  // #333380
            secondary_container: Srgb::new(0.3, 0.3, 0.7), // #4D4DB8
            on_secondary_container: Srgb::new(0.93, 0.93, 0.99), // #EDEDFC
            
            // Tertiary colors
            tertiary: Srgb::new(1.0, 0.8, 0.9),      // #FFCCE5
            on_tertiary: Srgb::new(0.5, 0.1, 0.4),  // #801A66
            tertiary_container: Srgb::new(0.7, 0.3, 0.6), // #B34D99
            on_tertiary_container: Srgb::new(0.99, 0.93, 0.97), // #FCEDF7
            
            // Error colors
            error: Srgb::new(1.0, 0.6, 0.6),        // #FF9999
            on_error: Srgb::new(0.4, 0.0, 0.0),     // #660000
            error_container: Srgb::new(0.8, 0.2, 0.2), // #CC3333
            on_error_container: Srgb::new(1.0, 0.9, 0.9), // #FFE6E6
            
            // Background colors
            background: Srgb::new(0.07, 0.07, 0.07), // #121212
            on_background: Srgb::new(0.9, 0.9, 0.9), // #E6E6E6
            
            // Surface colors
            surface: Srgb::new(0.1, 0.1, 0.1),      // #1A1A1A
            on_surface: Srgb::new(0.9, 0.9, 0.9),   // #E6E6E6
            surface_variant: Srgb::new(0.2, 0.2, 0.2), // #333333
            on_surface_variant: Srgb::new(0.8, 0.8, 0.8), // #CCCCCC
            
            // Outline colors
            outline: Srgb::new(0.4, 0.4, 0.4),      // #666666
            outline_variant: Srgb::new(0.3, 0.3, 0.3), // #4D4D4D
            
            // Shadow and scrim
            shadow: Srgb::new(0.0, 0.0, 0.0),        // Black
            scrim: Srgb::new(0.0, 0.0, 0.0),         // Black
            
            // Inverse colors
            inverse_surface: Srgb::new(0.9, 0.9, 0.9), // #E6E6E6
            inverse_on_surface: Srgb::new(0.2, 0.2, 0.2), // #333333
            inverse_primary: Srgb::new(0.0, 0.46, 0.71), // #0077B5
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::color::ColorRoles;

    #[test]
    fn test_dark_theme_creation() {
        let theme = dark_theme();
        
        // Verify theme variant
        assert!(theme.is_dark());
        assert!(!theme.is_light());
        
        // Verify some key colors
        assert!(theme.colors.primary.r > 0.5);
        assert!(theme.colors.primary.g > 0.8);
        assert!(theme.colors.primary.b > 0.9);
        
        // Verify text on primary is dark
        assert!(theme.colors.on_primary.r < 0.1);
        assert!(theme.colors.on_primary.g < 0.3);
        assert!(theme.colors.on_primary.b < 0.4);
        
        // Verify surface colors are dark
        assert!(theme.colors.surface.r < 0.2);
        assert!(theme.colors.surface.g < 0.2);
        assert!(theme.colors.surface.b < 0.2);
    }
}
