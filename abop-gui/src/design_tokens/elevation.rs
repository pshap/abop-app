//! Elevation tokens for consistent shadows and depth

/// Elevation tokens for consistent shadows and depth
#[derive(Debug, Clone)]
pub struct ElevationTokens {
    /// 0px - No elevation
    pub none: f32,
    /// 2px - Small elevation
    pub sm: f32,
    /// 4px - Default elevation
    pub md: f32,
    /// 8px - Large elevation
    pub lg: f32,
    /// 16px - Extra large elevation
    pub xl: f32,
}

impl Default for ElevationTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl ElevationTokens {
    /// Create a new set of elevation tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            none: 0.0,
            sm: 2.0,
            md: 4.0,
            lg: 8.0,
            xl: 16.0,
        }
    }
}

/// Elevation token constants for global use
pub mod constants {
    /// 0px - No elevation
    pub const NONE: f32 = 0.0;
    /// 2px - Small elevation
    pub const SM: f32 = 2.0;
    /// 4px - Default elevation
    pub const MD: f32 = 4.0;
    /// 8px - Large elevation
    pub const LG: f32 = 8.0;
    /// 16px - Extra large elevation
    pub const XL: f32 = 16.0;
}

// Re-export constants at module level for easy access
pub use constants::*;
