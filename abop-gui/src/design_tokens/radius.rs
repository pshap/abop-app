//! Border radius tokens for consistent rounded corners

/// Border radius tokens for consistent rounded corners
#[derive(Debug, Clone)]
pub struct RadiusTokens {
    /// 0px - No radius
    pub none: f32,
    /// 4px - Small radius
    pub sm: f32,
    /// 8px - Default radius
    pub md: f32,
    /// 12px - Large radius
    pub lg: f32,
    /// 16px - Extra large radius
    pub xl: f32,
    /// 9999px - Fully rounded
    pub full: f32,
}

impl Default for RadiusTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl RadiusTokens {
    /// Create a new set of border radius tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            none: 0.0,
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
            full: 9999.0,
        }
    }
}

/// Border radius token constants for global use
pub mod constants {
    /// 0px - No radius
    pub const NONE: f32 = 0.0;
    /// 4px - Small radius
    pub const SM: f32 = 4.0;
    /// 8px - Default radius
    pub const MD: f32 = 8.0;
    /// 12px - Large radius
    pub const LG: f32 = 12.0;
    /// 16px - Extra large radius
    pub const XL: f32 = 16.0;
    /// 9999px - Fully rounded
    pub const FULL: f32 = 9999.0;
}

// Re-export constants at module level for easy access
pub use constants::*;
