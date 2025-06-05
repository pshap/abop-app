//! Surface color tokens for Material Design 3
//!
//! This module defines the surface color tokens that represent the various
//! surface colors used in Material Design 3, including surface tints and elevations.

use super::super::Srgb;

/// Surface color tokens
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceTokens {
    /// Base surface color
    pub base: Srgb,
    /// On-surface color for content
    pub on: Srgb,
    /// Bright surface variant (for higher elevation surfaces)
    pub bright: Srgb,
    /// Dim surface variant (for lower contrast)
    pub dim: Srgb,
    /// Container surface color
    pub container: Srgb,
    /// Lowest elevation surface color
    pub lowest: Srgb,
    /// Low elevation surface color
    pub low: Srgb,
    /// Medium elevation surface color
    pub medium: Srgb,
    /// High elevation surface color
    pub high: Srgb,
    /// Highest elevation surface color
    pub highest: Srgb,
}

impl Default for SurfaceTokens {
    fn default() -> Self {
        // Default light theme surface colors
        Self {
            base: Srgb::new(1.0, 1.0, 1.0),
            on: Srgb::new(0.1, 0.1, 0.1),
            bright: Srgb::new(0.98, 0.98, 0.98),
            dim: Srgb::new(0.9, 0.9, 0.9),
            container: Srgb::new(0.98, 0.98, 0.98),
            lowest: Srgb::new(1.0, 1.0, 1.0),
            low: Srgb::new(0.99, 0.99, 0.99),
            medium: Srgb::new(0.98, 0.98, 0.98),
            high: Srgb::new(0.97, 0.97, 0.97),
            highest: Srgb::new(0.96, 0.96, 0.96),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_surface_tokens() {
        let tokens = SurfaceTokens::default();
        
        // Basic validation of default values
        assert_eq!(tokens.base.r, 1.0);
        assert_eq!(tokens.on.r, 0.1);
        assert_eq!(tokens.bright.r, 0.98);
        assert_eq!(tokens.dim.r, 0.9);
        
        // Elevation colors should decrease in brightness
        assert!(tokens.lowest.r > tokens.low.r);
        assert!(tokens.low.r > tokens.medium.r);
        assert!(tokens.medium.r > tokens.high.r);
        assert!(tokens.high.r > tokens.highest.r);
    }
}
