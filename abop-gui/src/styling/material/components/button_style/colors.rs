//! Button Color Utilities
//!
//! This module provides button-specific color calculations that work with
//! the centralized `MaterialColors` system. It follows the same pattern as
//! `SelectionColors` to maintain architectural consistency.

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::{MaterialColors, StateOpacity};
use iced::Color;

use super::ButtonStyleVariant;

/// Centralized color calculation for button components
///
/// This structure provides button-specific color logic while leveraging
/// the centralized `MaterialColors` system for consistency and maintainability.
#[derive(Debug, Clone)]
pub struct ButtonColors {
    /// The material color scheme to use
    pub colors: MaterialColors,
    /// State opacity values for interactions
    pub state_opacity: StateOpacity,
}

impl ButtonColors {
    /// Create new button colors with the given color scheme
    #[must_use]
    pub const fn new(colors: MaterialColors) -> Self {
        Self {
            colors,
            state_opacity: StateOpacity::new(),
        }
    }

    /// Get the background color for a button variant and state
    #[must_use]
    pub fn background_color(
        &self,
        variant: ButtonStyleVariant,
        is_hovered: bool,
        is_pressed: bool,
    ) -> Color {
        match variant {
            ButtonStyleVariant::Filled => {
                if is_pressed {
                    ColorUtils::with_alpha(self.colors.primary.base, 0.88) // pressed state
                } else if is_hovered {
                    self.apply_state_layer(self.colors.primary.base, self.state_opacity.hover)
                } else {
                    self.colors.primary.base
                }
            }
            ButtonStyleVariant::FilledTonal => {
                if is_pressed {
                    ColorUtils::with_alpha(self.colors.secondary.container, 0.88)
                } else if is_hovered {
                    self.apply_state_layer(
                        self.colors.secondary.container,
                        self.state_opacity.hover,
                    )
                } else {
                    self.colors.secondary.container
                }
            }
            ButtonStyleVariant::Outlined => {
                if is_pressed {
                    self.apply_state_layer(Color::TRANSPARENT, self.state_opacity.pressed)
                } else if is_hovered {
                    self.apply_state_layer(Color::TRANSPARENT, self.state_opacity.hover)
                } else {
                    Color::TRANSPARENT
                }
            }
            ButtonStyleVariant::Text => {
                if is_pressed {
                    self.apply_state_layer(Color::TRANSPARENT, self.state_opacity.pressed)
                } else if is_hovered {
                    self.apply_state_layer(Color::TRANSPARENT, self.state_opacity.hover)
                } else {
                    Color::TRANSPARENT
                }
            }
            ButtonStyleVariant::Elevated => {
                if is_pressed {
                    ColorUtils::with_alpha(self.colors.surface, 0.88)
                } else if is_hovered {
                    self.apply_state_layer(self.colors.surface, self.state_opacity.hover)
                } else {
                    self.colors.surface
                }
            }
            ButtonStyleVariant::Icon => {
                if is_pressed {
                    self.apply_state_layer(Color::TRANSPARENT, self.state_opacity.pressed)
                } else if is_hovered {
                    self.apply_state_layer(Color::TRANSPARENT, self.state_opacity.hover)
                } else {
                    Color::TRANSPARENT
                }
            }
            ButtonStyleVariant::Fab => {
                if is_pressed {
                    ColorUtils::with_alpha(self.colors.primary.container, 0.88)
                } else if is_hovered {
                    self.apply_state_layer(self.colors.primary.container, self.state_opacity.hover)
                } else {
                    self.colors.primary.container
                }
            }
        }
    }

    /// Get the text color for a button variant
    #[must_use]
    pub const fn text_color(&self, variant: ButtonStyleVariant) -> Color {
        match variant {
            ButtonStyleVariant::Filled => self.colors.primary.on_base,
            ButtonStyleVariant::FilledTonal => self.colors.secondary.on_container,
            ButtonStyleVariant::Outlined => self.colors.primary.base,
            ButtonStyleVariant::Text => self.colors.primary.base,
            ButtonStyleVariant::Elevated => self.colors.primary.base,
            ButtonStyleVariant::Icon => self.colors.on_surface_variant,
            ButtonStyleVariant::Fab => self.colors.primary.on_container,
        }
    }

