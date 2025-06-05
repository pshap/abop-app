//! Semantic color tokens for consistent color usage across components

use iced::Color;

/// Semantic color tokens for consistent color usage across components
#[derive(Debug, Clone)]
pub struct SemanticColors {
    /// Primary brand color for main actions and branding
    pub primary: Color,
    /// Secondary color for alternative actions
    pub secondary: Color,
    /// Success color for positive feedback
    pub success: Color,
    /// Warning color for cautionary feedback
    pub warning: Color,
    /// Error color for negative feedback and destructive actions
    pub error: Color,
    /// Info color for informational feedback
    pub info: Color,
    /// Surface color for elevated components
    pub surface: Color,
    /// Text color that contrasts with surface
    pub on_surface: Color,
}

impl SemanticColors {
    /// Create semantic colors for dark theme
    #[must_use]
    pub const fn dark() -> Self {
        Self {
            primary: Color::from_rgb(0.2, 0.6, 0.8),    // Blue
            secondary: Color::from_rgb(0.5, 0.5, 0.5),  // Gray
            success: Color::from_rgb(0.2, 0.7, 0.3),    // Green
            warning: Color::from_rgb(0.9, 0.6, 0.1),    // Orange
            error: Color::from_rgb(0.8, 0.2, 0.2),      // Red
            info: Color::from_rgb(0.3, 0.7, 0.9),       // Light Blue
            surface: Color::from_rgb(0.15, 0.15, 0.15), // Dark Gray
            on_surface: Color::from_rgb(0.9, 0.9, 0.9), // Light Gray
        }
    }

    /// Create semantic colors for light theme
    #[must_use]
    pub const fn light() -> Self {
        Self {
            primary: Color::from_rgb(0.1, 0.4, 0.7),    // Darker Blue
            secondary: Color::from_rgb(0.4, 0.4, 0.4),  // Dark Gray
            success: Color::from_rgb(0.1, 0.6, 0.2),    // Dark Green
            warning: Color::from_rgb(0.8, 0.5, 0.0),    // Dark Orange
            error: Color::from_rgb(0.7, 0.1, 0.1),      // Dark Red
            info: Color::from_rgb(0.2, 0.5, 0.8),       // Dark Blue
            surface: Color::from_rgb(0.98, 0.98, 0.98), // Off White
            on_surface: Color::from_rgb(0.1, 0.1, 0.1), // Dark Gray
        }
    }

    /// Get semantic colors for the given theme mode
    #[must_use]
    pub const fn for_theme(is_dark: bool) -> Self {
        if is_dark { Self::dark() } else { Self::light() }
    }
}

impl Default for SemanticColors {
    fn default() -> Self {
        Self::light()
    }
}
