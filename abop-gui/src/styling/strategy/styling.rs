//! Component styling data structures
//!
//! This module contains the data structures returned by styling strategies,
//! providing all necessary styling information for component rendering.

use iced::{Background, Border, Color};

/// Complete styling information for a component
///
/// This structure contains all the styling properties that a component
/// needs for consistent Material Design 3 appearance.
#[derive(Debug, Clone)]
pub struct ComponentStyling {
    /// Background styling (color, gradient, etc.)
    pub background: Background,

    /// Text color for labels and content
    pub text_color: Color,

    /// Icon color (may differ from text for better contrast)
    pub icon_color: Option<Color>,

    /// Border styling
    pub border: Border,

    /// Shadow/elevation effect
    pub shadow: Option<iced::Shadow>,

    /// Additional state-specific opacity
    pub opacity: f32,
}

impl ComponentStyling {
    /// Create a new component styling with default values
    #[must_use]
    pub fn new() -> Self {
        Self {
            background: Background::Color(Color::TRANSPARENT),
            text_color: Color::BLACK,
            icon_color: None,
            border: Border::default(),
            shadow: None,
            opacity: 1.0,
        }
    }

    /// Get the effective icon color (falls back to text color if not specified)
    #[must_use]
    pub fn effective_icon_color(&self) -> Color {
        self.icon_color.unwrap_or(self.text_color)
    }

    /// Apply opacity modifier to all colors
    #[must_use]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;

        // Apply opacity to background if it's a solid color
        if let Background::Color(color) = self.background {
            self.background =
                Background::Color(Color::new(color.r, color.g, color.b, color.a * opacity));
        }

        // Apply opacity to text color
        self.text_color = Color::new(
            self.text_color.r,
            self.text_color.g,
            self.text_color.b,
            self.text_color.a * opacity,
        );

        // Apply opacity to icon color if specified
        if let Some(icon_color) = self.icon_color {
            self.icon_color = Some(Color::new(
                icon_color.r,
                icon_color.g,
                icon_color.b,
                icon_color.a * opacity,
            ));
        }

        self
    }
}

impl Default for ComponentStyling {
    fn default() -> Self {
        Self::new()
    }
}
