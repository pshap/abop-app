//! Builder pattern for creating custom elevation styles

use super::{ElevationError, ElevationLevel, ElevationStyle, ShadowParams};
use iced::Color;

/// Validation result for builder operations
#[derive(Debug, Clone)]
pub enum ValidationResult {
    /// The configuration is valid
    Valid,
    /// Warning about the configuration
    Warning(String),
    /// Error in the configuration
    Error(String),
}

/// Builder pattern for creating custom elevation styles with validation
pub struct ElevationStyleBuilder {
    level: ElevationLevel,
    shadow_color: Option<Color>,
    tint_color: Option<Color>,
    custom_shadow: Option<ShadowParams>,
    custom_tint_opacity: Option<f32>,
    validation_enabled: bool,
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
            validation_enabled: true,
        }
    }

    /// Disable validation for this builder (use with caution)
    #[must_use]
    pub const fn without_validation(mut self) -> Self {
        self.validation_enabled = false;
        self
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

    /// Set custom shadow parameters with validation
    #[must_use]
    pub fn with_custom_shadow(mut self, params: ShadowParams) -> Self {
        if self.validation_enabled {
            // Validate shadow parameters
            if params.blur_radius < 0.0 || params.opacity < 0.0 || params.opacity > 1.0 {
                // For now, we'll just log a warning. In a full implementation,
                // you might want to return a Result type
                eprintln!("Warning: Invalid shadow parameters provided");
            }
        }
        self.custom_shadow = Some(params);
        self
    }

    /// Set custom tint opacity with validation
    #[must_use]
    pub fn with_custom_tint_opacity(mut self, opacity: f32) -> Self {
        if self.validation_enabled && !(0.0..=1.0).contains(&opacity) {
            eprintln!("Warning: Tint opacity should be between 0.0 and 1.0, got {opacity}");
        }
        self.custom_tint_opacity = Some(opacity.clamp(0.0, 1.0));
        self
    }

    /// Validate the current configuration
    pub fn validate(&self) -> ValidationResult {
        if !self.validation_enabled {
            return ValidationResult::Valid;
        }

        // Check custom shadow parameters
        if let Some(shadow) = &self.custom_shadow {
            if shadow.blur_radius < 0.0 {
                return ValidationResult::Error("Blur radius cannot be negative".to_string());
            }
            if shadow.opacity < 0.0 || shadow.opacity > 1.0 {
                return ValidationResult::Error(
                    "Shadow opacity must be between 0.0 and 1.0".to_string(),
                );
            }
            if shadow.blur_radius > 50.0 {
                return ValidationResult::Warning(
                    "Very large blur radius may impact performance".to_string(),
                );
            }
        }

        // Check custom tint opacity
        if let Some(opacity) = self.custom_tint_opacity
            && (!(0.0..=1.0).contains(&opacity))
        {
            return ValidationResult::Error("Tint opacity must be between 0.0 and 1.0".to_string());
        }

        ValidationResult::Valid
    }

    /// Build the elevation style with validation
    pub fn build(self) -> Result<ElevationStyle, ElevationError> {
        // Validate before building
        match self.validate() {
            ValidationResult::Error(msg) => {
                return Err(ElevationError::CustomNotFound(msg));
            }
            ValidationResult::Warning(msg) => {
                eprintln!("Warning during elevation style creation: {msg}");
            }
            ValidationResult::Valid => {}
        }

        let shadow_color = self.shadow_color.unwrap_or(Color::BLACK);
        let tint_color = self.tint_color.unwrap_or(Color::WHITE);

        let style = self.custom_shadow.map_or_else(
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
        );

        Ok(style)
    }

    /// Build the elevation style without validation (use with caution)
    pub fn build_unchecked(self) -> ElevationStyle {
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
        let style = ElevationStyleBuilder::new(ElevationLevel::Level2)
            .build()
            .unwrap();
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
            .build()
            .unwrap();

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
            .build()
            .unwrap();

        assert_eq!(style.shadow.offset.y, 4.0);
        assert_eq!(style.shadow.blur_radius, 8.0);
        assert_eq!(style.shadow.color.a, 0.25);
    }

    #[test]
    fn test_builder_with_custom_tint_opacity() {
        let style = ElevationStyleBuilder::new(ElevationLevel::Level2)
            .with_custom_tint_opacity(0.15)
            .build()
            .unwrap();

        assert_eq!(style.tint_opacity, 0.15);
    }

    #[test]
    fn test_builder_validation() {
        // Invalid shadow parameters
        let custom_shadow_invalid = ShadowParams {
            offset_y: 4.0,
            blur_radius: -1.0, // Invalid blur radius
            opacity: 0.25,
        };

        let style_invalid = ElevationStyleBuilder::new(ElevationLevel::Level1)
            .with_custom_shadow(custom_shadow_invalid)
            .build();

        assert!(style_invalid.is_err());

        // Check warning for large blur radius
        let custom_shadow_warning = ShadowParams {
            offset_y: 4.0,
            blur_radius: 60.0, // Very large blur radius
            opacity: 0.25,
        };

        let style_warning = ElevationStyleBuilder::new(ElevationLevel::Level1)
            .with_custom_shadow(custom_shadow_warning)
            .build_unchecked(); // Use unchecked build to allow creation

        assert_eq!(style_warning.level(), ElevationLevel::Level1);
    }
}
