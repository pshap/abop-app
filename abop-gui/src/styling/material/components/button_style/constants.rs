//! Constants for Material Design 3 button styling
//!
//! This module centralizes all magic numbers and design tokens used in button styling,
//! providing a single source of truth that aligns with Material Design 3 specifications.
//!
//! Note: Opacity values have been moved to `tokens.states.opacity` for centralized management.

/// Size constants for button dimensions
pub mod sizing {
    /// Small button height
    pub const SMALL_HEIGHT: f32 = 32.0;

    /// Medium button height (default)
    pub const MEDIUM_HEIGHT: f32 = 40.0;

    /// Large button height
    pub const LARGE_HEIGHT: f32 = 48.0;

    /// Icon button small size
    pub const ICON_SMALL: f32 = 24.0;

    /// Icon button medium size
    pub const ICON_MEDIUM: f32 = 32.0;

    /// Icon button large size
    pub const ICON_LARGE: f32 = 40.0;

    /// FAB small size
    pub const FAB_SMALL: f32 = 40.0;

    /// FAB medium size
    pub const FAB_MEDIUM: f32 = 56.0;

    /// FAB large size
    pub const FAB_LARGE: f32 = 96.0;

    /// Extended FAB height
    pub const EXTENDED_FAB_HEIGHT: f32 = 56.0;
}

/// Border radius constants
pub mod radius {
    /// Small button radius (Material Design 3 corner small)
    pub const SMALL: f32 = 8.0;

    /// Medium button radius (Material Design 3 corner medium - default for buttons)
    pub const MEDIUM: f32 = 12.0;

    /// Large button radius (Material Design 3 corner large)
    pub const LARGE: f32 = 16.0;

    /// Icon button radius (circular/large for better touch targets)
    pub const ICON: f32 = 20.0;

    /// FAB radius (Material Design 3 corner large)
    pub const FAB: f32 = 28.0;

    /// Extended FAB radius (Material Design 3 corner large)
    pub const EXTENDED_FAB: f32 = 16.0;
}

/// Padding constants for button content
pub mod padding {
    /// Small button horizontal padding
    pub const SMALL_HORIZONTAL: f32 = 16.0;

    /// Medium button horizontal padding
    pub const MEDIUM_HORIZONTAL: f32 = 24.0;

    /// Large button horizontal padding
    pub const LARGE_HORIZONTAL: f32 = 32.0;

    /// Vertical padding for all button sizes
    pub const VERTICAL: f32 = 0.0;

    /// Icon button padding
    pub const ICON: f32 = 8.0;

    /// FAB padding
    pub const FAB: f32 = 16.0;
}

/// Border width constants
pub mod border {
    /// Standard border width for outlined buttons
    pub const STANDARD: f32 = 1.0;

    /// Focus ring border width
    pub const FOCUS_RING: f32 = 2.0;
}

/// Elevation constants (Material Design 3 levels)
pub mod elevation {
    /// Level 0 - No elevation
    pub const LEVEL_0: f32 = 0.0;

    /// Level 1 - Elevated buttons resting state
    pub const LEVEL_1: f32 = 1.0;

    /// Level 2 - Elevated buttons hover state
    pub const LEVEL_2: f32 = 3.0;

    /// Level 3 - FAB resting state
    pub const LEVEL_3: f32 = 6.0;

    /// Level 4 - FAB hover state
    pub const LEVEL_4: f32 = 8.0;
}

/// Animation durations (in milliseconds)
pub mod animation {
    /// Standard transition duration for state changes
    pub const STANDARD: u64 = 200;

    /// Fast transition for quick interactions
    pub const FAST: u64 = 100;

    /// Slow transition for complex animations
    pub const SLOW: u64 = 300;
}
