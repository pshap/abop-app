//! Enhanced Material Design 3 Token System - Core Types
//!
//! This module contains the main EnhancedMaterialTokens struct and its factory methods
//! for creating light and dark theme token sets.

use crate::styling::material::unified_colors::MaterialColors;

/// Complete Material Design 3 token system
#[derive(Debug, Clone)]
pub struct EnhancedMaterialTokens {
    /// Core color tokens
    pub colors: MaterialColors,
    /// Typography tokens
    pub typography: TypographyTokens,
    /// Spacing tokens
    pub spacing: SpacingTokens,
    /// Elevation tokens
    pub elevation: ElevationTokens,
    /// Shape tokens
    pub shapes: ShapeTokens,
    /// Motion tokens
    pub motion: MotionTokens,
    /// State layer tokens
    pub state_layers: StateLayerTokens,
}

impl EnhancedMaterialTokens {
    /// Create tokens for light theme
    pub fn light() -> Self {
        Self {
            colors: MaterialColors::light_default(),
            typography: TypographyTokens::default(),
            spacing: SpacingTokens::default(),
            elevation: ElevationTokens::default(),
            shapes: ShapeTokens::default(),
            motion: MotionTokens::default(),
            state_layers: StateLayerTokens::light(),
        }
    }

    /// Create tokens for dark theme
    pub fn dark() -> Self {
        Self {
            colors: MaterialColors::dark_default(),
            typography: TypographyTokens::default(),
            spacing: SpacingTokens::default(),
            elevation: ElevationTokens::default(),
            shapes: ShapeTokens::default(),
            motion: MotionTokens::default(),
            state_layers: StateLayerTokens::light(),
        }
    }
}
