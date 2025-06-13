//! Semantic color tokens for consistent color usage
//!
//! This module provides semantic color mappings that translate Material Design
//! color roles into application-specific semantic meanings with guaranteed
//! color hues for semantic purposes.

use iced::Color;

/// Semantic color tokens for consistent color usage
///
/// This struct provides a semantic layer over Material Design color tokens,
/// mapping color roles to their intended semantic meaning in the application.
/// This abstraction allows for consistent color usage across the interface
/// while maintaining flexibility for theme changes.
#[derive(Debug, Clone)]
pub struct SemanticColors {
    /// Main brand color used for primary actions
    ///
    /// This color represents the primary brand identity and is used for
    /// main call-to-action buttons, active states, and primary navigation.
    pub primary: Color,

    /// Secondary brand color used for complementary actions
    ///
    /// This color complements the primary color and is used for secondary
    /// actions, alternative navigation, and supporting interface elements.
    pub secondary: Color,

    /// Success/positive feedback color (typically green)
    ///
    /// Used to indicate successful operations, positive states, and
    /// confirmations throughout the interface.
    pub success: Color,

    /// Warning/caution feedback color (typically amber)
    ///
    /// Used to indicate warning states, caution messages, and actions
    /// that require user attention before proceeding.
    pub warning: Color,

    /// Error/danger feedback color (typically red)
    ///
    /// Used to indicate error states, destructive actions, and
    /// critical messages that require immediate user attention.
    pub error: Color,

    /// Information feedback color (typically blue)
    ///
    /// Used to provide informational messages, help text, and
    /// neutral status indicators throughout the interface.
    pub info: Color,

    /// Background color for surfaces (e.g., cards)
    ///
    /// The base surface color used for cards, panels, and other
    /// elevated interface elements that need to stand out from the background.
    pub surface: Color,

    /// Text/icon color on surfaces
    ///
    /// The appropriate text and icon color that provides sufficient
    /// contrast when placed on surface backgrounds.
    pub on_surface: Color,
}

impl Default for SemanticColors {
    fn default() -> Self {
        // Default to light theme semantic colors
        Self::light()
    }
}

impl SemanticColors {
    /// Creates semantic colors optimized for light theme
    #[must_use]
    pub fn light() -> Self {
        let material_colors = crate::styling::material::MaterialColors::light_default();

        Self {
            primary: material_colors.primary.base,
            secondary: material_colors.secondary.base,
            // Dedicated semantic colors that are guaranteed to be the right hue
            success: Color::from_rgb(0.0, 0.6, 0.0), // Dark green for light theme
            warning: Color::from_rgb(0.8, 0.5, 0.0), // Dark amber for light theme
            error: material_colors.error.base,       // MD3 error is correctly red
            info: Color::from_rgb(0.0, 0.3, 0.8),    // Dark blue for light theme
            surface: material_colors.surface,
            on_surface: material_colors.on_surface,
        }
    }

    /// Creates semantic colors optimized for dark theme  
    #[must_use]
    pub fn dark() -> Self {
        let material_colors = crate::styling::material::MaterialColors::dark_default();

        Self {
            primary: material_colors.primary.base,
            secondary: material_colors.secondary.base,
            // Dedicated semantic colors that are guaranteed to be the right hue
            success: Color::from_rgb(0.2, 0.8, 0.2), // Bright green for dark theme
            warning: Color::from_rgb(1.0, 0.7, 0.0), // Bright amber for dark theme
            error: material_colors.error.base,       // MD3 error is correctly red
            info: Color::from_rgb(0.4, 0.7, 1.0),    // Light blue for dark theme
            surface: material_colors.surface,
            on_surface: material_colors.on_surface,
        }
    }
}