    /// Get the border color for a button variant
    #[must_use]
    pub const fn border_color(&self, variant: ButtonStyleVariant) -> Color {
        match variant {
            ButtonStyleVariant::Outlined => self.colors.outline,
            _ => Color::TRANSPARENT,
        }
    }

    /// Get the disabled background color for a button variant
    #[must_use]
    pub const fn disabled_background_color(&self, variant: ButtonStyleVariant) -> Color {
        match variant {
            ButtonStyleVariant::Filled | ButtonStyleVariant::FilledTonal => {
                ColorUtils::with_alpha(self.colors.on_surface, self.state_opacity.disabled)
            }
            _ => Color::TRANSPARENT,
        }
    }

    /// Get the disabled text color for any button variant
    #[must_use]
    pub const fn disabled_text_color(&self) -> Color {
        ColorUtils::with_alpha(self.colors.on_surface, self.state_opacity.disabled)
    }

    /// Apply state layer effect to a base color
    #[must_use]
    fn apply_state_layer(&self, base: Color, opacity: f32) -> Color {
        if base == Color::TRANSPARENT {
            ColorUtils::with_alpha(self.colors.primary.base, opacity)
        } else {
            // Blend state layer on top of base color
            let state_layer = ColorUtils::with_alpha(self.colors.primary.base, opacity);
            self.blend_colors(base, state_layer)
        }
    }

    /// Blend two colors using alpha compositing
    #[must_use]
    fn blend_colors(&self, base: Color, overlay: Color) -> Color {
        let alpha = overlay.a;
        let inv_alpha = 1.0 - alpha;

        Color {
            r: base.r.mul_add(inv_alpha, overlay.r * alpha),
            g: base.g.mul_add(inv_alpha, overlay.g * alpha),
            b: base.b.mul_add(inv_alpha, overlay.b * alpha),
            a: overlay.a.mul_add(1.0 - base.a, base.a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::material::MaterialPalette;

    #[test]
    fn test_button_colors_creation() {
        let palette = MaterialPalette::default();
        let material_colors = MaterialColors::light(&palette);
        let button_colors = ButtonColors::new(material_colors.clone());

        // Test that we can create button colors
        assert_eq!(
            button_colors.colors.primary.base,
            material_colors.primary.base
        );
    }

    #[test]
    fn test_filled_button_colors() {
        let palette = MaterialPalette::default();
        let material_colors = MaterialColors::light(&palette);
        let button_colors = ButtonColors::new(material_colors.clone());

        // Test filled button uses primary colors
        let bg = button_colors.background_color(ButtonStyleVariant::Filled, false, false);
        let text = button_colors.text_color(ButtonStyleVariant::Filled);

        assert_eq!(bg, material_colors.primary.base);
        assert_eq!(text, material_colors.primary.on_base);
    }

    #[test]
    fn test_outlined_button_colors() {
        let palette = MaterialPalette::default();
        let material_colors = MaterialColors::light(&palette);
        let button_colors = ButtonColors::new(material_colors.clone());

        // Test outlined button has transparent background
        let bg = button_colors.background_color(ButtonStyleVariant::Outlined, false, false);
        let border = button_colors.border_color(ButtonStyleVariant::Outlined);

        assert_eq!(bg, Color::TRANSPARENT);
        assert_eq!(border, material_colors.outline);
    }

    #[test]
    fn test_disabled_colors() {
        let palette = MaterialPalette::default();
        let material_colors = MaterialColors::light(&palette);
        let button_colors = ButtonColors::new(material_colors.clone());

        let disabled_text = button_colors.disabled_text_color();

        // Disabled text should be on_surface with reduced opacity
        assert_eq!(disabled_text.r, material_colors.on_surface.r);
        assert!(disabled_text.a < material_colors.on_surface.a);
    }
}
