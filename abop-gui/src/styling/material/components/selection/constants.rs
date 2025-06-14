//! Constants for Material Design 3 selection components
//!
//! This module provides all the constants used across selection components,
//! organized by category for better maintainability and discoverability.
//!
//! # Usage Examples
//!
//! ```rust,ignore
//! use crate::styling::material::components::selection::constants::*;
//!
//! // UI constants
//! let min_touch_size = ui::MIN_TOUCH_TARGET_SIZE; // 48.0
//! let max_label_len = ui::MAX_LABEL_LENGTH; // 200
//!
//! // Animation timing
//! let duration = animation::DEFAULT_DURATION_MS; // 200
//! let fast_duration = animation::FAST_DURATION_MS; // 100
//!
//! // Size constants
//! let large_touch_target = sizes::touch_targets::LARGE; // 48.0
//! let medium_border = sizes::borders::MEDIUM; // 2.0
//!
//! // Chip-specific constants
//! let chip_max_label = chips::MAX_LABEL_LENGTH; // 100
//! let chip_padding = chips::HORIZONTAL_PADDING; // 16.0
//! ```
//!
//! # Organization
//!
//! Constants are organized into logical modules:
//! - [`ui`] - General UI constants (touch targets, label lengths)
//! - [`animation`] - Animation timing and motion constants
//! - [`chips`] - Chip-specific styling constants
//! - [`sizes`] - Component sizing with sub-modules for different properties
//! - [`metadata_keys`] - Type-safe keys for component metadata
//! - [`validation`] - Input validation and form constants
//! - [`accessibility`] - WCAG compliance and contrast ratios

/// UI-related constants following Material Design 3 specifications
pub mod ui {
    /// Minimum touch target size per Material Design guidelines (48px)
    pub const MIN_TOUCH_TARGET_SIZE: f32 = 48.0;

    /// Maximum recommended label length for accessibility
    pub const MAX_LABEL_LENGTH: usize = 200;

    /// Default empty label for components that don't require labels
    pub const DEFAULT_LABEL: &str = "";
}

/// Animation-related constants following Material Design 3 motion system
pub mod animation {
    /// Default animation duration for state transitions (matches Material Design 3)
    pub const DEFAULT_DURATION_MS: u64 = 200;

    /// Animation duration when reduced motion is enabled
    pub const REDUCED_MOTION_DURATION_MS: u64 = 0;

    /// Fast animation duration for quick feedback
    pub const FAST_DURATION_MS: u64 = 100;

    /// Slow animation duration for prominent transitions
    pub const SLOW_DURATION_MS: u64 = 300;
}

/// Chip-specific constants with stricter requirements
pub mod chips {
    /// Maximum label length for chip components (stricter than general components)
    pub const MAX_LABEL_LENGTH: usize = 100;

    /// Minimum chip height following Material Design 3
    pub const MIN_HEIGHT: f32 = 32.0;
    /// Standard chip padding
    pub const HORIZONTAL_PADDING: f32 = 16.0;
    /// Vertical padding for chips following Material Design 3 spacing guidelines
    pub const VERTICAL_PADDING: f32 = 8.0;
}

/// Size-related constants for consistent component dimensions
pub mod sizes {
    /// Small component size in pixels
    pub const SMALL_SIZE_PX: f32 = 16.0;

    /// Medium component size in pixels  
    pub const MEDIUM_SIZE_PX: f32 = 20.0;

    /// Large component size in pixels
    pub const LARGE_SIZE_PX: f32 = 24.0;

    /// Touch target sizes for each component size
    pub mod touch_targets {
        /// Touch target for small components
        pub const SMALL: f32 = 32.0;

        /// Touch target for medium components
        pub const MEDIUM: f32 = 40.0;

        /// Touch target for large components  
        pub const LARGE: f32 = 48.0;
    }

    /// Border widths for each component size
    pub mod borders {
        /// Border width for small components
        pub const SMALL: f32 = 1.5;

        /// Border width for medium components
        pub const MEDIUM: f32 = 2.0;

        /// Border width for large components
        pub const LARGE: f32 = 2.5;
    }

    /// Text sizes for labels at each component size
    pub mod text {
        /// Text size for small components
        pub const SMALL: f32 = 12.0;

        /// Text size for medium components
        pub const MEDIUM: f32 = 14.0;

        /// Text size for large components
        pub const LARGE: f32 = 16.0;
    }
}

/// Metadata key constants for type-safe metadata access
pub mod metadata_keys {
    /// Leading icon configuration key
    pub const LEADING_ICON: &str = "leading_icon";

    /// Trailing icon configuration key
    pub const TRAILING_ICON: &str = "trailing_icon";

    /// Badge configuration key
    pub const BADGE: &str = "badge";

    /// Badge color configuration key
    pub const BADGE_COLOR: &str = "badge_color";

    /// Layout configuration key
    pub const LAYOUT: &str = "layout";

    /// Spacing configuration key
    pub const SPACING: &str = "spacing";

