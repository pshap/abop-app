//! Spacing tokens for consistent padding, margins, and gaps

/// Spacing tokens for consistent padding, margins, and gaps
#[derive(Debug, Clone)]
pub struct SpacingTokens {
    /// 4px - Tight spacing
    pub xs: f32,
    /// 8px - Small spacing
    pub sm: f32,
    /// 16px - Default spacing
    pub md: f32,
    /// 24px - Large spacing
    pub lg: f32,
    /// 32px - Extra large spacing
    pub xl: f32,
    /// 48px - Extra extra large spacing
    pub xxl: f32,
}

impl Default for SpacingTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl SpacingTokens {
    /// Create a new set of spacing tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
            xxl: 48.0,
        }
    }
}
