//! Typography tokens for consistent font sizes

/// Typography tokens for consistent font sizes
#[derive(Debug, Clone)]
pub struct TypographyTokens {
    /// 12px - Small labels, captions
    pub caption: u16,
    /// 14px - Default body text
    pub body: u16,
    /// 16px - Large body text
    pub body_large: u16,
    /// 18px - Small headings
    pub heading_3: u16,
    /// 20px - Medium headings
    pub heading_2: u16,
    /// 24px - Large headings
    pub heading_1: u16,
    /// 32px - Display text
    pub display: u16,
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl TypographyTokens {
    /// Create a new set of typography tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            caption: 12,
            body: 14,
            body_large: 16,
            heading_3: 18,
            heading_2: 20,
            heading_1: 24,
            display: 32,
        }
    }
}

/// Typography token constants for global use
pub mod constants {
    /// 12px - Small labels, captions
    pub const CAPTION: u16 = 12;
    /// 14px - Default body text
    pub const BODY: u16 = 14;
    /// 16px - Large body text
    pub const BODY_LARGE: u16 = 16;
    /// 18px - Small headings
    pub const HEADING_3: u16 = 18;
    /// 20px - Medium headings
    pub const HEADING_2: u16 = 20;
    /// 24px - Large headings
    pub const HEADING_1: u16 = 24;
    /// 32px - Display text
    pub const DISPLAY: u16 = 32;
}

// Re-export constants at module level for easy access
pub use constants::*;
