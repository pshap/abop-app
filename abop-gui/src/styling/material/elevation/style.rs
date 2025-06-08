//! Material Design 3 elevation style
//!
//! Defines the elevation style struct that contains all the visual properties
//! for a specific elevation level including shadow and surface tint.

use crate::styling::material::elevation::{
    color_blending, shadow_calculations, constants, level::ElevationLevel
};
use iced::{Color, Shadow, Vector};
// Serde traits will be used when serialization is needed

/// Shadow configuration parameters for Material Design elevation levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowParams {
    /// The vertical offset of the shadow in pixels
    pub offset_y: f32,
    /// The blur radius of the shadow in pixels
    pub blur_radius: f32,
    /// The opacity of the shadow (0.0-1.0)
    pub opacity: f32,
}

/// Individual elevation style with shadow properties
///
/// Represents a single elevation level with its associated visual properties
/// including shadow configuration and surface tint opacity for Material Design 3.
#[derive(Debug, Clone, PartialEq)]
pub struct ElevationStyle {
    /// The elevation level as a numeric value (0-5)
    pub level: u8,
    /// The elevation distance in density-independent pixels (dp)
    pub dp: f32,
    /// The shadow configuration for this elevation level
    pub shadow: Shadow,
    /// The opacity of surface tint to apply at this elevation level (0.0-1.0)
    pub tint_opacity: f32,
}

impl ElevationStyle {
    /// Create a new elevation style
    #[must_use]
    pub fn new(level: ElevationLevel, shadow_color: Color, _tint_color: Color) -> Self {
        let dp = level.dp();
        let tint_opacity = Self::calculate_tint_opacity(level);

        Self {
            level: level.as_u8(),
            dp,
            shadow: Self::calculate_shadow(level, shadow_color),
            tint_opacity,
        }
    }

    /// Calculate shadow for elevation level
    fn calculate_shadow(level: ElevationLevel, shadow_color: Color) -> Shadow {
        let dp = level.dp();

        if dp == 0.0 {
            return Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::ZERO,
                blur_radius: 0.0,
            };
        }

        // Get shadow parameters from constants
        let params = constants::SHADOW_PARAMS[level.as_u8() as usize];

        Shadow {
            color: Color {
                a: params.opacity,
                ..shadow_color
            },
            offset: Vector::new(0.0, params.offset_y),
            blur_radius: params.blur_radius,
        }
    }

    /// Calculate tint opacity for custom elevation value
    const fn calculate_tint_opacity(level: ElevationLevel) -> f32 {
        // Material Design 3 uses surface tint to indicate elevation
        // Higher elevation = more tint opacity
        constants::TINT_OPACITIES[level.as_u8() as usize]
    }

    /// Get the elevation level
    #[must_use]
    pub fn level(&self) -> ElevationLevel {
        debug_assert!(
            self.level <= constants::MAX_ELEVATION_LEVEL,
            "Invalid elevation level: {}",
            self.level
        );
        ElevationLevel::from_u8(self.level).unwrap_or_else(|| {
            eprintln!(
                "Warning: Invalid elevation level {}, using Level0",
                self.level
            );
            ElevationLevel::Level0
        })
    }

    /// Apply surface tint to a base color
    #[must_use]
    pub fn apply_surface_tint(&self, base_color: Color, tint_color: Color) -> Color {
        color_blending::apply_surface_tint(base_color, tint_color, self.tint_opacity)
    }

    /// Create a variant with different shadow color
    #[must_use]
    pub fn with_shadow_color(&self, shadow_color: Color) -> Self {
        Self {
            shadow: Shadow {
                color: Color {
                    a: self.shadow.color.a,
                    ..shadow_color
                },
                ..self.shadow
            },
            ..self.clone()
        }
    }

    /// Create a variant with different opacity
    #[must_use]
    pub fn with_opacity(&self, opacity: f32) -> Self {
        Self {
            shadow: Shadow {
                color: Color {
                    a: opacity.clamp(0.0, 1.0),
                    ..self.shadow.color
                },
                ..self.shadow
            },
            ..self.clone()
        }
    }

    /// Create an elevation style with custom shadow and tint parameters
    #[must_use]
    pub fn custom(
        dp: f32,
        shadow_color: Color,
        _tint_color: Color,
        custom_shadow_params: Option<ShadowParams>,
    ) -> Self {
        let level = if dp == 0.0 {
            0
        } else {
            let level_f32 = (dp / constants::LEVEL_CALCULATION_DIVISOR).ceil();
            if level_f32 <= 0.0 {
                0
            } else if level_f32 >= f32::from(constants::MAX_ELEVATION_LEVEL) {
                constants::MAX_ELEVATION_LEVEL
            } else {
                // Safe cast: we've checked bounds above
                level_f32 as u8
            }
        };

        let shadow = custom_shadow_params.map_or_else(
            || shadow_calculations::calculate_custom_shadow(dp, shadow_color),
            |params| Shadow {
                color: Color {
                    a: params.opacity,
                    ..shadow_color
                },
                offset: Vector::new(0.0, params.offset_y),
                blur_radius: params.blur_radius,
            },
        );

        let tint_opacity = shadow_calculations::calculate_custom_tint_opacity(dp);

        Self {
            level,
            dp,
            shadow,
            tint_opacity,
        }
    }

    /// Check if this elevation style represents no elevation
    #[must_use]
    pub fn is_flat(&self) -> bool {
        self.dp == 0.0 && self.level == 0
    }

    /// Get a description of this elevation level
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self.level() {
            ElevationLevel::Level0 => "Surface level (no elevation)",
            ElevationLevel::Level1 => "Raised surfaces (subtle shadow)",
            ElevationLevel::Level2 => "Slightly raised surfaces",
            ElevationLevel::Level3 => "Floating elements",
            ElevationLevel::Level4 => "App bars and prominent surfaces",
            ElevationLevel::Level5 => "Navigation and modal surfaces",
        }
    }

    /// Create a variant with custom tint opacity
    #[must_use]
    pub fn with_tint_opacity(&self, tint_opacity: f32) -> Self {
        Self {
            tint_opacity: tint_opacity.clamp(0.0, 1.0),
            ..self.clone()
        }
    }
}

impl std::fmt::Display for ElevationStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Elevation {} - {}", self.level(), self.description())
    }
}
