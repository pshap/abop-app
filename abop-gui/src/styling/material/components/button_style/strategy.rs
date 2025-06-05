//! Strategy pattern for button variant styling
//!
//! This module defines the `ButtonStyleStrategy` trait that allows different button variants
//! to implement their own styling logic while maintaining a consistent interface.

use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::{Background, Border, Color};

/// Button state for styling calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// Default button state (no interaction)
    Default,
    /// Button is being hovered over by the cursor
    Hovered,
    /// Button is being pressed/clicked
    Pressed,
    /// Button is disabled and non-interactive
    Disabled,
    /// Button has keyboard focus
    Focused,
}

/// Comprehensive styling properties for a button state
#[derive(Debug, Clone)]
pub struct ButtonStyling {
    /// Background color or gradient for the button
    pub background: Background,
    /// Text color for button labels
    pub text_color: Color,
    /// Border styling including color, width, and radius
    pub border: Border,
    /// Optional shadow for elevation effects
    pub shadow: Option<iced::Shadow>,
    /// Optional color for button icons
    pub icon_color: Option<Color>,
}

/// Strategy trait for button variant styling
pub trait ButtonStyleStrategy {
    /// Get styling for a specific button state
    fn get_styling(
        &self,
        state: ButtonState,
        tokens: &MaterialTokens,
        colors: &MaterialColors,
        elevation: &MaterialElevation,
        shapes: &MaterialShapes,
    ) -> ButtonStyling;

    /// Get the variant name for debugging and logging
    fn variant_name(&self) -> &'static str;

    /// Whether this variant supports elevation changes
    fn supports_elevation(&self) -> bool {
        false
    }

    /// Whether this variant has a border by default
    fn has_border(&self) -> bool {
        false
    }

    /// Get the base elevation level for this variant
    fn base_elevation(&self) -> f32 {
        0.0
    }
}

/// Context information for button styling
#[derive(Debug, Clone, Default)]
pub struct ButtonStyleContext {
    /// Whether this is a primary action button
    pub is_primary: bool,
    /// Whether this button represents a destructive action
    pub is_destructive: bool,
    /// Whether the button contains an icon
    pub has_icon: bool,
    /// Whether the button is in a loading state
    pub is_loading: bool,
}
