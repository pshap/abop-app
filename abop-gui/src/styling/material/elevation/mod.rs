//! Material Design 3 Elevation System
//!
//! Implements the complete Material Design 3 elevation system including:
//! - Six elevation levels (0-5) with corresponding shadow values
//! - Shadow calculations and color blending
//! - Integration with Iced shadow system
//! - Strong type safety with newtypes
//! - Trait-based extensibility
//! - Cache-optimized theme-aware context
//! - Serialization support

pub use self::{
    builder::ElevationStyleBuilder, constants::*, context::ElevationContext,
    registry::ElevationRegistry, utils::ComponentType,
};

pub mod builder;
pub mod color_blending;
pub mod constants;
pub mod context;
pub mod registry;
pub mod shadow_calculations;
pub mod utils;

pub mod serde_impl;

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
mod tests;

// Re-export core types from the original file
use crate::styling::material::colors::MaterialColors;
use iced::{Color, Shadow, Vector};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Float comparison epsilon for elevation calculations
const FLOAT_EPSILON: f32 = f32::EPSILON * 4.0;

/// Strong newtype for density-independent pixels (dp)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Dp(pub f32);

impl fmt::Display for Dp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}dp", self.0)
    }
}

impl Dp {
    /// Get the underlying f32 value
    #[must_use]
    pub const fn as_f32(&self) -> f32 {
        self.0
    }

    /// Clamp the dp value between min and max
    #[must_use]
    pub const fn clamp(self, min: f32, max: f32) -> Self {
        Self(self.0.clamp(min, max))
    }
}

/// Strong newtype for opacity (0.0-1.0)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Opacity(pub f32);

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl Opacity {
    /// Get the underlying f32 value
    #[must_use]
    pub const fn as_f32(&self) -> f32 {
        self.0
    }

    /// Clamp the opacity value between min and max
    #[must_use]
    pub const fn clamp(self, min: f32, max: f32) -> Self {
        Self(self.0.clamp(min, max))
    }
}

/// Error type for elevation system
#[derive(Debug, thiserror::Error)]
pub enum ElevationError {
    /// Invalid elevation level provided
    #[error("Invalid elevation level: {0}")]
    InvalidLevel(u8),
    /// Custom elevation not found in registry
    #[error("Custom elevation not found: {0}")]
    CustomNotFound(String),
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Trait for extensible elevation support
pub trait Elevatable {
    /// Get the elevation level for this component
    fn elevation_level(&self) -> ElevationLevel;

    /// Get optional custom elevation key
    fn custom_elevation_key(&self) -> Option<&'static str> {
        None
    }
}

/// Example Elevatable implementation for a custom component
pub struct ExampleComponent;

impl Elevatable for ExampleComponent {
    fn elevation_level(&self) -> ElevationLevel {
        ElevationLevel::Level2
    }
}

/// Wrapper for `iced::Color` to make it hashable
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorKey(pub Color);

impl std::hash::Hash for ColorKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the RGBA components as u32 for consistency
        (self.0.r.to_bits()).hash(state);
        (self.0.g.to_bits()).hash(state);
        (self.0.b.to_bits()).hash(state);
        (self.0.a.to_bits()).hash(state);
    }
}

impl Eq for ColorKey {}

impl From<Color> for ColorKey {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl From<ColorKey> for Color {
    fn from(key: ColorKey) -> Self {
        key.0
    }
}

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

/// Material Design elevation levels
///
/// Defines the six standard elevation levels used in Material Design 3.
/// Each level corresponds to a specific distance in dp and has associated
/// shadow and surface tint properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElevationLevel {
    /// Surface level with no elevation (0dp)
    Level0 = 0,
    /// Raised surfaces with subtle elevation (1dp)
    Level1 = 1,
    /// Slightly raised surfaces (3dp)
    Level2 = 2,
    /// Floating action buttons and similar components (6dp)
    Level3 = 3,
    /// App bars and prominent surfaces (8dp)
    Level4 = 4,
    /// Navigation drawers and modal dialogs (12dp)
    Level5 = 5,
}

