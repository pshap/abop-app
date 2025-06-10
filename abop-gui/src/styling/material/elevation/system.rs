//! Material Design 3 elevation system
//!
//! Defines the complete elevation system with all six elevation levels
//! and provides methods for creating, scaling, and manipulating elevation styles.

use crate::styling::material::{
    colors::MaterialColors,
    elevation::{constants, level::ElevationLevel, style::ElevationStyle},
};
use iced::{Color, Shadow, Vector};

/// Float comparison epsilon for elevation calculations
const FLOAT_EPSILON: f32 = f32::EPSILON * 4.0;

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
        // Use iterator to eliminate repetition and ensure consistency
        let styles: Vec<ElevationStyle> = ElevationLevel::all()
            .iter()
            .map(|&level| ElevationStyle::new(level, shadow_color, tint_color))
            .collect();

        // Convert to array - safe because we know ElevationLevel::all() returns exactly 6 items
        styles.try_into().unwrap_or_else(|_| {
            panic!(
                "ElevationLevel::all() must return exactly {} items",
                constants::ELEVATION_LEVEL_COUNT
            )
        })
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

        // Use iterator to eliminate repetition
        let scaled_styles: Vec<ElevationStyle> = self
            .iter()
            .map(|(_, style)| Self::scale_elevation_style(style, scale))
            .collect();

        // Convert back to MaterialElevation structure
        Self {
            level0: scaled_styles[0].clone(),
            level1: scaled_styles[1].clone(),
            level2: scaled_styles[2].clone(),
            level3: scaled_styles[3].clone(),
            level4: scaled_styles[4].clone(),
            level5: scaled_styles[5].clone(),
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
        let default_colors = MaterialColors::default();
        Self::new(&default_colors)
    }
}
