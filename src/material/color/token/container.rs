//! Container color tokens for Material Design 3
//!
//! This module defines the container color tokens that represent the various
//! container colors used in Material Design 3, including filled and outlined variants.

use super::super::Srgb;

/// Container color tokens
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContainerTokens {
    /// Primary container color
    pub primary: Srgb,
    /// On-primary container color
    pub on_primary: Srgb,
    /// Secondary container color
    pub secondary: Srgb,
    /// On-secondary container color
    pub on_secondary: Srgb,
    /// Tertiary container color
    pub tertiary: Srgb,
    /// On-tertiary container color
    pub on_tertiary: Srgb,
    /// Error container color
    pub error: Srgb,
    /// On-error container color
    pub on_error: Srgb,
    /// Surface container color
    pub surface: Srgb,
    /// Surface container low color
    pub surface_low: Srgb,
    /// Surface container high color
    pub surface_high: Srgb,
    /// Surface container highest color
    pub surface_highest: Srgb,
    /// Filled container color
    pub filled: Srgb,
    /// Filled container variant color
    pub filled_variant: Srgb,
    /// On-filled container color
    pub on_filled: Srgb,
    /// Outlined container color
    pub outlined: Srgb,
    /// Outlined container variant color
    pub outlined_variant: Srgb,
    /// On-outlined container color
    pub on_outlined: Srgb,
}

impl Default for ContainerTokens {
    fn default() -> Self {
        // Default light theme container colors
        Self {
            primary: Srgb::new(0.9, 0.9, 0.9),
            on_primary: Srgb::new(0.1, 0.1, 0.1),
            secondary: Srgb::new(0.9, 0.92, 0.98),
            on_secondary: Srgb::new(0.1, 0.1, 0.2),
            tertiary: Srgb::new(0.98, 0.92, 0.98),
            on_tertiary: Srgb::new(0.2, 0.1, 0.2),
            error: Srgb::new(1.0, 0.9, 0.9),
            on_error: Srgb::new(0.4, 0.0, 0.0),
            surface: Srgb::new(0.98, 0.98, 0.98),
            surface_low: Srgb::new(0.96, 0.96, 0.96),
            surface_high: Srgb::new(0.94, 0.94, 0.94),
            surface_highest: Srgb::new(0.92, 0.92, 0.92),
            filled: Srgb::new(0.95, 0.95, 0.95),
            filled_variant: Srgb::new(0.93, 0.93, 0.93),
            on_filled: Srgb::new(0.1, 0.1, 0.1),
            outlined: Srgb::new(1.0, 1.0, 1.0),
            outlined_variant: Srgb::new(0.98, 0.98, 0.98),
            on_outlined: Srgb::new(0.1, 0.1, 0.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_container_tokens() {
        let tokens = ContainerTokens::default();
        
        // Basic validation of default values
        assert_eq!(tokens.primary.r, 0.9);
        assert_eq!(tokens.on_primary.r, 0.1);
        assert_eq!(tokens.error.r, 1.0);
        assert_eq!(tokens.on_error.r, 0.4);
        
        // Surface containers should have decreasing brightness
        assert!(tokens.surface.r > tokens.surface_low.r);
        assert!(tokens.surface_low.r > tokens.surface_high.r);
        assert!(tokens.surface_high.r > tokens.surface_highest.r);
    }
}