impl ElevationLevel {
    /// Get all elevation levels
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Level0,
            Self::Level1,
            Self::Level2,
            Self::Level3,
            Self::Level4,
            Self::Level5,
        ]
    }

    /// Get the DP (density-independent pixel) value for this level
    #[must_use]
    pub const fn dp(&self) -> f32 {
        match self {
            Self::Level0 => 0.0,
            Self::Level1 => 1.0,
            Self::Level2 => 3.0,
            Self::Level3 => 6.0,
            Self::Level4 => 8.0,
            Self::Level5 => 12.0,
        }
    }

    /// Get the level as a u8
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Create from u8 value
    #[must_use]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Level0),
            1 => Some(Self::Level1),
            2 => Some(Self::Level2),
            3 => Some(Self::Level3),
            4 => Some(Self::Level4),
            5 => Some(Self::Level5),
            _ => None,
        }
    }

    /// Get the level name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Level0 => "level0",
            Self::Level1 => "level1",
            Self::Level2 => "level2",
            Self::Level3 => "level3",
            Self::Level4 => "level4",
            Self::Level5 => "level5",
        }
    }
}

impl std::fmt::Display for ElevationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Level {} ({}dp)", self.as_u8(), self.dp())
    }
}

/// Material Design 3 elevation levels
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialElevation {
    /// Level 0 - Surface level, no shadow (0dp)
    pub level0: ElevationStyle,
    /// Level 1 - Raised surfaces, subtle shadow (1dp)
    pub level1: ElevationStyle,
    /// Level 2 - Slightly raised surfaces, small shadow (3dp)
    pub level2: ElevationStyle,
    /// Level 3 - Floating action buttons, medium shadow (6dp)
    pub level3: ElevationStyle,
    /// Level 4 - App bars, prominent shadow (8dp)
    pub level4: ElevationStyle,
    /// Level 5 - Navigation drawers and modal dialogs, large shadow (12dp)
    pub level5: ElevationStyle,
}

impl MaterialElevation {
    /// Create elevation system with Material Design colors
    #[must_use]
    pub fn new(colors: &MaterialColors) -> Self {
        let shadow_color = colors.shadow;
        let tint_color = colors.surface_tint;

        Self::with_colors(shadow_color, tint_color)
    }

    /// Create elevation system with custom colors
    #[must_use]
    pub fn with_colors(shadow_color: Color, tint_color: Color) -> Self {
        let styles = Self::create_all_styles(shadow_color, tint_color);
        Self {
            level0: styles[0].clone(),
            level1: styles[1].clone(),
            level2: styles[2].clone(),
            level3: styles[3].clone(),
            level4: styles[4].clone(),
            level5: styles[5].clone(),
        }
    }

    /// Get elevation style by level
    #[must_use]
    pub const fn get_level(&self, level: ElevationLevel) -> &ElevationStyle {
        let idx = level.as_u8() as usize;
        let arr = self.as_array();
        if idx < arr.len() { arr[idx] } else { arr[0] }
    }
    /// Get mutable elevation style by level
    pub const fn get_level_mut(&mut self, level: ElevationLevel) -> &mut ElevationStyle {
        match level.as_u8() {
            1 => &mut self.level1,
            2 => &mut self.level2,
            3 => &mut self.level3,
            4 => &mut self.level4,
            5 => &mut self.level5,
            _ => &mut self.level0, // 0 and any invalid values
        }
    }

    /// Helper: get all elevation styles as array
    const fn as_array(&self) -> [&ElevationStyle; 6] {
        [
            &self.level0,
            &self.level1,
            &self.level2,
            &self.level3,
            &self.level4,
            &self.level5,
        ]
    }

    /// Helper: get all elevation styles as mutable array
    #[allow(dead_code)]
    const fn as_array_mut(&mut self) -> [&mut ElevationStyle; 6] {
        [
            &mut self.level0,
            &mut self.level1,
            &mut self.level2,
            &mut self.level3,
            &mut self.level4,
            &mut self.level5,
        ]
    }

