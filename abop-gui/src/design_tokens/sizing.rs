//! Sizing tokens for consistent component dimensions

/// Sizing tokens for consistent component dimensions
#[derive(Debug, Clone)]
pub struct SizingTokens {
    /// 44px - Standard button height (medium) - increased for better text visibility
    pub button_height: f32,
    /// 36px - Small button height - increased for better text visibility
    pub button_height_sm: f32,
    /// 52px - Large button height - increased for better text visibility
    pub button_height_lg: f32,
    /// 36px - Standard input height
    pub input_height: f32,
    /// 56px - Unified toolbar height - consolidation of previous 64px and 48px variants
    pub toolbar_height: f32,
    /// 16px - Small icon size
    pub icon_sm: f32,
    /// 20px - Default icon size
    pub icon_md: f32,
    /// 24px - Large icon size
    pub icon_lg: f32,
    /// 40px - Small icon button size
    pub icon_button_sm: f32,
    /// 48px - Medium icon button size
    pub icon_button_md: f32,
    /// 56px - Large icon button size
    pub icon_button_lg: f32,
    /// 1200px - Max container width
    pub container_max_width: f32,
    /// 80px - Default minimum column width
    pub min_column_width: f32,
    /// 70px - App title width
    pub app_title_width: f32,
}

impl Default for SizingTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl SizingTokens {
    /// Create a new set of sizing tokens with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            button_height: 44.0,    // Increased from 40.0 for better text visibility
            button_height_sm: 36.0, // Increased from 32.0 for better text visibility
            button_height_lg: 52.0, // Increased from 48.0 for better text visibility
            input_height: 36.0,
            toolbar_height: 56.0, // Unified toolbar height - consolidation of previous 64px and 48px variants
            icon_sm: 16.0,
            icon_md: 20.0,
            icon_lg: 24.0,
            icon_button_sm: 40.0,
            icon_button_md: 48.0,
            icon_button_lg: 56.0,
            container_max_width: 1200.0,
            min_column_width: 80.0,
            app_title_width: 70.0,
        }
    }
}
