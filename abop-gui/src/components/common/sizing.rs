//! Button and component sizing constants and converters

use crate::styling::material::components::widgets::ButtonSize;

/// Material Design 3 button size constants (in pixels)
pub mod constants {
    /// Small button/icon dimensions
    pub const SMALL: f32 = 32.0;
    /// Medium button/icon dimensions (default)
    pub const MEDIUM: f32 = 40.0;
    /// Large button/icon dimensions
    pub const LARGE: f32 = 48.0;

    /// Small FAB dimensions
    pub const FAB_SMALL: f32 = 40.0;
    /// Medium FAB dimensions (default)
    pub const FAB_MEDIUM: f32 = 56.0;
    /// Large FAB dimensions
    pub const FAB_LARGE: f32 = 72.0;

    /// Compact button height
    pub const COMPACT_HEIGHT: f32 = 32.0;
    /// Prominent button height
    pub const PROMINENT_HEIGHT: f32 = 48.0;
}

// Re-export constants at the module level for backward compatibility
pub use constants::*;

/// Convert ButtonSize enum to pixel dimensions for regular buttons
#[must_use]
pub const fn button_size_to_pixels(size: ButtonSize) -> f32 {
    match size {
        ButtonSize::Small => SMALL,
        ButtonSize::Medium => MEDIUM,
        ButtonSize::Large => LARGE,
    }
}

/// Convert ButtonSize enum to pixel dimensions for FABs
#[must_use]
pub const fn fab_size_to_pixels(size: ButtonSize) -> f32 {
    match size {
        ButtonSize::Small => FAB_SMALL,
        ButtonSize::Medium => FAB_MEDIUM,
        ButtonSize::Large => FAB_LARGE,
    }
}
