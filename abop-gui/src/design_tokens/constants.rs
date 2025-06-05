//! Global design token constants for convenient access

// Import specific constants with namespace prefixes to avoid conflicts
pub use super::spacing::constants::{
    LG as SPACING_LG, MD as SPACING_MD, SM as SPACING_SM, XL as SPACING_XL, XS as SPACING_XS,
    XXL as SPACING_XXL,
};

pub use super::typography::constants::{
    BODY as TYPOGRAPHY_BODY, BODY_LARGE as TYPOGRAPHY_BODY_LARGE, CAPTION as TYPOGRAPHY_CAPTION,
    DISPLAY as TYPOGRAPHY_DISPLAY, HEADING_1 as TYPOGRAPHY_HEADING_1,
    HEADING_2 as TYPOGRAPHY_HEADING_2, HEADING_3 as TYPOGRAPHY_HEADING_3,
};

pub use super::radius::constants::{
    FULL as RADIUS_FULL, LG as RADIUS_LG, MD as RADIUS_MD, NONE as RADIUS_NONE, SM as RADIUS_SM,
    XL as RADIUS_XL,
};

pub use super::elevation::constants::{
    LG as ELEVATION_LG, MD as ELEVATION_MD, NONE as ELEVATION_NONE, SM as ELEVATION_SM,
    XL as ELEVATION_XL,
};

// Re-export namespaced modules for those who prefer explicit namespacing
pub use super::elevation::constants as elevation;
pub use super::radius::constants as radius;
pub use super::spacing::constants as spacing;
pub use super::typography::constants as typography;
pub use super::ui::constants as ui;

// Sizing constants for component dimensions
/// Component sizing constants following Material Design 3 specifications
///
/// This module provides standardized dimensions for common UI components
/// to ensure consistent sizing across the application.
pub mod sizing {
    /// 44px - Standard button height (medium) - increased for better text visibility
    pub const BUTTON_HEIGHT: f32 = 44.0;
    /// 36px - Small button height - increased for better text visibility
    pub const BUTTON_HEIGHT_SM: f32 = 36.0;
    /// 52px - Large button height - increased for better text visibility
    pub const BUTTON_HEIGHT_LG: f32 = 52.0;
    /// 36px - Standard input height
    pub const INPUT_HEIGHT: f32 = 36.0;
    /// 56px - Standard toolbar height (unified from previous 64px and 48px variants)
    pub const TOOLBAR_HEIGHT: f32 = 56.0;
    /// 16px - Small icon size
    pub const ICON_SM: f32 = 16.0;
    /// 20px - Default icon size
    pub const ICON_MD: f32 = 20.0;
    /// 24px - Large icon size
    pub const ICON_LG: f32 = 24.0;
    /// 40px - Small icon button size
    pub const ICON_BUTTON_SM: f32 = 40.0;
    /// 48px - Medium icon button size
    pub const ICON_BUTTON_MD: f32 = 48.0;
    /// 56px - Large icon button size
    pub const ICON_BUTTON_LG: f32 = 56.0;
    /// 1200px - Max container width
    pub const CONTAINER_MAX_WIDTH: f32 = 1200.0;
    /// 80px - Default minimum column width
    pub const MIN_COLUMN_WIDTH: f32 = 80.0;
    /// 70px - App title width
    pub const APP_TITLE_WIDTH: f32 = 70.0;
}

/// Default border radius for buttons
pub const BUTTON_RADIUS: f32 = RADIUS_MD;
/// Default border radius for containers
pub const CONTAINER_RADIUS: f32 = RADIUS_LG;
/// Default border radius for input fields
pub const INPUT_RADIUS: f32 = RADIUS_SM;
/// Thin border width for subtle outlines
pub const BORDER_WIDTH_THIN: f32 = ui::BORDER_WIDTH_STANDARD;
/// Thick border width for strong outlines
pub const BORDER_WIDTH_THICK: f32 = ui::BORDER_WIDTH_THICK;
/// Small border radius for subtle rounding
pub const BORDER_RADIUS_SM: f32 = RADIUS_SM;
/// Medium border radius for moderate rounding
pub const BORDER_RADIUS_MD: f32 = RADIUS_MD;
/// Large border radius for pronounced rounding
pub const BORDER_RADIUS_LG: f32 = RADIUS_LG;

// Global export of common UI visual effects
/// Subtle hover opacity adjustment
pub const HOVER_EFFECT: f32 = ui::HOVER_OPACITY_ADJUSTMENT;
/// Stronger pressed state opacity adjustment
pub const PRESS_EFFECT: f32 = ui::PRESSED_OPACITY_ADJUSTMENT;
/// Standard disabled opacity
pub const DISABLED_OPACITY: f32 = ui::DISABLED_OPACITY;
/// Standard surface overlay opacity
pub const SURFACE_OVERLAY: f32 = ui::SURFACE_OVERLAY_OPACITY;
