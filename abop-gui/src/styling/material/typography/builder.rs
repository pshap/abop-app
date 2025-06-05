//! Builder pattern for typography customization

use super::{
    roles::TypographyRole,
    scale::{MaterialTypography, TypeStyle},
};
use std::collections::HashMap;

/// Float comparison epsilon for typography calculations
const FLOAT_EPSILON: f32 = f32::EPSILON * 4.0;

/// Configuration for typography generation
#[derive(Debug, Clone)]
pub struct TypographyConfig {
    /// Global scale factor for all typography sizes
    pub scale_factor: f32,
    /// Custom overrides for specific roles
    pub custom_styles: HashMap<TypographyRole, TypeStyle>,
}

impl Default for TypographyConfig {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            custom_styles: HashMap::new(),
        }
    }
}

/// Builder for creating customized typography scales
pub struct TypographyBuilder {
    config: TypographyConfig,
}

impl TypographyBuilder {
    /// Create a new typography builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: TypographyConfig::default(),
        }
    }

    /// Set the global scale factor for all typography
    #[must_use]
    pub const fn with_scale(mut self, scale: f32) -> Self {
        self.config.scale_factor = scale;
        self
    }

    /// Override a specific typography role with a custom style
    #[must_use]
    pub fn with_custom_style(mut self, role: TypographyRole, style: TypeStyle) -> Self {
        self.config.custom_styles.insert(role, style);
        self
    }

    /// Override multiple typography roles at once
    #[must_use]
    pub fn with_custom_styles(mut self, styles: HashMap<TypographyRole, TypeStyle>) -> Self {
        self.config.custom_styles.extend(styles);
        self
    }

    /// Build the final typography scale
    #[must_use]
    pub fn build(self) -> MaterialTypography {
        let mut typography = MaterialTypography::new();

        // Apply global scaling
        if (self.config.scale_factor - 1.0).abs() >= FLOAT_EPSILON {
            typography = typography.with_scale(self.config.scale_factor);
        }

        // Apply custom overrides
        for (role, style) in self.config.custom_styles {
            typography = typography.with_style_override(role, style);
        }

        typography
    }
}

impl Default for TypographyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialTypography {
    /// Create a typography scale with the given configuration
    #[must_use]
    pub fn with_config(config: TypographyConfig) -> Self {
        TypographyBuilder { config }.build()
    }

    /// Create a builder for customizing typography
    #[must_use]
    pub fn builder() -> TypographyBuilder {
        TypographyBuilder::new()
    }

    /// Apply a style override for a specific role
    const fn with_style_override(mut self, role: TypographyRole, style: TypeStyle) -> Self {
        match role {
            TypographyRole::DisplayLarge => self.display_large = style,
            TypographyRole::DisplayMedium => self.display_medium = style,
            TypographyRole::DisplaySmall => self.display_small = style,
            TypographyRole::HeadlineLarge => self.headline_large = style,
            TypographyRole::HeadlineMedium => self.headline_medium = style,
            TypographyRole::HeadlineSmall => self.headline_small = style,
            TypographyRole::TitleLarge => self.title_large = style,
            TypographyRole::TitleMedium => self.title_medium = style,
            TypographyRole::TitleSmall => self.title_small = style,
            TypographyRole::LabelLarge => self.label_large = style,
            TypographyRole::LabelMedium => self.label_medium = style,
            TypographyRole::LabelSmall => self.label_small = style,
            TypographyRole::BodyLarge => self.body_large = style,
            TypographyRole::BodyMedium => self.body_medium = style,
            TypographyRole::BodySmall => self.body_small = style,
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::material::typography::font_mapping::{MaterialFont, MaterialWeight};

    #[test]
    fn test_builder_pattern() {
        let custom_style =
            TypeStyle::new(MaterialFont::Plain, MaterialWeight::Bold, 20.0, 28.0, 0.5);

        let typography = MaterialTypography::builder()
            .with_scale(1.2)
            .with_custom_style(TypographyRole::BodyLarge, custom_style.clone())
            .build();

        // Check that scaling was applied to other styles
        assert_eq!(typography.body_medium.size(), 14.0 * 1.2);

        // Check that custom override was applied
        assert_eq!(typography.body_large, custom_style);
    }

    #[test]
    fn test_config_creation() {
        let mut custom_styles = HashMap::new();
        custom_styles.insert(
            TypographyRole::HeadlineLarge,
            TypeStyle::new(MaterialFont::Brand, MaterialWeight::Bold, 36.0, 44.0, 0.0),
        );

        let config = TypographyConfig {
            scale_factor: 1.5,
            custom_styles,
        };

        let typography = MaterialTypography::with_config(config);
        assert_eq!(typography.headline_large.size(), 36.0);
    }
}
