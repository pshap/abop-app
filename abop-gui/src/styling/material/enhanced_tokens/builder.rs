//! Theme Builder
//!
//! This module provides a fluent builder pattern for creating
//! enhanced Material Design 3 token systems.

use super::*;

/// Builder for creating enhanced material token systems
pub struct ThemeBuilder {
    colors: Option<MaterialColors>,
    typography: Option<TypographyTokens>,
    spacing: Option<SpacingTokens>,
    elevation: Option<ElevationTokens>,
    shapes: Option<ShapeTokens>,
    motion: Option<MotionTokens>,
}

impl ThemeBuilder {
    /// Create a new theme builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            colors: None,
            typography: None,
            spacing: None,
            elevation: None,
            shapes: None,
            motion: None,
        }
    }

    /// Set custom colors
    #[must_use]
    pub const fn colors(mut self, colors: MaterialColors) -> Self {
        Self {
            colors: Some(colors),
            typography: self.typography,
            spacing: self.spacing,
            elevation: self.elevation,
            shapes: self.shapes,
            motion: self.motion,
        }
    }

    /// Set custom typography
    #[must_use]
    pub const fn typography(mut self, typography: TypographyTokens) -> Self {
        Self {
            colors: self.colors,
            typography: Some(typography),
            spacing: self.spacing,
            elevation: self.elevation,
            shapes: self.shapes,
            motion: self.motion,
        }
    }

    /// Set custom spacing
    #[must_use]
    pub const fn spacing(mut self, spacing: SpacingTokens) -> Self {
        Self {
            colors: self.colors,
            typography: self.typography,
            spacing: Some(spacing),
            elevation: self.elevation,
            shapes: self.shapes,
            motion: self.motion,
        }
    }

    /// Set custom elevation
    #[must_use]
    pub const fn elevation(mut self, elevation: ElevationTokens) -> Self {
        Self {
            colors: self.colors,
            typography: self.typography,
            spacing: self.spacing,
            elevation: Some(elevation),
            shapes: self.shapes,
            motion: self.motion,
        }
    }

    /// Set custom shapes
    #[must_use]
    pub const fn shapes(mut self, shapes: ShapeTokens) -> Self {
        Self {
            colors: self.colors,
            typography: self.typography,
            spacing: self.spacing,
            elevation: self.elevation,
            shapes: Some(shapes),
            motion: self.motion,
        }
    }

    /// Set custom motion
    #[must_use]
    pub const fn motion(mut self, motion: MotionTokens) -> Self {
        Self {
            colors: self.colors,
            typography: self.typography,
            spacing: self.spacing,
            elevation: self.elevation,
            shapes: self.shapes,
            motion: Some(motion),
        }
    }

    /// Build enhanced material tokens
    #[must_use]
    pub fn build(self) -> EnhancedMaterialTokens {
        EnhancedMaterialTokens {
            colors: self
                .colors
                .unwrap_or_else(|| MaterialColors::light_default()),
            typography: self.typography.unwrap_or_default(),
            spacing: self.spacing.unwrap_or_default(),
            elevation: self.elevation.unwrap_or_default(),
            shapes: self.shapes.unwrap_or_default(),
            motion: self.motion.unwrap_or_default(),
            state_layers: StateLayerTokens::light(),
        }
    }
}