    /// Helper function to create all elevation styles with given colors
    fn create_all_styles(shadow_color: Color, tint_color: Color) -> [ElevationStyle; 6] {
        [
            ElevationStyle::new(ElevationLevel::Level0, shadow_color, tint_color),
            ElevationStyle::new(ElevationLevel::Level1, shadow_color, tint_color),
            ElevationStyle::new(ElevationLevel::Level2, shadow_color, tint_color),
            ElevationStyle::new(ElevationLevel::Level3, shadow_color, tint_color),
            ElevationStyle::new(ElevationLevel::Level4, shadow_color, tint_color),
            ElevationStyle::new(ElevationLevel::Level5, shadow_color, tint_color),
        ]
    }

    /// Helper function to scale a single elevation style
    #[must_use]
    pub fn scale_elevation_style(style: &ElevationStyle, scale: f32) -> ElevationStyle {
        ElevationStyle {
            shadow: Shadow {
                offset: Vector::new(style.shadow.offset.x * scale, style.shadow.offset.y * scale),
                blur_radius: style.shadow.blur_radius * scale,
                ..style.shadow
            },
            ..style.clone()
        }
    }

    /// Scale all elevation values by a factor
    #[must_use]
    pub fn with_scale(&self, scale: f32) -> Self {
        let scale = scale.max(constants::MIN_SCALE_FACTOR);

        Self {
            level0: Self::scale_elevation_style(&self.level0, scale),
            level1: Self::scale_elevation_style(&self.level1, scale),
            level2: Self::scale_elevation_style(&self.level2, scale),
            level3: Self::scale_elevation_style(&self.level3, scale),
            level4: Self::scale_elevation_style(&self.level4, scale),
            level5: Self::scale_elevation_style(&self.level5, scale),
        }
    }

    /// Create elevated surface color for a given level
    #[must_use]
    pub fn elevated_surface(
        &self,
        base_surface: Color,
        level: ElevationLevel,
        tint_color: Color,
    ) -> Color {
        let elevation_style = self.get_level(level);
        elevation_style.apply_surface_tint(base_surface, tint_color)
    }

    /// Get all elevation styles as an iterator
    pub fn iter(&self) -> impl Iterator<Item = (ElevationLevel, &ElevationStyle)> {
        ElevationLevel::all()
            .iter()
            .map(|&level| (level, self.get_level(level)))
    }

    /// Create elevated surface color using the stored surface tint color
    #[must_use]
    pub fn elevated_surface_with_stored_tint(
        &self,
        base_surface: Color,
        level: ElevationLevel,
        colors: &MaterialColors,
    ) -> Color {
        self.elevated_surface(base_surface, level, colors.surface_tint)
    }

    /// Apply elevation styling to get both shadow and surface color
    #[must_use]
    pub fn get_elevation_styling(
        &self,
        base_surface: Color,
        level: ElevationLevel,
        tint_color: Color,
    ) -> (Shadow, Color) {
        let style = self.get_level(level);
        let elevated_surface = style.apply_surface_tint(base_surface, tint_color);
        (style.shadow, elevated_surface)
    }

    /// Check if this elevation system matches another in terms of structure
    #[must_use]
    pub fn has_same_structure(&self, other: &Self) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|((level_a, style_a), (level_b, style_b))| {
                level_a == level_b
                    && style_a.level == style_b.level
                    && (style_a.dp - style_b.dp).abs() < FLOAT_EPSILON
                    && (style_a.tint_opacity - style_b.tint_opacity).abs() < FLOAT_EPSILON
                    && style_a.shadow.offset == style_b.shadow.offset
                    && (style_a.shadow.blur_radius - style_b.shadow.blur_radius).abs()
                        < FLOAT_EPSILON
            })
    }
}

impl Default for MaterialElevation {
    fn default() -> Self {
        // Create default colors for elevation
        let default_colors = crate::styling::material::colors::MaterialColors::default();
        Self::new(&default_colors)
    }
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
        if self.tint_opacity == 0.0 {
            return base_color;
        }

        // Blend the tint color with the base color
        Color {
            r: base_color
                .r
                .mul_add(1.0 - self.tint_opacity, tint_color.r * self.tint_opacity),
            g: base_color
                .g
                .mul_add(1.0 - self.tint_opacity, tint_color.g * self.tint_opacity),
            b: base_color
                .b
                .mul_add(1.0 - self.tint_opacity, tint_color.b * self.tint_opacity),
            a: base_color.a,
        }
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
            let level_f32 = (dp / 2.0).ceil();
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
