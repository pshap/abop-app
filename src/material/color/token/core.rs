//! Core color tokens for Material Design 3
//!
//! This module defines the core semantic color tokens that represent the primary,
//! secondary, tertiary, and error color roles in the Material Design 3 system.

use super::super::Srgb;

/// Core semantic color tokens
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoreTokens {
    /// Primary color token
    pub primary: Srgb,
    /// On-primary color token (text/iconography on primary)
    pub on_primary: Srgb,
    /// Primary container color token
    pub primary_container: Srgb,
    /// On-primary container color token
    pub on_primary_container: Srgb,
    
    /// Secondary color token
    pub secondary: Srgb,
    /// On-secondary color token
    pub on_secondary: Srgb,
    /// Secondary container color token
    pub secondary_container: Srgb,
    /// On-secondary container color token
    pub on_secondary_container: Srgb,
    
    /// Tertiary color token
    pub tertiary: Srgb,
    /// On-tertiary color token
    pub on_tertiary: Srgb,
    /// Tertiary container color token
    pub tertiary_container: Srgb,
    /// On-tertiary container color token
    pub on_tertiary_container: Srgb,
    
    /// Error color token
    pub error: Srgb,
    /// On-error color token
    pub on_error: Srgb,
    /// Error container color token
    pub error_container: Srgb,
    /// On-error container color token
    pub on_error_container: Srgb,
    
    /// Background color token
    pub background: Srgb,
    /// On-background color token
    pub on_background: Srgb,
    
    /// Surface color token
    pub surface: Srgb,
    /// On-surface color token
    pub on_surface: Srgb,
    /// Surface variant color token
    pub surface_variant: Srgb,
    /// On-surface variant color token
    pub on_surface_variant: Srgb,
    
    /// Outline color token
    pub outline: Srgb,
    /// Outline variant color token
    pub outline_variant: Srgb,
    
    /// Shadow color token
    pub shadow: Srgb,
    /// Scrim color token
    pub scrim: Srgb,
    
    /// Inverse surface color token
    pub inverse_surface: Srgb,
    /// Inverse on-surface color token
    pub inverse_on_surface: Srgb,
    /// Inverse primary color token
    pub inverse_primary: Srgb,
}

impl Default for CoreTokens {
    fn default() -> Self {
        // Material Design 3 light theme colors with proper WCAG AA contrast ratios
        // Based on Material Design 3 baseline light theme color tokens
        Self {
            // Primary: Material blue-purple 600 (#6750A4) - provides good contrast with white text
            primary: Srgb::new(0.404, 0.314, 0.643),
            on_primary: Srgb::new(1.0, 1.0, 1.0), // White on primary
            primary_container: Srgb::new(0.918, 0.898, 1.0), // Light purple
            on_primary_container: Srgb::new(0.129, 0.067, 0.267),
            
            // Secondary: Material neutral variant 600
            secondary: Srgb::new(0.376, 0.365, 0.412),
            on_secondary: Srgb::new(1.0, 1.0, 1.0),
            secondary_container: Srgb::new(0.914, 0.898, 0.949),
            on_secondary_container: Srgb::new(0.129, 0.118, 0.165),
            
            // Tertiary: Material error-complementary
            tertiary: Srgb::new(0.482, 0.278, 0.424),
            on_tertiary: Srgb::new(1.0, 1.0, 1.0),
            tertiary_container: Srgb::new(1.0, 0.843, 0.933),
            on_tertiary_container: Srgb::new(0.196, 0.055, 0.165),
            
            // Error: Material error 600 - good contrast
            error: Srgb::new(0.729, 0.071, 0.212),
            on_error: Srgb::new(1.0, 1.0, 1.0),
            error_container: Srgb::new(1.0, 0.898, 0.906),
            on_error_container: Srgb::new(0.259, 0.0, 0.063),
            
            // Neutral colors
            background: Srgb::new(0.996, 0.996, 1.0),
            on_background: Srgb::new(0.067, 0.067, 0.078),
            
            surface: Srgb::new(0.996, 0.996, 1.0),
            on_surface: Srgb::new(0.067, 0.067, 0.078),
            surface_variant: Srgb::new(0.918, 0.910, 0.929),
            on_surface_variant: Srgb::new(0.286, 0.275, 0.314),
            
            outline: Srgb::new(0.459, 0.447, 0.486),
            outline_variant: Srgb::new(0.769, 0.757, 0.796),
            
            shadow: Srgb::new(0.0, 0.0, 0.0),
            scrim: Srgb::new(0.0, 0.0, 0.0),
            
            inverse_surface: Srgb::new(0.110, 0.110, 0.122),
            inverse_on_surface: Srgb::new(0.929, 0.929, 0.941),
            inverse_primary: Srgb::new(0.733, 0.694, 1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_core_tokens() {
        let tokens = CoreTokens::default();
        
        // Validate that we're using proper Material Design 3 colors
        // Primary should be Material blue-purple, not pure black
        assert!(tokens.primary.r > 0.3 && tokens.primary.r < 0.5);
        assert_eq!(tokens.on_primary, Srgb::new(1.0, 1.0, 1.0)); // White on primary
        
        // Error should be Material error red
        assert!(tokens.error.r > 0.7);
        assert!(tokens.error.g < 0.1);
        
        // Surface colors should be near-white
        assert!(tokens.surface.r > 0.99);
        assert!(tokens.background.r > 0.99);
    }
}
