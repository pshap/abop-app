//! Semantic color tokens for consistent color usage
//!
//! This module provides semantic color mappings that translate Material Design
//! color roles into application-specific semantic meanings.

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
        // Use Material Design default colors for better consistency
        let material_colors = crate::styling::material::MaterialColors::light_default();
        
        Self {
            primary: material_colors.primary.base,
            secondary: material_colors.secondary.base,
            success: material_colors.tertiary.base,    // Use tertiary for success (green)
            warning: material_colors.error.container,  // Use error container for warning
            error: material_colors.error.base,
            info: material_colors.primary.container,   // Use primary container for info
            surface: material_colors.surface,
            on_surface: material_colors.on_surface,
        }
    }
}
