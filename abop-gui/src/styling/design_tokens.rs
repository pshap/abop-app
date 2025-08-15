//! Centralized design tokens for ABOP application
//!
//! This module provides unified design tokens that eliminate duplication
//! across the application. All components should use these tokens instead
//! of hardcoded values.
//!
//! # Usage
//!
//! ```rust
//! use crate::styling::design_tokens::{animation, validation, spacing};
//!
//! // Animation durations
//! let duration = animation::STANDARD_DURATION;
//! let slow_duration = animation::SLOW_DURATION;
//!
//! // Validation limits
//! let max_label = validation::MAX_LABEL_LENGTH;
//! let max_multiline = validation::MAX_MULTILINE_INPUT;
//! ```

use std::time::Duration;

/// Animation design tokens following Material Design 3 motion system
pub mod animation {
    use super::Duration;

    /// Standard animation duration for most state transitions (200ms)
    pub const STANDARD_DURATION: Duration = Duration::from_millis(200);

    /// Slow animation duration for prominent transitions (300ms)
    pub const SLOW_DURATION: Duration = Duration::from_millis(300);

    /// Fast animation duration for quick feedback (100ms)
    pub const FAST_DURATION: Duration = Duration::from_millis(100);

    /// Animation duration when reduced motion is enabled (0ms)
    pub const REDUCED_MOTION_DURATION: Duration = Duration::from_millis(0);

    /// Validation debounce duration for input fields (300ms)
    pub const VALIDATION_DEBOUNCE_DURATION: Duration = Duration::from_millis(300);

    /// Standard duration in milliseconds (for legacy compatibility)
    pub const STANDARD_DURATION_MS: u64 = 200;

    /// Slow duration in milliseconds (for legacy compatibility)
    pub const SLOW_DURATION_MS: u64 = 300;

    /// Fast duration in milliseconds (for legacy compatibility)
    pub const FAST_DURATION_MS: u64 = 100;
}

/// Validation design tokens for consistent limits across components
pub mod validation {
    /// Maximum recommended label length for accessibility and UI consistency
    pub const MAX_LABEL_LENGTH: usize = 200;

    /// Maximum label length for compact components like chips
    pub const MAX_COMPACT_LABEL_LENGTH: usize = 100;

    /// Maximum multiline input length to prevent performance issues
    pub const MAX_MULTILINE_INPUT: usize = 2000;

    /// Maximum lines for multiline text areas
    pub const MAX_TEXT_LINES: usize = 20;

    /// Validation debounce duration in milliseconds
    pub const VALIDATION_DEBOUNCE_MS: u64 = 300;
}

/// Spacing design tokens following Material Design 3 spacing system
pub mod spacing {
    /// Minimum touch target size per Material Design guidelines (48px)
    pub const MIN_TOUCH_TARGET_SIZE: f32 = 48.0;

    /// Standard padding unit (8dp in Material Design)
    pub const UNIT: f32 = 8.0;

    /// Extra small spacing (4dp)
    pub const XS: f32 = 4.0;

    /// Small spacing (8dp)
    pub const SM: f32 = 8.0;

    /// Medium spacing (16dp)
    pub const MD: f32 = 16.0;

    /// Large spacing (24dp)
    pub const LG: f32 = 24.0;

    /// Extra large spacing (32dp)
    pub const XL: f32 = 32.0;
}

/// Typography design tokens for consistent text sizing
pub mod typography {
    /// Default line height ratio for readable text
    pub const LINE_HEIGHT_RATIO: f32 = 1.4;

    /// Minimum font size for accessibility
    pub const MIN_FONT_SIZE: f32 = 12.0;

    /// Default body text size
    pub const BODY_SIZE: f32 = 14.0;

    /// Large text size for headers
    pub const HEADER_SIZE: f32 = 18.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_duration_consistency() {
        assert_eq!(animation::STANDARD_DURATION, Duration::from_millis(animation::STANDARD_DURATION_MS));
        assert_eq!(animation::SLOW_DURATION, Duration::from_millis(animation::SLOW_DURATION_MS));
        assert_eq!(animation::FAST_DURATION, Duration::from_millis(animation::FAST_DURATION_MS));
    }

    #[test]
    fn test_validation_limits_are_reasonable() {
        assert!(validation::MAX_LABEL_LENGTH > validation::MAX_COMPACT_LABEL_LENGTH);
        assert!(validation::MAX_MULTILINE_INPUT > validation::MAX_LABEL_LENGTH);
        assert!(validation::MAX_TEXT_LINES > 0);
    }

    #[test]
    fn test_spacing_progression() {
        assert!(spacing::XS < spacing::SM);
        assert!(spacing::SM < spacing::MD);
        assert!(spacing::MD < spacing::LG);
        assert!(spacing::LG < spacing::XL);
    }

    #[test]
    fn test_touch_target_accessibility() {
        assert!(spacing::MIN_TOUCH_TARGET_SIZE >= 44.0); // WCAG minimum
    }

    #[test]
    fn test_typography_accessibility() {
        assert!(typography::MIN_FONT_SIZE >= 12.0); // Accessibility minimum
        assert!(typography::LINE_HEIGHT_RATIO >= 1.2); // Readability minimum
    }
}