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

/// Configuration for variant-specific colors and behaviors
#[derive(Debug, Clone)]
pub struct ButtonVariantConfig {
    /// Base background color for default state
    pub base_background: Color,
    /// Text color for the button
    pub text_color: Color,
    /// Icon color (usually same as text_color)
    pub icon_color: Color,
    /// Border color (transparent for filled variants)
    pub border_color: Color,
    /// Border width (0.0 for filled variants)
    pub border_width: f32,
    /// Border radius
    pub border_radius: f32,
    /// Optional shadow for elevation
    pub shadow: Option<iced::Shadow>,
    /// Whether this variant uses surface colors on hover/press
    pub uses_surface_on_interaction: bool,
    /// Custom hover background (if None, will darken base_background)
    pub custom_hover_background: Option<Color>,
    /// Custom pressed background (if None, will darken base_background more)
    pub custom_pressed_background: Option<Color>,
}

/// Common state handling logic for all button variants
pub struct ButtonStateHandler;

impl ButtonStateHandler {
    /// Apply common state styling based on configuration
    pub fn apply_state_styling(
        state: ButtonState,
        config: &ButtonVariantConfig,
        tokens: &MaterialTokens,
        colors: &MaterialColors,
    ) -> ButtonStyling {
        use crate::styling::color_utils::ColorUtils;
        use super::variants::create_button_border;
        
        match state {
            ButtonState::Default => ButtonStyling {
                background: Background::Color(config.base_background),
                text_color: config.text_color,
                border: create_button_border(config.border_color, config.border_width, config.border_radius),
                shadow: config.shadow.clone(),
                icon_color: Some(config.icon_color),
            },
            
            ButtonState::Hovered => {
                let (hover_bg, hover_text, hover_icon) = if config.uses_surface_on_interaction {
                    (colors.surface_variant, colors.on_surface_variant, colors.on_surface_variant)
                } else {
                    let hover_bg = config.custom_hover_background
                        .unwrap_or_else(|| ColorUtils::darken(config.base_background, 0.05));
                    (hover_bg, config.text_color, config.icon_color)
                };
                
                ButtonStyling {
                    background: Background::Color(hover_bg),
                    text_color: hover_text,
                    border: create_button_border(config.border_color, config.border_width, config.border_radius),
                    shadow: config.shadow.clone(),
                    icon_color: Some(hover_icon),
                }
            },
            
            ButtonState::Pressed => {
                let (pressed_bg, pressed_text, pressed_icon) = if config.uses_surface_on_interaction {
                    let pressed_bg = ColorUtils::darken(colors.surface_variant, 0.1);
                    (pressed_bg, colors.on_surface_variant, colors.on_surface_variant)
                } else {
                    let pressed_bg = config.custom_pressed_background
                        .unwrap_or_else(|| ColorUtils::darken(config.base_background, 0.1));
                    (pressed_bg, config.text_color, config.icon_color)
                };
                
                ButtonStyling {
                    background: Background::Color(pressed_bg),
                    text_color: pressed_text,
                    border: create_button_border(config.border_color, config.border_width, config.border_radius),
                    shadow: config.shadow.clone(),
                    icon_color: Some(pressed_icon),
                }
            },
            
            ButtonState::Disabled => {
                let disabled_alpha = tokens.states.opacity.disabled;
                let disabled_bg = if config.base_background == Color::TRANSPARENT {
                    Color::TRANSPARENT
                } else {
                    ColorUtils::with_alpha(config.base_background, 0.38)
                };
                let disabled_text = ColorUtils::with_alpha(colors.on_surface, disabled_alpha);
                let disabled_border = if config.border_width > 0.0 {
                    ColorUtils::with_alpha(colors.on_surface, disabled_alpha)
                } else {
                    Color::TRANSPARENT
                };
                
                ButtonStyling {
                    background: Background::Color(disabled_bg),
                    text_color: disabled_text,
                    border: create_button_border(disabled_border, config.border_width, config.border_radius),
                    shadow: None,
                    icon_color: Some(disabled_text),
                }
            },
            
            ButtonState::Focused => {
                let (focus_bg, focus_text, focus_icon) = if config.uses_surface_on_interaction {
                    let focus_bg = ColorUtils::darken(colors.surface_variant, 0.05);
                    (focus_bg, colors.on_surface_variant, colors.on_surface_variant)
                } else {
                    let focus_bg = ColorUtils::darken(config.base_background, 0.075);
                    (focus_bg, config.text_color, config.icon_color)
                };
                
                let focus_border = if config.border_width > 0.0 {
                    create_button_border(colors.primary.base, 2.0, config.border_radius) // Focus ring
                } else {
                    create_button_border(config.border_color, config.border_width, config.border_radius)
                };
                
                ButtonStyling {
                    background: Background::Color(focus_bg),
                    text_color: focus_text,
                    border: focus_border,
                    shadow: config.shadow.clone(),
                    icon_color: Some(focus_icon),
                }
            },
        }
    }
}
