//! Builder pattern for creating custom elevation styles

use super::{ElevationLevel, ElevationStyle, ShadowParams};
use iced::Color;

/// Builder pattern for creating custom elevation styles
pub struct ElevationStyleBuilder {
    level: ElevationLevel,
    shadow_color: Option<Color>,
    tint_color: Option<Color>,
    custom_shadow: Option<ShadowParams>,
    custom_tint_opacity: Option<f32>,
}

impl ElevationStyleBuilder {
    /// Create a new builder for the given elevation level
    #[must_use]
    pub const fn new(level: ElevationLevel) -> Self {
        Self {
            level,
            shadow_color: None,
            tint_color: None,
            custom_shadow: None,
            custom_tint_opacity: None,
        }
    }

    /// Set the shadow color
    #[must_use]
    pub const fn with_shadow_color(mut self, color: Color) -> Self {
        self.shadow_color = Some(color);
        self
    }

    /// Set the tint color
    #[must_use]
    pub const fn with_tint_color(mut self, color: Color) -> Self {
        self.tint_color = Some(color);
        self
    }

    /// Set custom shadow parameters
    #[must_use]
    pub const fn with_custom_shadow(mut self, params: ShadowParams) -> Self {
        self.custom_shadow = Some(params);
        self
    }

    /// Set custom tint opacity
    #[must_use]
    pub const fn with_custom_tint_opacity(mut self, opacity: f32) -> Self {
        self.custom_tint_opacity = Some(opacity);
        self
    }

    /// Build the elevation style
    #[must_use]
    pub fn build(self) -> ElevationStyle {
        let shadow_color = self.shadow_color.unwrap_or(Color::BLACK);
        let tint_color = self.tint_color.unwrap_or(Color::WHITE);

        self.custom_shadow.map_or_else(
            || {
                // Create standard style
                let mut style = ElevationStyle::new(self.level, shadow_color, tint_color);

                // Override tint opacity if provided
                if let Some(tint_opacity) = self.custom_tint_opacity {
                    style.tint_opacity = tint_opacity;
                }

                style
            },
            |shadow_params| {
                // Create custom style with custom shadow parameters
                let mut style = ElevationStyle::custom(
                    self.level.dp(),
                    shadow_color,
                    tint_color,
                    Some(shadow_params),
                );

                // Override tint opacity if provided
                if let Some(tint_opacity) = self.custom_tint_opacity {
                    style.tint_opacity = tint_opacity;
                }

                style
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let style = ElevationStyleBuilder::new(ElevationLevel::Level2).build();
        assert_eq!(style.level(), ElevationLevel::Level2);
        assert_eq!(style.dp, 3.0);
    }

    #[test]
    fn test_builder_with_colors() {
        let shadow_color = Color::from_rgb(0.2, 0.2, 0.2);
        let tint_color = Color::from_rgb(0.8, 0.8, 0.8);

        let style = ElevationStyleBuilder::new(ElevationLevel::Level3)
            .with_shadow_color(shadow_color)
            .with_tint_color(tint_color)
            .build();

        assert_eq!(style.shadow.color.r, shadow_color.r);
        assert_eq!(style.shadow.color.g, shadow_color.g);
        assert_eq!(style.shadow.color.b, shadow_color.b);
    }

    #[test]
    fn test_builder_with_custom_shadow() {
        let custom_shadow = ShadowParams {
            offset_y: 4.0,
            blur_radius: 8.0,
            opacity: 0.25,
        };

        let style = ElevationStyleBuilder::new(ElevationLevel::Level1)
            .with_custom_shadow(custom_shadow)
            .build();

        assert_eq!(style.shadow.offset.y, 4.0);
        assert_eq!(style.shadow.blur_radius, 8.0);
        assert_eq!(style.shadow.color.a, 0.25);
    }

    #[test]
    fn test_builder_with_custom_tint_opacity() {
        let style = ElevationStyleBuilder::new(ElevationLevel::Level2)
            .with_custom_tint_opacity(0.15)
            .build();

        assert_eq!(style.tint_opacity, 0.15);
    }
}