    /// All supported metadata keys for validation
    pub const ALL_SUPPORTED: &[&str] = &[
        LEADING_ICON,
        TRAILING_ICON,
        BADGE,
        BADGE_COLOR,
        LAYOUT,
        SPACING,
    ];
}

/// Validation-related constants for form validation and user input
pub mod validation {
    /// Minimum length for meaningful text input
    pub const MIN_INPUT_LENGTH: usize = 1;

    /// Maximum recommended characters for single-line inputs
    pub const MAX_SINGLE_LINE_INPUT: usize = 255;

    /// Maximum characters for multi-line text areas
    pub const MAX_MULTILINE_INPUT: usize = 2000;

    /// Debounce delay for real-time validation (milliseconds)
    pub const VALIDATION_DEBOUNCE_MS: u64 = 300;

    /// Maximum number of validation errors to display simultaneously
    pub const MAX_VISIBLE_ERRORS: usize = 3;
}

/// Accessibility-related constants following WCAG guidelines
pub mod accessibility {
    /// Minimum contrast ratio for normal text (WCAG AA)
    pub const MIN_CONTRAST_NORMAL: f32 = 4.5;

    /// Minimum contrast ratio for large text (WCAG AA)
    pub const MIN_CONTRAST_LARGE: f32 = 3.0;

    /// Minimum contrast ratio for UI components (WCAG AA)
    pub const MIN_CONTRAST_UI: f32 = 3.0;

    /// Enhanced contrast ratio for AAA compliance
    pub const MIN_CONTRAST_AAA: f32 = 7.0;
}

#[cfg(test)]
mod tests {
    use super::*;    #[test]
    fn test_ui_constants() {
        assert_eq!(ui::MIN_TOUCH_TARGET_SIZE, 48.0);
        assert_eq!(ui::MAX_LABEL_LENGTH, 200);
        assert!(!ui::DEFAULT_LABEL.is_empty()); // Check that default label is not empty
    }

    #[test]
    fn test_animation_constants() {
        assert_eq!(animation::DEFAULT_DURATION_MS, 200);
        assert_eq!(animation::REDUCED_MOTION_DURATION_MS, 0);
        // Validate duration relationships - removed assertions that are always true
    }    #[test]
    fn test_chip_constants() {
        // Removed constant assertions that are always true
        assert_eq!(chips::MAX_LABEL_LENGTH, 100);
    }

    #[test]
    fn test_size_constants() {
        // Test actual values instead of relationships
        assert_eq!(sizes::SMALL_SIZE_PX, 24.0);
        assert_eq!(sizes::MEDIUM_SIZE_PX, 32.0);
        assert_eq!(sizes::LARGE_SIZE_PX, 40.0);
    }

    #[test]
    fn test_metadata_keys() {
        // Test that all keys are present in the supported list
        assert!(metadata_keys::ALL_SUPPORTED.contains(&metadata_keys::LEADING_ICON));
        assert!(metadata_keys::ALL_SUPPORTED.contains(&metadata_keys::TRAILING_ICON));
        assert!(metadata_keys::ALL_SUPPORTED.contains(&metadata_keys::BADGE));
        assert!(metadata_keys::ALL_SUPPORTED.contains(&metadata_keys::BADGE_COLOR));
        assert!(metadata_keys::ALL_SUPPORTED.contains(&metadata_keys::LAYOUT));
        assert!(metadata_keys::ALL_SUPPORTED.contains(&metadata_keys::SPACING));

        // Test count matches
        assert_eq!(metadata_keys::ALL_SUPPORTED.len(), 6);
    }

    #[test]
    fn test_validation_constants() {
        // Test validation constant relationships
        assert!(validation::MIN_INPUT_LENGTH > 0);
        assert!(validation::MAX_SINGLE_LINE_INPUT < validation::MAX_MULTILINE_INPUT);
        assert!(validation::MAX_SINGLE_LINE_INPUT > ui::MAX_LABEL_LENGTH);
        assert!(validation::VALIDATION_DEBOUNCE_MS > 0);
        assert!(validation::MAX_VISIBLE_ERRORS >= 1);
    }

    #[test]
    fn test_accessibility_constants() {
        // Test accessibility contrast ratios follow WCAG guidelines
        assert!(accessibility::MIN_CONTRAST_NORMAL >= 4.5);
        assert!(accessibility::MIN_CONTRAST_LARGE >= 3.0);
        assert!(accessibility::MIN_CONTRAST_UI >= 3.0);
        assert!(accessibility::MIN_CONTRAST_AAA >= 7.0);

        // Test relationships between contrast levels
        assert!(accessibility::MIN_CONTRAST_AAA > accessibility::MIN_CONTRAST_NORMAL);
        assert!(accessibility::MIN_CONTRAST_NORMAL > accessibility::MIN_CONTRAST_LARGE);
        assert!(accessibility::MIN_CONTRAST_NORMAL >= accessibility::MIN_CONTRAST_UI);
    }
}
