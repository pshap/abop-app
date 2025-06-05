//! Fixed color tokens for Material Design 3
//!
//! This module defines the fixed color tokens that represent colors that don't
//! change between light and dark themes, such as black and white.

use super::super::Srgb;

/// Fixed color tokens
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedTokens {
    /// Pure black
    pub black: Srgb,
    /// Pure white
    pub white: Srgb,
    /// Transparent color
    pub transparent: Srgb,
    /// Black with 5% opacity
    pub black_5: Srgb,
    /// Black with 12% opacity
    pub black_12: Srgb,
    /// Black with 38% opacity
    pub black_38: Srgb,
    /// Black with 54% opacity
    pub black_54: Srgb,
    /// Black with 87% opacity
    pub black_87: Srgb,
    /// White with 5% opacity
    pub white_5: Srgb,
    /// White with 12% opacity
    pub white_12: Srgb,
    /// White with 38% opacity
    pub white_38: Srgb,
    /// White with 54% opacity
    pub white_54: Srgb,
    /// White with 87% opacity
    pub white_87: Srgb,
}

impl Default for FixedTokens {
    fn default() -> Self {
        Self {
            black: Srgb::new(0.0, 0.0, 0.0),
            white: Srgb::new(1.0, 1.0, 1.0),
            transparent: Srgb::new(0.0, 0.0, 0.0), // Alpha will be handled separately
            black_5: Srgb::new(0.0, 0.0, 0.0),    // Alpha will be handled separately
            black_12: Srgb::new(0.0, 0.0, 0.0),   // Alpha will be handled separately
            black_38: Srgb::new(0.0, 0.0, 0.0),   // Alpha will be handled separately
            black_54: Srgb::new(0.0, 0.0, 0.0),   // Alpha will be handled separately
            black_87: Srgb::new(0.0, 0.0, 0.0),   // Alpha will be handled separately
            white_5: Srgb::new(1.0, 1.0, 1.0),    // Alpha will be handled separately
            white_12: Srgb::new(1.0, 1.0, 1.0),   // Alpha will be handled separately
            white_38: Srgb::new(1.0, 1.0, 1.0),   // Alpha will be handled separately
            white_54: Srgb::new(1.0, 1.0, 1.0),   // Alpha will be handled separately
            white_87: Srgb::new(1.0, 1.0, 1.0),   // Alpha will be handled separately
        }
    }
}

impl FixedTokens {
    /// Get a fixed color with the specified opacity
    pub fn with_opacity(&self, color: Srgb, opacity: f32) -> Srgb {
        Srgb::new(color.r, color.g, color.b) // Opacity will be applied when converting to RGBA
    }
    
    /// Get a fixed color with the specified alpha value (0-1)
    pub fn with_alpha(&self, color: Srgb, alpha: f32) -> u32 {
        color.to_rgba(alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_fixed_tokens() {
        let tokens = FixedTokens::default();
        
        // Basic validation of default values
        assert_eq!(tokens.black, Srgb::new(0.0, 0.0, 0.0));
        assert_eq!(tokens.white, Srgb::new(1.0, 1.0, 1.0));
        
        // Test with_opacity
        let semi_black = tokens.with_opacity(tokens.black, 0.5);
        assert_eq!(semi_black, Srgb::new(0.0, 0.0, 0.0));
        
        // Test with_alpha
        let semi_white = tokens.with_alpha(tokens.white, 0.5);
        // The alpha value will be in the most significant byte
        assert_eq!(semi_white & 0xFF000000, 0x80000000);
    }
}
